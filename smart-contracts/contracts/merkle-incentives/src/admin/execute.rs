use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{
    state::{INCENTIVES_ADMIN, MERKLE_ROOT},
    ContractError,
};

use super::helpers::{is_contract_admin, is_incentives_admin};

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
    msg: AdminExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        AdminExecuteMsg::UpdateMerkleRoot { new_root } => {
            update_merkle_root(deps, env, info, new_root)
        }
        AdminExecuteMsg::UpdateAdmin { new_admin } => update_admin(deps, env, info, new_admin),
    }
}

pub fn update_merkle_root(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_root: String,
) -> Result<Response, ContractError> {
    is_incentives_admin(deps.as_ref(), &info.sender)?;

    MERKLE_ROOT.save(deps.storage, &new_root)?;

    Ok(Response::default())
}

pub fn update_admin(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response, ContractError> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;

    let address_validated = deps.api.addr_validate(&new_admin)?;
    INCENTIVES_ADMIN.save(deps.storage, &address_validated)?;

    Ok(Response::default())
}
