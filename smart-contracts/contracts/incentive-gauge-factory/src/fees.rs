use cosmwasm_std::{BankMsg, Decimal, DepsMut, Env, MessageInfo, Response};

use crate::{
    ContractError,
    msg::FeeMsg,
    state::{ADMIN, GAUGES, GAUGE_FEES}, types::Fee,
};

pub fn handle_execute_fee(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: FeeMsg,
) -> Result<Response, ContractError> {
    match msg {
        FeeMsg::Distribute { addr } => distribute(deps, env, addr),
        FeeMsg::Update { addr, fees } => update(deps, info, addr, fees),
    }
}

fn update(
    deps: DepsMut,
    info: MessageInfo,
    addr: String,
    fees: Fee
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let contract_addr = deps.api.addr_validate(&addr)?;

    if GAUGE_FEES.has(deps.storage, contract_addr.clone()) {
        GAUGE_FEES.save(deps.storage, contract_addr, &fees)?;
        return Ok(Response::default().add_attribute("action", "fee_update"))
    }

    Err(ContractError::NoSuchGauge { addr })
}

// Note: by removing the fees from the gauge the read cost remains the same but the write cost is reduced
fn distribute(deps: DepsMut, env: Env, gauge_addr: String) -> Result<Response, ContractError> {
    let gauge_addr = deps.api.addr_validate(&gauge_addr)?;

    let gauge = GAUGES.load(deps.storage, gauge_addr.clone())?;

    let mut fees = GAUGE_FEES.load(deps.storage, gauge_addr.clone())?;

    println!("{:#?}", env.block.height);
    println!("{:#?}", gauge.period.start);

    let elapsed_time = env.block.height - gauge.period.start;
    let total_time = gauge.period.end - gauge.period.start;

    // calculate what % of the gauge has passed
    let elapsed_ratio = Decimal::from_ratio(elapsed_time, total_time);
    let claimable_until_now = fees.total.mul_ratio(elapsed_ratio);

    let claimed = fees.total.checked_sub(&fees.remaining)?;

    // calculate the difference between what fees were already paid out and what is claimable
    let to_receive = claimable_until_now.clone() - claimed;
    let new_remaining_fees = fees.total.checked_sub(&claimable_until_now)?;

    fees.remaining = new_remaining_fees;

    GAUGE_FEES.save(deps.storage, gauge_addr, &fees)?;

    Ok(Response::default()
        .add_attribute("action", "fee_distribute")
        .add_message(BankMsg::Send {
            to_address: fees.reciever.to_string(),
            amount: to_receive.coins(),
        }))
}
