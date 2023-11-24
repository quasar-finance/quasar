use crate::{
    error::ContractError,
    state::{RANGE_EXECUTOR_ADMIN, RANGE_SUBMITTER_ADMIN},
};
use cosmwasm_std::{Addr, Storage};

pub fn is_range_submitter_admin(
    storage: &dyn Storage,
    sus_admin: &Addr,
) -> Result<(), ContractError> {
    let contract_admin = RANGE_SUBMITTER_ADMIN.may_load(storage)?;
    if let Some(contract_admin) = contract_admin {
        if contract_admin != *sus_admin {
            return Err(ContractError::Unauthorized {});
        }
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub fn is_range_executor_admin(
    storage: &dyn Storage,
    sus_admin: &Addr,
) -> Result<(), ContractError> {
    let contract_admin = RANGE_EXECUTOR_ADMIN.may_load(storage)?;
    if let Some(contract_admin) = contract_admin {
        if contract_admin != *sus_admin {
            return Err(ContractError::Unauthorized {});
        }
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}
