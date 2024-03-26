pub use anyhow::Result;
pub use derivative::Derivative;

pub use crate::contract::{execute, instantiate, query};
pub use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};
pub use cosmwasm_std::{coin, BlockInfo, Coin, Decimal, Empty, StdResult, Uint128};
pub use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};

pub const USER: &str = "user";
pub const DEPLOYER: &str = "deployer";
pub const EXECUTOR: &str = "executor";
pub const DENOM: &str = "uosmo";
pub const LOCAL_DENOM: &str = "ibc/ilovemymom";

pub fn contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}
