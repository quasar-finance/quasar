use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use mars_owner::OwnerUpdate;

use crate::{
    state::{OWNER, RANGE_EXECUTOR_OWNER, RANGE_SUBMITTER_OWNER},
    ContractError,
};

#[cw_serde]
pub enum AdminExecuteMsg {
    /// Update the range submitter owner.
    UpdateRangeSubmitterOwner(OwnerUpdate),
    /// Update the range executor owner.
    UpdateRangeExecutorOwner(OwnerUpdate),
}

pub fn execute_admin_msg(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    admin_msg: AdminExecuteMsg,
) -> Result<Response, ContractError> {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    match admin_msg {
        AdminExecuteMsg::UpdateRangeSubmitterOwner(update) => {
            Ok(RANGE_SUBMITTER_OWNER.update(deps, info, update)?)
        }
        AdminExecuteMsg::UpdateRangeExecutorOwner(update) => {
            Ok(RANGE_EXECUTOR_OWNER.update(deps, info, update)?)
        }
    }
}
