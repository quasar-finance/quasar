use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Addr, Coin, Env, QuerierWrapper, Response, StdError};
use cw_utils::PaymentError;
use cw_vault_multi_standard::VaultInfoResponse as MultiVaultInfoResponse;
use cw_vault_standard::VaultInfoResponse;

use crate::ContractError;

use super::r#trait::{Adapter, VaultAdapter};


#[cw_serde]
pub enum VaultAction {
    /// Deposit in to the vault
    Deposit {
        assets: Vec<Coin>
    },
    /// Withdraw from the vault
    Withdraw {
        shares: Coin,
    },
    /// Claim any incentives from the vault
    Claim {}
}

#[cw_serde]
pub enum VaultAdapters {
    SingleAsset,
    MultiAsssetExact,
    MultiAssetAny,
}

pub struct SingeAssetVaultAdapterWrapper {
    pub address: Addr,
}

impl VaultAdapter for SingeAssetVaultAdapterWrapper {
    type AdapterError = ContractError;

    fn deposit(self, assets: Vec<Coin>) -> Result<Response, Self::AdapterError> {
        let coin = match assets.len() {
            0 => Err(PaymentError::NoFunds {}),
            1 => {
                let coin = &assets[0];
                if coin.amount.is_zero() {
                    Err(PaymentError::NoFunds {})
                } else {
                    Ok(coin.clone())
                }
            }
            _ => Err(PaymentError::MultipleDenoms {}),
        }.map_err(|e| StdError::generic_err(e.to_string()))?;

        let msg: cw_vault_standard::VaultStandardExecuteMsg = cw_vault_standard::VaultStandardExecuteMsg::Deposit { amount: coin.amount, recipient: None };

        Self::call(self.address, to_json_binary(&msg)?, assets)
    }

    fn withdraw(self, shares: Coin) -> Result<Response, Self::AdapterError> {
        let msg: cw_vault_multi_standard::VaultStandardExecuteMsg  = cw_vault_multi_standard::VaultStandardExecuteMsg::Redeem { recipient: None, amount: shares.amount };

        Self::call(self.address, to_json_binary(&msg)?, vec![shares])
    }

    fn claim_incentives(self) -> Result<Response, Self::AdapterError> {
        todo!()
    }
}


impl Adapter for SingeAssetVaultAdapterWrapper {
    fn assets_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {
        let query: cw_vault_standard::VaultStandardQueryMsg  = cw_vault_standard::VaultStandardQueryMsg::Info {};
        let info: VaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_json_binary(&query)?)?;
        
        let balance = querier.query_balance(env.contract.address, info.base_token)?;

        Ok(vec![balance])
    }

    fn vault_token_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Coin, StdError> {
        let query: cw_vault_standard::VaultStandardQueryMsg  = cw_vault_standard::VaultStandardQueryMsg::Info {};
        let info: VaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_json_binary(&query)?)?;
        
        let balance = querier.query_balance(env.contract.address, info.vault_token)?;

        Ok(balance)
    }
}

pub struct MultiAssetExactDepositVaultAdapterWrapper {
    pub address: Addr,
}

impl VaultAdapter for MultiAssetExactDepositVaultAdapterWrapper {
    type AdapterError = ContractError;

    fn deposit(self, assets: Vec<Coin>) -> Result<Response, ContractError> {
        let msg: cw_vault_multi_standard::VaultStandardExecuteMsg  = cw_vault_multi_standard::VaultStandardExecuteMsg::ExactDeposit { recipient: None };

        Self::call(self.address, to_json_binary(&msg)?, assets)
    }

    fn withdraw(self, shares: Coin) -> Result<Response, ContractError> {
        let msg: cw_vault_multi_standard::VaultStandardExecuteMsg = cw_vault_multi_standard::VaultStandardExecuteMsg::Redeem { recipient: None, amount: shares.amount };

        Self::call(self.address, to_json_binary(&msg)?, vec![shares])
    }

    fn claim_incentives(self) -> Result<Response, ContractError> {
        todo!()
    }
}

impl Adapter for MultiAssetExactDepositVaultAdapterWrapper {
    fn assets_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {
        let query: cw_vault_multi_standard::VaultStandardQueryMsg  = cw_vault_multi_standard::VaultStandardQueryMsg::Info {};
        let info: MultiVaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_json_binary(&query)?)?;
        
        let balances: Result<Vec<Coin>, StdError> = info.tokens.iter().map(|token| {
            querier.query_balance(&env.contract.address, token)
        }).collect();

        balances
    }
    
    fn vault_token_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Coin, StdError> {
        let query: cw_vault_standard::VaultStandardQueryMsg  = cw_vault_standard::VaultStandardQueryMsg::Info {};
        let info: MultiVaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_json_binary(&query)?)?;
        
        let balance = querier.query_balance(env.contract.address, info.vault_token)?;

        Ok(balance)
    }

  
}

pub struct MultiAssetAnyDepositVaultAdapterWrapper {
    pub address: Addr,
}

impl VaultAdapter for MultiAssetAnyDepositVaultAdapterWrapper {
    type AdapterError = ContractError;

    fn deposit(self, assets: Vec<Coin>) -> Result<Response, ContractError> {
        let msg: cw_vault_multi_standard::VaultStandardExecuteMsg  = cw_vault_multi_standard::VaultStandardExecuteMsg::ExactDeposit { recipient: None };

        Self::call(self.address, to_json_binary(&msg)?, assets)
    }

    fn withdraw(self, shares: Coin) -> Result<Response, ContractError> {
        let msg: cw_vault_multi_standard::VaultStandardExecuteMsg  = cw_vault_multi_standard::VaultStandardExecuteMsg::Redeem { recipient: None, amount: shares.amount };

        Self::call(self.address, to_json_binary(&msg)?, vec![shares])
    }

    fn claim_incentives(self) -> Result<Response, ContractError> {
        todo!()
    }

 
}

impl Adapter for MultiAssetAnyDepositVaultAdapterWrapper {    
    fn assets_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {
        let query: cw_vault_multi_standard::VaultStandardQueryMsg  = cw_vault_multi_standard::VaultStandardQueryMsg::Info {};
        let info: MultiVaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_json_binary(&query)?)?;
        
        let balances: Result<Vec<Coin>, StdError> = info.tokens.iter().map(|token| {
            querier.query_balance(&env.contract.address, token)
        }).collect();

        balances
    }
    
    fn vault_token_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Coin, StdError> {
        let query: cw_vault_standard::VaultStandardQueryMsg  = cw_vault_standard::VaultStandardQueryMsg::Info {};
        let info: MultiVaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_json_binary(&query)?)?;
        
        let balance = querier.query_balance(env.contract.address, info.vault_token)?;

        Ok(balance)
    }
}
