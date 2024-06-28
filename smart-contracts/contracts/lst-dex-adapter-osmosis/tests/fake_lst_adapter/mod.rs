use abstract_app::objects::ans_host::AnsHostError;
use abstract_app::sdk::AbstractResponse;
use abstract_app::sdk::AbstractSdkError;
use abstract_app::std::AbstractError;
use abstract_app::AppError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdError;
use cosmwasm_std::{to_json_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response};
use cw_asset::AssetError;
use cw_storage_plus::Item;
use quasar_types::lst_adapter::RedemptionRate;
use thiserror::Error;

#[cfg(not(target_arch = "wasm32"))]
use abstract_app::std::manager::ModuleInstallConfig;
use abstract_app::AppContract;
#[allow(unused)]
use abstract_app::{std::objects::dependency::StaticDependency, traits::AbstractNameService};

#[cfg(not(target_arch = "wasm32"))]
use abstract_interface::AbstractInterfaceError;

pub const STATE: Item<Decimal> = Item::new("state");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MY_NAMESPACE: &str = "quasar";
pub const MY_APP_NAME: &str = "fake-lst-adapter";
pub const MY_APP_ID: &str = const_format::formatcp!("{MY_NAMESPACE}:{MY_APP_NAME}");

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Abstract(#[from] AbstractError),

    #[error("{0}")]
    AbstractSdk(#[from] AbstractSdkError),

    #[error("{0}")]
    AnsHost(#[from] AnsHostError),

    #[error("{0}")]
    Asset(#[from] AssetError),

    #[error("{0}")]
    DappError(#[from] AppError),
}

#[cw_serde]
pub struct FakeLstInstantiateMsg {
    pub redemption_rate: Decimal,
}

#[cw_serde]
pub struct FakeLstMigrateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum FakeLstExecuteMsg {
    Update { redemption_rate: Decimal },
}

#[cw_serde]
pub enum FakeLstQueryMsg {
    RedemptionRate {},
}

pub fn instantiate_(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _app: FakeLst,
    msg: FakeLstInstantiateMsg,
) -> Result<Response, ContractError> {
    STATE.save(deps.storage, &msg.redemption_rate)?;

    Ok(Response::new())
}

pub fn execute_(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _app: FakeLst,
    msg: FakeLstExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        FakeLstExecuteMsg::Update { redemption_rate } => {
            STATE.update(deps.storage, |_| -> Result<_, ContractError> {
                Ok(redemption_rate)
            })?;
        }
    }
    Ok(Response::new())
}

pub fn query_(
    deps: Deps,
    _env: Env,
    _app: &FakeLst,
    msg: FakeLstQueryMsg,
) -> Result<Binary, ContractError> {
    match msg {
        FakeLstQueryMsg::RedemptionRate {} => Ok(to_json_binary(&RedemptionRate {
            redemption_rate: STATE.load(deps.storage)?,
        })?),
    }
}

pub fn migrate_(
    _deps: DepsMut,
    _env: Env,
    app: FakeLst,
    _msg: FakeLstMigrateMsg,
) -> Result<Response, ContractError> {
    Ok(app.response("migrate"))
}

pub type FakeLst = AppContract<
    ContractError,
    FakeLstInstantiateMsg,
    FakeLstExecuteMsg,
    FakeLstQueryMsg,
    FakeLstMigrateMsg,
>;

abstract_app::app_msg_types!(FakeLst, FakeLstExecuteMsg, FakeLstQueryMsg);

const APP: FakeLst = FakeLst::new(MY_APP_ID, APP_VERSION, None)
    .with_instantiate(instantiate_)
    .with_execute(execute_)
    .with_query(query_)
    .with_migrate(migrate_)
    .with_replies(&[])
    .with_dependencies(&[]);

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, FakeLst);

abstract_app::cw_orch_interface!(APP, FakeLst, FakeLstInterface);

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::fake_lst_adapter::interface::FakeLstInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;

    fn dependency_install_configs(
        _configuration: Self::DependenciesConfig,
    ) -> Result<Vec<ModuleInstallConfig>, AbstractInterfaceError> {
        Ok(vec![])
    }
}
