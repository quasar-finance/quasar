// pub enum Reply {
//     //
//     DepositCreatePool = 1,
//     Unknow,
// }

use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, Reply, Response};

use crate::{
    state::{Replies, REPLIES},
    vault::deposit::{handle_create_position_reply, handle_swap_reply},
    ContractError,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    // TODO this needs and error check and error handling
    let reply = REPLIES.load(deps.storage, msg.id)?;
    match reply {
        Replies::Swap { user_addr, amount0 } => {
            handle_swap_reply(deps, env, user_addr, amount0, msg)
        }
        Replies::CreatePosition { user_addr } => {
            handle_create_position_reply(deps, env, user_addr, msg)
        }
    }
}
