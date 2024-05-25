use cosmwasm_std::{DepsMut, Reply, Response};
// use cw_utils::{parse_reply_execute_data, parse_reply_instantiate_data};
use cw_utils::parse_reply_instantiate_data;

use crate::{state::{GAUGES, GAUGE_FEES, GAUGE_IN_PROCESS, GAUGE_KINDS}, ContractError};

pub const REPLY_ON_GAUGE_INIT: u64 = 0;

pub fn gauge_init_success(deps: DepsMut, reply: Reply) -> Result<Response, ContractError> {
    let res = parse_reply_instantiate_data(reply);

    if res.is_err() {
        return Err(res.unwrap_err().into());
    }

    // after this point we are sure that the gauge contract started correctly
    let res = res.unwrap();
    let gauge_address = deps.api.addr_validate(&res.contract_address)?;

    let gauge = GAUGE_IN_PROCESS.load(deps.storage)?;

    GAUGES.save(deps.storage, gauge_address.clone(), &gauge.gauge)?;
    GAUGE_KINDS.save(deps.storage, gauge_address.clone(), &gauge.kind)?;
    GAUGE_FEES.save(deps.storage, gauge_address, &gauge.fee)?;

    Ok(Response::default())
}