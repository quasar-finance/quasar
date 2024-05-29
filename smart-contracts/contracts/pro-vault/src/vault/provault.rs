use cosmwasm_std::{
    attr, ensure, ensure_eq, Addr, Deps, DepsMut, Env, Event, MessageInfo, Response, StdError,
    StdResult, Binary, to_json_binary, Storage};
use cw_controllers::Admin;
use cw_storage_plus::Item;
use cosmwasm_schema::cw_serde;

use serde::{Serialize,Deserialize};
use schemars::JsonSchema;

use crate::vault::query::{VaultRunningStateResponse, StrategyInfoResponse};
use crate::error::ContractError;
use crate::strategy::strategy::{Strategy, STRATEGY, StrategyAction, StrategyKey}; 
use crate::ownership::ownership::{OwnerProposal, Ownership, query_owner, query_ownership_proposal, 
    handle_claim_ownership, handle_ownership_proposal, handle_ownership_proposal_rejection};


// Constants for the provault
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


// Pro vault state struct with last updated block height.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vault {
    pub state: VaultRunningState,
    pub last_statechange_bh: u64, // last statechange block height
}

  
// Vault method implementations
impl Vault {
     pub fn new() -> Self {
        Vault {
            state: VaultRunningState::Init,
            last_statechange_bh: 0,
        }
    }

    // To be called from update_vault_state entry point 
    pub fn update_state(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        new_state: VaultRunningState,
    ) -> Result<Response, ContractError> {
        // Verify that the sender is the current vault owner
        let owner = VAULT_OWNER.get(deps.as_ref())?;
        ensure!(owner == Some(info.sender), ContractError::Unauthorized {});

        // Update the state and last state change block height
        self.state = new_state;
        self.last_statechange_bh = env.block.height;

        // Save the updated state
        VAULT_STATE.save(deps.storage, self)?;

        Ok(Response::new()
            .add_attribute("action", "update_state")
            .add_attribute("new_state", format!("{:?}", self.state))
            .add_attribute("last_statechange_bh", self.last_statechange_bh.to_string()))
    }

    pub fn execute_action(storage: &mut dyn Storage, action: VaultAction) -> StdResult<()> {
        match action {
            VaultAction::CreateStrategy { name, description } => {
                todo!();
                // try_create_strategy(deps, env, info, name, description)
                Self::try_create_strategy_2(storage, name, description);

            }
            VaultAction::UpdateRunningState { new_state } => {
                todo!()
            }
            VaultAction::UpdateStrategyOwner {  } => {
                todo!()
            }
            VaultAction::UpdateVaultOwner {  } => {
                todo!()
            }
        }
    }

    fn try_update_running_state(
        deps: DepsMut, env: Env, info: MessageInfo, new_state: VaultRunningState) 
        -> Result<Response, ContractError> {
        let mut vault = VAULT_STATE.load(deps.storage)?;
        vault.update_state(deps, env, info, new_state);
    
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
    // Function to create a strategy with ID 1
    pub fn try_create_strategy(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        name: String,
        description: String,
    ) -> StdResult<Response> {
        // TODO - Validation checks to be added.
        // Initially, for the simplicity only one instance of strategy should be supported 
        // within one provault contract. 
        // TODO - Other parameters to be added soon.
        let strategy = Strategy {
            id: 1,
            name,
            description,
        };

        STRATEGY.save(deps.storage, &StrategyKey::new(1), &strategy)?;

        Ok(Response::new()
            .add_attribute("action", "create_strategy")
            .add_attribute("strategy_id", "1"))

    }
      // Function to create a strategy with ID 1
    pub fn try_create_strategy_2(
        storage: &mut dyn Storage,
        name: String,
        description: String,
    ) -> StdResult<Response> {
        // TODO - Validation checks to be added.
        // Initially, for the simplicity only one instance of strategy should be supported 
        // within one provault contract. 
        // TODO - Other parameters to be added soon.
        let strategy = Strategy {
            id: 1,
            name,
            description,
        };

        STRATEGY.save(storage, &StrategyKey::new(1), &strategy)?;

        Ok(Response::new()
            .add_attribute("action", "create_strategy")
            .add_attribute("strategy_id", "1"))

    }

        
}

// Implement the Ownership trait for Vault so vault ownership can be updated
// for performing operations.
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

