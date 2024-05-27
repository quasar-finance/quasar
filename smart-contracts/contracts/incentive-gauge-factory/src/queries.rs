use cosmwasm_std::{Addr, Deps, Order, StdResult};

use cw_storage_plus::Bound;

use crate::{
    state::{GAUGES, GAUGE_FEES, GAUGE_KINDS},
    types::{Fee, Gauge, GaugeKind, GaugeListResponse},
};

pub const PAGINATION_MAX_LIMIT: u32 = 100;
pub const PAGINATION_DEFAULT_LIMIT: u32 = 32;

pub fn query_gauge(deps: Deps, addr: Addr) -> StdResult<Gauge> {
    GAUGES.load(deps.storage, addr)
}

pub fn query_gauge_list(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<GaugeListResponse> {
    let limit = limit
        .unwrap_or(PAGINATION_DEFAULT_LIMIT)
        .min(PAGINATION_MAX_LIMIT) as usize;

    let start_bound = match start_after {
        Some(addr) => {
            let addr = deps.api.addr_validate(&addr)?;
            Some(Bound::exclusive(addr.clone()))
        }
        None => None,
    };

    let gauges = GAUGES
        .range(deps.storage, start_bound.clone(), None, Order::Ascending)
        .take(limit)
        .map(|x| x.unwrap().1)
        .collect::<Vec<Gauge>>();

    let kinds = GAUGE_KINDS
        .range(deps.storage, start_bound.clone(), None, Order::Ascending)
        .take(limit)
        .map(|x| x.unwrap().1)
        .collect::<Vec<GaugeKind>>();

    let fees = GAUGE_FEES
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(limit)
        .map(|x| x.unwrap().1)
        .collect::<Vec<Fee>>();

    Ok(GaugeListResponse {
        gauges,
        kinds,
        fees,
    })
}
