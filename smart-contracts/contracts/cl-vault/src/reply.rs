use cosmwasm_std::{DepsMut, Env, Reply, Response, StdError};
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{
    contract::{handle_create_denom_reply, handle_instantiate_create_position_reply},
    merge::{handle_merge_create_position_reply, handle_merge_withdraw_reply},
    rewards::{handle_collect_incentives_reply, handle_collect_spread_rewards_reply},
    vault::{
        deposit::handle_deposit_create_position_reply,
        range::{
            handle_initial_create_position_reply, handle_iteration_create_position_reply,
            handle_merge_response, handle_swap_reply, handle_withdraw_position_reply,
        },
        withdraw::handle_withdraw_user_reply,
    },
    ContractError,
};

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u64)]
pub enum Replies {
    // handles position creation for a user deposit
    DepositCreatePosition = 1,
    // create the initial position while instantiating the contract
    InstantiateCreatePosition,
    // when handling rewards, we first collect incentives, then collect rewards
    CollectIncentives,
    // after gathering rewards, we divide them over share holders
    CollectSpreadRewards,

    // withdraw position
    WithdrawPosition,
    // create position in the modify range inital step
    RangeInitialCreatePosition,
    // create position in the modify range iteration step
    RangeIterationCreatePosition,
    // swap
    Swap,
    /// Merge positions, used to merge positions
    Merge,

    // handle user withdraws after liquidity is removed from the position
    WithdrawUser,
    // after creating a denom in initialization, register the created denom
    CreateDenom,
    /// to merge positions, we need to withdraw positions, used internally for merging
    WithdrawMerge,
    // create a new singular position in the merge, used internally for merging
    CreatePositionMerge,
    #[default]
    Unknown,
}

pub fn handle_reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id.into() {
        Replies::DepositCreatePosition => {
            handle_deposit_create_position_reply(deps, env, msg.result)
        }
        Replies::InstantiateCreatePosition => {
            handle_instantiate_create_position_reply(deps, env, msg.result)
        }
        Replies::CollectIncentives => handle_collect_incentives_reply(deps, env, msg.result),
        Replies::CollectSpreadRewards => handle_collect_spread_rewards_reply(deps, env, msg.result),
        Replies::WithdrawPosition => handle_withdraw_position_reply(deps, env, msg.result),
        Replies::RangeInitialCreatePosition => {
            handle_initial_create_position_reply(deps, env, msg.result)
        }
        Replies::RangeIterationCreatePosition => {
            handle_iteration_create_position_reply(deps, env, msg.result)
        }
        Replies::Swap => handle_swap_reply(deps, env, msg.result),
        Replies::Merge => handle_merge_response(deps, msg.result),
        Replies::CreateDenom => handle_create_denom_reply(deps, msg.result),
        Replies::WithdrawUser => handle_withdraw_user_reply(deps, msg.result),
        Replies::WithdrawMerge => handle_merge_withdraw_reply(deps, env, msg.result),
        Replies::CreatePositionMerge => handle_merge_create_position_reply(deps, env, msg.result),
        Replies::Unknown => unimplemented!(),
    }
}
