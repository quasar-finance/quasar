use cosmwasm_std::{
    ensure, Addr, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, StdError,
};
use serde::{Serialize,Deserialize};
use cw_storage_plus::{Map, Item};
use cosmwasm_schema::cw_serde;
use cw_controllers::Admin;
use crate::error::ContractError;
use crate::strategy::error::StrategyError::AdaptorAlreadyExists;

use crate::ownership::ownership::{
    OwnerProposal, Ownership, OwnershipActions, query_owner, query_ownership_proposal,
    handle_claim_ownership, handle_ownership_proposal, handle_ownership_proposal_rejection
};

use super::error;

const ADAPTERS: Map<&str, bool> = Map::new("adapters");
pub const STRATEGY: Map<&[u8], Strategy> = Map::new("strategy");

pub const STRATEGY_OWNER: Admin = Admin::new("strategy_owner");
pub const STRATEGY_PROPOSAL: Item<OwnerProposal> = Item::new("strategy_proposal");

// Strategy Key 
pub struct StrategyKey;


impl StrategyKey {
    pub fn new(id: u64) -> Vec<u8> {
        id.to_be_bytes().to_vec()
    }
}



#[cw_serde]
pub enum StrategyAction {
    DistributeFundWithPresetAdaptorRatio, // Distributing funds across adaptors as per preset ratios
    DistributeFundWithCustomAdaptorRatios { custom_ratios: String }, // CustomAdaptorRatio (A1:R1, A2:R2, A3:R3)
    RemoveAdaptor { adaptor: String }, // Remove Adaptor Ai
    AddNewAdaptor { adaptor: String }, // Add a new adaptor of type Ai. Should fail if already one is present of type A1.
    UpdateStrategyParams,
    UpdateAdaptorRunningState { adaptor: String },
    UpdateStrategyRunningState,
    Ownership(OwnershipActions),
}


// TODO - Impl ownership to the strategy.

// Stratey here takes the control of the fund movement from the contract treasury balance to 
// the pro vault adaptors as per the instructions sent to strategy module in the contract.
// Fund distribution could be based on preset ratio or sent via external trigger, which depends on how
// an external strategiest proposal followed by decentralised vote and execution of instructions. 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Strategy {
    pub id: u64,
    pub name: String,
    pub description: String,
}

// TODO - Strategy actions to be protected by strategy owner
impl Strategy {
    // pub fn execute_action(storage: &mut dyn Storage, action: StrategyAction) -> StdResult<()> 
    pub fn execute_action(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        action: StrategyAction,
    ) -> Result<Response, ContractError>
    {
        match action {
            StrategyAction::DistributeFundWithPresetAdaptorRatio => {
               todo!()
            }
            StrategyAction::DistributeFundWithCustomAdaptorRatios { custom_ratios } => {
                Self::distribute_funds_with_custom_ratios(deps.storage, custom_ratios)
            }
            StrategyAction::RemoveAdaptor { adaptor } => {
                // TODO - Validation checks
                ADAPTERS.remove(deps.storage, adaptor.as_str());
                Ok(Response::new()
                .add_attribute("action", "remove_adaptor"))
             }
            StrategyAction::AddNewAdaptor { adaptor } => {
                // TODO - Validation checks
                Self::add_adapter(deps.storage, Addr::unchecked(adaptor))
            }
            StrategyAction::UpdateStrategyParams => {
                // Placeholder for updating strategy parameters
                todo!()
            }
            StrategyAction::UpdateAdaptorRunningState { adaptor } => {
                // Placeholder for updating adaptor running state
                todo!()
            }
            StrategyAction::UpdateStrategyRunningState => {
                todo!()
            }
            StrategyAction::Ownership(oa) => {
                // Ownership actions 
                match oa {
                    OwnershipActions::ProposeNewOwner { new_owner, duration } => 
                    {                     
                        handle_ownership_proposal(deps, info, env, new_owner, duration, 
                            &STRATEGY_OWNER, &STRATEGY_PROPOSAL)
                    }
                    OwnershipActions::RejectOwnershipProposal { } => { todo!() }
                    OwnershipActions::ClaimOwnership { } => { todo!() }
                }
             }
        }
    }

    // TODO - Adaptor object string and type check should be done here instead of Addr.
    // For simplification, there should be only one adaptor of one adaptor type. So maximum one
    // instance of CLVault, maximum one instance DebtMarket adaptor, and max one for the 
    // Swap Market. 
    pub fn add_adapter(storage: &mut dyn Storage, 
        adapter: Addr) -> Result<Response, ContractError> {
        if ADAPTERS.has(storage, adapter.as_str()) {
           // Err(cosmwasm_std::StdError::generic_err("Adapter already exists"))
            Err(AdaptorAlreadyExists{}.into())

        } else {
            ADAPTERS.save(storage, adapter.as_str(), &true)?;
            todo!()
        }
    }

    // To be triggered by strategy owner via strategy action entry point.
    pub fn distribute_funds_with_custom_ratios(storage: &mut dyn Storage, 
        custom_ratios: String) -> Result<Response, ContractError> {
        // Parse custom_ratios and distribute funds accordingly
        // Use the position manager module to check available fund in the provault treasury. 
        // Use the adaptor list and ratio to do the calculation. 
        // Update the shares allocated to each adaptors on successful execution on each adaptor 
        todo!()
    }

    pub fn distribute_funds(total_funds: u128, ratios: Vec<u128>) -> Vec<u128> {
        // TODO - Validation.
        let sum_ratios: u128 = ratios.iter().sum();
        ratios.iter().map(|r| total_funds * r / sum_ratios).collect()
    }
}



// Helpers methods for execute entry points
/* 
fn try_add_strategy_adapter(
    deps: DepsMut,
    adapter: String,
) -> Result<Response, ContractError> {
    let adapter_addr = deps.api.addr_validate(&adapter)?;
    let added = Strategy::add_adapter(deps.storage, adapter_addr)?;
    let status = if added { "adapter added" } else { "adapter already exists" };

    Ok(Response::new()
        .add_attribute("method", "try_add_strategy_adapter")
        .add_attribute("status", status))
}

fn try_distribute_strategy_funds(
    deps: DepsMut,
    total_funds: u128,
    ratios: Vec<u128>,
) -> Result<Response, ContractError> {
    let distributions = Strategy::distribute_funds(total_funds, ratios);
    Ok(Response::new()
        .add_attribute("method", "try_distribute_strategy_funds")
        .add_attribute("distributions", format!("{:?}", distributions)))
}
*/
