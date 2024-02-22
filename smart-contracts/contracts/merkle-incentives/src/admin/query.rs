use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

use crate::state::INCENTIVES_ADMIN;

#[cw_serde]
#[derive(QueryResponses)]
pub enum AdminQueryMsg {
    // Get the range submitter admin
    #[returns(String)]
    GetIncentivesAdmin {},
}

pub fn match_query_admin(deps: Deps, _env: Env, query_msg: AdminQueryMsg) -> StdResult<Binary> {
    match query_msg {
        AdminQueryMsg::GetIncentivesAdmin {} => query_incentives_admin(deps),
    }
}

pub fn query_incentives_admin(deps: Deps) -> StdResult<Binary> {
    let incentives_admin = INCENTIVES_ADMIN.may_load(deps.storage)?;
    to_json_binary(&incentives_admin)
}
