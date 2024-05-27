use cosmwasm_std::Env;

use crate::{types::BlockPeriod, ContractError};

pub fn check_time_conf(env: Env, period: &BlockPeriod) -> Result<(), ContractError> {
    if env.block.height.ge(&period.start)  {
        return Err(ContractError::StartTimeMustBeAhead)
    }

    if env.block.height.ge(&period.end)  {
        return Err(ContractError::EndTimeMustBeAhead)
    }

    if env.block.height.ge(&period.expiry)  {
        return Err(ContractError::ExpiryTimeMustBeAhead)
    }

    if period.end.le(&period.start) {
        return Err(ContractError::EndTimeBiggerThanStart)
    }

    if period.expiry.le(&period.start) {
        return Err(ContractError::ExpiryTimeBiggerThanStart)
    }

    Ok(())
}
