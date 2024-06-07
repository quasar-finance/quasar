use cosmwasm_std::{
    ensure, Addr, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, StdError, Uint128,
};
use serde::{Serialize, Deserialize};
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

const ADAPTERS: Map<&str, AdaptorInfo> = Map::new("adapters");
const ADAPTOR_RATIOS: Map<&str, u128> = Map::new("adaptor_ratios");
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

// TODO - Creation of unique ID for the adaptor at the time of adding new adaptor. 
#[cw_serde]
pub struct AdaptorInfo {
    pub name: String,
    pub unique_id: String, 
}

#[cw_serde]
pub struct AdaptorRatio {
    pub adaptor_id: String,
    pub ratio: u128,
}

// Ratio struct, Here the implementation require the sum to be always equal to 100%.
// Normalised ratio is avoided here, to keep debugging simpler. This will require adjusting
// the ratio every time a new adaptor is added or removed by the strategy owner account. 
// TODO - Validate Ratio should be checked in all operations which affect the distribution
// of fund amoung active adaptor sets.
pub struct Ratio;

impl Ratio {
    pub fn calculate_ratios(total_funds: u128, ratios: Vec<u128>) -> Vec<u128> {
        let sum_ratios: u128 = ratios.iter().sum();
        ratios.iter().map(|r| total_funds * r / sum_ratios).collect()
    }

    pub fn parse_custom_ratios(custom_ratios: String) -> StdResult<Vec<AdaptorRatio>> {
        let ratios: Vec<AdaptorRatio> = custom_ratios.split(',')
            .map(|pair| {
                let mut parts = pair.split(':');
                let adaptor_id = parts.next().unwrap().to_string();
                let ratio = parts.next().unwrap().parse().unwrap();
                AdaptorRatio { adaptor_id, ratio }
            })
            .collect();
        Ok(ratios)
    }

    pub fn validate_ratios(ratios: &Vec<AdaptorRatio>) -> StdResult<()> {
        let sum: u128 = ratios.iter().map(|r| r.ratio).sum();
        ensure!(sum == 100, StdError::generic_err("Ratios must sum to 100"));
        Ok(())
    }
}

#[cw_serde]
pub enum StrategyAction {
    DistributeFundWithPresetAdaptorRatio, // Distributing funds across adaptors as per preset ratios
    DistributeFundWithCustomAdaptorRatios { custom_ratios: String }, // CustomAdaptorRatio (A1:R1, A2:R2, A3:R3)
    RemoveAdaptor { unique_id: String }, // Remove Adaptor by unique_id
    AddNewAdaptor { name: String, unique_id: String }, // Add a new adaptor with name and unique_id. Should fail if already one is present.
    UpdateStrategyParams,
    UpdateAdaptorRunningState { unique_id: String },
    UpdateStrategyRunningState,
    Ownership(OwnershipActions),
}

// TODO - Impl ownership to the strategy.

