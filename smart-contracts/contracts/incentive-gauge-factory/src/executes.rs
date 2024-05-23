use cosmwasm_std::{Addr, BankMsg, Decimal, DepsMut, Env, Response};

use crate::{state::GAUGES, ContractError};

pub fn create_incentive_gauge() -> Result<Response, ContractError> {
    // instantiate an instance of the incentive gauge

    // save the instance in the Gauge overview

    // depending on the gauge type, execute verification, eg verify that a vault is a quasar cl vault

    todo!()
}

pub fn claim_fees(deps: DepsMut, env: Env, gauge_addr: Addr) -> Result<Response, ContractError> {
    let mut gauge = GAUGES.load(deps.storage, gauge_addr.clone())?;

    let mut fees = gauge.fee;

    let elapsed_blocks = env.block.height - gauge.start_block;
    let total_blocks = gauge.end_block - gauge.start_block;

    // calculate what % of the gauge has passed
    let elapsed_ratio = Decimal::from_ratio(elapsed_blocks, total_blocks);
    let claimable_until_now = fees.total.mul_ratio(elapsed_ratio);

    let claimed = fees.total.checked_sub(&fees.remaining)?;

    // calculate the difference between what fees were already paid out and what is claimable
    let to_receive = claimable_until_now.clone() - claimed;
    let new_remaining_fees = fees.total.checked_sub(&claimable_until_now)?;

    fees.remaining = new_remaining_fees;
    gauge.fee = fees.clone();
    GAUGES.save(deps.storage, gauge_addr, &gauge)?;

    let bank_msg = BankMsg::Send {
        to_address: fees.address.to_string(),
        amount: to_receive.coins(),
    };

    Ok(Response::new()
        .add_attribute("action", "claim_fees")
        .add_message(bank_msg))
}
