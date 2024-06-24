use cosmwasm_std::{Addr, Binary, Coin, CosmosMsg, DepsMut, Deps, Env, QuerierWrapper, Response, StdError, Uint128, WasmMsg, Api, StdResult};
use cw_storage_plus::Item;

/// Enum for the different market types
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MarketType {
    Debt,
    CLVault,
    Swap,
}

/// Metadata struct to provide information about each adapter.
/// With the use of this metadata, any external adaptor contract 
/// is simply a plug and play by the strategy module which is further 
/// controlled by strategy owner.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdapterMetadata {
    pub name: String,
    pub desc: String,
    pub dest_chain_id: String,
    pub dest_contract_addr: String,
    pub dest_market_type: MarketType,
}

impl AdapterMetadata {
    pub fn new(
        name: String,
        desc: String,
        dest_chain_id: String,
        dest_contract_addr: String,
        dest_market_type: MarketType,
    ) -> Self {
        AdapterMetadata {
            name,
            desc,
            dest_chain_id,
            dest_contract_addr,
            dest_market_type,
        }
    }
}

// Storage keys for AdapterMetadata
const ADAPTER_METADATA: Item<AdapterMetadata> = Item::new("adapter_metadata");

/// Load AdapterMetadata from storage
pub fn load_metadata(storage: &dyn cosmwasm_std::Storage) -> StdResult<AdapterMetadata> {
    ADAPTER_METADATA.load(storage)
}

/// Store AdapterMetadata in storage
pub fn store_metadata(storage: &mut dyn cosmwasm_std::Storage, metadata: &AdapterMetadata) -> StdResult<()> {
    ADAPTER_METADATA.save(storage, metadata)
}

/// Adapter trait that defines the common behavior for all adapters
pub trait Adapter {
    /// Metadata for the adapter
    fn metadata(&self) -> AdapterMetadata;

    // Returns the total net assets available in the adaptor. ( allocated - withdraw ) 
    // It can go negative as well, if total withdraw > allocated. This is running value to be 
    // updated. This postion to be managed in the position manager module.
    // TODO - Rethink.
    fn query_net_assets(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError>;

    // TODO - Rethink. based on current query against the adaptor including pending unbonding.
    fn query_expected_available_assets(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError>;

    // Returns the shares associated to this adaptor at a given point of time. This is more 
    // suitable measure as compared to the asset related queries.
    fn query_allocated_shares(&self, querier: &QuerierWrapper, env: Env) -> Result<Coin, StdError>;

    /// Executes a call to another contract. The other contract in this case is an 
    /// adaptor contract. 
    fn execute_call(
        contract_addr: Addr,
        msg: Binary,
        funds: Vec<Coin>,
    ) -> Result<Response, StdError> {
        Ok(Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds,
        })))
    }
}

/// Trait for VaultAdapter with additional methods specific to vault operations
pub trait VaultAdapter: Adapter {
    type AdapterError;

    fn deposit_assets(&self, assets: Vec<Coin>) -> Result<Response, Self::AdapterError>;

    fn withdraw_assets(&self, shares: Coin) -> Result<Response, Self::AdapterError>;

    fn claim_incentives(&self) -> Result<Response, Self::AdapterError>;
}

/// Trait for DebtAdapter with additional methods specific to debt operations
pub trait DebtAdapter: Adapter {
    type AdapterError;

    fn deposit_collateral(&self, deps: DepsMut, assets: Vec<Coin>) -> Result<Response, Self::AdapterError>;

    fn withdraw_collateral(&self, deps: DepsMut, shares: Coin) -> Result<Response, Self::AdapterError>;

    //fn borrow_assets(&self, deps: DepsMut, want: Vec<Coin>) -> Result<Response, Self::AdapterError>;

    //fn repay_assets(&self, deps: DepsMut, assets: Vec<Coin>) -> Result<Response, Self::AdapterError>;
}

/// Trait for SwapAdapter with additional methods specific to swap operations
pub trait SwapAdapter: Adapter {
    type AdapterError;
    type SwapConfig;
    /*
    fn swap_assets(
        &self,
        asset_in: Coin,
        asset_out: String,
        swap_config: Config,
    ) -> Result<Response, Self::AdapterError>;
    */
}

/* Example usage 
/// Example implementation of a VaultAdapter
pub struct ExampleVaultAdapter {
    metadata: AdapterMetadata,
}

impl Adapter for ExampleVaultAdapter {
    fn metadata(&self) -> AdapterMetadata {
        self.metadata.clone()
    }

    fn query_net_assets(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {
        // Implement the logic to get the net assets
        unimplemented!()
    }

    fn query_expected_available_assets(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {
        // Implement the logic to get the expected available assets
        unimplemented!()
    }

    fn query_allocated_shares(&self, querier: &QuerierWrapper, env: Env) -> Result<Coin, StdError> {
        // Implement the logic to get the allocated shares
        unimplemented!()
    }
}

impl VaultAdapter for ExampleVaultAdapter {
    type AdapterError = StdError;

    fn deposit_assets(&self, assets: Vec<Coin>) -> Result<Response, Self::AdapterError> {
        // Implement the deposit logic
        unimplemented!()
    }

    fn withdraw_assets(&self, shares: Coin) -> Result<Response, Self::AdapterError> {
        // Implement the withdraw logic
        unimplemented!()
    }

    fn claim_incentives(&self) -> Result<Response, Self::AdapterError> {
        // Implement the claim incentives logic
        unimplemented!()
    }
}

// Similarly, implement DebtAdapter and SwapAdapter as needed

*/
