#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::admin::execute_admin;
use crate::error::ContractError;
use crate::execute::execute_split;
#[cfg(claim)]
use crate::execute_claim;
use crate::instantiate::do_instantiate;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_admin, query_receivers};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:splitter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    do_instantiate(deps, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Admin(msg) => execute_admin(deps, env, info, msg),
        ExecuteMsg::Split {} => execute_split(deps.as_ref(), env),
        #[cfg(claim)]
        ExecuteMsg::Claim { claims } => execute_claim(claims),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetReceivers {} => to_json_binary(&query_receivers(deps)?),
        QueryMsg::GetAdmin {} => to_json_binary(&query_admin(deps)?),
    }
}

#[cfg(test)]
mod tests {}
