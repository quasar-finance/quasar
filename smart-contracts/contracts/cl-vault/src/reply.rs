use cosmwasm_std::{DepsMut, Env, Reply, Response, StdError};
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{
    contract::handle_create_denom_reply,
    rewards::{handle_collect_incentives_reply, handle_collect_spread_rewards_reply},
    vault::{deposit::handle_deposit_create_position_reply, withdraw::handle_withdraw_user_reply},
    ContractError,
};

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u64)]
pub enum Replies {
    // handles position creation for a user deposit
    DepositCreatePosition = 1,
    // when handling rewards, we first collect incentives, then collect rewards
    CollectIncentives,
    // after gathering rewards, we divide them over share holders
    CollectSpreadRewards,

    // withdraw position
    WithdrawPosition,
    // create position
    CreatePosition,
    // swap
    Swap,
    // fungify
    Fungify,

    // handle user withdraws after liquidity is removed from the position
    WithdrawUser,
    // after creating a denom in initialization, register the created denom
    CreateDenom,
    #[default]
    Unknown,
}

pub fn handle_reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id.into() {
        Replies::DepositCreatePosition => {
            handle_deposit_create_position_reply(deps, env, msg.result)
        }
        Replies::CollectIncentives => handle_collect_incentives_reply(deps, env, msg.result),
        Replies::CollectSpreadRewards => handle_collect_spread_rewards_reply(deps, env, msg.result),
        Replies::WithdrawPosition => todo!(),
        Replies::CreatePosition => todo!(),
        Replies::Swap => todo!(),
        Replies::Fungify => todo!(),
        Replies::CreateDenom => handle_create_denom_reply(deps, msg.result),
        Replies::WithdrawUser => handle_withdraw_user_reply(deps, msg.result),
        Replies::Unknown => unimplemented!(),
    }
}
