use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

use crate::state::{NewRange, PENDING_RANGES};

#[cw_serde]
#[derive(QueryResponses)]
pub enum RangeQueryMsg {
    // Get the pending ranges
    #[returns(Vec<NewRange>)]
    GetQueuedRangeUpdates {},
    // Get the pending ranges for a specific contract
    #[returns(NewRange)]
    GetQueuedRangeUpdatesForContract { contract_address: String },
}

pub fn query_range(deps: Deps, _env: Env, query_msg: RangeQueryMsg) -> StdResult<Binary> {
    match query_msg {
        RangeQueryMsg::GetQueuedRangeUpdates {} => get_queued_range_updates(deps),
        RangeQueryMsg::GetQueuedRangeUpdatesForContract { contract_address } => {
            get_queued_range_updates_for_contract(deps, contract_address)
        }
    }
}

pub fn get_queued_range_updates(deps: Deps) -> StdResult<Binary> {
    let mut pending_ranges = vec![];

    for pending_range in
        PENDING_RANGES.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
    {
        let (_, pending_range) = pending_range?;
        pending_ranges.push(pending_range);
    }

    to_json_binary(&pending_ranges)
}

pub fn get_queued_range_updates_for_contract(
    deps: Deps,
    contract_address: String,
) -> StdResult<Binary> {
    let pending_range =
        PENDING_RANGES.load(deps.storage, deps.api.addr_validate(&contract_address)?)?;

    to_json_binary(&pending_range)
}
