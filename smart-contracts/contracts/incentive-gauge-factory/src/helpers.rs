use cosmwasm_std::Env;

use crate::{types::Gauge, ContractError};

pub fn check_time_conf(_env: Env, _guage: Gauge) -> Result<(), ContractError> {
    todo!()
}
