use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{
    state::{RANGE_EXECUTOR_ADMIN, RANGE_SUBMITTER_ADMIN},
    ContractError,
};

use super::helpers::is_contract_admin;

#[cw_serde]
pub enum AdminExecuteMsg {
    /// Update the range submitter admin.
    UpdateRangeSubmitterAdmin { new_admin: String },
    /// Update the range executor admin.
    UpdateRangeExecutorAdmin { new_admin: String },
}

pub fn execute_admin_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin_msg: AdminExecuteMsg,
) -> Result<Response, ContractError> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;
    match admin_msg {
        AdminExecuteMsg::UpdateRangeSubmitterAdmin { new_admin } => {
            update_range_submitter_admin(deps, env, info, new_admin)
        }
        AdminExecuteMsg::UpdateRangeExecutorAdmin { new_admin } => {
            update_range_executor_admin(deps, env, info, new_admin)
        }
    }
}

pub fn update_range_submitter_admin(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response, ContractError> {
    RANGE_SUBMITTER_ADMIN.save(deps.storage, &deps.api.addr_validate(&new_admin)?)?;

    Ok(Response::default())
}

pub fn update_range_executor_admin(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response, ContractError> {
    RANGE_EXECUTOR_ADMIN.save(deps.storage, &deps.api.addr_validate(&new_admin)?)?;

    Ok(Response::default())
}
