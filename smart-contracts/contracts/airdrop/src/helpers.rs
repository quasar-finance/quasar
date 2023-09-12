use cosmwasm_std::{Addr, Env, QuerierWrapper};

use crate::AirdropErrors;

pub fn is_contract_admin(
    querier: &QuerierWrapper,
    env: &Env,
    sus_admin: &Addr,
) -> Result<(), AirdropErrors> {
    let contract_admin = querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;
    if let Some(contract_admin) = contract_admin {
        if contract_admin != *sus_admin {
            return Err(AirdropErrors::Unauthorized {});
        }
    } else {
        return Err(AirdropErrors::Unauthorized {});
    }
    Ok(())
}
