use cosmwasm_std::{
    to_json_binary, Addr, Deps, DepsMut, Empty, Env, MessageInfo, Response, SubMsg, WasmMsg,
};

use crate::{
    helpers::check_time_conf, msg::GaugeMsg, replies::REPLY_ON_GAUGE_INIT, state::{ADMIN, GAUGES, GAUGE_CODE, GAUGE_FEES, GAUGE_IN_PROCESS, GAUGE_KINDS}, types::{Fee, Gauge, GaugeInProcess, GaugeKind}, ContractError
};

pub fn handle_execute_gauge(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: GaugeMsg,
) -> Result<Response, ContractError> {
    match msg {
        // update the code for the gauge contract
        GaugeMsg::CodeUpdate { code } => code_update(deps, info, code),

        // initialize a new gauge contract
        GaugeMsg::Create { kind, gauge, fee } => create(deps, env, info, gauge, fee, kind),

        // update the info for the gauge contract
        GaugeMsg::Update {
            addr,
            gauge,
            fees,
            kind,
        } => update(deps, env, info, addr, gauge, fees, kind),

        GaugeMsg::Remove { addr } => remove(deps, info, addr),

        // send a new merkle to the gauge contract
        GaugeMsg::MerkleUpdate { addr, merkle } => merkle_update(deps, info, addr, merkle),
    }
}

/// write the code for the guage contract
fn code_update(deps: DepsMut, info: MessageInfo, code: u64) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    GAUGE_CODE.save(deps.storage, &code)?;
    Ok(Response::default().add_attribute("action", "code_update"))
}

/// verify that the guage exists in our map
fn check_gauge_exists(deps: Deps, contract_addr: Addr) -> Result<(), ContractError> {
    if !GAUGES.has(deps.storage, contract_addr.clone()) {
        return Err(ContractError::NoSuchGauge {
            addr: contract_addr.into_string(),
        });
    }
    Ok(())
}

/// initializes a gauge contract
fn create(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    gauge: Gauge,
    fee: Fee,
    kind: GaugeKind,
) -> Result<Response, ContractError> {
    // instantiate an instance of the incentive gauge
    // save the instance in the Gauge overview
    // depending on the gauge type, execute verification, eg verify that a vault is a quasar cl vault
    let code_id = GAUGE_CODE.load(deps.storage)?;
    let factory = env.contract.address.clone();

    check_time_conf(env, &gauge.period)?;

    let msg = merkle_incentives::msg::InstantiateMsg {
        config: merkle_incentives::state::Config {
            clawback_address: deps.api.addr_validate(&gauge.clawback)?,
            start_block: gauge.period.start,
            end_block: gauge.period.end,
            expiration_block: gauge.period.expiry,
        },
    };

    // check fee reciever is a valid address
    deps.api.addr_validate(&fee.reciever)?;

    // pre save the gauge data
    // it will be copied to the relevant maps when the gauge contract replies on init
    GAUGE_IN_PROCESS.save(deps.storage, &GaugeInProcess { gauge, kind, fee })?;

    Ok(Response::default()
        .add_attribute("action", "gauge_create")
        .add_submessage(SubMsg::<Empty>::reply_on_success(
            WasmMsg::Instantiate {
                label: "Incentives gauge".into(),
                admin: Some(factory.into_string()),
                code_id,
                funds: info.funds,
                msg: to_json_binary(&msg)?,
            },
            REPLY_ON_GAUGE_INIT,
        )))
}

/// primarly updates the gauge information locally
/// NOTE: this might need more work
fn update(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    addr: String,
    new_gauge: Gauge,
    new_fees: Option<Fee>,
    new_kind: Option<GaugeKind>,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let addr = deps.api.addr_validate(&addr)?;

    check_gauge_exists(deps.as_ref(), addr.clone())?;

    check_time_conf(env, &new_gauge.period)?;

    GAUGES.save(deps.storage, addr.clone(), &new_gauge)?;

    if let Some(fees) = new_fees {
        GAUGE_FEES.save(deps.storage, addr.clone(), &fees)?;
    }

    if let Some(kind) = new_kind {
        GAUGE_KINDS.save(deps.storage, addr, &kind)?;
    }

    Ok(Response::default().add_attribute("action", "gauge_update"))
}

/// validates gauge exists and then removes it and its dependencies
fn remove(deps: DepsMut, info: MessageInfo, addr: String) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let contract_addr = deps.api.addr_validate(&addr)?;

    check_gauge_exists(deps.as_ref(), contract_addr.clone())?;

    GAUGES.remove(deps.storage, contract_addr.clone());
    GAUGE_KINDS.remove(deps.storage, contract_addr.clone());
    GAUGE_FEES.remove(deps.storage, contract_addr);

    Ok(Response::default().add_attribute("action", "gauge_remove"))
}

/// sends a merkle root to the gauge if the gauge exists
fn merkle_update(
    deps: DepsMut,
    info: MessageInfo,
    addr: String,
    merkle: String,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let contract_addr = deps.api.addr_validate(&addr)?;

    check_gauge_exists(deps.as_ref(), contract_addr)?;

    Ok(Response::default()
        .add_attribute("action", "merkle_update")
        .add_message(WasmMsg::Execute {
            contract_addr: addr.clone(),
            msg: to_json_binary(&merkle_incentives::msg::ExecuteMsg::AdminMsg(
                merkle_incentives::admin::execute::AdminExecuteMsg::UpdateMerkleRoot {
                    new_root: merkle,
                },
            ))?,
            funds: vec![],
        }))
}
