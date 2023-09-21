use cosmwasm_std::{Deps, Order, StdResult};
use std::string::String;

use crate::msg::{ConfigResponse, ContractStateResponse, UserInfoResponse};
use crate::state::{UserInfo, AIRDROP_CONFIG, USER_INFO};

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = AIRDROP_CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        airdrop_config: config,
    })
}

pub fn query_user(deps: Deps, user: String) -> StdResult<UserInfoResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let user_info = USER_INFO.load(deps.storage, user_addr.to_string())?;
    Ok(UserInfoResponse { user_info })
}

pub fn query_contract_state(deps: Deps) -> StdResult<ContractStateResponse> {
    let config = AIRDROP_CONFIG.load(deps.storage)?;
    let mut user_infos: Vec<(String, UserInfo)> = Vec::new();
    for res in USER_INFO.range(deps.storage, None, None, Order::Ascending) {
        let unwrapped_res = res.unwrap();
        user_infos.push((unwrapped_res.0, unwrapped_res.1))
    }
    Ok(ContractStateResponse {
        airdrop_config: config,
        user_info: user_infos,
    })
}
