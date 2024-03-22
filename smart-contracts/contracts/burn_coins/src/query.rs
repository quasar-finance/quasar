use cosmwasm_std::{Deps, StdResult};

use crate::msg::TotalBurntResponse;
use crate::state::AMOUNT_BURNT;

pub fn query_total_burn(deps: Deps) -> StdResult<TotalBurntResponse> {
    let amount_burnt = AMOUNT_BURNT.load(deps.storage)?;
    Ok(TotalBurntResponse {
        amount: amount_burnt,
    })
}
