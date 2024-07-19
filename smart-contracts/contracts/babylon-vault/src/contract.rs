use crate::error::VaultError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::OWNER;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "quasar:babylon-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type VaultResult<T = Response> = Result<T, VaultError>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> VaultResult {
    OWNER.initialize(
        deps.storage,
        deps.api,
        mars_owner::OwnerInit::SetInitialOwner { owner: msg.owner },
    )?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> VaultResult {
    match msg {
        ExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
        _ => Ok(Response::default()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> VaultResult<Binary> {
    match msg {
        QueryMsg::Owner {} => Ok(to_json_binary(&OWNER.query(deps.storage)?)?),
        _ => Ok(Binary::default()),
    }
}
