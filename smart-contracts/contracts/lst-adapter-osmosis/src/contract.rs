use crate::msg::{
    LstAdapterExecuteMsg, LstAdapterInstantiateMsg, LstAdapterMigrateMsg, LstAdapterQueryMsg,
};
use crate::state::{LST_DENOM, VAULT};
use crate::{LstAdapterError, LST_ADAPTER_OSMOSIS_ID, LST_ADAPTER_OSMOSIS_VERSION};
use abstract_app::abstract_interface::AbstractInterfaceError;
use abstract_app::{abstract_interface, AppContract};
use abstract_sdk::{AbstractResponse, IbcInterface, TransferInterface};
use abstract_std::manager::ModuleInstallConfig;
use abstract_std::objects::chain_name::ChainName;
use cosmwasm_std::{
    to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env, IbcMsg, IbcTimeout, IbcTimeoutBlock,
    MessageInfo, Response,
};
use mars_owner::OwnerInit::SetInitialOwner;
use osmosis_std::cosmwasm_to_proto_coins;
use osmosis_std::types::ibc::applications::transfer::v1::MsgTransfer;
use osmosis_std::types::ibc::core::client::v1::Height;
use quasar_types::error::assert_funds_single_token;

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
    VAULT.initialize(deps.storage, deps.api, SetInitialOwner { owner: msg.vault })?;
    LST_DENOM.save(deps.storage, &msg.lst_denom)?;
    // app.
    // let msg = MsgTransfer{
    //     source_port: ,
    //     source_channel: ,
    //     token: ,
    //     sender: ,
    //     receiver: ,
    //     timeout_height: ,
    //     timeout_timestamp: ,
    //     memo: ,
    // };
    Ok(Response::default())
}
// pub struct MsgTransfer {
//     #[prost(string, tag = "1")]
//     pub source_port: String,
//     #[prost(string, tag = "2")]
//     pub source_channel: String,
//     #[prost(message, optional, tag = "3")]
//     pub token: ::core::option::Option<osmosis_std::types::cosmos::base::v1beta1::Coin>,
//     #[prost(string, tag = "4")]
//     pub sender: String,
//     #[prost(string, tag = "5")]
//     pub receiver: String,
//     #[prost(message, optional, tag = "6")]
//     pub timeout_height: Option<Height>,
//     #[prost(uint64, optional, tag = "7")]
//     pub timeout_timestamp: ::core::option::Option<u64>,
//     #[prost(string, tag = "8")]
//     pub memo: String,
// }
pub fn execute_(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    app: LstAdapter,
    msg: LstAdapterExecuteMsg,
) -> LstAdapterResult {
    match msg {
        LstAdapterExecuteMsg::Unbond {} => unbond(deps, env, info, app),
        LstAdapterExecuteMsg::Claim {} => claim(deps, env, info, app),
    }
}

fn unbond(deps: DepsMut, env: Env, info: MessageInfo, app: LstAdapter) -> LstAdapterResult {
    VAULT.assert_owner(deps.storage, &info.sender)?;
    let lst_denom = LST_DENOM.load(deps.storage)?;
    assert_funds_single_token(&info.funds, &lst_denom)?;

    // let mut transfer_msgs = app.bank(deps.as_ref()).deposit(info.funds.clone())?;
    // // ibc transfer
    // let ibc_client = app.ibc_client(deps.as_ref());
    // let ibc_msg = ibc_client.ics20_transfer(
    //     ChainName::from_chain_id("stargaze-1").to_string(),
    //     info.funds,
    // )?;
    // transfer_msgs.push(ibc_msg);
    // "msgs": [
    //     {
    //       "type": "cosmos-sdk/MsgTransfer",
    //       "value": {
    //         "receiver": "stride1587yfq507a3pmutq4qtw8rx78cpcvga4f8mt5e",
    //         "sender": "osmo1587yfq507a3pmutq4qtw8rx78cpcvga4zhg8k8",
    //         "source_channel": "channel-326",
    //         "source_port": "transfer",
    //         "timeout_height": {
    //           "revision_height": "9720877",
    //           "revision_number": "1"
    //         },
    //         "token": {
    //           "amount": "100000",
    //           "denom": "ibc/A8CA5EE328FA10C9519DF6057DA1F69682D28F7D0F5CCC7ECB72E3DCA2D157A4"
    //         }
    //       }
    //     }
    //   ],
    // let msg = IbcMsg::Transfer {
    //     channel_id: "channel-0".to_string(),
    //     to_address: app
    //         .ibc_client(deps.as_ref())
    //         .remote_proxy_addr("stargaze")?
    //         .unwrap(),
    //     amount: info.funds[0].clone(),
    //     timeout: IbcTimeout::with_block(IbcTimeoutBlock {
    //         revision: 5,
    //         height: env.block.height + 5,
    //     }),
    // };
    let m = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: "channel-0".to_string(),
        token: Some(cosmwasm_to_proto_coins(info.funds)[0].clone()),
        sender: env.contract.address.to_string(),
        receiver: app
            .ibc_client(deps.as_ref())
            .remote_proxy_addr("stargaze")?
            .unwrap(),
        timeout_height: Some(Height {
            revision_number: 5,
            revision_height: env.block.height + 5,
        }),
        timeout_timestamp: env.block.time.nanos() + 100_000_000_000,
        memo: "".to_string(),
    };
    // let stargate_msg = CosmosMsg::Stargate {
    //     type_url: "/ibc.applications.transfer.v1.MsgTransfer".to_string(),
    //     value: to_json_binary(&m)?,
    // };
    Ok(app.response("unbond").add_message(m))
    // Ok(app.response("unbond").add_messages(transfer_msgs))
}

fn claim(deps: DepsMut, env: Env, info: MessageInfo, app: LstAdapter) -> LstAdapterResult {
    VAULT.assert_owner(deps.storage, &info.sender)?;
    Ok(app.response("claim"))
}

pub fn query_(
    _deps: Deps,
    _env: Env,
    _app: &LstAdapter,
    _msg: LstAdapterQueryMsg,
) -> LstAdapterResult<Binary> {
    Ok(Binary::default())
    // match msg {
    //     QueryMsg::Config {} => to_json_binary(&STATE.load(deps.storage)?),
    //     QueryMsg::Pending { address } => to_json_binary(&query_pending(deps, address)?),
    // }
}

pub fn migrate_(
    _deps: DepsMut,
    _env: Env,
    app: LstAdapter,
    _msg: LstAdapterMigrateMsg,
) -> LstAdapterResult {
    Ok(app.response("migrate"))
}
