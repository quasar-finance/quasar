use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

use crate::state::{RangeUpdates, PENDING_RANGES};

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::QueryFns;

#[cw_serde]
#[cfg_attr(not(target_arch = "wasm32"), derive(QueryFns))]
#[derive(QueryResponses)]
pub enum RangeQueryMsg {
    // Get the pending ranges
    #[returns(Vec<RangeUpdates>)]
    GetQueuedRangeUpdates {},
    // Get the pending ranges for a specific contract
    #[returns(RangeUpdates)]
    GetQueuedRangeUpdatesForContract { contract_address: String },
}

pub fn query_range(deps: Deps, _env: Env, query_msg: RangeQueryMsg) -> StdResult<Binary> {
    match query_msg {
        RangeQueryMsg::GetQueuedRangeUpdates {} => to_json_binary(&get_queued_range_updates(deps)?),
        RangeQueryMsg::GetQueuedRangeUpdatesForContract { contract_address } => to_json_binary(
            &get_queued_range_updates_for_contract(deps, contract_address)?,
        ),
    }
}

pub fn get_queued_range_updates(deps: Deps) -> StdResult<Vec<RangeUpdates>> {
    let mut pending_ranges = vec![];

    for pending_range in
        PENDING_RANGES.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
    {
        let (_, pending_range) = pending_range?;
        pending_ranges.push(pending_range);
    }

    Ok(pending_ranges)
}

pub fn get_queued_range_updates_for_contract(
    deps: Deps,
    contract_address: String,
) -> StdResult<RangeUpdates> {
    let pending_range =
        PENDING_RANGES.may_load(deps.storage, deps.api.addr_validate(&contract_address)?)?;

    Ok(pending_range.unwrap_or(RangeUpdates {
        cl_vault_address: contract_address,
        updates: Default::default(),
    }))
}
