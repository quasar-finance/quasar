use crate::error::assert_vault;
use crate::msg::{
    LstAdapterExecuteMsg, LstAdapterInstantiateMsg, LstAdapterMigrateMsg, LstAdapterQueryMsg,
};
use crate::state::{IbcConfig, IBC_CONFIG, LST_DENOM, OWNER, VAULT};
use crate::{
    LstAdapterError, LST_ADAPTER_OSMOSIS_ID, LST_ADAPTER_OSMOSIS_NAME, LST_ADAPTER_OSMOSIS_VERSION,
};
#[cfg(not(target_arch = "wasm32"))]
use abstract_app::abstract_interface::AbstractInterfaceError;
use abstract_app::{abstract_interface, AppContract};
use abstract_sdk::{AbstractResponse, IbcInterface, TransferInterface};
#[cfg(not(target_arch = "wasm32"))]
use abstract_std::manager::ModuleInstallConfig;
use abstract_std::objects::chain_name::ChainName;
use cosmwasm_std::{
    to_json_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, Event, IbcMsg, IbcTimeout,
    IbcTimeoutBlock, MessageInfo, Response,
};
use mars_owner::OwnerInit::SetInitialOwner;
use osmosis_std::cosmwasm_to_proto_coins;
use osmosis_std::types::ibc::applications::transfer::v1::MsgTransfer;
use osmosis_std::types::ibc::core::client::v1::Height;
use prost::Message;
use quasar_types::{
    error::assert_funds_single_token,
    stride::{get_autopilot_msg, Action},
};

const IBC_MSG_TRANSFER_TYPE_URL: &str = "/ibc.applications.transfer.v1.MsgTransfer";

pub type LstAdapterResult<T = Response> = Result<T, LstAdapterError>;

pub type LstAdapter = AppContract<
    LstAdapterError,
    LstAdapterInstantiateMsg,
    LstAdapterExecuteMsg,
    LstAdapterQueryMsg,
    LstAdapterMigrateMsg,
>;

const APP: LstAdapter = LstAdapter::new(LST_ADAPTER_OSMOSIS_ID, LST_ADAPTER_OSMOSIS_VERSION, None)
    .with_instantiate(instantiate_)
    .with_execute(execute_)
    .with_query(query_)
    .with_migrate(migrate_);

#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, LstAdapter);

abstract_app::cw_orch_interface!(APP, LstAdapter, LstAdapterInterface);

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::LstAdapterInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;

    fn dependency_install_configs(
        _configuration: Self::DependenciesConfig,
    ) -> Result<Vec<ModuleInstallConfig>, AbstractInterfaceError> {
        Ok(vec![])
    }
}

pub fn instantiate_(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _app: LstAdapter,
    msg: LstAdapterInstantiateMsg,
) -> LstAdapterResult {
    OWNER.initialize(deps.storage, deps.api, SetInitialOwner { owner: msg.owner })?;
    VAULT.save(deps.storage, &deps.api.addr_validate(&msg.vault)?)?;
    LST_DENOM.save(deps.storage, &msg.lst_denom)?;
    Ok(Response::default())
}

pub fn execute_(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    app: LstAdapter,
    msg: LstAdapterExecuteMsg,
) -> LstAdapterResult {
    match msg {
        LstAdapterExecuteMsg::Unbond {} => unbond(deps, env, info, app),
        LstAdapterExecuteMsg::UpdateIbcConfig {
            remote_chain,
            channel,
            revision,
            block_offset,
            timeout_secs,
        } => update_ibc_config(
            deps,
            info,
            app,
            remote_chain,
            channel,
            revision,
            block_offset,
            timeout_secs,
        ),
        LstAdapterExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
        LstAdapterExecuteMsg::Update { vault, lst_denom } => {
            update(deps, info, app, vault, lst_denom)
        }
    }
}

