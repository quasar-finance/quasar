use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, Reply, Response};
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::ContractError;

#[cw_serde]
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u64)]
pub enum Replies {
    //
    DepositCreatePool = 1,
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

    #[default]
    Unknown,
}

pub fn handle_reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id.into() {
        Replies::DepositCreatePool => todo!(),
        Replies::CollectIncentives => todo!(),
        Replies::CollectSpreadRewards => todo!(),
        Replies::WithdrawPosition => todo!(),
        Replies::CreatePosition => todo!(),
        Replies::Unknown => todo!(),
        Replies::Swap => todo!(),
        Replies::Fungify => todo!(),
    }
}
