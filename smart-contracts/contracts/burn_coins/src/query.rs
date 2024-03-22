use cosmwasm_std::{Deps, StdResult};

use crate::msg::TotalBurntResponse;
use crate::state::AMOUNT_BURNT;

/// Queries and returns the current airdrop configuration.
///
/// # Arguments
///
/// * `deps` - Deps is a struct providing access to the contract's dependencies like storage.
///
/// # Returns
///
/// Returns a `ConfigResponse` containing the current airdrop configuration.
pub fn query_total_burn(deps: Deps) -> StdResult<TotalBurntResponse> {
    let amount_burnt = AMOUNT_BURNT.load(deps.storage)?;
    Ok(TotalBurntResponse {
        amount: amount_burnt,
    })
}
