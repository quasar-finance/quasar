use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, Storage};
use cw_utils::nonpayable;

use crate::{msg::ModifyRange, state::RANGE_ADMIN, ContractError};

use super::{
    create_position::create_new_position,
    delete_position::delete_position,
    modify_position_funds::{decrease_position_funds, increase_position_funds},
    move_position::{self, execute_move_position},
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
    env: &Env,
    info: MessageInfo,
    msg: ModifyRange,
) -> Result<Response, ContractError> {
    assert_range_admin(deps.storage, &info.sender)?;
    nonpayable(&info)?;

    match msg {
        ModifyRange::MovePosition(msg) => execute_move_position(
            deps,
            env,
            info,
            msg.position_id,
            msg.lower_price,
            msg.upper_price,
            msg.max_slippage,
            msg.ratio_of_swappable_funds_to_use,
            msg.twap_window_seconds,
            msg.recommended_swap_route,
            msg.force_swap_route,
            msg.claim_after,
        ),
        ModifyRange::IncreaseFunds(msg) => {
            increase_position_funds(deps, env, msg.position_id, msg.token0, msg.token1)
        }
        ModifyRange::DecreaseFunds(msg) => {
            decrease_position_funds(deps, env, msg.position_id, msg.liquidity)
        }
        ModifyRange::CreatePosition(msg) => {
            create_new_position(deps, env, msg.lower_price, msg.upper_price, msg.claim_after)
        }
        ModifyRange::DeletePosition(msg) => delete_position(deps, env, msg.position_id),
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
