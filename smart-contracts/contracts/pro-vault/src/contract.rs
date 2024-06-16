use cw_storage_plus::Map;
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, 
    StdResult, StdError, Uint128, BankMsg,CosmosMsg,Attribute
    };

    use crate::error::ContractError;

use crate::msg::{InstantiateMsg, QueryMsg, MigrateMsg, ExecuteMsg};
use crate::msg::ProExtensionExecuteMsg;
use crate::msg::ExtensionExecuteMsg;

use crate::strategy::strategy::{Strategy, StrategyAction};

use crate::vault::provault::{VaultRunningState, VAULT_STATE, VAULT_OWNER, Vault, VaultAction};
use crate::vault::config::{VAULT_CONFIG, Config};
use crate::vault::query::{VaultQueryMsg, query_vault_config, 
    query_vault_running_state, query_all_strategies};

use crate::proshare::share_allocator::{allocate_shares, ShareConfig, ShareType};

use crate::position_manager::vault_position_manager::{initialize as initialize_vault,
    deposit as vault_deposit, 
    query_provault_position, query_deposit};
use crate::position_manager::user_position_manager::{allocate_position as user_allocate_position
    , query_position};


pub const WHITELIST_DENOMS: Map<&str, bool> = Map::new("whitelist_denoms");


  
// TODO - 
// 1. Locality of local variables to be strucured, that will reduce number of imports from
// other modules and will make the contract.rs cleaner, smaller, and easy to read and maintain.
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
                            let _ = try_exec_vault_actions(deps, env, info, action);}
                        ProExtensionExecuteMsg::ExecStrategyActions{action} => { 
                            let _ = try_exec_strategy_actions(deps, env, info, action); }
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
    // Ensure the admin and vault owner are the same
    if msg.provault_config.admin != info.sender {
        return Err(ContractError::AdminVaultOwnerMismatch {});
    }

    // Set the vault owner
    VAULT_OWNER.set(deps.branch(), Some(info.sender.clone())).map_err(|e| {
        ContractError::SetVaultOwnerError(e.to_string())
    })?;

    // Construct the config from the message
    let config = Config {
        max_deposit_cap: msg.provault_config.max_deposit_cap,
        deposit_denom: msg.provault_config.deposit_denom.clone(),
        share_denom: msg.provault_config.share_denom.clone(),
        max_strategy_inst: msg.provault_config.max_strategy_inst,
        admin: msg.provault_config.admin,
    };

    // Save the config to storage
    VAULT_CONFIG.save(deps.storage, &config).map_err(|e| {
        ContractError::SaveConfigError(e.to_string())
    })?;

    // Initialize the whitelist-denoms that are allowed to deposit in this 
    // instance of contract
   for denom in msg.whitelisted_denoms {
        WHITELIST_DENOMS.save(deps.storage, &denom, &true)?;
    }

    // Clone necessary parts of info before moving it
    let info_clone = info.clone();

    // Initialize the vault state
    let mut vault = Vault::new();
    let update_state_response = vault.update_state(deps, env, 
        info_clone, VaultRunningState::Init)
        .map_err(|e| {
            ContractError::UpdateVaultStateError(e.to_string())
        })?;

    // Emit events using the new function
    Ok(emit_instantiate_events(&info, &config, update_state_response.attributes))
}


fn emit_instantiate_events(
    info: &MessageInfo,
    config: &Config,
    update_state_attributes: Vec<Attribute>
) -> Response {
    Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("vault_owner", info.sender.to_string())
        .add_attribute("max_deposit_cap", config.max_deposit_cap.to_string())
        .add_attribute("deposit_denom", config.deposit_denom.clone())
        .add_attribute("share_denom", config.share_denom.clone())
        .add_attribute("max_strategy_inst", config.max_strategy_inst.to_string())
        .add_attribute("admin", config.admin.to_string())
        .add_attributes(update_state_attributes) // Merge attributes from update_state
}


