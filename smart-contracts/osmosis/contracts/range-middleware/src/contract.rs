#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
// use cw2::set_contract_version;

use crate::admin::execute::execute_admin_msg;
use crate::admin::query::query_admin;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::range::execute::execute_range_msg;
use crate::range::query::query_range;
use crate::state::{RANGE_EXECUTOR_ADMIN, RANGE_SUBMITTER_ADMIN};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:range-middleware";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    RANGE_SUBMITTER_ADMIN.save(
        deps.storage,
        &deps.api.addr_validate(&msg.range_submitter_admin)?,
    )?;
    RANGE_EXECUTOR_ADMIN.save(
        deps.storage,
        &deps.api.addr_validate(&msg.range_executor_admin)?,
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RangeMsg(range_msg) => execute_range_msg(deps, env, info, range_msg),
        ExecuteMsg::AdminMsg(admin_msg) => execute_admin_msg(deps, env, info, admin_msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::RangeQuery(range_query) => query_range(deps, env, range_query),
        QueryMsg::AdminQuery(admin_query) => query_admin(deps, env, admin_query),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    cw2::assert_contract_version(deps.storage, CONTRACT_NAME, "0.3.0")?;
    let old_version =
        cw2::ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("old version", old_version.to_string())
        .add_attribute("new version", CONTRACT_VERSION))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cw2::{get_contract_version, ContractVersion};

    use super::*;

    #[test]
    fn migrate_works() {
        let mut deps = mock_dependencies();
        set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "0.1.0").unwrap();

        let env = mock_env();
        let msg = MigrateMsg {};

        migrate(deps.as_mut(), env, msg).unwrap();
        assert_eq!(
            get_contract_version(deps.as_ref().storage).unwrap(),
            ContractVersion {
                contract: CONTRACT_NAME.into(),
                version: CONTRACT_VERSION.into()
            }
        )
    }
}
