use std::string;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Addr, Coin, Env, QuerierWrapper, Response, StdError};
use cw_utils::PaymentError;
// use cw_vault_multi_standard::VaultInfoResponse as MultiVaultInfoResponse;
use cw_vault_standard::{VaultInfoResponse, VaultStandardExecuteMsg};
// use crate::{ContractError, ownership::{Ownership, Admin, OwnerProposal}};
use crate::error::ContractError;
use crate::ownership::ownership::{OwnerProposal,Ownership,query_owner};
use cw_storage_plus::Item;
use crate::adaptors::base_adaptor::{Adapter, AdapterMetadata};
use cw_controllers::Admin;
use crate::msg::ExtensionExecuteMsg;

#[cw_serde]
pub enum VaultAction {
    /// Deposit into the vault
    Deposit { assets: Vec<Coin> },
    /// Withdraw from the vault
    Withdraw { shares: Coin },
    /// Claim any incentives from the vault
    Claim {},
}

/// Trait for vault adapters
// pub trait VaultAdapter: Adapter + Ownership {
pub trait VaultAdapter: Adapter {

    type AdapterError;

    fn deposit(&self, assets: Vec<Coin>) -> Result<Response, Self::AdapterError>;
    fn withdraw(&self, shares: Coin) -> Result<Response, Self::AdapterError>;
    fn claim_incentives(&self) -> Result<Response, Self::AdapterError>;
}

/// Single asset vault adapter
pub struct SingleAssetVaultAdapter {
    //pub address: Addr, 
    pub metadata: AdapterMetadata,
    // pub owner: Admin,
    // Owner of the adaptor will be same as owner of the strategy. Or 
    // provault should be the owner of the adaptor.
    // pub owner : String,
    // pub ownership_proposal: OwnerProposal,
}


impl VaultAdapter for SingleAssetVaultAdapter {
    type AdapterError = ContractError;

    fn deposit(&self, assets: Vec<Coin>) -> Result<Response, Self::AdapterError> {
        // Ensure only one asset is deposited
        if assets.len() != 1 || assets[0].amount.is_zero() {
            return Err(ContractError::InvalidFundsAmount { });
        }

        let msg :VaultStandardExecuteMsg<ExtensionExecuteMsg> = VaultStandardExecuteMsg::Deposit {
            amount: assets[0].amount,
            recipient: None,
        };
        // Convert String to Addr
        let dest_contract_addr = Addr::unchecked(&self.metadata.dest_contract_addr); 

        Ok(Self::execute_call(dest_contract_addr, to_json_binary(&msg)?, assets)?)
    }

    fn withdraw(&self, shares: Coin) -> Result<Response, Self::AdapterError> {
        if shares.amount.is_zero() {
            return Err(ContractError::InvalidFundsAmount { } );
        }

        let msg : VaultStandardExecuteMsg<ExtensionExecuteMsg> = VaultStandardExecuteMsg::Redeem {
            recipient: None,
            amount: shares.amount,};
        
        // Convert String to Addr
        let dest_contract_addr = Addr::unchecked(&self.metadata.dest_contract_addr); 
        Ok(Self::execute_call(dest_contract_addr, to_json_binary(&msg)?, vec![shares])?)
    }

    fn claim_incentives(&self) -> Result<Response, Self::AdapterError> {
        todo!()
    }
}


impl Adapter for SingleAssetVaultAdapter {
    fn metadata(&self) -> AdapterMetadata {
        self.metadata.clone()
    }

