use cosmwasm_std::{DepsMut, Env, Response, StdError};

use crate::{
    state::{MAIN_POSITION, POSITIONS},
    vault::concentrated_liquidity::{get_position, withdraw_from_position},
    ContractError,
};

pub fn delete_position(
    deps: DepsMut,
    env: Env,
    position_id: u64,
) -> Result<Response, ContractError> {
    if position_id == MAIN_POSITION.load(deps.storage)? {
        return Err(ContractError::Std(StdError::generic_err(
            "Cannot delete main position",
        )));
    }

    // query the position
    let position = get_position(&deps.querier, position_id)?;

    // withdraw all funds from the position
    let withdraw = withdraw_from_position(
        &env,
        position_id,
        position.position.unwrap().liquidity.parse()?,
    )?;

    // delete the position from the contracts local state
    // we don't want to check existence of positions in POSITIONS here, if we ever leak a position somehow, we would not be able to remove it from the map
    POSITIONS.remove(deps.storage, position_id);

    Ok(Response::new().add_message(withdraw))
}
