#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cosmwasm_std::{Reply, Storage};
use cw2::{get_contract_version, set_contract_version};
use semver::Version;

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, FeeMsg, GaugeMsg, InstantiateMsg, MigrateMsg, QueryMsg},
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

        ExecuteMsg::GaugeMsg(gauge_msg) => handle_execute_gauge(deps, env, info, gauge_msg),

        ExecuteMsg::FeeMsg(fee_msg) => handle_execute_fee(deps, env, info, fee_msg),
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
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        crate::replies::REPLY_ON_GAUGE_INIT => crate::replies::gauge_init_success(deps, reply),
        _ => Err(ContractError::UnknownReply {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}

fn migrate_contract(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let (version_previous, version_new) = get_versions(deps.storage, msg)?;

    if version_new >= version_previous {
        set_contract_version(deps.storage, CONTRACT_NAME, version_new.to_string())?;
    }

    Ok(Response::new().add_attribute("action", "migrate"))
}

fn get_versions(
    storage: &dyn Storage,
    msg: MigrateMsg,
) -> Result<(Version, Version), ContractError> {
    let version_previous: Version = get_contract_version(storage)?
        .version
        .parse()
        .map_err(|_| ContractError::ParsingPrevVersion)?;

    let version_new: Version = env!("CARGO_PKG_VERSION")
        .parse()
        .map_err(|_| ContractError::ParsingNewVersion)?;

    if version_new.to_string() != msg.version {
        Err(ContractError::ImproperMsgVersion)?;
    }

    Ok((version_previous, version_new))
}

fn handle_execute_gauge(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: GaugeMsg,
) -> Result<Response, ContractError> {
    match msg {
        // update the code for the gauge contract
        GaugeMsg::CodeUpdate { code } => crate::types::Gauge::code_update(deps, info, code),

        // initialize a new gauge contract
        GaugeMsg::Create { kind, gauge, fee } => {
            crate::types::Gauge::create(deps, env, info, gauge, fee, kind)
        }

        // update the info for the gauge contract
        GaugeMsg::Update {
            addr,
            gauge,
            fees,
            kind,
        } => crate::types::Gauge::update(deps, env, info, addr, gauge, fees, kind),

        GaugeMsg::Remove { addr } => crate::types::Gauge::remove(deps, info, addr),

        GaugeMsg::Migrate {
            addr,
            code_id,
            version,
        } => crate::types::Gauge::migrate(deps, info, addr, code_id, version),

        // send a new merkle to the gauge contract
        GaugeMsg::MerkleUpdate { addr, merkle } => {
            crate::types::Gauge::merkle_update(deps, info, addr, merkle)
        }
    }
}

fn handle_execute_fee(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: FeeMsg,
) -> Result<Response, ContractError> {
    match msg {
        FeeMsg::Distribute { addr } => crate::types::Fee::distribute(deps, env, addr),
        FeeMsg::Update { addr, fees } => crate::types::Fee::update(deps, info, addr, fees),
    }
}
