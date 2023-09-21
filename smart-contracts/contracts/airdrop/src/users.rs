use cosmwasm_std::{Addr, DepsMut, Env, Response};
use cw20_base::contract::query_balance;
use cw_asset::Asset;

use crate::state::{AIRDROP_CONFIG, USER_INFO};
use crate::AirdropErrors;

pub fn execute_claim(deps: DepsMut, env: Env, user: Addr) -> Result<Response, AirdropErrors> {
    // Load the current airdrop configuration from storage
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    if current_airdrop_config.start_height == 0
        || current_airdrop_config.end_height == 0
        || env.block.height > current_airdrop_config.end_height
        || env.block.height < current_airdrop_config.start_height
    {
        return Err(AirdropErrors::InvalidClaim {});
    }

    // Validate the withdrawal address
    deps.api.addr_validate(user.as_ref())?;

    let user_info = USER_INFO.load(deps.storage, user.to_string())?;
    if user_info.get_claimed_flag() {
        return Err(AirdropErrors::AlreadyClaimed {});
    }

    // Get the admin address of the contract
    let admin_address = deps
        .querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;

    // Get the contract's bank balance
    let contract_bank_balance = query_balance(deps.as_ref(), admin_address.unwrap())
        .unwrap()
        .balance;

    if user_info.get_claimable_amount() > contract_bank_balance {
        return Err(AirdropErrors::InsufficientFundsInContractAccount {});
    }

    // Transfer the airdrop asset to the withdrawal address
    // TODO: Store this transaction as an event
    Asset::new(current_airdrop_config.airdrop_asset, contract_bank_balance).transfer_msg(&user)?;

    // Return a default response if all checks pass
    // TODO: Add events
    Ok(Response::default())
}
