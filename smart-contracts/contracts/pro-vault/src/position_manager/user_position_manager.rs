use cosmwasm_std::{Addr, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use cw_storage_plus::Map;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use crate::position_manager::vault_position_manager::{PROVAULT_POSITION, ProVaultPosition};

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
    
   // Load the provault position or create a new one if it doesn't exist
   let mut provault_position: ProVaultPosition = PROVAULT_POSITION
   .may_load(deps.storage)?
   .unwrap_or_else(|| ProVaultPosition::new(Uint128::zero(),Uint128::zero(),
                    Uint128::zero(), Uint128::zero()));


    // Load the user's position or create a new one if it doesn't exist
    let mut position = USER_POSITIONS.may_load(deps.storage, &info.sender)?
        .unwrap_or_else(|| UserPosition::new(info.sender.clone(), 
            Uint128::zero()));
    
    // Calculate the number of shares to allocate based on the amount
    let shares = calculate_shares(amount,provault_position.total_shares,
         provault_position.total_assets);
     
    position.shares += shares;
    USER_POSITIONS.save(deps.storage, &info.sender, &position)?;

    Ok(Response::new().
        add_attribute("method", "allocate_position").
        add_attribute("new_allocated_share", shares).
        add_attribute("total_user_share", position.shares))
}

pub fn query_position(deps: Deps, address: Addr) -> StdResult<UserPosition> {
    let position = USER_POSITIONS.load(deps.storage, &address)?;
    Ok(position)
}
