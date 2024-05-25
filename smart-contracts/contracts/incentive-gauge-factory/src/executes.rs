// use merkle_incentives::msg::InstantiateMsg;
use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::{state::ADMIN, ContractError};

pub fn update_admin(
    mut deps: DepsMut,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    let new_admin = deps.api.addr_validate(&new_admin)?;
    ADMIN.set(deps.branch(), Some(new_admin))?;
    Ok(Response::new().add_attributes(vec![("action", "update_admin")]))
}
