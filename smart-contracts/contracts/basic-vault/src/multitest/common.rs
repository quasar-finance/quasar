pub use anyhow::Result;
pub use derivative::Derivative;

pub use crate::contract::{
    execute as execute_vault, instantiate as instantiate_vault, query as query_vault,
};
pub use crate::{
    error::ContractError as VaultContractError,
    msg::{
        ExecuteMsg as VaultExecuteMsg, InstantiateMsg as VaultInstantiateMsg,
        QueryMsg as VaultQueryMsg,
    },
};
pub use cosmwasm_std::{coin, BlockInfo, Coin, Decimal, Empty, StdResult, Uint128};
pub use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};

pub use cw_utils::Duration;

pub use lp_strategy::{
    contract::{
        execute as execute_primitive, instantiate as instantiate_primitive,
    },
    queries::query as query_primitive,
    msg::{
        ExecuteMsg as PrimitiveExecuteMsg, InstantiateMsg as PrimitiveInstantiateMsg,
        QueryMsg as PrimitiveQueryMsg,
    },
};

pub const USER: &str = "user";
pub const DEPLOYER: &str = "deployer";
pub const EXECUTOR: &str = "executor";
pub const DENOM: &str = "uosmo";
pub const LOCAL_DENOM: &str = "ibc/ilovemymom";

pub fn contract_vault() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute_vault, instantiate_vault, query_vault);
    Box::new(contract)
}

pub fn contract_primitive() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute_primitive, instantiate_primitive, query_primitive);
    Box::new(contract)
}
