use crate::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{testing::MockApi, Addr, Empty};
use cw_orch::interface;
use cw_orch::mock::Mock;
use cw_orch::prelude::*;
use cw_orch::{contract::interface_traits::CwOrchUpload, mock::MockBase};

pub const SENDER: &str = "sender";

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Vault;

impl<Chain> Uploadable for Vault<Chain> {
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
    }
}

pub struct TestEnv<T: cosmwasm_std::Api> {
    pub chain: MockBase<T>,
    pub vault: Vault<MockBase<T>>,
}

pub fn create_test_vault() -> TestEnv<MockApi> {
    let sender = Addr::unchecked(SENDER);
    let chain = Mock::new(&sender);

    let vault: Vault<Mock> = Vault::new("vault", chain.clone());
    vault.upload().unwrap();
    TestEnv { chain, vault }
}
