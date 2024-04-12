use cosmwasm_std::{to_json_binary, Addr, Coin, Env, QuerierWrapper, Response, StdError};
use cw_utils::PaymentError;
use cw_vault_multi_standard::VaultInfoResponse as MultiVaultInfoResponse;
use cw_vault_standard::VaultInfoResponse;

use super::r#trait::{Adapter, VaultAdapter};

struct SingeAssetVaultAdapterWrapper {
    pub address: Addr,
}

impl VaultAdapter for SingeAssetVaultAdapterWrapper {
    type AdapterError = StdError;

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
    const IDENTIFIER: &'static str = "single_asset_vault_adapter";

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

struct MultiAssetExactDepositVaultAdapterWrapper {
    pub address: Addr,
}

impl VaultAdapter for MultiAssetExactDepositVaultAdapterWrapper {
    type AdapterError = StdError;


    fn deposit(self, assets: Vec<Coin>) -> Result<Response, Self::AdapterError> {
        let msg: cw_vault_multi_standard::VaultStandardExecuteMsg  = cw_vault_multi_standard::VaultStandardExecuteMsg::ExactDeposit { recipient: None };

        Self::call(self.address, to_json_binary(&msg)?, assets)
    }

    fn withdraw(self, shares: Coin) -> Result<Response, Self::AdapterError> {
        let msg: cw_vault_multi_standard::VaultStandardExecuteMsg = cw_vault_multi_standard::VaultStandardExecuteMsg::Redeem { recipient: None, amount: shares.amount };

        Self::call(self.address, to_json_binary(&msg)?, vec![shares])
    }

    fn claim_incentives(self) -> Result<Response, Self::AdapterError> {
        todo!()
    }
}

impl Adapter for MultiAssetExactDepositVaultAdapterWrapper {
    const IDENTIFIER: &'static str = "multi_asset_exact_depost_vault_adapter";
    
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

struct MultiAssetAnyDepositVaultAdapterWrapper {
    pub address: Addr,
}

impl VaultAdapter for MultiAssetAnyDepositVaultAdapterWrapper {
    type AdapterError = StdError;

    fn deposit(self, assets: Vec<Coin>) -> Result<Response, Self::AdapterError> {
        let msg: cw_vault_multi_standard::VaultStandardExecuteMsg  = cw_vault_multi_standard::VaultStandardExecuteMsg::ExactDeposit { recipient: None };

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

impl Adapter for MultiAssetAnyDepositVaultAdapterWrapper {
    const IDENTIFIER: &'static str = "multi_asset_vault_any_deposit_adapter";
    
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
