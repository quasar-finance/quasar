use cosmwasm_std::Reply;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    state::{ADMIN, GAUGE_CODE},
};

// version info for migration info
pub const CONTRACT_NAME: &str = "crates.io:incentive-gauge-factory";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if let Some(codeid) = msg.gauge_codeid {
        GAUGE_CODE.save(deps.storage, &codeid)?;
    }

    if let Some(admin) = msg.admin {
        let admin = deps.api.addr_validate(&admin)?;
        ADMIN.set(deps, Some(admin))?;
    } else {
        ADMIN.set(deps, Some(info.sender))?;
    }

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", CONTRACT_NAME)
        .add_attribute("version", CONTRACT_VERSION))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AdminUpdate { addr } => crate::executes::update_admin(deps, info, addr),

        ExecuteMsg::GaugeMsg(gauge_msg) => {
            crate::gauge::handle_execute_gauge(deps, env, info, gauge_msg)
        }

        ExecuteMsg::FeeMsg(fee_msg) => crate::fees::handle_execute_fee(deps, env, info, fee_msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Gauge { address } => to_json_binary(&crate::queries::query_gauge(
            deps,
            deps.api.addr_validate(&address)?,
        )?),
        QueryMsg::ListGauges { start_after, limit } => {
            to_json_binary(&crate::queries::query_gauge_list(deps, start_after, limit)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    crate::migrate::migrate_contract(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        crate::replies::REPLY_ON_GAUGE_INIT => crate::replies::gauge_init_success(deps, reply),
        _ => Err(ContractError::UnknownReply {}),
    }
}
