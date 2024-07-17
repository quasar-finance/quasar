use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

use crate::state::{RANGE_EXECUTOR_OWNER, RANGE_SUBMITTER_OWNER};

#[cw_serde]
#[derive(QueryResponses)]
pub enum AdminQueryMsg {
    // Get the range submitter admin
    #[returns(String)]
    GetRangeSubmitterOwner {},
    // Get the range executor admin
    #[returns(String)]
    GetExecutionOwner {},
}

pub fn query_admin(deps: Deps, _env: Env, query_msg: AdminQueryMsg) -> StdResult<Binary> {
    match query_msg {
        AdminQueryMsg::GetRangeSubmitterOwner {} => {
            to_json_binary(&RANGE_SUBMITTER_OWNER.query(deps.storage)?.owner)
        }
        AdminQueryMsg::GetExecutionOwner {} => {
            to_json_binary(&RANGE_EXECUTOR_OWNER.query(deps.storage)?.owner)
        }
    }
}
