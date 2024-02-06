use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

#[cw_serde]
#[derive(QueryResponses)]
pub enum AdminQueryMsg {
    // Get the range submitter admin
    #[returns(String)]
    GetIncentivesAdmin {},
}

pub fn query_admin(deps: Deps, _env: Env, query_msg: AdminQueryMsg) -> StdResult<Binary> {
    match query_msg {
        AdminQueryMsg::GetIncentivesAdmin {} => get_incentives_admin(deps),
    }
}

pub fn get_incentives_admin(deps: Deps) -> StdResult<Binary> {
    todo!()
}
