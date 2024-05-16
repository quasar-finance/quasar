use cosmwasm_std::{DepsMut, Response};

use crate::{
    msg::InstantiateMsg,
    state::{ADMIN, RECEIVERS},
    ContractError,
};

pub fn do_instantiate(deps: DepsMut, msg: InstantiateMsg) -> Result<Response, ContractError> {
    ADMIN.save(deps.storage, &deps.api.addr_validate(&msg.admin)?)?;
    RECEIVERS.save(deps.storage, &msg.receivers.try_into()?)?;

    Ok(Response::new())
}