    fn query_net_assets(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {
        // TODO - Either to check the position management or query the other real adaptor.
        Ok(vec![])
    }

    fn query_expected_available_assets(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {

       // TODO - Either to check the position management or query the other real adaptor.
        Ok(vec![])
    }

    fn query_allocated_shares(&self, querier: &QuerierWrapper, env: Env) -> Result<Coin, StdError> {
        // TODO - Either to check the position management or query the other real adaptor.
        Ok(Coin::default())
    }



    /* 
    fn assets_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {
        let query = cw_vault_standard::VaultStandardQueryMsg::Info {};
        let info: VaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_binary(&query)?)?;

        let balance = querier.query_balance(env.contract.address, info.base_token)?;

        Ok(vec![balance])
    }

    fn vault_token_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Coin, StdError> {
        let query = cw_vault_standard::VaultStandardQueryMsg::Info {};
        let info: VaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_binary(&query)?)?;

        let balance = querier.query_balance(env.contract.address, info.vault_token)?;

        Ok(balance)
    }
    */
}


/* 
/// Multi-asset exact deposit vault adapter
pub struct MultiAssetExactDepositVaultAdapter {
    pub address: Addr,
    pub metadata: AdapterMetadata,
    pub owner: Admin,
    pub ownership_proposal: Item<OwnerProposal>,
}

impl VaultAdapter for MultiAssetExactDepositVaultAdapter {
    type AdapterError = ContractError;

    fn deposit(&self, assets: Vec<Coin>) -> Result<Response, ContractError> {
        if assets.is_empty() {
            return Err(ContractError::InvalidFunds {});
        }

        let msg = cw_vault_multi_standard::VaultStandardExecuteMsg::ExactDeposit { recipient: None };

        Ok(Self::execute_call(self.address.clone(), to_binary(&msg)?, assets)?)
    }

    fn withdraw(&self, shares: Coin) -> Result<Response, ContractError> {
        if shares.amount.is_zero() {
            return Err(ContractError::InvalidFunds {});
        }

        let msg = cw_vault_multi_standard::VaultStandardExecuteMsg::Redeem {
            recipient: None,
            amount: shares.amount,
        };

        Ok(Self::execute_call(self.address.clone(), to_binary(&msg)?, vec![shares])?)
    }

    fn claim_incentives(&self) -> Result<Response, ContractError> {
        todo!()
    }
}

impl Adapter for MultiAssetExactDepositVaultAdapter {
    fn metadata(&self) -> AdapterMetadata {
        self.metadata.clone()
    }

    fn assets_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {
        let query = cw_vault_multi_standard::VaultStandardQueryMsg::Info {};
        let info: MultiVaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_binary(&query)?)?;

        let balances: Result<Vec<Coin>, StdError> = info
            .tokens
            .iter()
            .map(|token| querier.query_balance(&env.contract.address, token))
            .collect();

        balances
    }

    fn vault_token_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Coin, StdError> {
        let query = cw_vault_multi_standard::VaultStandardQueryMsg::Info {};
        let info: MultiVaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_binary(&query)?)?;

        let balance = querier.query_balance(env.contract.address, info.vault_token)?;

        Ok(balance)
    }
}

/// Multi-asset any deposit vault adapter
pub struct MultiAssetAnyDepositVaultAdapter {
    pub address: Addr,
    pub metadata: AdapterMetadata,
    pub owner: Admin,
    pub ownership_proposal: Item<OwnerProposal>,
}

impl VaultAdapter for MultiAssetAnyDepositVaultAdapter {
    type AdapterError = ContractError;

    fn deposit(&self, assets: Vec<Coin>) -> Result<Response, ContractError> {
        if assets.is_empty() {
            return Err(ContractError::InvalidFunds {});
        }

        let msg = cw_vault_multi_standard::VaultStandardExecuteMsg::AnyDeposit { recipient: None };

        Ok(Self::execute_call(self.address.clone(), to_binary(&msg)?, assets)?)
    }

    fn withdraw(&self, shares: Coin) -> Result<Response, ContractError> {
        if shares.amount.is_zero() {
            return Err(ContractError::InvalidFunds {});
        }

        let msg = cw_vault_multi_standard::VaultStandardExecuteMsg::Redeem {
            recipient: None,
            amount: shares.amount,
        };

        Ok(Self::execute_call(self.address.clone(), to_binary(&msg)?, vec![shares])?)
    }

    fn claim_incentives(&self) -> Result<Response, ContractError> {
        todo!()
    }
}

impl Adapter for MultiAssetAnyDepositVaultAdapter {
    fn metadata(&self) -> AdapterMetadata {
        self.metadata.clone()
    }

    fn assets_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {
        let query = cw_vault_multi_standard::VaultStandardQueryMsg::Info {};
        let info: MultiVaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_binary(&query)?)?;

        let balances: Result<Vec<Coin>, StdError> = info
            .tokens
            .iter()
            .map(|token| querier.query_balance(&env.contract.address, token))
            .collect();

        balances
    }

    fn vault_token_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Coin, StdError> {
        let query = cw_vault_multi_standard::VaultStandardQueryMsg::Info {};
        let info: MultiVaultInfoResponse = querier.query_wasm_smart(self.address.clone(), &to_binary(&query)?)?;

        let balance = querier.query_balance(env.contract.address, info.vault_token)?;

        Ok(balance)
    }
}
*/
