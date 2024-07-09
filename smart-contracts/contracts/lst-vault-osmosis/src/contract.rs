use std::vec;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, PendingResponse, QueryMsg};
use crate::state::{Claim, Config, PENDING, STATE};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_json_binary, BankMsg, BankQuery, Binary, CustomQuery, Decimal, Deps, DepsMut, Env,
    MessageInfo, QueryRequest, Reply, Response, StdError, StdResult, SubMsg, Uint128, WasmMsg,
};
use cosmwasm_std::{Order, SupplyResponse};
use cw2::set_contract_version;
use lst_adapter_osmosis::msg::{LstAdapterExecuteMsg, LstAdapterQueryMsg};
use lst_dex_adapter_osmosis::msg::DexAdapterExecuteMsg as DexExecuteMsg;
use osmosis_std::types::cosmos::base::v1beta1::Coin as ProtoCoin;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};
use quasar_types::abstract_sdk::ExecuteMsg as AbstractExecuteMsg;
use quasar_types::error::assert_funds_single_token;
use quasar_types::query::query_contract_balance;

const CONTRACT_NAME: &str = "quasar:quasar-lst-vault-osmosis";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const SUBDENOM: &str = "uqlst";
pub const SWAP_REPLY_ID: u64 = 1;

pub fn get_factory_denom(addr: &str, subdenom: &str) -> String {
    format!("factory/{}/{}", addr, subdenom)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    STATE.save(
        deps.storage,
        &Config {
            dex_adapter: deps.api.addr_validate(&msg.dex_adapter)?,
            lst_adapter: deps.api.addr_validate(&msg.lst_adapter)?,
            deposit_denom: msg.deposit_denom,
            lst_denom: msg.lst_denom,
            denom: get_factory_denom(env.contract.address.as_ref(), SUBDENOM),
            unbonding_time_seconds: msg.unbonding_time_seconds,
        },
    )?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_message(MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: SUBDENOM.to_string(),
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
        ExecuteMsg::Swap { amount, slippage } => swap(deps, env, amount, slippage),
    }
}

fn get_supply<C: CustomQuery>(deps: &Deps<C>, denom: String) -> StdResult<Uint128> {
    let response: SupplyResponse = deps
        .querier
        .query(&QueryRequest::<C>::Bank(BankQuery::Supply { denom }))?;
    Ok(response.amount.amount)
}

fn vault_tokens(deps: Deps, env: &Env) -> StdResult<Uint128> {
    let config = STATE.load(deps.storage)?;
    let contract_balance = query_contract_balance(&deps.querier, env, &config.deposit_denom)?;
    let lst_adapter_balance = deps.querier.query_wasm_smart::<Uint128>(
        config.lst_adapter.to_string(),
        &LstAdapterQueryMsg::BalanceInUnderlying {},
    )?;
    Ok(contract_balance + lst_adapter_balance)
}

fn deposit(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = STATE.load(deps.storage)?;
    assert_funds_single_token(&info.funds, &config.deposit_denom)?;

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
    let config = STATE.load(deps.storage)?;
    assert_funds_single_token(&info.funds, &config.denom)?;

    let contract_address = env.contract.address.to_string();
    let contract_total = vault_tokens(deps.as_ref(), &env)?;
    let existing_shares = get_supply(&deps.as_ref(), config.denom.clone())?;
    let claim_amount =
        contract_total * Decimal::from_ratio(info.funds[0].amount.clone(), existing_shares);
    PENDING.update(
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

    Ok(Response::default().add_message(MsgBurn {
        sender: contract_address.clone(),
        amount: Some(ProtoCoin {
            amount: info.funds[0].amount.to_string(),
            denom: config.denom,
        }),
        burn_from_address: contract_address,
    }))
}

fn claim(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = STATE.load(deps.storage)?;
    let pending = PENDING.may_load(deps.storage, info.sender.clone())?;
    if let Some(mut pending) = pending {
        let claimable: Uint128 = pending
            .iter()
            .filter(|claim| claim.expiration <= env.block.time)
            .map(|claim| claim.amount)
            .sum();
        if !claimable.is_zero() {
            pending = pending
                .into_iter()
                .filter(|claim| claim.expiration > env.block.time)
                .collect();
            if pending.is_empty() {
                PENDING.remove(deps.storage, info.sender.clone());
            } else {
                PENDING.save(deps.storage, info.sender.clone(), &pending)?;
            }
            let available = query_contract_balance(&deps.querier, &env, &config.deposit_denom)?;
            let mut response = Response::default();
            if available < claimable {
                let claimable_from_adapter = deps.querier.query_wasm_smart::<Uint128>(
                    config.lst_adapter.to_string(),
                    &LstAdapterQueryMsg::Claimable {},
                )?;
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
                amount: coins(claimable.into(), config.denom),
            }));
        }
    }
    Err(ContractError::NothingToClaim {})
}

