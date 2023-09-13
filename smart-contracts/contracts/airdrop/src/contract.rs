use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::admin::{execute_add_users, execute_update_airdrop_config};
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
    if msg.end_height <= msg.start_height {
        return Err(AirdropErrors::InvalidAirdropWindow {});
    }

    // Create a new airdrop configuration based on the provided parameters
    let airdrop_config = AirdropConfig {
        airdrop_description: msg.airdrop_description,
        airdrop_amount: msg.airdrop_amount,
        airdrop_denom: msg.airdrop_denom,
        total_claimed: msg.total_claimed,
        start_height: msg.start_height,
        end_height: msg.end_height,
        claim_enabled: msg.claim_enabled,
        unclaimed_tokens: msg.unclaimed_tokens,
    };

    // Save the new airdrop configuration to storage
    AIRDROP_CONFIG.save(deps.storage, &airdrop_config)?;

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
                AdminExecuteMsg::AddUser { user, amount } => {}
                AdminExecuteMsg::RemoveUsers(users) => {}
                AdminExecuteMsg::RemoveUser(user) => {}
                AdminExecuteMsg::WithdrawFunds() => {}
            }
        }
        ExecuteMsg::ClaimAirdrop() => {}
    }
}
