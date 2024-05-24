use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use cw_storage_plus::Item;
use crate::error::ContractError;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

pub const PROVAULT_POSITION: Item<ProVaultPosition> = Item::new("provault_position");

// #[cw_serde]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProVaultPosition {
    pub provault_share_balance: Vec<Coin>,  // Total pro vault share
    pub last_updated: u64,
    pub total_unallocated: Uint128,  // Remaining amount of deposited token
}

impl ProVaultPosition {
    pub fn new() -> Self {
        ProVaultPosition {
            provault_share_balance: Vec::new(),
            last_updated: 0,
            total_unallocated: Uint128::zero(),
        }
    }

    pub fn new_with_values(provault_share_balance: Vec<Coin>, last_updated: u64, total_unallocated: Uint128) -> Self {
        ProVaultPosition {
            provault_share_balance,
            last_updated,
            total_unallocated,
        }
    }
}

pub fn update_provault_position(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    provault_share_balance: Vec<Coin>,
    total_unallocated: Uint128,
) -> Result<Response, ContractError> {
    let position = ProVaultPosition::new_with_values(
        provault_share_balance,
        env.block.time.seconds(),
        total_unallocated
    );

    PROVAULT_POSITION.save(deps.storage, &position)?;

    Ok(Response::new()
        .add_attribute("action", "update_provault_position")
        .add_attribute("sender", info.sender.to_string()))
}

pub fn get_provault_position(
    deps: DepsMut,
) -> StdResult<ProVaultPosition> {
    PROVAULT_POSITION.load(deps.storage)
}
