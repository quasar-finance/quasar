use cosmwasm_std::{to_json_binary, DepsMut, Empty, Env, MessageInfo, Response, SubMsg, WasmMsg};

use crate::{
    msg::GaugeMsg,
    replies::REPLY_ON_GAUGE_INIT,
    state::{ADMIN, GAUGES, GAUGE_CODE, GAUGE_FEES, GAUGE_IN_PROCESS, GAUGE_KINDS},
    types::{Fee, Gauge, GaugeInProcess, GaugeKind},
    ContractError,
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
        GaugeMsg::Create { kind, gauge, fee } => gauge_create(deps, env, info, gauge, fee, kind),

        // update the info for the gauge contract
        GaugeMsg::Update {
            addr,
            gauge,
            fees,
            kind,
        } => gauge_update(deps, info, addr, gauge, fees, kind),

        // send a new merkle to the gauge contract
        GaugeMsg::MerkleUpdate { addr, merkle } => merkle_update(deps, info, addr, merkle),
    }
}

fn code_update(deps: DepsMut, info: MessageInfo, code: u64) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    GAUGE_CODE.save(deps.storage, &code)?;
    Ok(Response::default().add_attribute("action", "code_update"))
}

/// initializes a gauge contract
fn gauge_create(
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

    let msg = merkle_incentives::msg::InstantiateMsg {
        config: merkle_incentives::state::Config {
            clawback_address: deps.api.addr_validate(&gauge.clawback)?,
            start_block: gauge.period.start,
            end_block: gauge.period.end,
            expiration_block: gauge.period.expiry,
        },
    };

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
fn gauge_update(
    deps: DepsMut,
    info: MessageInfo,
    addr: String,
    new_gauge: Gauge,
    new_fees: Option<Fee>,
    new_kind: Option<GaugeKind>,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let addr = deps.api.addr_validate(&addr)?;

    GAUGES.save(deps.storage, addr.clone(), &new_gauge)?;

    if let Some(fees) = new_fees {
        GAUGE_FEES.save(deps.storage, addr.clone(), &fees)?;
    }

    if let Some(kind) = new_kind {
        GAUGE_KINDS.save(deps.storage, addr, &kind)?;
    }

    Ok(Response::default().add_attribute("action", "gauge_update"))
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

    if GAUGES.has(deps.storage, contract_addr) {
        return Ok(Response::default()
            .add_attribute("action", "merkle_update")
            .add_message(WasmMsg::Execute {
                contract_addr: addr.clone(),
                msg: to_json_binary(
                    &merkle_incentives::admin::execute::AdminExecuteMsg::UpdateMerkleRoot {
                        new_root: merkle,
                    },
                )?,
                funds: vec![],
            }));
    }

    Err(ContractError::NoSuchGauge { addr })
}
