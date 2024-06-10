use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw_storage_plus::Map;
use crate::error::ContractError;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

pub const ADAPTOR_POSITIONS: Map<Addr, AdaptorPosition> = Map::new("adaptor_positions");
// TODO #1 - Real shares from the yield destination to be added here. 
// TODO #2 - Bookkeeping of the total proshare allocated to an adaptor and outstanding 
//           deposit to be managed.
// TODO #3 - State and amounts of the shares in the adaptor is to be managed. 
//           allocated_from_strategy, send_to_yield_destination, 
//           rcvd_real_return_token, save_real_return_tokens, clean the state. 
             
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AdaptorPosition {
    pub total_share: Uint128,  // Total share allocated to an adaptor
    pub total_deposit: Uint128,  // Total deposit sent to an adaptor
    pub last_updated: u64,  // Timestamp of the last update
}

impl AdaptorPosition {
    pub fn new() -> Self {
        AdaptorPosition {
            total_share: Uint128::zero(),
            total_deposit: Uint128::zero(),
            last_updated: 0,
        }
    }

    pub fn new_with_values(total_share: Uint128, total_deposit: Uint128, last_updated: u64) -> Self {
        AdaptorPosition {
            total_share,
            total_deposit,
            last_updated,
        }
    }
}

pub fn update_adaptor_position(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    adaptor_address: Addr,
    total_share: Uint128,
    total_deposit: Uint128,
) -> Result<Response, ContractError> {
    let position = AdaptorPosition::new_with_values(
        total_share,
        total_deposit,
        env.block.time.seconds(),
    );

    ADAPTOR_POSITIONS.save(deps.storage, adaptor_address.clone(), &position)?;

    Ok(Response::new()
        .add_attribute("action", "update_adaptor_position")
        .add_attribute("adaptor_address", adaptor_address.to_string())
        .add_attribute("total_share", total_share.to_string())
        .add_attribute("total_deposit", total_deposit.to_string())
        .add_attribute("sender", info.sender.to_string()))
}

pub fn get_adaptor_position(
    deps: DepsMut,
    adaptor_address: Addr,
) -> StdResult<AdaptorPosition> {
    ADAPTOR_POSITIONS.load(deps.storage, adaptor_address)
}
