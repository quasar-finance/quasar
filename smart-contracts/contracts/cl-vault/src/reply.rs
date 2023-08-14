use cosmwasm_std::{DepsMut, Env, Reply, Response, StdError};
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{contract::handle_create_denom_reply, ContractError};

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
        Replies::DepositCreatePool => todo!(),
        Replies::CollectIncentives => todo!(),
        Replies::CollectSpreadRewards => todo!(),
        Replies::WithdrawPosition => todo!(),
        Replies::CreatePosition => todo!(),
        Replies::Swap => todo!(),
        Replies::Fungify => todo!(),
        Replies::WithdrawUser => todo!(),
        Replies::CreateDenom => handle_create_denom_reply(
            deps,
            msg.result
                .into_result()
                .map_err(StdError::generic_err)?
                .data
                .unwrap(), // TODO this unwrap should probably be an ok_or
        ),
        Replies::WithdrawUser => todo!(),
        Replies::Unknown => todo!(),
    }
}
