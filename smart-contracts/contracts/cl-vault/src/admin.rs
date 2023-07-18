use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{msg::AdminExtensionExecuteMsg, ContractError};

pub(crate) fn execute_admin_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin_msg: AdminExtensionExecuteMsg,
) -> Result<Response, ContractError> {
    match admin_msg {
        AdminExtensionExecuteMsg::UpdateConfig { updates } => todo!(),
        AdminExtensionExecuteMsg::UpdateAdmin { address } => todo!(),
        AdminExtensionExecuteMsg::AcceptAdminTransfer {} => todo!(),
        AdminExtensionExecuteMsg::DropAdminTransfer {} => todo!(),
    }
}
