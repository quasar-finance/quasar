use cosmwasm_std::{
    to_json_binary, Addr, Binary, Coin, CosmosMsg, Env, QuerierWrapper, Response, StdError, WasmMsg,
};
use osmosis_std::types::cosmos::app::v1alpha1::Config;

use crate::ContractError;

// TODO add some metadata to easily display multiple adapters and their destination/purpose
pub trait Adapter {
    /// describes the effective balance of the vault in the adapter
    fn assets_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError>;

    /// descrives the base asset balance of the vault in the adapter
    fn vault_token_balance(&self, querier: &QuerierWrapper, env: Env) -> Result<Coin, StdError>;
}

pub trait VaultAdapter: Adapter {
    type AdapterError;

    fn deposit(self, assets: Vec<Coin>) -> Result<Response, Self::AdapterError>;

    fn withdraw(self, shares: Coin) -> Result<Response, Self::AdapterError>;

    fn claim_incentives(self) -> Result<Response, Self::AdapterError>;

    fn call(
        contract_addr: Addr,
        msg: Binary,
        funds: Vec<Coin>,
    ) -> Result<Response, Self::AdapterError> {
        Ok(
            Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: contract_addr.into(),
                msg,
                funds,
            })),
        )
    }
}

pub trait DebtAdapter<T: Adapter> {
    type AdapterError;
    type Config;

    fn deposit_collateral(self, assets: Vec<Coin>) -> Result<Response, Self::AdapterError>;

    fn withdraw_collateral(self, shares: Coin) -> Result<Response, Self::AdapterError>;

    fn borrow(self, want: Vec<Coin>) -> Result<Response, Self::AdapterError>;

    fn repay(self, assets: Vec<Coin>) -> Result<Response, Self::AdapterError>;
}

pub trait SwapAdapter<T: Adapter> {
    type AdapterError;
    type SwapConfig;

    fn swap(self, asset_in: Coin, asset_out: String, swap_config: Config);
}
