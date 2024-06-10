use cosmwasm_std::{Addr, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use cw_storage_plus::Map;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

use crate::proshare::share_allocator::{ShareConfig, calculate_shares};


pub const USER_POSITIONS: Map<&Addr, UserPosition> = Map::new("user_positions");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserPosition {
    pub user: Addr,
    pub shares: Uint128,
}

impl UserPosition {
    pub fn new(user: Addr, shares: Uint128) -> Self {
        Self {
            user,
            shares,
        }
    }
}

pub fn allocate_position(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    config: &ShareConfig,
) -> StdResult<Response> {
    // Calculate the number of shares to allocate based on the amount
    let shares = calculate_shares(amount, config);

    // Load the user's position or create a new one if it doesn't exist
    let mut position = USER_POSITIONS.may_load(deps.storage, &info.sender)?
        .unwrap_or_else(|| UserPosition::new(info.sender.clone(), 
            Uint128::zero()));

    // Update the user's position with the new shares
    position.shares += shares;

    // Save the updated position
    USER_POSITIONS.save(deps.storage, &info.sender, &position)?;

    Ok(Response::new().add_attribute("method", "allocate_position"))
}

pub fn query_position(deps: Deps, address: Addr) -> StdResult<UserPosition> {
    let position = USER_POSITIONS.load(deps.storage, &address)?;
    Ok(position)
}
