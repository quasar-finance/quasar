use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

use crate::{
    range,
    state::{RANGE_EXECUTOR_ADMIN, RANGE_SUBMITTER_ADMIN},
    ContractError,
};

#[cw_serde]
#[derive(QueryResponses)]
pub enum AdminQueryMsg {
    // Get the range submitter admin
    #[returns(String)]
    GetRangeSubmitterAdmin {},
    // Get the range executor admin
    #[returns(String)]
    GetExecutionAdmin {},
}

pub fn query_admin(deps: Deps, env: Env, query_msg: AdminQueryMsg) -> StdResult<Binary> {
    match query_msg {
        AdminQueryMsg::GetRangeSubmitterAdmin {} => get_range_submitter_admin(deps),
        AdminQueryMsg::GetExecutionAdmin {} => get_execution_admin(deps),
    }
}

pub fn get_range_submitter_admin(deps: Deps) -> StdResult<Binary> {
    let range_submitter_admin = RANGE_SUBMITTER_ADMIN.load(deps.storage)?;

    Ok(to_json_binary(&range_submitter_admin)?)
}

pub fn get_execution_admin(deps: Deps) -> StdResult<Binary> {
    let range_executor_admin = RANGE_EXECUTOR_ADMIN.load(deps.storage)?;

    Ok(to_json_binary(&range_executor_admin)?)
}
