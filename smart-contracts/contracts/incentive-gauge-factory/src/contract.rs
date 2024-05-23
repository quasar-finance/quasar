#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::queries;
use crate::state::GAUGES;
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:multihop-router";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", CONTRACT_NAME)
        .add_attribute("version", CONTRACT_VERSION))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateIncentiveGauge { kind: _, gauge: _ } => todo!(),
        ExecuteMsg::ClaimGaugeFees { gauge_address: _ } => todo!(),
    }
}

pub fn execute_create_incentive_gauge() -> Result<Response, ContractError> {
    // instantiate an instance of the incentive gauge

    // save the instance in the Gauge overview

    // depending on the gauge type, execute verification, eg verify that a vault is a quasar cl vault

    todo!()
}

pub fn execute_claim_fees(
    deps: DepsMut,
    env: Env,
    gauge_addr: Addr,
) -> Result<Response, ContractError> {
    let mut gauge = GAUGES.load(deps.storage, gauge_addr.clone())?;

    let mut fees = gauge.fee;

    let elapsed_blocks = env.block.height - gauge.start_block;
    let total_blocks = gauge.end_block - gauge.start_block;

    // calculate what % of the gauge has passed
    let elapsed_ratio = Decimal::from_ratio(elapsed_blocks, total_blocks);
    let claimable_until_now = fees.total_fees.mul_ratio(elapsed_ratio);

    let claimed = fees
        .total_fees
        .checked_sub(&fees.remaining_fees)
        .map_err(StdError::overflow)?;

    // calculate the difference between what fees were already paid out and what is claimable
    let to_receive = claimable_until_now.clone() - claimed;
    let new_remaining_fees = fees
        .total_fees
        .checked_sub(&claimable_until_now)
        .map_err(StdError::overflow)?;

    fees.remaining_fees = new_remaining_fees;
    gauge.fee = fees.clone();
    GAUGES.save(deps.storage, gauge_addr, &gauge)?;

    let bank_msg = BankMsg::Send {
        to_address: fees.fee_address.to_string(),
        amount: to_receive.coins(),
    };

    Ok(Response::new().add_message(bank_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Gauge { address } => to_json_binary(&queries::query_gauge(
            deps,
            deps.api.addr_validate(&address)?,
        )?),
        QueryMsg::ListGauges { start_after, limit } => {
            to_json_binary(&queries::query_gauge_list(deps, start_after, limit)?)
        }
    }
}

#[cfg(test)]
mod tests {}
