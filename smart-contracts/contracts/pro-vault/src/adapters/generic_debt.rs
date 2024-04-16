use cosmwasm_std::{Addr, Coin};

use crate::ContractError;

use super::r#trait::{Adapter, DebtAdapter};

pub enum DebtAction {
    DepositCollateral {
        assets: Vec<Coin>
    },
    WithdrawCollateral {

    }
}

pub struct DebtAdapterWrapper {
    address: Addr
}

impl DebtAdapter for DebtAdapterWrapper {
    type AdapterError = ContractError;

    fn deposit_collateral(self, assets: Vec<Coin>) -> Result<cosmwasm_std::Response, Self::AdapterError> {
        let msg = todo!();

        Ok(Self::call(self.address, msg, funds)?)
    }

    fn withdraw_collateral(self, asset: Coin) -> Result<cosmwasm_std::Response, Self::AdapterError> {
        todo!()
    }

    fn borrow(self, want: Vec<Coin>) -> Result<cosmwasm_std::Response, Self::AdapterError> {
        todo!()
    }

    fn repay(self, assets: Vec<Coin>) -> Result<cosmwasm_std::Response, Self::AdapterError> {
        todo!()
    }
}

impl Adapter for DebtAdapterWrapper {
    fn assets_balance(&self, querier: &cosmwasm_std::QuerierWrapper, env: cosmwasm_std::Env) -> Result<Vec<Coin>, cosmwasm_std::StdError> {
        todo!()
    }

    fn vault_token_balance(&self, querier: &cosmwasm_std::QuerierWrapper, env: cosmwasm_std::Env) -> Result<Coin, cosmwasm_std::StdError> {
        todo!()
    }
}