// TODO - Query Enums and match to be resturctured. Like Vault config query should be in vault module,
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
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    recipient: Option<String>, // Used for future 
) -> Result<Response, ContractError> {
    
    // Check if the sent funds match the requested deposit amount
    if info.funds.len() != 1 || info.funds[0].amount != amount {
        return Err(ContractError::Std(StdError::generic_err("Incorrect deposit amount")));
    }
 
    // Whitelist denom verification.
    let denom = &info.funds[0].denom; 
    if !WHITELIST_DENOMS.has(deps.storage, denom) {
        return Err(ContractError::Std(StdError::generic_err("Denom not allowed")));
    }


    // TODO - Share calculation and allocation to be done. 
    // Options - 1. Mint share here at this point of time based on the current value of 
    // provault share in terms of deposited denoms. Fetch the value from the oracle here and 
    // calc if the oracle pricing and last updated time is acceptable. 
    // Options - 2. Delayed calculation. [ Async Handling To be done in this case. ]
    // a. Store the deposited value in storage from the user with block height 
    // b. Store the oracle pricing that should be used for share calculation. 
    // c. Deploy the fund across the adaptors. 
    // d. On successfully adaptor deployment, we do the calculation, and allocate share based on 
    //    previously stored values 

    // DUMMY IMPLEMENTATION FOR SHARE FOR TESTING
    // Define the share configuration (using the deposit denom to create the share denom)
    let share_config = ShareConfig::new(ShareType::Number, denom);

    // Allocate shares using the proshare module
    // let allocate_response = allocate_shares(deps, env.clone(), info.clone(), recipient.clone(), amount, &share_config)?; 
    
    // First, allocate shares for the user
    let user_allocate_resp = user_allocate_position(deps.branch(), env.clone(), info.clone(), amount, &share_config)?;
    let vault_deposit_resp = vault_deposit(deps, env.clone(), info.clone(), amount, &share_config)?;

    Ok(Response::new()
    .add_attribute("method", "try_deposit")
    .add_attribute("amount", amount.to_string())
    .add_attribute("sender", info.sender.to_string())
    .add_attribute("recipient", env.contract.address.to_string())
    .add_attributes(user_allocate_resp.attributes)
    .add_attributes(vault_deposit_resp.attributes))
        
}

fn try_exec_strategy_actions(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: StrategyAction,
) -> Result<Response, ContractError> {
    Strategy::execute_action(deps, env, info, action)?;
    Ok(Response::new()
        .add_attribute("method", "try_exec_strategy_actions"))
}

 
fn try_exec_vault_actions(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: VaultAction,
) -> Result<Response, ContractError> {
    Vault::execute_action(deps, env, info, action)?;
    Ok(Response::new()
        .add_attribute("method", "try_exec_vault_actions"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, Addr, Response, Uint128, Uint64, SystemResult, ContractResult, Binary, Empty};

    use crate::msg::InstantiateMsg;
    use crate::vault::config::Config;

    // Initialization Test #1 
    #[test] 
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            provault_config: Config {
                admin: "admin".to_string(),
                max_deposit_cap: Uint128::new(1000000),
                deposit_denom: "uosmo".to_string(),
                share_denom: "share".to_string(),
                max_strategy_inst: Uint64::new(10),
            },
            name: "test_name".to_string(),
            thesis: "test_thesis".to_string(),
        };
        let info = mock_info("admin", &[]);

        // Try instantiating the contract
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

        // Check response attributes
        assert_eq!(res.attributes, vec![
            attr("method", "instantiate"),
            attr("vault_owner", "admin"),
            attr("max_deposit_cap", "1000000"),
            attr("deposit_denom", "uosmo"),
            attr("share_denom", "share"),
            attr("max_strategy_inst", "10"),
            attr("admin", "admin"),
            attr("action", "update_state"),
            attr("new_state", "Init"),
            attr("last_statechange_bh", "12345"), // This might be the block height
        ]);

        // Check state
        let config: Config = VAULT_CONFIG.load(&deps.storage).unwrap();
        assert_eq!(config, Config {
            max_deposit_cap: Uint128::new(1000000),
            deposit_denom: "uosmo".to_string(),
            share_denom: "share".to_string(),
            max_strategy_inst: Uint64::new(10),
            admin: "admin".to_string(),
        });

        let vault_owner = VAULT_OWNER.get(deps.as_ref()).unwrap();
        assert_eq!(vault_owner, Some(Addr::unchecked("admin")));
    }

    // Initialization test #2 
    #[test]
    fn initialization_fails_if_admin_not_sender() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            provault_config: Config {
                admin: "admin".to_string(),
                max_deposit_cap: Uint128::new(1000000),
                deposit_denom: "uosmo".to_string(),
                share_denom: "share".to_string(),
                max_strategy_inst: Uint64::new(10),
            },
            name: "test_name".to_string(),
            thesis: "test_thesis".to_string(),
        };
        let info = mock_info("not_admin", &[]);

        // Try instantiating the contract
        let err = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        // Check error
        match err {
            ContractError::AdminVaultOwnerMismatch {} => {}
            _ => panic!("unexpected error: {:?}", err),
        }
    } 
}
