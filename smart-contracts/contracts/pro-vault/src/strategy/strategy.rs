use cosmwasm_std::{
    ensure, Addr, Deps, DepsMut, Env, MessageInfo, Response, StdResult, 
    Storage, StdError, Uint128, Decimal, Decimal256, Coin,
    QueryRequest, BankQuery, AllBalanceResponse, BalanceResponse
};

use serde::{Serialize, Deserialize};
use cw_storage_plus::{Map, Item};
use cosmwasm_schema::cw_serde;
use cw_controllers::Admin;
use crate::error::ContractError;
use crate::strategy::error::StrategyError::{AdaptorAlreadyExists, AdaptorNotFound};

use crate::ownership::ownership::{
    OwnerProposal, Ownership, OwnershipActions, query_owner, query_ownership_proposal,
    handle_claim_ownership, handle_ownership_proposal, handle_ownership_proposal_rejection
};

use super::error;


// Strategy persistent state variables
const ADAPTERS: Map<&str, AdaptorInfo> = Map::new("adapters");
const ADAPTOR_RATIOS: Map<&str, u128> = Map::new("adaptor_ratios");
pub const STRATEGY: Map<&[u8], Strategy> = Map::new("strategy");
const LATEST_ADAPTOR_ID: Item<u64> = Item::new("latest_adaptor_id");

// Ownership related state variables
pub const STRATEGY_OWNER: Admin = Admin::new("strategy_owner");
pub const STRATEGY_PROPOSAL: Item<OwnerProposal> = Item::new("strategy_proposal");

// Strategy Key 
pub struct StrategyKey;

impl StrategyKey {
    pub fn new(id: u64) -> Vec<u8> {
        id.to_be_bytes().to_vec()
    }
}
// TODO - Actual adaptor object from the adaptor module to be instaniated.
#[cw_serde]
pub struct AdaptorInfo {
    pub name: String,
    pub unique_id: String, 
    pub creation_block: u64,
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
 
    /// Calculates fund allocations based on given allocation percentages.
    /// Returns a tuple containing the allocations and any residual amount.
    /// TODO - Should we use Decimal256 or bigger?
    pub fn calculate_fund_allocations(total_funds: u128, allocation_percentages: Vec<u128>) -> StdResult<(Vec<u128>, u128)> {
        // Sum all allocation percentages
        let total_percentage: u128 = allocation_percentages.iter().sum();
    
        // Convert the total percentage to a Decimal for precise arithmetic
        let total_percentage_decimal = Decimal::from_ratio(total_percentage, 1u128);
    
        // Calculate the allocation for each percentage
        let allocations: Vec<u128> = allocation_percentages.iter()
            .map(|&percentage| {
                // Convert the individual percentage to a Decimal
                let percentage_decimal = Decimal::from_ratio(percentage, 1u128);
                // Calculate the allocation as a Decimal and convert it to u128
                let allocation_decimal = percentage_decimal * Decimal::from_ratio(total_funds, 
                    1u128) / total_percentage_decimal;
                // Convert the Decimal to Uint128 and then to u128
                allocation_decimal * Uint128::one()
            })
            .map(|uint128| uint128.u128())
            .collect();
    
         
        let total_allocated: u128 = allocations.iter().sum();
        let residual: u128 = total_funds - total_allocated;
    
        Ok((allocations, residual))
    }
    

    /// Parses a string of custom ratios into a vector of `AdaptorRatio` structs.
    /// The `custom_ratios` string should contain adaptor ratio pairs separated by commas,
    /// with each pair formatted as `adaptor_id:ratio`. For example, "adaptor1:30,adaptor2:40".
    /// Each `adaptor_id` is a string, and each `ratio` should be a parsable integer representing a percentage.
    /// The function validates that the sum of ratios is 100, returning an error if not.
    /// The function will return an error if the input string is malformed or if parsing fails.
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
    /// Distribute funds across adaptors as per preset ratios
    DistributeFundWithPresetAdaptorRatio, 
    /// Distribute funds across adaptors using custom ratios
    DistributeFundWithCustomAdaptorRatios { custom_ratios: String }, 
    /// Remove an adaptor by its unique ID
    RemoveAdaptor { unique_id: String }, 
    /// Add a new adaptor with the specified name.
    /// TODO - adaptor type to be added so we can instantiate right adaptor type.
    AddNewAdaptor { name: String }, 
    /// Update parameters of the strategy
    UpdateStrategyParams,
    /// Update the running state of a specific adaptor
    UpdateAdaptorRunningState { unique_id: String },
    /// Update the running state of the strategy
    UpdateStrategyRunningState,
    /// Execute ownership-related actions
    Ownership(OwnershipActions),
}

