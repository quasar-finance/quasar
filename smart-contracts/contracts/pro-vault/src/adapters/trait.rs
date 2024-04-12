use cosmwasm_std::{Addr, Coin, Response};
use osmosis_std::types::cosmos::app::v1alpha1::Config;

pub trait Adapter {
    const IDENTIFIER: String;

    /// describes the effective balance of the vault in the adapter
    fn assets_balance() -> ();

    /// descrives the base asset balance of the vault in the adapter
    fn base_asset_balance() -> ();
}

pub trait VaultAdapter<T: Adapter> {
    type AdapterError;

    type ActionConfig;

    fn deposit(assets: Vec<Coin>) -> Result<Response, Self::AdapterError>;

    fn withdraw(shares: Coin) -> Result<Response, Self::AdapterError>;

    fn claim_incentives() -> Result<Response, Self::AdapterError>;
}

pub trait DebtAdapter<T: Adapter> {
    type AdapterError;
    type Config;

    fn deposit_collateral(assets: Vec<Coin>) -> Result<Response, Self::AdapterError>;

    fn withdraw_collateral(shares: Coin) -> Result<Response, Self::AdapterError>;

    fn borrow(want: Vec<Coin>) -> Result<Response, Self::AdapterError>;

    fn repay(assets: Vec<Coin>) -> Result<Response, Self::AdapterError>;
}

pub trait SwapAdapter<T: Adapter> {
    type AdapterError;
    type Config;
    type SwapConfig;

    fn swap(asset_in: Coin, asset_out: String, swap_config: Config);
}
