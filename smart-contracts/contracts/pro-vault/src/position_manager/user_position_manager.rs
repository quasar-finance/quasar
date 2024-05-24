use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw_storage_plus::Map;
use crate::error::ContractError;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

pub const USER_POSITIONS: Map<Addr, UserPosition> = Map::new("user_positions");

// #[cw_serde]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserPosition {
    pub user_address: Addr,
    pub total_share: u128,
    pub deposit_amount: u128,
    pub last_updated: u64,
}

impl UserPosition {
    pub fn new(user_address: Addr, total_share: u128, deposit_amount: u128, last_updated: u64) -> Self {
        UserPosition {
            user_address,
            total_share,
            deposit_amount,
            last_updated,
        }
    }
}

pub fn update_user_position(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user_address: Addr,
    total_share: u128,
    deposit_amount: u128,
) -> Result<Response, ContractError> {
    let position = UserPosition::new(user_address.clone(), total_share, deposit_amount, env.block.time.seconds());

    USER_POSITIONS.save(deps.storage, user_address.clone(), &position)?;

    Ok(Response::new()
        .add_attribute("action", "update_user_position")
        .add_attribute("user_address", user_address.to_string())
        .add_attribute("total_share", total_share.to_string())
        .add_attribute("deposit_amount", deposit_amount.to_string())
        .add_attribute("sender", info.sender.to_string()))
}

pub fn get_user_position(
    deps: DepsMut,
    user_address: Addr,
) -> StdResult<UserPosition> {
    USER_POSITIONS.load(deps.storage, user_address)
}
