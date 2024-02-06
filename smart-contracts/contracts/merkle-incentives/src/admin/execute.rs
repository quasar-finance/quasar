use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response};

use crate::ContractError;

use super::helpers::is_contract_admin;

#[cw_serde]
pub enum AdminExecuteMsg {
    /// Update the range submitter admin.
    UpdateMerkleRoot { new_root: String },
    /// Update the range executor admin.
    UpdateAdmin { new_admin: String },
}

pub fn execute_admin_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin_msg: AdminExecuteMsg,
) -> Result<Response, ContractError> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;
    match admin_msg {
        AdminExecuteMsg::UpdateMerkleRoot { new_root } => {
            update_merkle_root(deps, env, info, new_root)
        }
        AdminExecuteMsg::UpdateAdmin { new_admin } => {
            update_incentives_admin(deps, env, info, new_admin)
        }
    }
}

pub fn update_merkle_root(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_root: String,
) -> Result<Response, ContractError> {
    todo!()
}

pub fn update_incentives_admin(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response, ContractError> {
    todo!()
}
