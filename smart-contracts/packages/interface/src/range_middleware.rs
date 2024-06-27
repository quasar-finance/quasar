use cw_orch::{interface, prelude::*};

use range_middleware::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

pub const CONTRACT_ID: &str = "range_middleware";

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg, id = CONTRACT_ID)]
pub struct RangeMiddlewareContract;

impl<Chain> Uploadable for RangeMiddlewareContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("range_middleware")
            .unwrap()
    }
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                range_middleware::contract::execute,
                range_middleware::contract::instantiate,
                range_middleware::contract::query,
            )
            .with_migrate(range_middleware::contract::migrate),
        )
    }
}
