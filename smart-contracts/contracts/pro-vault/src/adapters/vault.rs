use cosmwasm_std::{to_json_binary, Addr, Binary, Coin, Response, WasmMsg};

use super::r#trait::{Adapter, VaultAdapter};

struct SingeAssetVaultAdapterWrapper {
    pub address: Addr,
}

impl VaultAdapter for SingeAssetVaultAdapterWrapper {
    type AdapterError;


    type ActionConfig;

    fn deposit(assets: Vec<Coin>) -> Result<Response, Self::AdapterError> {

        todo!()
    }

    fn withdraw(shares: Coin) -> Result<Response, Self::AdapterError> {
        todo!()
    }

    fn claim_incentives() -> Result<Response, Self::AdapterError> {
        todo!()
    }


}

impl SingeAssetVaultAdapterWrapper {
    fn call(contract_addr: Addr, msg: Binary, funds: Vec<Coin>) -> Result<Response, <SingeAssetVaultAdapterWrapper as VaultAdapter>::AdapterError> {
        Ok(Response::new().add_message(to_json_binary(&WasmMsg::Execute { contract_addr, msg, funds })?))
    }
}

impl Adapter for MultiAssetVaultAdapterWrapper {
    const IDENTIFIER: String = Self::address;

    fn assets_balance() -> () {
        todo!()
    }

    fn base_asset_balance() -> () {
        todo!()
    }
}

struct MultiAssetVaultAdapterWrapper {
    pub address: Addr,
}

impl VaultAdapter for MultiAssetVaultAdapterWrapper {
    type AdapterError;


    type ActionConfig;

    fn deposit(assets: Vec<Coin>) -> Result<Response, Self::AdapterError> {

        todo!()
    }

    fn withdraw(shares: Coin) -> Result<Response, Self::AdapterError> {
        todo!()
    }

    fn claim_incentives() -> Result<Response, Self::AdapterError> {
        todo!()
    }


}

impl MultiAssetVaultAdapterWrapper {
    fn call(contract_addr: Addr, msg: Binary, funds: Vec<Coin>) -> Result<Response, <SingeAssetVaultAdapterWrapper as VaultAdapter>::AdapterError> {
        Ok(Response::new().add_message(to_json_binary(&WasmMsg::Execute { contract_addr, msg, funds })?))
    }
}

impl Adapter for MultiAssetVaultAdapterWrapper {
    const IDENTIFIER: String = Self::address;

    fn assets_balance() -> () {
        todo!()
    }

    fn base_asset_balance() -> () {
        todo!()
    }
}