fn unbond(deps: DepsMut, env: Env, info: MessageInfo, app: LstAdapter) -> LstAdapterResult {
    assert_vault(&info.sender, &VAULT.load(deps.storage)?)?;
    let lst_denom = LST_DENOM.load(deps.storage)?;
    assert_funds_single_token(&info.funds, &lst_denom)?;

    let mut transfer_msgs = app.bank(deps.as_ref()).deposit(info.funds.clone())?;
    // ibc transfer
    let ibc_client = app.ibc_client(deps.as_ref());
    let ibc_msg = ibc_client.ics20_transfer(
        ChainName::from_chain_id("stargaze-1").to_string(),
        info.funds.clone(),
    )?;
    transfer_msgs.push(ibc_msg);

    // let ibc_config = IBC_CONFIG.load(deps.storage)?;
    // let remote_addr = app
    //     .ibc_client(deps.as_ref())
    //     .remote_proxy_addr(&ibc_config.remote_chain)?;
    // if remote_addr.is_none() {
    //     return Err(LstAdapterError::MissingRemoteAddress {
    //         chain: ibc_config.remote_chain,
    //     });
    // }
    // let remote_addr = remote_addr.unwrap();
    // let autopilot_redeem_msg = get_autopilot_msg(
    //     Action::RedeemStake,
    //     remote_addr.as_ref(),
    //     Some(info.sender.to_string()),
    // );
    // let msg = MsgTransfer {
    //     source_port: "transfer".to_string(),
    //     source_channel: ibc_config.channel,
    //     token: Some(info.funds[0].clone().into()),
    //     sender: env.contract.address.to_string(),
    //     receiver: remote_addr,
    //     timeout_height: Some(Height {
    //         revision_number: 5,
    //         revision_height: env.block.height + 5,
    //     }),
    //     timeout_timestamp: env.block.time.nanos()
    //         + ibc_config.timeout_secs.unwrap_or_default() * 1_000_000_000,
    //     memo: serde_json::to_string(&autopilot_redeem_msg)
    //         .map_err(|err| LstAdapterError::Json(err.to_string()))?,
    // };
    // Ok(app.response("unbond").add_message(msg))
    Ok(app.response("unbond").add_messages(transfer_msgs))
}

fn update_ibc_config(
    deps: DepsMut,
    info: MessageInfo,
    app: LstAdapter,
    remote_chain: String,
    channel: String,
    revision: Option<u64>,
    block_offset: Option<u64>,
    timeout_secs: Option<u64>,
) -> LstAdapterResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    IBC_CONFIG.save(
        deps.storage,
        &IbcConfig {
            remote_chain,
            revision,
            block_offset,
            timeout_secs,
            channel,
        },
    )?;
    Ok(app.response("update ibc config"))
}

fn update(
    deps: DepsMut,
    info: MessageInfo,
    app: LstAdapter,
    vault: Option<String>,
    lst_denom: Option<String>,
) -> LstAdapterResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    if let Some(vault) = vault {
        VAULT.save(deps.storage, &deps.api.addr_validate(&vault)?)?;
    }
    if let Some(lst_denom) = lst_denom {
        LST_DENOM.save(deps.storage, &lst_denom)?;
    }
    Ok(app.response("update"))
}

pub fn query_(
    deps: Deps,
    _env: Env,
    _app: &LstAdapter,
    msg: LstAdapterQueryMsg,
) -> LstAdapterResult<Binary> {
    match msg {
        LstAdapterQueryMsg::IbcConfig {} => Ok(to_json_binary(
            &IBC_CONFIG.may_load(deps.storage)?.unwrap_or_default(),
        )?),
        LstAdapterQueryMsg::Owner {} => Ok(to_json_binary(
            &OWNER
                .current(deps.storage)?
                .map(String::from)
                .unwrap_or_default(),
        )?),
        LstAdapterQueryMsg::Vault {} => Ok(to_json_binary(&VAULT.load(deps.storage)?.to_string())?),
        LstAdapterQueryMsg::LstDenom {} => Ok(to_json_binary(&LST_DENOM.load(deps.storage)?)?),
    }
}

pub fn migrate_(
    _deps: DepsMut,
    _env: Env,
    app: LstAdapter,
    _msg: LstAdapterMigrateMsg,
) -> LstAdapterResult {
    Ok(app.response("migrate"))
}
