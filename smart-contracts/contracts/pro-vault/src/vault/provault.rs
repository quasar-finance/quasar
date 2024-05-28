use cosmwasm_std::{
    attr, ensure, ensure_eq, Addr, Deps, DepsMut, Env, Event, MessageInfo, Response, StdError,
    StdResult, Binary, to_json_binary,};
use cw_controllers::Admin;
use cw_storage_plus::Item;

use serde::{Serialize,Deserialize};
use schemars::JsonSchema;

use crate::vault::query::{VaultRunningStateResponse, StrategyInfoResponse};
use crate::error::ContractError;
use crate::strategy::strategy::{Strategy, STRATEGY}; 
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

