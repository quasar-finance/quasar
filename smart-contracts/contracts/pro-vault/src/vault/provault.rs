use cosmwasm_std::{
    ensure, Addr, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, StdError,
};
use cw_controllers::Admin;
use cw_storage_plus::Item;
use cosmwasm_schema::cw_serde;

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::error::ContractError;
use crate::ownership;
use crate::vault::error::VaultError::{StrategyAlreadyExists, InvalidVaultState};
use crate::strategy::strategy::{Strategy, StrategyKey, STRATEGY, STRATEGY_OWNER, STRATEGY_PROPOSAL};
use crate::ownership::ownership::{
    OwnerProposal, Ownership, OwnershipActions, query_owner, query_ownership_proposal,
    handle_claim_ownership, handle_ownership_proposal, handle_ownership_proposal_rejection,
    MAX_DURATION
};


// Vaule module state variables. VAULT_OWNER and VAULT_PROPOSAL state items are used by
// ownership module which faciliate the ownership of the vault.
pub const VAULT_OWNER: Admin = Admin::new("vault_owner");
pub const VAULT_PROPOSAL: Item<OwnerProposal> = Item::new("vault_proposal");

// Vault state indicate the running state of the Vault.state (VaultRunningState) , represented by 
// Vault struct. Vault state is internally used to control which operations are allowed and 
// which is not based on the current state of the vault
pub const VAULT_STATE: Item<Vault> = Item::new("vault_state");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] 
pub enum VaultRunningState {
  // Initalized, and waiting to come Normal once vault is ready to accept deposit. 
  Init, 
  // Normal operating mode
  Running, 
  // Temporary halted
  Paused, 
  // Terminated forever 
  Terminated, 
}

// VaultAction is a set of actions that can be performed on the vault module. 
#[cw_serde]
pub enum VaultAction {
    UpdateRunningState {
        new_state: VaultRunningState,
    }, 
    UpdateStrategyOwner {},
    CreateStrategy {
        name: String,
        description: String,
        owner: String,
    },
    Ownership(OwnershipActions),
}


// Vault state wrapper, and abstraction for vault operations.
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
        // 2. STATE WISE ACTION PROTECTION, in which state what action can be performed.
        match action {
            VaultAction::CreateStrategy { name, description, owner } => {               
                Self::try_create_strategy(deps, env, info, name, description, owner)
            }
            VaultAction::UpdateRunningState { new_state } => {
                Self::try_update_running_state(deps, env, info, new_state)
            }
            VaultAction::UpdateStrategyOwner {} => {
                Self::try_update_strategy_owner(deps)
            }
            VaultAction::Ownership(oa) => {
                // Ownership actions 
                match oa {
                    OwnershipActions::ProposeNewOwner { new_owner, duration } => {                     
                        handle_ownership_proposal(deps, info, env, 
                            new_owner, duration, 
                            &VAULT_OWNER, &VAULT_PROPOSAL)
                    }
                    OwnershipActions::RejectOwnershipProposal {  } => { 
                        handle_ownership_proposal_rejection(deps, info, 
                            &VAULT_OWNER, &VAULT_PROPOSAL)
                    }
                    OwnershipActions::ClaimOwnership {  } => {
                        handle_claim_ownership(deps, info, env,  
                            &VAULT_OWNER, &VAULT_PROPOSAL)
                    }
                }
            }
        }
    }

    fn try_update_running_state(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        new_state: VaultRunningState,
    ) -> Result<Response, ContractError> {
        // Ownership verification. 
        let owner = VAULT_OWNER.get(deps.as_ref())?;
        ensure!(owner == Some(info.sender), ContractError::Unauthorized {});

        // TODO - State transition verification logic to be added.
        let mut vault: Vault = VAULT_STATE.load(deps.storage)?;

        vault.state = new_state;
        vault.last_statechange_bh = env.block.height;

        VAULT_STATE.save(deps.storage, &vault)?;

        Ok(Response::new()
            .add_attribute("action", "update_state")
            .add_attribute("new_state", format!("{:?}", vault.state))
            .add_attribute("last_statechange_bh", vault.last_statechange_bh.to_string()))
    }
    

    pub fn try_create_strategy(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        name: String,
        description: String,
        proposed_owner: String,
    ) -> Result<Response, ContractError> {
        // Ownership verification. 
        let owner = VAULT_OWNER.get(deps.as_ref())?;
        ensure!(owner == Some(info.sender.clone()), ContractError::Unauthorized {});

        let vault = VAULT_STATE.load(deps.storage)?;

        // Ensure the vault is in the Init state
        if vault.state != VaultRunningState::Init {
            return Err(InvalidVaultState {
                expected: "Init".to_string(),
                actual: format!("{:?}", vault.state),
            }
            .into());
        }

        // Current implementation support only one strategy in one provault.
        // This implentation can be enhanced to support multiple strategy in the single
        // vault to support complex distribution. 
        if STRATEGY.has(deps.storage, &StrategyKey::new(1)) {
            return Err(StrategyAlreadyExists{}.into());
        }

        // Strategy object initialization.   
        let strategy = Strategy {
            id: 1,
            name: name.clone(),
            description: description.clone(),
        };
        STRATEGY.save(deps.storage, &StrategyKey::new(1), &strategy)?;
        // At the time of creating strategy object info.sender has to become the default owner.
        // This is the design constraint of ownership module, info.sender then, further propose the 
        // change on the same call. 
        STRATEGY_OWNER.set(deps.branch(), Some(info.sender.clone()));  


        // Ownership proposal to be created on the same call. Then proposed owner will need 
        // to claim the ownership of the strategy for performing operations. Till that time 
        // info.sender can do all the strategy operations.
        let strategy_current_owner = STRATEGY_OWNER.get(deps.as_ref())?.unwrap();
        strategy.handle_ownership_proposal(deps, info, env, 
            proposed_owner.clone(), MAX_DURATION,
            &STRATEGY_OWNER, &STRATEGY_PROPOSAL); 

        // TODO - Near Future Extension:
        // Add adapters and preset distribution-related parameters. 
        // These settings could be optional, allowing them to be set in a separate transaction 
        // by the current owner. If we support adding adapters here, it would orchestrate 
        // everything in one step. For now, we'll keep it as a separate option using 
        // add_adaptor methods in the strategy.
        Ok(Response::new()
            .add_attribute("action", "create_strategy")
            .add_attribute("strategy_id", "1")
            .add_attribute("strategy_name", name)
            .add_attribute("strategy_description", description)
            .add_attribute("strategy_current_owner", strategy_current_owner.to_string())
            .add_attribute("strategy_proposed_owner", proposed_owner))
    }

    // TODO - Near Future Extension.
    fn try_update_strategy_owner(
        deps: DepsMut,
    ) -> Result<Response, ContractError> {
        // let owner = VAULT_OWNER.get(deps.as_ref())?;
        // ensure!(owner == Some(info.sender), ContractError::Unauthorized {});

        // Implementation for UpdateStrategyOwner. It should create a proposal to change the 
        // strategy owner if sender has the authority to propose. In the current implementation
        // only current owner can propose. A design enhancement is added in the comment section of the
        // ownership module to have a whitelist authority for proposals, which most probablity should be 
        // some dao dao account. 
        Ok(Response::new()
            .add_attribute("action", "update_strategy_owner"))
    }

}
