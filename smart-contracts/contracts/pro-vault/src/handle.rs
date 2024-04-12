use vaultenator::handlers::Handle;

use crate::{config::Config, state::VaultConfig, vault::Vault};

impl Handle<Config, VaultConfig> for Vault {
    fn handle_instantiate<M>(
        &self,
        deps: cosmwasm_std::DepsMut,
        env: cosmwasm_std::Env,
        info: cosmwasm_std::MessageInfo,
        msg: M,
    ) -> Result<cosmwasm_std::Response, vaultenator::errors::ContractError>
    where
        M: serde::Serialize + serde::de::DeserializeOwned,
    {
        todo!()
    }

    fn handle_update_config(
        &self,
        deps: cosmwasm_std::DepsMut,
        info: cosmwasm_std::MessageInfo,
    ) -> Result<cosmwasm_std::Response, vaultenator::errors::ContractError> {
        todo!()
    }

    fn handle_deposit(
        &self,
        deps: cosmwasm_std::DepsMut,
        env: cosmwasm_std::Env,
        info: cosmwasm_std::MessageInfo,
    ) -> Result<cosmwasm_std::Response, vaultenator::errors::ContractError> {
        todo!()
    }

    fn handle_redeem(
        &self,
        deps: cosmwasm_std::DepsMut,
        env: cosmwasm_std::Env,
        info: cosmwasm_std::MessageInfo,
    ) -> Result<cosmwasm_std::Response, vaultenator::errors::ContractError> {
        todo!()
    }
}