// Strategy here takes the control of the fund movement from the contract treasury balance to 
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
    pub fn execute_action(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        action: StrategyAction,
    ) -> Result<Response, ContractError>
    {
        match action {
            StrategyAction::DistributeFundWithPresetAdaptorRatio => {
                Self::distribute_funds_with_preset_ratios(deps, env)
            }
            StrategyAction::DistributeFundWithCustomAdaptorRatios { custom_ratios } => {
                Self::distribute_funds_with_custom_ratios(deps, env, info, custom_ratios)
            }
            StrategyAction::RemoveAdaptor { unique_id } => {
                // TODO - Validation checks
                ADAPTERS.remove(deps.storage, unique_id.as_str());
                Ok(Response::new()
                .add_attribute("action", "remove_adaptor"))
             }
            StrategyAction::AddNewAdaptor { name, unique_id } => {
                // TODO - Validation checks
                // Use the code from adaptor module to properly register an adaptor with address or so. 
                Self::add_adapter(deps.storage, AdaptorInfo { name, unique_id })
            }
            StrategyAction::UpdateStrategyParams => {
                // Placeholder for updating strategy parameters
                todo!()
            }
            StrategyAction::UpdateAdaptorRunningState { unique_id } => {
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
                        handle_ownership_proposal(deps, info, env, 
                            new_owner, duration, 
                            &STRATEGY_OWNER, &STRATEGY_PROPOSAL)
                    }
                    OwnershipActions::RejectOwnershipProposal { } => { 
                            handle_ownership_proposal_rejection(deps, info, 
                                &STRATEGY_OWNER, &STRATEGY_PROPOSAL)
                    }
                    OwnershipActions::ClaimOwnership { } => { 
                            handle_claim_ownership(deps, info, env, 
                                &STRATEGY_OWNER, &STRATEGY_PROPOSAL)
                    }
                }
             }
        }
    }

    
    
    
    // TODO - Adaptor object string and type check should be done here instead of Addr.
    // For simplification, there should be only one adaptor of one adaptor type. So maximum one
    // instance of CLVault, maximum one instance DebtMarket adaptor, and max one for the 
    // Swap Market. 
    pub fn add_adapter(storage: &mut dyn Storage, 
        adaptor_info: AdaptorInfo) -> Result<Response, ContractError> {
        if ADAPTERS.has(storage, adaptor_info.unique_id.as_str()) {
            Err(AdaptorAlreadyExists{}.into())
        } else {
            ADAPTERS.save(storage, adaptor_info.unique_id.as_str(), &adaptor_info)?;
            Ok(Response::new()
                .add_attribute("action", "add_adapter")
                .add_attribute("adaptor_name", adaptor_info.name)
                .add_attribute("unique_id", adaptor_info.unique_id))
        }
    }

    // To be triggered by strategy owner via strategy action entry point.
    pub fn distribute_funds_with_custom_ratios(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        custom_ratios: String,
    ) -> Result<Response, ContractError> {
        // Parse and validate custom ratios
        let ratios = Ratio::parse_custom_ratios(custom_ratios)?;
        Ratio::validate_ratios(&ratios)?;

        // Fetch the total funds available in the treasury
        let total_funds = Self::get_treasury_funds(deps.storage)?;

        // Calculate allocations based on custom ratios
        let allocations = Ratio::calculate_ratios(total_funds.u128(), ratios.iter().map(|r| r.ratio).collect());

        // Implement the logic to transfer `allocations[i]` amount to the adaptor with id `ratios[i].adaptor_id`.
        let mut response = Response::new().add_attribute("action", "distribute_funds_with_custom_ratios");

        for (i, ratio) in ratios.iter().enumerate() {
            Self::transfer_funds_to_adaptor(deps.branch(), &env, &info, &ratio.adaptor_id, allocations[i])?;
            response = response.add_attribute(format!("allocation_{}", ratio.adaptor_id), allocations[i].to_string());
        }

        Ok(response)
    }

    fn get_treasury_funds(storage: &dyn Storage) -> StdResult<Uint128> {
        // Placeholder function to get the total funds available in the treasury
        // Replace with actual logic to fetch the treasury balance
        Ok(Uint128::new(1000))
    }

    fn transfer_funds_to_adaptor(
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        adaptor_id: &str,
        amount: u128,
    ) -> Result<(), ContractError> {
        // Placeholder function to transfer funds to a specific adaptor
        // Replace with actual logic to perform the transfer
        // Example: invoke a transfer message or update state accordingly

        // Log the transfer action
        deps.api.debug(&format!(
            "Transferring {} funds to adaptor {}",
            amount, adaptor_id
        ));

        // Example logic: update a mock balance in the state
        // BALANCES.update(deps.storage, &Addr::unchecked(adaptor_id), |balance| -> StdResult<_> {
        //     let mut balance = balance.unwrap_or_default();
        //     balance += Uint128::new(amount);
        //     Ok(balance)
        // })?;

        Ok(())
    }

    pub fn distribute_funds_with_preset_ratios(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
        let mut response = Response::new().add_attribute("action", "distribute_funds_with_preset_ratios");

        let total_funds = Uint128::new(1000); // Placeholder for the actual amount in the treasury.
        let ratios = ADAPTOR_RATIOS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .map(|item| item.map(|(k, v)| AdaptorRatio { adaptor_id: k, ratio: v }))
            .collect::<StdResult<Vec<AdaptorRatio>>>()?;

        Ratio::validate_ratios(&ratios)?;

        let allocations = Ratio::calculate_ratios(total_funds.u128(), ratios.iter().map(|r| r.ratio).collect());

        for (i, ratio) in ratios.iter().enumerate() {
            // TODO - Adaptor module should also support a add_fund mendatory methods towards the adaptors. 
            // As the actual methods of deployment is different in each adaptors, 
            // We can implement something like, adaptor type and match statement helper.
            // In the match if is is Mars then we do deposit, for SWAP we do swap operation on the actual adaptor. 
            // Implement the logic to transfer `allocations[i]` amount to the adaptor with id `ratio.adaptor_id`.
            response = response.add_attribute(format!("allocation_{}", ratio.adaptor_id), allocations[i].to_string());
        }

        Ok(response)
    }
}
 