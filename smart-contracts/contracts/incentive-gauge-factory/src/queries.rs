use cosmwasm_std::{Addr, Deps, Order, StdResult};

use cw_storage_plus::Bound;

use crate::{gauge::Gauge, state::GAUGES};

pub const PAGINATION_MAX_LIMIT: u32 = 100;
pub const PAGINATION_DEFAULT_LIMIT: u32 = 32;

pub fn query_gauge(
    deps: Deps,
    addr: Addr,
) -> StdResult<Gauge> {
    Ok(GAUGES.load(deps.storage, addr)?)
}

pub fn query_gauge_list(
    deps: Deps,
    start_after: Option<Addr>,
    limit: Option<u32>,
) -> StdResult<Vec<Gauge>> {
    let limit = limit
        .unwrap_or(PAGINATION_DEFAULT_LIMIT)
        .min(PAGINATION_MAX_LIMIT) as usize;

    let start_bound = match start_after {
        Some(addr) => {
            Some(Bound::exclusive(addr.clone()))
        }
        None => None,
    };

    Ok(GAUGES
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(limit)
        .map(|x| x.unwrap().1)
        .collect::<Vec<Gauge>>())
}