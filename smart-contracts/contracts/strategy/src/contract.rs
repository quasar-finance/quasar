#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::error::ContractError::PaymentError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::queue::{dequeue, enqueue};
use crate::state::{WithdrawRequest, OUTSTANDING_FUNDS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-4626";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const LOGO_SIZE_CAP: usize = 5 * 1024;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;

    // TODO fill in the instantiation
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // TODO decide if we want to do something with deposit
        ExecuteMsg::Deposit { .. } => execute_deposit(deps, env, info),
        ExecuteMsg::WithdrawRequest { .. } => {
            todo!()
        }
    }
}

pub fn execute_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // do things here with the funds. This is where the actual strategy starts placing funds on a new deposit
    // in the template version of the contract, all we do is check that funds are present
    if info.funds.is_empty() {
        return Err(PaymentError(cw_utils::PaymentError::NoFunds {}));
    }
    // if funds are sent to an outside contract, OUTSTANDING funds should be updated here
    // TODO add some more sensible attributes here
    Ok(Response::new().add_attribute("deposit", info.sender))
}

pub fn execute_withdraw_request(
    mut deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    owner: String,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    enqueue(
        deps.branch(),
        WithdrawRequest {
            denom,
            amount,
            owner,
        },
    )?;
    let res = try_withdraw(deps, env)?;
    Ok(res)
}

fn try_withdraw(mut deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let withdraw = dequeue(deps.branch());
    if withdraw.is_none() {
        return Err(ContractError::QueueError(
            "dequeue was none while queue should be some".to_string(),
        ));
    }
    let w = withdraw.unwrap();
    // check the free balance of the contract
    let free_balance = deps
        .querier
        .query_balance(env.contract.address, w.denom.clone())
        .map_err(|error| ContractError::Std(error))?;
    // if the contract has enough free balance, execute the withdraw
    if w.amount <= free_balance.amount {
        // remove the peeked withdraw request
        do_withdraw(w)
    } else {
        // else we start to unlock funds and return a response
        // TODO determine whether we need to dequeue the withdraw at this point or a later point
        unlock_funds(deps, w)
    }
}

// do_withdraw sends funds from the contract to the owner of the funds according to the withdraw request
fn do_withdraw(withdraw: WithdrawRequest) -> Result<Response, ContractError> {
    let msg = BankMsg::Send {
        to_address: withdraw.owner.clone(),
        amount: coins(withdraw.amount.u128(), withdraw.denom.clone()),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("withdraw", "executed")
        .add_attribute("amount", withdraw.amount)
        .add_attribute("denom", withdraw.denom)
        .add_attribute("owner", withdraw.owner))
}

fn unlock_funds(deps: DepsMut, withdraw: WithdrawRequest) -> Result<Response, ContractError> {
    // TODO this is where funds are locked or not present within the strategy contract. The withdraw happens 'async' here
    // the strategy needs to know where the funds are located, unlock the funds there(or do so)
    let outstanding = OUTSTANDING_FUNDS.load(deps.storage)?;
    if withdraw.amount > outstanding {
        return Err(ContractError::InsufficientOutStandingFunds);
    }
    todo!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::SubMsg;

    const DENOM: &str = "satoshi";
    const CREATOR: &str = "creator";
    const INVESTOR: &str = "investor";
    const BUYER: &str = "buyer";

    fn default_instantiate(
        supply_decimals: u8,
        reserve_decimals: u8,
        reserve_supply: Uint128,
    ) -> InstantiateMsg {
        InstantiateMsg {}
    }

    fn setup_test(
        deps: DepsMut,
        supply_decimals: u8,
        reserve_decimals: u8,
        reserve_supply: Uint128,
    ) {
    }

    #[test]
    fn deposit_works() {
        let mut deps = mock_dependencies();
        let info = mock_info("alice", &coins(100_000, "uqsar"));
        execute_deposit(deps.as_mut(), mock_env(), info).unwrap();
    }

    #[test]
    fn withdraw_with_sufficient_funds_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        deps.querier
            .update_balance(env.clone().contract.address, coins(100_000, "uqsar"));
        let res = execute_withdraw_request(
            deps.as_mut(),
            env,
            mock_info("alice", &[]),
            "alice".into(),
            "uqsar".to_string(),
            Uint128::new(100_000),
        )
        .unwrap();
        assert_eq!(res.messages.len(), 1);
        assert_eq!(
            res.messages[0],
            SubMsg::new(BankMsg::Send {
                to_address: "alice".to_string(),
                amount: coins(100_000, "uqsar")
            })
        )
    }
}
