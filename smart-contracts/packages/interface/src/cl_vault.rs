use cw_orch::{interface, prelude::*};

use cl_vault::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

pub const CONTRACT_ID: &str = "cl_vault";

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg, id = CONTRACT_ID)]
pub struct ClVaultContract;

impl<Chain> Uploadable for ClVaultContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cl_vault")
            .unwrap()
    }
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                cl_vault::contract::execute,
                cl_vault::contract::instantiate,
                cl_vault::contract::query,
            )
            .with_migrate(cl_vault::contract::migrate),
        )
    }
}
