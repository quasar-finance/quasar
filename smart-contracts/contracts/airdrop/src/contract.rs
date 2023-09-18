use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::admin::{execute_add_users, execute_remove_users, execute_update_airdrop_config};
use crate::error::AirdropErrors;
use crate::helpers::is_contract_admin;
use crate::msg::{AdminExecuteMsg, ExecuteMsg, InstantiateMsg};
use crate::state::{AirdropConfig, AIRDROP_CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "quasar_airdrop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, AirdropErrors> {
    // Set the contract version in storage
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Check if the provided end height is less than or equal to the start height.
    if msg.config.end_height <= msg.config.start_height {
        return Err(AirdropErrors::InvalidAirdropWindow {});
    }

    // Save the new airdrop configuration to storage
    AIRDROP_CONFIG.save(deps.storage, &msg.config)?;

    // Return a default response to indicate success
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, AirdropErrors> {
    match msg {
        ExecuteMsg::Admin(admin_msg) => {
            // Check if the sender is a contract admin
            is_contract_admin(&deps.querier, &env, &info.sender)?;

            match admin_msg {
                AdminExecuteMsg::UpdateAirdropConfig(airdrop_config) => {
                    // Call the function to update the airdrop configuration
                    execute_update_airdrop_config(deps, env, airdrop_config)
                }
                AdminExecuteMsg::AddUsers { users, amounts } => {
                    // Call the function to add users and their amounts
                    execute_add_users(deps, env, users, amounts)
                }
                AdminExecuteMsg::RemoveUsers(users) => execute_remove_users(deps, env, users),
                AdminExecuteMsg::WithdrawFunds() => {}
                AdminExecuteMsg::SetUsers { users, amounts } => {}
            }
        }
        ExecuteMsg::ClaimAirdrop() => {}
    }
}
