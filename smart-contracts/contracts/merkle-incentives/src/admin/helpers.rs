use crate::{error::ContractError, state::INCENTIVES_ADMIN};
use cosmwasm_std::{Addr, Deps, Env, QuerierWrapper};

pub fn is_contract_admin(
    querier: &QuerierWrapper,
    env: &Env,
    sus_admin: &Addr,
) -> Result<(), ContractError> {
    let contract_admin = querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;
    if let Some(contract_admin) = contract_admin {
        if contract_admin != *sus_admin {
            return Err(ContractError::Unauthorized {});
        }
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub fn is_incentives_admin(deps: Deps, sus_admin: &Addr) -> Result<(), ContractError> {
    let incentives_admin = INCENTIVES_ADMIN.may_load(deps.storage)?;

    match incentives_admin {
        Some(incentives_admin) => {
            if incentives_admin != sus_admin {
                Err(ContractError::Unauthorized {})
            } else {
                Ok(())
            }
        }
        None => Err(ContractError::Unauthorized {}),
    }
}
