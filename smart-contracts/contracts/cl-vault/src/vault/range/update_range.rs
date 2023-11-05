use cosmwasm_std::{DepsMut, Env, MessageInfo, Storage, Addr, Response};
use cw_utils::nonpayable;

use crate::{msg::ModifyRange, ContractError, state::RANGE_ADMIN};

use super::{move_position::move_position, modify_percentage::modify_percentage, create_position::create_new_position, delete_position::delete_position};


fn assert_range_admin(storage: &mut dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let admin = RANGE_ADMIN.load(storage)?;
    if admin != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub fn execute_update_range(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ModifyRange,
) -> Result<Response, ContractError> {
    assert_range_admin(deps.storage, &info.sender)?;
    nonpayable(&info)?;

    match msg {
        ModifyRange::MovePosition {
            old_position_id,
            new_lower_price,
            new_upper_price,
            max_slippage,
        } => move_position(
            deps,
            env,
            info,
            old_position_id,
            new_lower_price,
            new_upper_price,
            max_slippage,
        ),
        ModifyRange::ModifyPercentage {
            position_id,
            old_percentage,
            new_percentage,
        } => modify_percentage(),
        ModifyRange::CreatePosition {
            lower_price,
            upper_price,
            max_slippage,
            max_percentage,
        } => create_new_position(),
        ModifyRange::DeletePosition { position_id } => delete_position(),
    }
}