fn claim_unbonded(deps: DepsMut) -> Result<Response, ContractError> {
    let config = STATE.load(deps.storage)?;
    let claimable_from_adapter = deps.querier.query_wasm_smart::<Uint128>(
        config.lst_adapter.to_string(),
        &LstAdapterQueryMsg::Claimable {},
    )?;
    if claimable_from_adapter.is_zero() {
        return Err(ContractError::NothingToClaim {});
    }
    Ok(Response::default().add_message(WasmMsg::Execute {
        contract_addr: config.lst_adapter.to_string(),
        msg: to_json_binary(&LstAdapterExecuteMsg::Claim {})?,
        funds: vec![],
    }))
}

fn get_blocked_funds(deps: Deps, env: Env) -> StdResult<Uint128> {
    let pending: StdResult<Vec<_>> = PENDING
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let pending = pending?;
    let blocked = pending
        .iter()
        .map(|(_, claims)| -> Uint128 {
            claims
                .iter()
                .filter(|claim| claim.expiration <= env.block.time)
                .map(|claim| claim.amount)
                .sum()
        })
        .sum();
    Ok(blocked)
}

fn swap(
    deps: DepsMut,
    env: Env,
    amount: Uint128,
    slippage: Option<Decimal>,
) -> Result<Response, ContractError> {
    let config = STATE.load(deps.storage)?;
    let contract_balance = query_contract_balance(&deps.querier, &env, &config.deposit_denom)?;
    // TODO: consider claimable tokens from lst adapter, possibly do some kind of bucketing.
    // let claimable = deps.querier
    let blocked = get_blocked_funds(deps.as_ref(), env)?;
    if contract_balance < blocked + amount {
        return Err(ContractError::InsufficientFunds {});
    }
    Ok(Response::default().add_submessage(SubMsg::reply_on_success(
        WasmMsg::Execute {
            contract_addr: config.dex_adapter.to_string(),
            msg: to_json_binary(&AbstractExecuteMsg::Module {
                module: DexExecuteMsg::Swap { slippage },
            })?,
            funds: coins(amount.into(), config.deposit_denom),
        },
        SWAP_REPLY_ID,
    )))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&STATE.load(deps.storage)?),
        QueryMsg::Pending { address } => to_json_binary(&query_pending(deps, address)?),
    }
}

fn query_pending(deps: Deps, address: String) -> StdResult<PendingResponse> {
    let claims = PENDING.may_load(deps.storage, deps.api.addr_validate(&address)?)?;
    Ok(PendingResponse {
        pending: claims.unwrap_or_default(),
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        SWAP_REPLY_ID => {
            let config = STATE.load(deps.storage)?;
            let contract_balance = query_contract_balance(&deps.querier, &env, &config.lst_denom)?;
            let mut response = Response::default();
            if !contract_balance.is_zero() {
                response = response.add_message(WasmMsg::Execute {
                    contract_addr: config.lst_adapter.to_string(),
                    msg: to_json_binary(&LstAdapterExecuteMsg::Unbond {})?,
                    funds: coins(contract_balance.into(), config.lst_denom),
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
