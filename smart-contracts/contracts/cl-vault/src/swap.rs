use crate::ContractError;
use swaprouter::msg::QueryMsg::GetRoute;
use swaprouter::msg::ExecuteMsg::Swap;

/// swap queries the swap router, estimates the swaps over different routes and takes the best route.
/// The swap router should atleast try to swap the CL pool or over the "well established" Osmosis route
pub fn swap() -> Result<_,ContractError> {
    let route = GetRoute { input_denom: (), output_denom: () };
}