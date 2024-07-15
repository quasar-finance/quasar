use cosmwasm_std::{DepsMut, Env, Response, StdError};

use crate::{
    state::{MAIN_POSITION_ID, POSITIONS},
    vault::concentrated_liquidity::{get_position, withdraw_from_position},
    ContractError,
};

/// Delete a positon and remove it from being tracked in the vault.
/// Any position should be deleted explicitly through this method instead of withdrawing the entire
/// liquidity.
/// The main position of the vault cannot be deleted, but can only have less
/// than the total liquidity removed from it.
pub fn delete_position(
    deps: DepsMut,
    env: &Env,
    position_id: u64,
) -> Result<Response, ContractError> {
    if position_id == MAIN_POSITION_ID.load(deps.storage)? {
        return Err(ContractError::Std(StdError::generic_err(
            "Cannot delete main position",
        )));
    }

    // query the position

    // TODO add parsed position helper
    let position = get_position(&deps.querier, position_id)?;

    // withdraw all funds from the position
    let withdraw = withdraw_from_position(
        env,
        position_id,
        position.position.unwrap().liquidity.parse()?,
    )?;

    // delete the position from the contracts local state
    // we don't want to check existence of positions in POSITIONS here, if we ever leak a position somehow, we would not be able to remove it from the map
    POSITIONS.remove(deps.storage, position_id);

    Ok(Response::new().add_message(withdraw))
}
