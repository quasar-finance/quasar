use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Decimal256, Uint128};
use cw_dex_router::operations::SwapOperationsListUnchecked;
use cw_storage_plus::{Deque, Item, Map};
use vaultenator::state::ManageState;

/// VAULT_CONFIG: Base config struct for the contract.
#[cw_serde]
pub struct VaultConfig {
    /// Percentage of profit to be charged as performance fee
    pub performance_fee: Decimal,
    /// Account to receive fee payments
    pub treasury: Addr,
    /// swap max slippage
    pub vault_denom: String,
    ///
    pub admin_address: String,
    /// the address that is allowed to take strategist actions
    pub strategist: String,
    /// the underlying thesis of the vault's positions, eg aggresive
    pub thesis: String,
    /// the name of the vault
    pub name: String,
}

impl ManageState for VaultConfig {
    const STATE_KEY: &'static str = "vault_config";

    fn is_contract_open(
        deps: cosmwasm_std::Deps,
    ) -> Result<bool, vaultenator::errors::ContractError> {
        Ok(true)
    }

    fn is_contract_paused(
        deps: cosmwasm_std::Deps,
    ) -> Result<bool, vaultenator::errors::ContractError> {
        Ok(false)
    }

    fn set_open(&mut self, open: bool) {
        unimplemented!()
    }

    fn set_paused(&mut self, paused: bool) {
        unimplemented!()
    }

    fn init_state(
        deps: &mut cosmwasm_std::DepsMut,
        env: &cosmwasm_std::Env,
    ) -> Result<(), vaultenator::errors::ContractError>
    where
        Self: Sized,
    {
        let initial_config = VaultConfig {
            performance_fee: Decimal::default(),
            treasury: Addr::unchecked(""),
            vault_denom: String::default(),
            admin_address: String::default(),
            strategist: String::default(),
            thesis: String::default(),
            name: String::default(),
        };

        initial_config.save_to_storage(deps)
    }

    fn update_state(
        &mut self,
        deps: &mut cosmwasm_std::DepsMut,
    ) -> Result<(), vaultenator::errors::ContractError> {
        self.save_to_storage(deps)
    }
}
