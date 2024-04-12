use cosmwasm_std::{to_json_binary, Addr, Coin, QuerierWrapper, Response, StdError};
use cw_utils::PaymentError;
use cw_vault_standard::VaultInfoResponse;

use super::r#trait::{Adapter, VaultAdapter};

struct SingeAssetVaultAdapterWrapper {
    pub address: Addr,
}

impl VaultAdapter<SingeAssetVaultAdapterWrapper> for SingeAssetVaultAdapterWrapper {
    type AdapterError = <SingeAssetVaultAdapterWrapper as Adapter>::AdapterError;

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

    fn assets_balance(&self, querier: &QuerierWrapper) -> Result<Vec<cosmwasm_std::Coin>, cosmwasm_std::StdError> {
        let info: VaultInfoResponse = querier.query_wasm_smart(self.address, &to_json_binary(&cw_vault_standard::VaultStandardQueryMsg::Info {})?)?;
        
        let balance = querier.query_balance(self.address, info.base_token);
        todo!()
    }

    fn base_asset_balance() -> () {
        todo!()
    }
    
    type AdapterError = StdError;
}

struct MultiAssetExactDepositVaultAdapterWrapper {
    pub address: Addr,
}

impl VaultAdapter<MultiAssetExactDepositVaultAdapterWrapper> for MultiAssetExactDepositVaultAdapterWrapper {
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

    fn assets_balance() -> () {
        todo!()
    }

    fn base_asset_balance() -> () {
        todo!()
    }
}

struct MultiAssetAnyDepositVaultAdapterWrapper {
    pub address: Addr,
}

impl VaultAdapter<MultiAssetAnyDepositVaultAdapterWrapper> for MultiAssetAnyDepositVaultAdapterWrapper {
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

    fn assets_balance() -> () {
        todo!()
    }

    fn base_asset_balance() -> () {
        todo!()
    }
}