// The Strategy module manages the movement of funds from the contract treasury balance 
// to the various pro vault adaptors according to the instructions it receives.
// It also acts as an adaptor manager, handling the addition and removal of adaptors, 
// and managing their IDs and ratios. Fund distribution can be based on preset ratios 
// or triggered externally, depending on an external strategist's proposal, 
// which is followed by a decentralized vote (using platforms like daodao or keeper network) 
// and the execution of instructions. 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Strategy {
    pub id: u64,
    pub name: String,
    pub description: String,
}

// TODO - Strategy actions to be protected by strategy owner
impl Strategy {

    /// Generates a new unique ID for the adaptor by incrementing the latest ID stored in LATEST_ADAPTOR_ID.
    fn generate_new_adaptor_id(deps: DepsMut) -> StdResult<u64> {
        let mut latest_id = LATEST_ADAPTOR_ID.may_load(deps.storage)?.unwrap_or(0);
        latest_id += 1;
        LATEST_ADAPTOR_ID.save(deps.storage, &latest_id)?;
        Ok(latest_id)
    }

    pub fn execute_action(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        action: StrategyAction,
    ) -> Result<Response, ContractError>
    {
        match action {
            StrategyAction::DistributeFundWithPresetAdaptorRatio => {
                Self::distribute_funds_with_preset_ratios(deps, info, env)
            }
            StrategyAction::DistributeFundWithCustomAdaptorRatios { custom_ratios } => {
                Self::distribute_funds_with_custom_ratios(deps, env, info, custom_ratios)
            }
            StrategyAction::RemoveAdaptor { unique_id } => {
                // Ensure the adaptor exists
                ensure!(
                    ADAPTERS.has(deps.storage, unique_id.as_str()),
                    ContractError::Strategy(AdaptorNotFound { unique_id: unique_id.clone() })
                );

                // TODO - Authorization check. 
                // TODO - Adaptor state check. If adaptor is in some specific state like
                // unbonding. then it can not be removed. IT should first unbond fully 
                // and clear its fund.
                ADAPTERS.remove(deps.storage, unique_id.as_str());
                Ok(Response::new()
                    .add_attribute("action", "remove_adaptor")
                    .add_attribute("unique_id", unique_id))
             }
          
            StrategyAction::AddNewAdaptor { name } => {
                Self::add_adapter(deps, env, name)
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
  

    pub fn add_adapter(mut deps: DepsMut, env: Env, name: String) -> Result<Response, ContractError> {
        // TODO - Valid adaptor name and type validation. And Additional adaptor params based on the 
        // type of the adaptor requested. So ther will be a new message type here. 
        let new_id = Self::generate_new_adaptor_id(deps.branch())?;
        let unique_id = new_id.to_string();
        let adaptor_info = AdaptorInfo {
            name,
            unique_id: unique_id.clone(),
            creation_block: env.block.height,
        };

        if ADAPTERS.has(deps.storage, unique_id.as_str()) {
            return Err(AdaptorAlreadyExists{}.into());
        }

        ADAPTERS.save(deps.storage, unique_id.as_str(), &adaptor_info)?;
        LATEST_ADAPTOR_ID.save(deps.storage, &new_id)?;

        // TODO - Call into adaptor module to do initialise the requested adaptor.

        Ok(Response::new()
            .add_attribute("action", "add_adapter")
            .add_attribute("adaptor_name", adaptor_info.name)
            .add_attribute("unique_id", adaptor_info.unique_id)
            .add_attribute("creation_block", adaptor_info.creation_block.to_string()))
    
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
        // let total_funds = Self::get_treasury_funds(deps.storage)?;
        let denom = "uqsr"; // TODO - Should be coming from the method argument.
        let total_funds = Self::get_treasury_fund_with_denom(deps.branch(), 
            &env, denom);
        // Calculate allocations based on custom ratios
        let (allocations, residual) = Ratio::calculate_fund_allocations(
            total_funds?.u128(), ratios.iter().map(|r| r.ratio).collect())?;

        // Implement the logic to transfer `allocations[i]` amount to the adaptor with id `ratios[i].adaptor_id`.
        let mut response = Response::new().add_attribute("action", "distribute_funds_with_custom_ratios");

        for (i, ratio) in ratios.iter().enumerate() {
            Self::transfer_funds_to_adaptor(deps.branch(), &env, &info, &ratio.adaptor_id, allocations[i])?;
            response = response.add_attribute(format!("allocation_{}", ratio.adaptor_id), allocations[i].to_string());
        }

        // Add an attribute for the residual amount
        response = response.add_attribute("residual", residual.to_string());

        Ok(response)
    }

    fn get_treasury_funds_all(deps: Deps, env: &Env) -> StdResult<Vec<Coin>> {
        let contract_address = env.contract.address.clone();
        let all_balances: AllBalanceResponse = deps.querier.query(
            &QueryRequest::Bank(BankQuery::AllBalances {
            address: contract_address.to_string(),
        }))?;
    
        // Return the vector of coins
        Ok(all_balances.amount)
    }

    fn get_treasury_fund_with_denom(mut deps: DepsMut, env: &Env, denom: &str) ->  StdResult<Uint128> {
        let contract_address = env.contract.address.clone();
        let balance: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
            address: contract_address.to_string(),
            denom: denom.to_string(),
        }))?;
    
        // Return the balance amount as Uint128
        Ok(balance.amount.amount)
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

    pub fn distribute_funds_with_preset_ratios(mut deps: DepsMut, info: MessageInfo, env: Env) -> Result<Response, ContractError> {
        let mut response = Response::new().add_attribute("action", "distribute_funds_with_preset_ratios");

        // TODO #1 - total_outstanding share and amount To be taken from vault::vault_position_manager.
        // TODO #2 - open deposit objects should be cleared after successful distribution. 
        // TODO #3 - reply handlers should be added. Design decision to be taken for one step distribution 
        //           which will require async handlings. The good option is to just send fund to the adaptors.
        //           and adaptor will take care of further allocation in the actual allocation to yield destination.
        // TODO #4 - Real shares <denoms, amount> to be saved from the yield destinations in the position_manager 
        //           adaptor_position_manager. 
        // let total_funds = Uint128::new(1000); // Placeholder for the actual amount in the treasury.
        let denom = "uqsr"; // TODO - Should be coming from the method argument.
        let total_funds = Self::get_treasury_fund_with_denom(deps.branch(), 
            &env, denom)?;
        let ratios = ADAPTOR_RATIOS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .map(|item| item.map(|(k, v)| AdaptorRatio { adaptor_id: k, ratio: v }))
            .collect::<StdResult<Vec<AdaptorRatio>>>()?;

        Ratio::validate_ratios(&ratios)?;

        // Calculate allocations based on preset ratios
        let (allocations, residual) = Ratio::calculate_fund_allocations(total_funds.u128(), ratios.iter().map(|r| r.ratio).collect())?;

        for (i, ratio) in ratios.iter().enumerate() {
            // TODO - Adaptor module should also support a add_fund mandatory methods towards the adaptors. 
            // As the actual methods of deployment are different in each adaptors, 
            // We can implement something like, adaptor type and match statement helper.
            // In the match if it is Mars then we do deposit, for SWAP we do swap operation on the actual adaptor. 
            // Implement the logic to transfer `allocations[i]` amount to the adaptor with id `ratio.adaptor_id`.
            // The first implementation assume the async operation on the adaptor side for simplicity of testing. 
            // And also to avoid the complexity of handling multiple adaptors. 
            // TODO - Reply has to handled once adaptor sync deposit is handled here. 
            Self::transfer_funds_to_adaptor(deps.branch(), &env, &info, &ratio.adaptor_id, allocations[i])?;
            response = response.add_attribute(format!("allocation_{}", ratio.adaptor_id), allocations[i].to_string());
        }

        // Add an attribute for the residual amount
        response = response.add_attribute("residual", residual.to_string());

        Ok(response)
    }

}
 