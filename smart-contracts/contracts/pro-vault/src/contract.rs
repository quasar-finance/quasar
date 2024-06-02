use crate::error::ContractError;

use crate::msg::{InstantiateMsg, QueryMsg, MigrateMsg, ExecuteMsg};
use crate::msg::ProExtensionExecuteMsg;
use crate::msg::ExtensionExecuteMsg;

use crate::strategy::strategy::{Strategy, StrategyAction};

use crate::vault::provault::{VaultRunningState, VAULT_STATE, VAULT_OWNER, Vault, VaultAction};
use crate::vault::config::{VAULT_CONFIG, Config};
use crate::vault::query::{VaultQueryMsg, query_vault_config, 
    query_vault_running_state, query_all_strategies};

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, 
    StdResult, StdError, Uint128, BankMsg,CosmosMsg,
    };
  
// TODO - 
// 1. Locality of local variables to be strucured, that will reduce number of imports from
// other modules and will make the contract.rs cleaner, smaller, and easy to read and maintain.
// 2. Stategy Actions to be added here in the match cases.

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    match msg {
        // TODO - Error Handling and propagation.
        ExecuteMsg::Deposit { amount, recipient } => { 
            let _ = try_deposit(deps, env, info, amount, recipient);},
        ExecuteMsg::Redeem { recipient, amount } => todo!(),
        ExecuteMsg::VaultExtension(extension_msg) => {
            match extension_msg {
                 ExtensionExecuteMsg::ProExtension(pro_msg) => {
                    match pro_msg {
                        ProExtensionExecuteMsg::ExecVaultActions { action } => {
                            let _ = try_exec_vault_actions(deps, action); }
                        ProExtensionExecuteMsg::ExecStrategyActions{action} => { 
                            let _ = try_exec_strategy_actions(deps, action); }
                    }
                }
                // Handle other possible ExtensionExecuteMsg variants here
                _ => todo!(),
            }
        }
    }
    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    VAULT_OWNER.set(deps.branch(), Some(info.sender.clone()))?;


    let config = Config {
        max_deposit_cap: msg.provault_config.max_deposit_cap,
        deposit_denom: msg.provault_config.deposit_denom,
        share_denom: msg.provault_config.share_denom,
        max_strategy_inst: msg.provault_config.max_strategy_inst,
        admin: msg.provault_config.admin,
    };

    VAULT_CONFIG.save(deps.storage, &config)?;


    // Initialize the vault state
    let mut vault = Vault::new();
    let response = vault.update_state(deps, env, info, VaultRunningState::Init)?;
    
    // TODO - PROPER EVENT HANDLING AND ERROR PROPAGATION.
    Ok(Response::new().add_attribute("method", "instantiate"))
    // Ok(Response::default())
 }
 
// TODO - Query Enumsd and match to be resturctured. Like Vault config query should be in vault module,
// strategy config query to be in strategy module. 
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {

    match msg {
        QueryMsg::GetAllStrategies {} => query_all_strategies(deps),
        QueryMsg::VaultQuery(vault_msg) => {
            match vault_msg {
                VaultQueryMsg::GetVaultConfig {} => query_vault_config(deps),
                VaultQueryMsg::GetVaultRunningState {} => query_vault_running_state(deps),
            }
        }
        // Handle other queries ...
    }

    /* 
     // Create a default response message
     let response = "Default response message";

     // Convert the response message to a binary format
     let binary_response = to_json_binary(&response)?;
 
     // Return the binary response
     Ok(binary_response)
     */
}
 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
     // TODO - REPLY TO BE IMPLEMENTED
     Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    // MyVault.migrate(deps, env, msg)
    Ok(Response::default())
}


fn try_update_running_state(
    deps: DepsMut, env: Env, info: MessageInfo, new_state: VaultRunningState) 
    -> Result<Response, ContractError> {
    let mut vault = VAULT_STATE.load(deps.storage)?;
    let _ = vault.update_state(deps, env, info, new_state);

    Ok(Response::new()
        .add_attribute("method", "try_update_running_state"))
}

fn try_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    recipient: Option<String>, // Used for future 
) -> Result<Response, ContractError> {
    
    // Check if the sent funds match the requested deposit amount
    if info.funds.len() != 1 || info.funds[0].amount != amount {
        return Err(ContractError::Std(StdError::generic_err("Incorrect deposit amount")));
    }

    // TODO - Whitelist denom verification. Whitelist should be added in initialise message too. 
    //        there should be a separate update whitelist exection as well.
    // Ensure the denom matches
    if info.funds[0].denom != "uosmo" { // TODO - Replace with the appropriate denom
        return Err(ContractError::Std(StdError::generic_err("Incorrect denom")));
    }


    // TODO - share calculation and allocation to be done on reply handler on successful deposit. 

    Ok(Response::new()
        .add_attribute("method", "try_deposit")
        .add_attribute("amount", amount.to_string())
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("recipient", env.contract.address.to_string()))
}

fn try_update_vault_owner(
    deps: DepsMut,
) -> Result<Response, ContractError> {
    // Implementation for UpdateVaultOwner
    Ok(Response::new()
        .add_attribute("method", "try_update_vault_owner"))
}


fn try_exec_vault_actions(
    deps: DepsMut,
    action: VaultAction,
) -> Result<Response, ContractError> {
    Vault::execute_action(deps.storage, action)?;
    Ok(Response::new()
        .add_attribute("method", "try_exec_vault_actions"))
}

fn try_exec_strategy_actions(
    deps: DepsMut,
    action: StrategyAction,
) -> Result<Response, ContractError> {
    Strategy::execute_action(deps.storage, action)?;
    Ok(Response::new()
        .add_attribute("method", "try_exec_strategy_actions"))
}

