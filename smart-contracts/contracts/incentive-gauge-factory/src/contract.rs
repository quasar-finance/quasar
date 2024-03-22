#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, Addr, BankMsg, Binary, Decimal, Deps, DepsMut, Env, Fraction, MessageInfo, Response, StdError,
};
// use cw2::set_contract_version;

use crate::error::{ContractError, ContractResult};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateIncentiveGauge { r#type, gauge } => todo!(),
        ExecuteMsg::ClaimGaugeFees { gauge_address } => todo!(),
    }
}

pub fn execute_create_incentive_gauge() -> Result<Response, ContractError> {
    // instantiate an instance of the incentive gauge

    // save the instance in the Gauge overview

    todo!()
}

pub fn execute_claim_fees(
    deps: DepsMut,
    env: Env,
    gauge_addr: Addr,
) -> Result<Response, ContractError> {
    let current_block = env.block.height;

    let mut gauge = GAUGES.load(deps.storage, gauge_addr.clone())?;

    let mut fees = gauge.fee;

    let elapsed_blocks = env.block.height - gauge.start_block;
    let total_blocks = gauge.end_block - gauge.start_block;
    let elapsed_ratio = Decimal::from_ratio(elapsed_blocks, total_blocks);
    let claimable_until_now = fees
        .total_fees
        .mul_ratio(elapsed_ratio);

    // TODO remove clones
    let claimed = fees.total_fees.checked_sub(&fees.remaining_fees).map_err(StdError::overflow)?;

    let to_receive = claimable_until_now.clone() - claimed;
    let new_remaining_fees = fees.total_fees.checked_sub(&claimable_until_now).map_err(StdError::overflow)?;

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
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    match msg {}
}

#[cfg(test)]
mod tests {}
