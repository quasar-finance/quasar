use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, Storage};
use cw_utils::nonpayable;

use crate::{msg::ModifyRange, state::RANGE_ADMIN, ContractError};

use super::{
    create_position::create_new_position,
    delete_position::delete_position,
    modify_percentage::{add_ratio, decrease_position_funds, increase_position_funds, lower_ratio},
    move_position::move_position,
};

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
        ModifyRange::AddRatio {
            position_id,
            old_ratio,
            new_ratio,
            ratio_of_free_funds,
        } => add_ratio(
            deps,
            env,
            position_id,
            old_ratio,
            new_ratio,
            ratio_of_free_funds,
        ),
        ModifyRange::LowerRatio {
            position_id,
            old_ratio,
            new_ratio,
        } => lower_ratio(deps, env, position_id, old_ratio, new_ratio),
        ModifyRange::IncreaseFunds {
            position_id,
            token0,
            token1,
        } => increase_position_funds(deps, env, position_id, token0, token1),
        ModifyRange::DecreaseFunds {
            position_id,
            liquidity,
        } => decrease_position_funds(deps, env, position_id, liquidity),
        ModifyRange::CreatePosition {
            lower_price,
            upper_price,
            ratio,
        } => create_new_position(deps, env, lower_price, upper_price, ratio),
        ModifyRange::DeletePosition { position_id } => delete_position(deps, env, position_id),
        ModifyRange::Rebalance {} => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_info},
        Addr,
    };

    use super::*;
    use crate::state::RANGE_ADMIN;

    #[test]
    fn test_assert_range_admin() {
        let mut deps = mock_dependencies();
        let info = mock_info("addr0000", &[]);

        RANGE_ADMIN.save(&mut deps.storage, &info.sender).unwrap();

        assert_range_admin(&mut deps.storage, &info.sender).unwrap();

        let info = mock_info("addr0001", &[]);
        assert_range_admin(&mut deps.storage, &info.sender).unwrap_err();

        let info = mock_info("addr0000", &[]);
        RANGE_ADMIN.save(&mut deps.storage, &info.sender).unwrap();

        assert_range_admin(&mut deps.storage, &Addr::unchecked("someoneelse")).unwrap_err();
    }
}
