#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw_storage_plus::Item;
use mars_owner::OwnerInit::SetInitialOwner;

use crate::admin::execute::execute_admin_msg;
use crate::admin::query::query_admin;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::range::execute::execute_range_msg;
use crate::range::query::query_range;
use crate::state::{
    OWNER, RANGE_EXECUTOR_OWNER, RANGE_SUBMITTER_OWNER,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:range-middleware";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.initialize(
        deps.storage,
        deps.api,
        SetInitialOwner {
            owner: info.sender.to_string(),
        },
    )?;

    RANGE_SUBMITTER_OWNER.initialize(
        deps.storage,
        deps.api,
        SetInitialOwner {
            owner: msg.range_submitter_owner,
        },
    )?;

    RANGE_EXECUTOR_OWNER.initialize(
        deps.storage,
        deps.api,
        SetInitialOwner {
            owner: msg.range_executor_owner,
        },
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
        ExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
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
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    msg: MigrateMsg,
) -> Result<Response, ContractError> {
    pub const RANGE_SUBMITTER_ADMIN: Item<Addr> = Item::new("range_submitter_admin");
    pub const RANGE_EXECUTOR_ADMIN: Item<Addr> = Item::new("range_executor_admin");
    
    let submitter_admin = RANGE_SUBMITTER_ADMIN.load(deps.storage)?;
    let executor_admin = RANGE_EXECUTOR_ADMIN.load(deps.storage)?;

    OWNER.initialize(
        deps.storage,
        deps.api,
        SetInitialOwner {
            owner: msg.new_owner,
        },
    )?;

    RANGE_SUBMITTER_OWNER.initialize(
        deps.storage,
        deps.api,
        SetInitialOwner {
            owner: submitter_admin.into_string(),
        },
    )?;

    RANGE_EXECUTOR_OWNER.initialize(
        deps.storage,
        deps.api,
        SetInitialOwner {
            owner: executor_admin.into_string(),
        },
    )?;
    RANGE_SUBMITTER_ADMIN.remove(deps.storage);
    RANGE_EXECUTOR_ADMIN.remove(deps.storage);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("migrate", "successful"))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        Addr, Api,
    };

    use super::*;

    #[test]
    fn migrate_works() {
        //legacy state items
        pub const RANGE_SUBMITTER_ADMIN: Item<Addr> = Item::new("range_submitter_admin");
        pub const RANGE_EXECUTOR_ADMIN: Item<Addr> = Item::new("range_executor_admin");

        let mut deps = mock_dependencies();
        let env = mock_env();
        let owner_addr = deps.api.addr_make("owner");

        // set initial state
        let submitter_addr = deps.api.addr_make("submitter");
        RANGE_SUBMITTER_ADMIN
            .save(deps.as_mut().storage, &submitter_addr)
            .unwrap();
        let executor_addr = deps.api.addr_make("executor");
        RANGE_EXECUTOR_ADMIN
            .save(deps.as_mut().storage, &executor_addr)
            .unwrap();

        let migrate_response = migrate(
            deps.as_mut(),
            env.clone(),
            MigrateMsg {
                new_owner: owner_addr.to_string(),
            },
        )
        .unwrap();

        // assert migration execution
        assert_eq!(migrate_response.attributes[0].key, "migrate");
        assert_eq!(migrate_response.attributes[0].value, "successful");

        // assert new owners
        let owner = OWNER.query(deps.as_ref().storage).unwrap();
        assert_eq!(owner.owner.unwrap(), owner_addr.to_string());

        let submitter_admin = RANGE_SUBMITTER_OWNER.query(deps.as_ref().storage).unwrap();
        assert_eq!(submitter_admin.owner.unwrap(), submitter_addr.to_string());

        let executor_admin = RANGE_EXECUTOR_OWNER.query(deps.as_ref().storage).unwrap();
        assert_eq!(executor_admin.owner.unwrap(), executor_addr.to_string());
    }
}
