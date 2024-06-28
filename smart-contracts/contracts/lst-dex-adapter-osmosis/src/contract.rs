use crate::{
    error::DexAdapterError,
    handlers,
    msg::{
        DexAdapterExecuteMsg, DexAdapterInstantiateMsg, DexAdapterMigrateMsg, DexAdapterQueryMsg,
    },
    replies::{self, SWAP_REPLY_ID},
    APP_VERSION, MY_APP_ID,
};

use abstract_app::AppContract;
#[cfg(not(target_arch = "wasm32"))]
use abstract_app::{objects::module::ModuleInfo, std::manager::ModuleInstallConfig};
#[allow(unused)]
use abstract_app::{std::objects::dependency::StaticDependency, traits::AbstractNameService};
use abstract_dex_adapter::DEX_ADAPTER_ID;
#[cfg(not(target_arch = "wasm32"))]
use abstract_interface::AbstractInterfaceError;
use cosmwasm_std::Response;
const DEX_DEP: StaticDependency = StaticDependency::new(DEX_ADAPTER_ID, &[">=0.3.0"]);

pub type DexAdapterResult<T = Response> = Result<T, DexAdapterError>;

pub type DexAdapter = AppContract<
    DexAdapterError,
    DexAdapterInstantiateMsg,
    DexAdapterExecuteMsg,
    DexAdapterQueryMsg,
    DexAdapterMigrateMsg,
>;

const APP: DexAdapter = DexAdapter::new(MY_APP_ID, APP_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    .with_migrate(handlers::migrate_handler)
    .with_dependencies(&[])
    .with_replies(&[(SWAP_REPLY_ID, replies::swap_reply)])
    .with_dependencies(&[DEX_DEP]);

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, DexAdapter);

abstract_app::cw_orch_interface!(APP, DexAdapter, DexAdapterInterface);

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::DexAdapterInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;

    fn dependency_install_configs(
        _configuration: Self::DependenciesConfig,
    ) -> Result<Vec<ModuleInstallConfig>, AbstractInterfaceError> {
        let adapter_install_config = ModuleInstallConfig::new(
            ModuleInfo::from_id(
                abstract_dex_adapter::DEX_ADAPTER_ID,
                abstract_dex_adapter::contract::CONTRACT_VERSION.into(),
            )?,
            None,
        );

        Ok(vec![adapter_install_config])
    }
}
