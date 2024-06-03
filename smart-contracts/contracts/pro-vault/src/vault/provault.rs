use cosmwasm_std::{
    ensure, Addr, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, StdError,
};
use cw_controllers::Admin;
use cw_storage_plus::Item;
use cosmwasm_schema::cw_serde;

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::error::ContractError;
use crate::vault::error::VaultError;
use crate::strategy::strategy::{Strategy, STRATEGY, StrategyKey};
use crate::ownership::ownership::{
    OwnerProposal, Ownership, query_owner, query_ownership_proposal,
    handle_claim_ownership, handle_ownership_proposal, handle_ownership_proposal_rejection
};

pub const VAULT_OWNER: Admin = Admin::new("vault_owner");
pub const VAULT_PROPOSAL: Item<OwnerProposal> = Item::new("vault_proposal");
pub const VAULT_STATE: Item<Vault> = Item::new("vault_state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] 
pub enum VaultRunningState {
  // Initalized, and waiting come Normal once vault is ready to accept deposit. 
  Init, 
  // Normal operating mode
  Running, 
  // Temporary halted
  Paused, 
  // Terminated forever 
  Terminated, 
}

#[cw_serde]
pub enum VaultAction {
    UpdateRunningState {
        new_state: VaultRunningState,
    },
    UpdateVaultOwner {},
    UpdateStrategyOwner {},
    CreateStrategy {
        name: String,
        description: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vault {
    pub state: VaultRunningState,
    pub last_statechange_bh: u64,
}

impl Vault {
    pub fn new() -> Self {
        Vault {
            state: VaultRunningState::Init,
            last_statechange_bh: 0,
        }
    }

    pub fn update_state(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        new_state: VaultRunningState,
    ) -> Result<Response, ContractError> {
        let owner = VAULT_OWNER.get(deps.as_ref())?;
        ensure!(owner == Some(info.sender), ContractError::Unauthorized {});

        self.state = new_state;
        self.last_statechange_bh = env.block.height;

        VAULT_STATE.save(deps.storage, self)?;

        Ok(Response::new()
            .add_attribute("action", "update_state")
            .add_attribute("new_state", format!("{:?}", self.state))
            .add_attribute("last_statechange_bh", self.last_statechange_bh.to_string()))
    }

    fn update_state_internal(
        &mut self,
        storage: &mut dyn Storage,
        new_state: VaultRunningState,
        bh: u64,
    ) -> Result<Response, ContractError> {
        self.state = new_state;
        self.last_statechange_bh = bh;

        VAULT_STATE.save(storage, self)?;
        Ok(Response::new()
            .add_attribute("action", "update_state")
            .add_attribute("new_state", format!("{:?}", self.state))
            .add_attribute("last_statechange_bh", self.last_statechange_bh.to_string()))
    }

    pub fn execute_action(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        action: VaultAction,
    ) -> Result<Response, ContractError> {
        let mut vault = VAULT_STATE.load(deps.storage)?;
        // TODO - 
        // 1. PROVAULT METHOD PROTECTION BY OWNER
        // 2. STATE WISE ACTION PROTECTION, in which state what action can be performed.
        match action {
            VaultAction::CreateStrategy { name, description } => {
                Self::try_create_strategy(deps, env, info, name, description)
            }
            VaultAction::UpdateRunningState { new_state } => {
                Self::try_update_running_state(deps, env, info, new_state)
            }
            VaultAction::UpdateStrategyOwner {} => {
                Self::try_update_strategy_owner(deps)
            }
            VaultAction::UpdateVaultOwner {} => {
                Self::try_update_vault_owner(deps)
            }
        }
    }

    fn try_update_running_state(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        new_state: VaultRunningState,
    ) -> Result<Response, ContractError> {
        let mut vault: Vault = VAULT_STATE.load(deps.storage)?;
        vault.update_state(deps, env, info, new_state)?;

        Ok(Response::new()
            .add_attribute("method", "try_update_running_state"))
    }

    fn try_update_strategy_owner(
        deps: DepsMut,
    ) -> Result<Response, ContractError> {
        // Implementation for UpdateStrategyOwner
        Ok(Response::new()
            .add_attribute("method", "try_update_strategy_owner"))
    }

    pub fn try_create_strategy(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        name: String,
        description: String,
    ) -> Result<Response, ContractError> {
        if STRATEGY.has(deps.storage, &StrategyKey::new(1)) {
            return Err(VaultError::StrategyAlreadyExists {}.into());
        }

        let strategy = Strategy {
            id: 1,
            name: name.clone(),
            description: description.clone(),
        };

        STRATEGY.save(deps.storage, &StrategyKey::new(1), &strategy)?;

        Ok(Response::new()
            .add_attribute("action", "create_strategy")
            .add_attribute("strategy_id", "1")
            .add_attribute("strategy_name", name)
            .add_attribute("strategy_description", description))
    }

    fn try_update_vault_owner(
        deps: DepsMut,
    ) -> Result<Response, ContractError> {
        // Implementation for UpdateVaultOwner
        Ok(Response::new()
            .add_attribute("method", "try_update_vault_owner"))
    }
}

impl Ownership for Vault {
    fn handle_ownership_proposal(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        proposed_owner: String,
        duration: u64,
        owner: &Admin,
        proposal: &Item<OwnerProposal>,
    ) -> Result<Response, ContractError> {
        handle_ownership_proposal(deps, info, env, proposed_owner, duration, owner, proposal)
    }

    fn handle_ownership_proposal_rejection(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        owner: &Admin,
        proposal: &Item<OwnerProposal>,
    ) -> Result<Response, ContractError> {
        handle_ownership_proposal_rejection(deps, info, owner, proposal)
    }

    fn handle_claim_ownership(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        owner: &Admin,
        proposal: &Item<OwnerProposal>,
    ) -> Result<Response, ContractError> {
        handle_claim_ownership(deps, info, env, owner, proposal)
    }

    fn query_ownership_proposal(
        &self,
        deps: Deps,
        proposal: &Item<OwnerProposal>,
    ) -> StdResult<OwnerProposal> {
        query_ownership_proposal(deps, proposal)
    }

    fn query_owner(&self, deps: Deps, owner: &Admin) -> StdResult<Option<Addr>> {
        query_owner(deps, owner)
    }
}
