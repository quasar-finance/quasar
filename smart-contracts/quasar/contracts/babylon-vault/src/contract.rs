use crate::error::{assert_deposit_funds, assert_withdraw_funds, VaultError};
use crate::msg::{ExecuteMsg, InstantiateMsg, LstInfo, OracleQueryMsg, QueryMsg};
use crate::state::{LSTS, OWNER, VAULT_DENOM};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, BankQuery, Binary, Coin, CustomQuery, Decimal, Deps, DepsMut,
    Env, MessageInfo, Order, QueryRequest, Reply, Response, StdResult, Storage, SubMsg,
    SupplyResponse, Uint128,
};
use cw2::set_contract_version;
use quasar_std::quasarlabs::quasarnode::tokenfactory::v1beta1::{
    MsgBurn, MsgCreateDenom, MsgCreateDenomResponse, MsgMint,
};
use std::collections::HashMap;

const CONTRACT_NAME: &str = "quasar:babylon-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type VaultResult<T = Response> = Result<T, VaultError>;

pub(crate) const CREATE_DENOM_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> VaultResult {
    OWNER.initialize(
        deps.storage,
        deps.api,
        mars_owner::OwnerInit::SetInitialOwner { owner: msg.owner },
    )?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let msg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: msg.subdenom,
    };
    Ok(Response::new().add_submessage(SubMsg::reply_on_success(msg, CREATE_DENOM_REPLY_ID)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> VaultResult {
    match reply.id {
        CREATE_DENOM_REPLY_ID => {
            let response: MsgCreateDenomResponse = reply.result.try_into()?;
            VAULT_DENOM.save(deps.storage, &response.new_token_denom)?;

            Ok(Response::new().add_attribute("vault_denom", response.new_token_denom))
        }
        _ => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> VaultResult {
    match msg {
        ExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
        ExecuteMsg::RegisterLst { denom, interface } => register_lst(deps, info, denom, interface),
        ExecuteMsg::UnregisterLst { denom } => unregister_lst(deps, info, denom),
        ExecuteMsg::Deposit {} => deposit(deps, env, info),
        ExecuteMsg::Withdraw {} => withdraw(deps, env, info),
        _ => Ok(Response::default()),
    }
}

fn register_lst(deps: DepsMut, info: MessageInfo, denom: String, interface: String) -> VaultResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    LSTS.save(deps.storage, denom, &deps.api.addr_validate(&interface)?)?;
    Ok(Response::default())
}

fn unregister_lst(deps: DepsMut, info: MessageInfo, denom: String) -> VaultResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    let interface = LSTS.may_load(deps.storage, denom.clone())?;
    if interface.is_some() {
        LSTS.remove(deps.storage, denom);
        Ok(Response::default())
    } else {
        Err(VaultError::DenomNotFound { denom })
    }
}

fn get_supply<C: CustomQuery>(deps: &Deps<C>, denom: String) -> StdResult<Uint128> {
    let response: SupplyResponse = deps
        .querier
        .query(&QueryRequest::<C>::Bank(BankQuery::Supply { denom }))?;
    Ok(response.amount.amount)
}

fn get_lst_denoms(storage: &dyn Storage) -> StdResult<Vec<String>> {
    LSTS.keys(storage, None, None, Order::Ascending).collect()
}

fn get_prices(deps: &Deps, denoms: &[String]) -> VaultResult<HashMap<String, Decimal>> {
    let denoms_with_prices: StdResult<Vec<_>> = denoms
        .iter()
        .map(|denom| -> StdResult<(String, Decimal)> {
            let price = deps.querier.query_wasm_smart::<Decimal>(
                "oracle",
                &OracleQueryMsg::Price {
                    denom: denom.clone(),
                },
            )?;
            Ok((denom.clone(), price))
        })
        .collect();
    let denoms_with_prices = denoms_with_prices?;
    Ok(denoms_with_prices.into_iter().collect())
}

fn get_balances(deps: &Deps, address: String, denoms: &[String]) -> StdResult<Vec<Coin>> {
    denoms
        .iter()
        .map(|denom| -> StdResult<Coin> {
            deps.querier.query_balance(address.clone(), denom.clone())
        })
        .collect()
}

fn vault_value(balances: &[Coin], prices: &HashMap<String, Decimal>) -> VaultResult<Uint128> {
    let values: Result<Vec<Uint128>, _> = balances
        .iter()
        .map(|balance| -> VaultResult<Uint128> {
            let value = balance
                .amount
                .checked_mul_floor(*prices.get(&balance.denom).unwrap())?;
            Ok(value)
        })
        .collect();
    let value = values?.iter().sum();
    Ok(value)
}

fn deposit(deps: DepsMut, env: Env, info: MessageInfo) -> VaultResult {
    assert_deposit_funds(deps.storage, &info.funds)?;

    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let existing_shares = get_supply(&deps.as_ref(), vault_denom.clone())?;

    let contract_address = env.contract.address.to_string();
    let lst_denoms = get_lst_denoms(deps.storage)?;
    let prices = get_prices(&deps.as_ref(), &lst_denoms)?;
    let balances = get_balances(&deps.as_ref(), contract_address.clone(), &lst_denoms)?;
    let contract_value = vault_value(&balances, &prices)?;
    let deposit_value = info.funds[0]
        .amount
        .checked_mul_floor(*prices.get(&info.funds[0].denom).unwrap())?;

    let new_shares = if existing_shares.is_zero() {
        info.funds[0].amount
    } else {
        existing_shares.checked_mul_floor(Decimal::from_ratio(
            deposit_value,
            contract_value.checked_sub(deposit_value)?,
        ))?
    };

    Ok(Response::default().add_message(MsgMint {
        sender: contract_address,
        amount: Some(cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
            amount: new_shares.to_string(),
            denom: vault_denom,
        }),
        mint_to_address: info.sender.to_string(),
    }))
}

fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> VaultResult {
    assert_withdraw_funds(deps.storage, &info.funds)?;

    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let existing_shares = get_supply(&deps.as_ref(), vault_denom.clone())?;
    let lst_denoms = get_lst_denoms(deps.storage)?;
    let contract_address = env.contract.address.to_string();
    let balances = get_balances(&deps.as_ref(), contract_address.clone(), &lst_denoms)?;
    let claim_ratio = Decimal::from_ratio(info.funds[0].amount.clone(), existing_shares);

    let claim_funds: Result<_, _> = balances
        .into_iter()
        .map(|balance| -> VaultResult<Coin> {
            Ok(Coin {
                amount: balance.amount.checked_mul_floor(claim_ratio)?,
                denom: balance.denom,
            })
        })
        .collect();

    let burn_msg = MsgBurn {
        sender: contract_address.clone(),
        amount: Some(cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
            amount: info.funds[0].amount.to_string(),
            denom: info.funds[0].denom.clone(),
        }),
        burn_from_address: contract_address.clone(),
    };
    let send_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: claim_funds?,
    };
    Ok(Response::default()
        .add_message(burn_msg)
        .add_message(send_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> VaultResult<Binary> {
    match msg {
        QueryMsg::Owner {} => Ok(to_json_binary(&OWNER.query(deps.storage)?)?),
        QueryMsg::Lsts {} => Ok(to_json_binary(&query_lsts(deps)?)?),
        QueryMsg::Denom {} => Ok(to_json_binary(&VAULT_DENOM.load(deps.storage)?)?),
        QueryMsg::Value {} => Ok(to_json_binary(&query_value(deps, env)?)?),
    }
}

fn query_lsts(deps: Deps) -> StdResult<Vec<LstInfo>> {
    let lsts: StdResult<Vec<(String, Addr)>> = LSTS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let lsts = lsts?;
    let infos: Vec<LstInfo> = lsts
        .into_iter()
        .map(|(denom, interface)| -> LstInfo {
            LstInfo {
                denom,
                interface: interface.to_string(),
            }
        })
        .collect();
    Ok(infos)
}

fn query_value(deps: Deps, env: Env) -> VaultResult<Uint128> {
    let lst_denoms = get_lst_denoms(deps.storage)?;
    let prices = get_prices(&deps, &lst_denoms)?;
    let balances = get_balances(&deps, env.contract.address.to_string(), &lst_denoms)?;
    vault_value(&balances, &prices)
}
