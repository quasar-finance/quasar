use cosmwasm_std::{to_json_binary, Addr, Binary, Coin, CosmosMsg, QuerierWrapper, Response, WasmMsg};
use osmosis_std::types::cosmos::app::v1alpha1::Config;

// TODO add some metadata to easily display multiple adapters and their destination/purpose
pub trait Adapter {
    type AdapterError;

    const IDENTIFIER: &'static str;

    /// describes the effective balance of the vault in the adapter
    fn assets_balance(&self, querier: &QuerierWrapper) -> Result<Vec<Coin>, Self::AdapterError>;

    /// descrives the base asset balance of the vault in the adapter
    fn base_asset_balance(&self, querier: &QuerierWrapper) -> Result<Coin, Self::AdapterError>;
}

pub trait VaultAdapter<T: Adapter> {
    type AdapterError;

    fn deposit(self, assets: Vec<Coin>) -> Result<Response, T::AdapterError>;

    fn withdraw(self, shares: Coin) -> Result<Response, T::AdapterError>;

    fn claim_incentives(self) -> Result<Response, T::AdapterError>;

    fn call(contract_addr: Addr, msg: Binary, funds: Vec<Coin>) -> Result<Response, T::AdapterError> {
        Ok(Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute { contract_addr: contract_addr.into(), msg, funds })))
    }
}

pub trait DebtAdapter<T: Adapter> {
    type Config;

    fn deposit_collateral(self, assets: Vec<Coin>) -> Result<Response, T::AdapterError>;

    fn withdraw_collateral(self, shares: Coin) -> Result<Response, T::AdapterError>;

    fn borrow(self, want: Vec<Coin>) -> Result<Response, T::AdapterError>;

    fn repay(self, assets: Vec<Coin>) -> Result<Response, T::AdapterError>;
}

pub trait SwapAdapter<T: Adapter> {
    type AdapterError;
    type SwapConfig;

    fn swap(self, asset_in: Coin, asset_out: String, swap_config: Config);
}
