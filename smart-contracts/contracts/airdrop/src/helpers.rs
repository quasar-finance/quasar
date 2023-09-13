use cosmwasm_std::{Addr, DepsMut, Env, Order, QuerierWrapper, Uint128};

use crate::state::USER_INFO;
use crate::AirdropErrors;

pub fn is_contract_admin(
    querier: &QuerierWrapper,
    env: &Env,
    sus_admin: &Addr,
) -> Result<(), AirdropErrors> {
    // Get the contract admin address from the contract's information
    let contract_admin = querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;

    // Check if the contract admin address exists
    if let Some(contract_admin) = contract_admin {
        // Compare the contract admin address with the provided sus_admin address
        if contract_admin != *sus_admin {
            // If they don't match, return an Unauthorized error
            return Err(AirdropErrors::Unauthorized {});
        }
    } else {
        // If the contract admin address doesn't exist, return an Unauthorized error
        return Err(AirdropErrors::Unauthorized {});
    }

    // If all checks pass, return Ok() to indicate success
    Ok(())
}

pub fn get_total_in_user_info(deps: DepsMut) -> Uint128 {
    let mut total_claimable_amount = Uint128::zero();

    // Iterate over the entire USER_INFO map in ascending order
    // Use `Order::Ascending` to specify the desired order
    for (_key, value) in USER_INFO.range(deps.storage, None, None, Order::Ascending) {
        // 'value' is the Vec<UserInfo> associated with the key

        // Sum the claimable_amount values in 'value'
        for user_info_entry in value.iter() {
            total_claimable_amount += user_info_entry.claimable_amount;
        }
    }

    // Return the total claimable amount
    total_claimable_amount
}
