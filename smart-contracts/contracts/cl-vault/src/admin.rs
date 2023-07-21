use cosmwasm_std::{Addr, DepsMut, Env, Event, MessageInfo, Response};

use crate::{msg::AdminExtensionExecuteMsg, ContractError, state::ConfigUpdates, VAULT};

pub(crate) fn execute_admin_msg(
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
    // load store
    VAULT.may_load(deps.storage)?; // TODO ??
    VAULT.admin.assert_admin(deps.as_ref(), &info.sender)?;
    let admin_addr = deps.api.addr_validate(&address)?;
    self.admin_transfer.save(deps.storage, &admin_addr)?;
    let event = Event::new("apollo/vaults/autocompounding_vault").add_attributes(vec![
        ("action", "execute_update_admin"),
        (
            "previous_admin",
            self.admin
                .get(deps.as_ref())?
                .unwrap_or_else(|| Addr::unchecked(""))
                .as_ref(),
        ),
        ("new_admin", &address),
    ]);
    Ok(Response::new().add_event(event))
}

pub fn execute_accept_admin_transfer(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let new_admin = self.admin_transfer.load(deps.storage)?;
    if info.sender != new_admin {
        return Err(ContractError::Unauthorized {});
    }
    self.admin_transfer.remove(deps.storage);
    let event = Event::new("apollo/vaults/autocompounding_vault").add_attributes(vec![
        ("action", "execute_accept_admin_transfer"),
        (
            "previous_admin",
            self.admin
                .get(deps.as_ref())?
                .unwrap_or_else(|| Addr::unchecked(""))
                .as_ref(),
        ),
        ("new_admin", new_admin.as_ref()),
    ]);
    self.admin.set(deps.branch(), Some(new_admin))?;
    Ok(Response::new().add_event(event))

}

pub fn execute_drop_admin_transfer(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    self.admin.assert_admin(deps.as_ref(), &info.sender)?;
    self.admin_transfer.remove(deps.storage);
    let event = Event::new("apollo/vaults/autocompounding_vault")
        .add_attributes(vec![("action", "execute_drop_admin_transfer")]);
    Ok(Response::new().add_event(event))
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    updates: ConfigUpdates
) -> Result<Response, ContractError> {
    self.admin.assert_admin(deps.as_ref(), &info.sender)?;

    let new_config = self
        .config
        .load(deps.storage)?
        .update(deps.as_ref(), updates.clone())?;
    self.config.save(deps.storage, &new_config)?;

    let event = Event::new("apollo/vaults/autocompounding_vault").add_attributes(vec![
        ("action", "execute_update_config"),
        ("updates", &format!("{:?}", updates)),
    ]);

    Ok(Response::default().add_event(event))
}
