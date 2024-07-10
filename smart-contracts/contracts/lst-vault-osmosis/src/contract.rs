use std::vec;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Claim, Config, CONFIG, OWNER, PENDING_CLAIMS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_json_binary, Addr, BankMsg, BankQuery, Binary, CustomQuery, Decimal, Deps, DepsMut,
    Env, MessageInfo, QuerierWrapper, QueryRequest, Reply, Response, StdError, StdResult, SubMsg,
    Uint128, WasmMsg,
};
use cosmwasm_std::{Order, SupplyResponse};
use cw2::set_contract_version;
use lst_adapter_osmosis::msg::{LstAdapterExecuteMsg, LstAdapterQueryMsg};
use lst_adapter_osmosis::state::UnbondInfo;
use lst_dex_adapter_osmosis::msg::DexAdapterExecuteMsg as DexExecuteMsg;
use osmosis_std::types::cosmos::base::v1beta1::Coin as ProtoCoin;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};
use quasar_types::abstract_sdk::ExecuteMsg as AbstractExecuteMsg;
use quasar_types::denoms::{get_factory_denom, LstDenom};
use quasar_types::error::assert_funds_single_token;
use quasar_types::query::query_contract_balance;

const CONTRACT_NAME: &str = "quasar:quasar-lst-vault-osmosis";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const SWAP_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER.initialize(
        deps.storage,
        deps.api,
        mars_owner::OwnerInit::SetInitialOwner { owner: msg.owner },
    )?;
    CONFIG.save(
        deps.storage,
        &Config {
            dex_adapter: deps.api.addr_validate(&msg.dex_adapter)?,
            lst_adapter: deps.api.addr_validate(&msg.lst_adapter)?,
            lst_denom: msg.lst_denom,
            denom: get_factory_denom(env.contract.address.as_ref(), &msg.subdenom),
            unbonding_time_seconds: msg.unbonding_time_seconds,
        },
    )?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_message(MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: msg.subdenom,
    }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => deposit(deps, env, info),
        ExecuteMsg::Withdraw {} => withdraw(deps, env, info),
        ExecuteMsg::Claim {} => claim(deps, env, info),
        ExecuteMsg::ClaimUnbonded {} => claim_unbonded(deps),
        ExecuteMsg::Swap { amount, slippage } => swap(deps, env, info, amount, slippage),
        ExecuteMsg::Update {
            dex_adapter,
            lst_adapter,
            lst_denom,
            unbonding_time_seconds,
        } => update(
            deps,
            info,
            dex_adapter,
            lst_adapter,
            lst_denom,
            unbonding_time_seconds,
        ),
        ExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
    }
}

fn get_supply<C: CustomQuery>(deps: &Deps<C>, denom: String) -> StdResult<Uint128> {
    let response: SupplyResponse = deps
        .querier
        .query(&QueryRequest::<C>::Bank(BankQuery::Supply { denom }))?;
    Ok(response.amount.amount)
}

fn vault_tokens(deps: Deps, env: &Env) -> StdResult<Uint128> {
    let config = CONFIG.load(deps.storage)?;
    let contract_balance =
        query_contract_balance(&deps.querier, env, &config.lst_denom.underlying)?;
    let lst_adapter_balance = deps.querier.query_wasm_smart::<Uint128>(
        config.lst_adapter.to_string(),
        &LstAdapterQueryMsg::BalanceInUnderlying {},
    )?;
    Ok(contract_balance + lst_adapter_balance)
}

fn deposit(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_funds_single_token(&info.funds, &config.lst_denom.underlying)?;

    let existing_shares = get_supply(&deps.as_ref(), config.denom.clone())?;
    let contract_total = vault_tokens(deps.as_ref(), &env)?;

    let new_shares = if existing_shares.is_zero() {
        info.funds[0].amount
    } else {
        existing_shares * Decimal::from_ratio(info.funds[0].amount, contract_total)
    };

    Ok(Response::default().add_message(MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(ProtoCoin {
            amount: new_shares.to_string(),
            denom: config.denom,
        }),
        mint_to_address: info.sender.to_string(),
    }))
}

fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_funds_single_token(&info.funds, &config.denom)?;

    let contract_total = vault_tokens(deps.as_ref(), &env)?;
    let existing_shares = get_supply(&deps.as_ref(), config.denom.clone())?;
    let claim_amount = contract_total * Decimal::from_ratio(info.funds[0].amount, existing_shares);
    PENDING_CLAIMS.update(
        deps.storage,
        info.sender,
        |entry| -> Result<_, ContractError> {
            let new_claim = Claim {
                amount: claim_amount,
                expiration: env.block.time.plus_seconds(config.unbonding_time_seconds),
            };

            let mut claims = entry.unwrap_or_default();
            claims.push(new_claim);
            Ok(claims)
        },
    )?;

    let contract_address = env.contract.address.to_string();
    Ok(Response::default().add_message(MsgBurn {
        sender: contract_address.clone(),
        amount: Some(ProtoCoin {
            amount: info.funds[0].amount.to_string(),
            denom: config.denom,
        }),
        burn_from_address: contract_address,
    }))
}

fn get_claimable_from_adapter(querier: &QuerierWrapper<'_>, adapter: &Addr) -> StdResult<Uint128> {
    querier.query_wasm_smart::<Uint128>(adapter.to_string(), &LstAdapterQueryMsg::Claimable {})
}

fn claim(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let pending = PENDING_CLAIMS.may_load(deps.storage, info.sender.clone())?;
    if let Some(mut pending) = pending {
        let claimable: Uint128 = pending
            .iter()
            .filter(|claim| claim.expiration <= env.block.time)
            .map(|claim| claim.amount)
            .sum();
        if !claimable.is_zero() {
            pending.retain(|claim| claim.expiration > env.block.time);
            if pending.is_empty() {
                PENDING_CLAIMS.remove(deps.storage, info.sender.clone());
            } else {
                PENDING_CLAIMS.save(deps.storage, info.sender.clone(), &pending)?;
            }
            let available =
                query_contract_balance(&deps.querier, &env, &config.lst_denom.underlying)?;
            let mut response = Response::default();
            if available < claimable {
                let claimable_from_adapter =
                    get_claimable_from_adapter(&deps.querier, &config.lst_adapter)?;
                if available + claimable_from_adapter < claimable {
                    return Err(ContractError::InsufficientFunds {});
                } else {
                    response = response.add_message(WasmMsg::Execute {
                        contract_addr: config.lst_adapter.to_string(),
                        msg: to_json_binary(&LstAdapterExecuteMsg::Claim {})?,
                        funds: vec![],
                    })
                }
            }
            return Ok(response.add_message(BankMsg::Send {
                to_address: info.sender.to_string(),
                amount: coins(claimable.into(), config.lst_denom.underlying),
            }));
        }
    }
    Err(ContractError::NothingToClaim {})
}

fn claim_unbonded(deps: DepsMut) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let claimable_from_adapter = get_claimable_from_adapter(&deps.querier, &config.lst_adapter)?;
    if claimable_from_adapter.is_zero() {
        return Err(ContractError::NothingToClaim {});
    }
    Ok(Response::default().add_message(WasmMsg::Execute {
        contract_addr: config.lst_adapter.to_string(),
        msg: to_json_binary(&LstAdapterExecuteMsg::Claim {})?,
        funds: vec![],
    }))
}

fn get_blocked_amount(deps: Deps) -> StdResult<Uint128> {
    let pending: StdResult<Vec<_>> = PENDING_CLAIMS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let mut pending: Vec<Claim> = pending?
        .into_iter()
        .flat_map(|(_, claims)| claims)
        .collect();
    pending.sort_by(|a, b| a.expiration.cmp(&b.expiration));
    let config = CONFIG.load(deps.storage)?;
    let mut pending_unbonds: Vec<UnbondInfo> = deps
        .querier
        .query_wasm_smart(config.lst_adapter, &LstAdapterQueryMsg::PendingUnbonds {})?;
    pending_unbonds.sort_by(|a, b| a.unbond_start.cmp(&b.unbond_start));
    let mut idx = 0;
    let mut blocked = Uint128::zero();
    for claim in &pending {
        let mut amount = claim.amount;
        loop {
            if idx >= pending_unbonds.len() {
                break;
            }
            let unbond_info = &mut pending_unbonds[idx];
            if unbond_info
                .unbond_start
                .plus_seconds(config.unbonding_time_seconds)
                .plus_hours(3)
                < claim.expiration
            {
                if amount <= unbond_info.amount {
                    unbond_info.amount -= amount;
                    amount = Uint128::zero();
                    break;
                } else {
                    amount -= unbond_info.amount;
                    unbond_info.amount = Uint128::zero();
                    idx += 1;
                }
            } else {
                break;
            }
        }

        blocked += amount;
    }
    Ok(blocked)
}

