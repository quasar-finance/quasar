use cosmwasm_std::{Addr, DepsMut, Env, Event, MessageInfo, Response};
use cw_utils::nonpayable;
use crate::{msg::AdminExtensionExecuteMsg, ContractError};
use crate::state::{ConfigUpdates, ADMIN_CONFIG, VAULT_CONFIG};

pub(crate) fn execute_admin(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin_msg: AdminExtensionExecuteMsg,
) -> Result<Response, ContractError> {
    match admin_msg {
        AdminExtensionExecuteMsg::UpdateAdmin { address } => execute_update_admin(deps, info, address),
        AdminExtensionExecuteMsg::AcceptAdminTransfer {} => execute_accept_admin_transfer(deps, info),
        AdminExtensionExecuteMsg::DropAdminTransfer {} => execute_drop_admin_transfer(deps, info),
        AdminExtensionExecuteMsg::UpdateConfig { updates } => execute_update_config(deps, info, updates),
    }
}

pub fn execute_update_admin(
    deps: DepsMut,
    info: MessageInfo,
    address: String
) -> Result<Response, ContractError> {
    nonpayable(&info);

    let admin_config = ADMIN_CONFIG.load(deps.storage)?;

    admin_config.admin.assert_admin(deps.as_ref(), &info.sender)?;
    let admin_addr = deps.api.addr_validate(&address)?;
    admin_config.admin_transfer.save(deps.storage, &admin_addr)?;
    
    Ok(Response::new()
        .add_attribute("action", "execute_update_admin")
        .add_attribute(
            "previous_admin",
            admin_config.admin
                .get(deps.as_ref())?
                .unwrap_or_else(|| Addr::unchecked(""))
                .as_ref(),
        )
        .add_attribute("new_admin", &address))
}

pub fn execute_accept_admin_transfer(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    nonpayable(&info);

    let admin_config = ADMIN_CONFIG.load(deps.storage)?;

    let new_admin = admin_config.admin_transfer;
    if info.sender != new_admin {
        return Err(ContractError::Unauthorized {});
    }

    admin_config.admin_transfer.remove(deps.storage);
    admin_config.admin.set(deps.branch(), Some(new_admin))?;

    Ok(Response::new()
        .add_attribute("action", "execute_accept_admin_transfer")
        .add_attribute(
            "previous_admin",
            admin_config.admin
                .get(deps.as_ref())?
                .unwrap_or_else(|| Addr::unchecked(""))
                .as_ref(),
        )
        .add_attribute("new_admin", new_admin.as_ref()))
}

pub fn execute_drop_admin_transfer(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    nonpayable(&info);

    let admin_config = ADMIN_CONFIG.load(deps.storage)?;

    admin_config.admin.assert_admin(deps.as_ref(), &info.sender)?;
    admin_config.admin_transfer.remove(deps.storage);
    
    Ok(Response::new()
        .add_attribute("action", "execute_drop_admin_transfer"))
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    updates: ConfigUpdates
) -> Result<Response, ContractError> {
    nonpayable(&info);

    let admin_config = ADMIN_CONFIG.load(deps.storage)?;

    admin_config.admin.assert_admin(deps.as_ref(), &info.sender)?;

    let new_config = VAULT_CONFIG
        .load(deps.storage)?
        .update(deps.as_ref(), updates.clone())?;

    VAULT_CONFIG.save(deps.storage, &new_config)?;

    Ok(Response::default()
        .add_attribute("action", "execute_update_config")
        .add_attribute("updates", &format!("{:?}", updates)))
}
