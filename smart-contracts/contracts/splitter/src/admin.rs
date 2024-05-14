use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response};

use crate::{
    msg::AdminMsg,
    state::{Receiver, Receivers, ADMIN, RECEIVERS},
    ContractError,
};

pub fn execute_admin(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: AdminMsg,
) -> Result<Response, ContractError> {
    assert_admin(deps.as_ref(), env, info)?;
    match msg {
        AdminMsg::UpdateReceivers { new } => update_receivers(deps, new),
        // TODO make this a two step?
        AdminMsg::UpdateAdmin { new } => update_admin(deps, &new),
    }
}

pub fn update_receivers(deps: DepsMut, new: Vec<Receiver>) -> Result<Response, ContractError> {
    let recv: Receivers = new.try_into()?;
    RECEIVERS.save(deps.storage, &recv)?;

    Ok(Response::new()
        .add_attribute("action", "update_receivers")
        .add_attribute("new", recv.to_string()))
}

fn assert_admin(deps: Deps, env: Env, info: MessageInfo) -> Result<(), ContractError> {
    if ADMIN.load(deps.storage)? == info.sender {
        Ok(())
    } else if let Some(contract_admin) = deps
        .querier
        .query_wasm_contract_info(env.contract.address)?
        .admin
    {
        if contract_admin == info.sender {
            Ok(())
        } else {
            Err(ContractError::Unauthorized {})
        }
    } else {
        Err(ContractError::Unauthorized {})
    }
}

pub fn update_admin(deps: DepsMut, new: &str) -> Result<Response, ContractError> {
    let new_admin = deps.api.addr_validate(new)?;

    ADMIN.save(deps.storage, &new_admin)?;

    Ok(Response::new().add_attribute("new_admin", new_admin))
}