fn swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    slippage: Option<Decimal>,
) -> Result<Response, ContractError> {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    let config = CONFIG.load(deps.storage)?;
    let swappable = query_swappable(deps.as_ref(), env)?;
    if swappable < amount {
        return Err(ContractError::InsufficientFunds {});
    }
    Ok(Response::default().add_submessage(SubMsg::reply_on_success(
        WasmMsg::Execute {
            contract_addr: config.dex_adapter.to_string(),
            msg: to_json_binary(&AbstractExecuteMsg::Module {
                module: DexExecuteMsg::Swap { slippage },
            })?,
            funds: coins(amount.into(), config.lst_denom.underlying),
        },
        SWAP_REPLY_ID,
    )))
}

fn update(
    deps: DepsMut,
    info: MessageInfo,
    dex_adapter: Option<String>,
    lst_adapter: Option<String>,
    lst_denom: Option<LstDenom>,
    unbonding_time_seconds: Option<u64>,
) -> Result<Response, ContractError> {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    let dex_adapter = if let Some(dex_adapter) = dex_adapter {
        Some(deps.api.addr_validate(&dex_adapter)?)
    } else {
        None
    };
    let lst_adapter = if let Some(lst_adapter) = lst_adapter {
        Some(deps.api.addr_validate(&lst_adapter)?)
    } else {
        None
    };
    CONFIG.update(deps.storage, |mut config| -> StdResult<Config> {
        config.dex_adapter = dex_adapter.unwrap_or(config.dex_adapter);
        config.lst_adapter = lst_adapter.unwrap_or(config.lst_adapter);
        config.lst_denom = lst_denom.unwrap_or(config.lst_denom);
        config.unbonding_time_seconds =
            unbonding_time_seconds.unwrap_or(config.unbonding_time_seconds);
        Ok(config)
    })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let binary = match msg {
        QueryMsg::Config {} => to_json_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::Pending { address } => to_json_binary(&query_pending(deps, address)?),
        QueryMsg::Claimable { address } => to_json_binary(&query_claimable(deps, env, address)?),
        QueryMsg::Swappable {} => to_json_binary(&query_swappable(deps, env)?),
    }?;
    Ok(binary)
}

fn query_pending(deps: Deps, address: String) -> StdResult<Vec<Claim>> {
    let claims = PENDING_CLAIMS.may_load(deps.storage, deps.api.addr_validate(&address)?)?;
    Ok(claims.unwrap_or_default())
}

fn query_claimable(deps: Deps, env: Env, address: String) -> StdResult<Uint128> {
    let claims = PENDING_CLAIMS.may_load(deps.storage, deps.api.addr_validate(&address)?)?;
    Ok(claims
        .unwrap_or_default()
        .iter()
        .filter(|claim| claim.expiration <= env.block.time)
        .map(|claim| claim.amount)
        .sum())
}

fn query_swappable(deps: Deps, env: Env) -> Result<Uint128, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let contract_balance =
        query_contract_balance(&deps.querier, &env, &config.lst_denom.underlying)?;
    let blocked = get_blocked_amount(deps)?;
    Ok(contract_balance.checked_sub(blocked)?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        SWAP_REPLY_ID => {
            let config = CONFIG.load(deps.storage)?;
            let contract_balance =
                query_contract_balance(&deps.querier, &env, &config.lst_denom.denom)?;
            let mut response = Response::default();
            if !contract_balance.is_zero() {
                response = response.add_message(WasmMsg::Execute {
                    contract_addr: config.lst_adapter.to_string(),
                    msg: to_json_binary(&LstAdapterExecuteMsg::Unbond {})?,
                    funds: coins(contract_balance.into(), config.lst_denom.denom),
                });
            }
            Ok(response)
        }
        id => Err(ContractError::Std(StdError::generic_err(format!(
            "unknown reply id: {}",
            id
        )))),
    }
}
