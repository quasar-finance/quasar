use cl_vault::{
    msg::{ClQueryMsg, ExecuteMsg as VaultExecuteMsg, ModifyRangeMsg, QueryMsg as VaultQueryMsg},
    query::PoolResponse,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Decimal, DepsMut, Env, MessageInfo, Response, WasmMsg};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::{
    range::helpers::is_range_executor_admin,
    state::{NewRange, PENDING_RANGES},
    ContractError,
};

use super::helpers::is_range_submitter_admin;

#[cw_serde]
pub enum RangeExecuteMsg {
    /// Submit a range to the range middleware
    SubmitNewRange {
        new_range: NewRange,
    },
    /// Execute a new range
    ExecuteNewRange {
        cl_vault_address: String,
        max_slippage: Decimal,
        ratio_of_swappable_funds_to_use: Decimal,
        twap_window_seconds: u64,
        forced_swap_route: Option<Vec<SwapAmountInRoute>>,
        claim_after: Option<u64>,
    },
    RemoveRange {
        contract_address: String,
    },
}

pub struct RangeExecutionParams {
    pub cl_vault_address: String,
    pub max_slippage: Decimal,
    pub ratio_of_swappable_funds_to_use: Decimal,
    pub twap_window_seconds: u64,
    pub forced_swap_route: Option<Vec<SwapAmountInRoute>>,
    pub claim_after: Option<u64>,
}

pub fn execute_range_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    range_msg: RangeExecuteMsg,
) -> Result<Response, ContractError> {
    match range_msg {
        RangeExecuteMsg::SubmitNewRange { new_range } => {
            submit_new_range(deps, env, info, new_range)
        }
        RangeExecuteMsg::ExecuteNewRange {
            cl_vault_address,
            max_slippage,
            ratio_of_swappable_funds_to_use,
            twap_window_seconds,
            forced_swap_route,
            claim_after,
        } => execute_new_range(
            deps,
            env,
            info,
            RangeExecutionParams {
                cl_vault_address,
                max_slippage,
                ratio_of_swappable_funds_to_use,
                twap_window_seconds,
                forced_swap_route,
                claim_after,
            },
        ),
        RangeExecuteMsg::RemoveRange { contract_address } => {
            remove_range(deps, info, contract_address)
        }
    }
}

pub fn remove_range(
    deps: DepsMut,
    info: MessageInfo,
    contract_address: String,
) -> Result<Response, ContractError> {
    is_range_submitter_admin(deps.storage, &info.sender)?;
    PENDING_RANGES.remove(deps.storage, deps.api.addr_validate(&contract_address)?);
    Ok(Response::default())
}

pub fn submit_new_range(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_range: NewRange,
) -> Result<Response, ContractError> {
    is_range_submitter_admin(deps.storage, &info.sender)?;

    // get validated address
    let vault_address = deps.api.addr_validate(&new_range.cl_vault_address)?;

    // make sure it is a contract
    let contract_info_result = deps
        .querier
        .query_wasm_contract_info(new_range.cl_vault_address.clone());
    if contract_info_result.is_err() {
        return Err(ContractError::InvalidContractAddress {
            address: new_range.cl_vault_address.clone(),
        });
    }

    // try to query the contract to make sure it is a cl contract
    let pool_response_result: Result<PoolResponse, _> = deps.querier.query_wasm_smart(
        new_range.cl_vault_address.clone(),
        &VaultQueryMsg::VaultExtension(cl_vault::msg::ExtensionQueryMsg::ConcentratedLiquidity(
            ClQueryMsg::Pool {},
        )),
    );
    if pool_response_result.is_err() {
        return Err(ContractError::ClExpectedQueryFailed {
            address: new_range.cl_vault_address.clone(),
        });
    }

    // overwrite any previous submission
    PENDING_RANGES.save(deps.storage, vault_address, &new_range)?;

    Ok(Response::new()
        .add_attribute("action", "submit_new_range")
        .add_attribute("range_submitted", "true")
        .add_attribute("range_submitter", info.sender)
        .add_attribute("range_underlying_contract", new_range.cl_vault_address))
}

pub fn execute_new_range(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    params: RangeExecutionParams,
) -> Result<Response, ContractError> {
    is_range_executor_admin(deps.storage, &info.sender)?;

    let vault_address = deps.api.addr_validate(&params.cl_vault_address)?;

    let new_range_result = PENDING_RANGES.load(deps.storage, vault_address.clone());
    if new_range_result.is_err() {
        return Err(ContractError::NoRangeExists {
            address: params.cl_vault_address.clone(),
        });
    }
    let new_range = new_range_result?;

    // if range was completed, delete from pending ranges
    if params.ratio_of_swappable_funds_to_use == Decimal::one() {
        PENDING_RANGES.remove(deps.storage, vault_address.clone());
    }

    // construct message to send to cl vault
    let msg = WasmMsg::Execute {
        contract_addr: params.cl_vault_address.clone(),
        msg: to_json_binary(&VaultExecuteMsg::VaultExtension(
            cl_vault::msg::ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                lower_price: new_range.lower_price,
                upper_price: new_range.upper_price,
                max_slippage: params.max_slippage,
                ratio_of_swappable_funds_to_use: params.ratio_of_swappable_funds_to_use,
                twap_window_seconds: params.twap_window_seconds,
                forced_swap_route: params.forced_swap_route,
                claim_after: params.claim_after,
            }),
        ))?,

        funds: vec![],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "execute_new_range")
        .add_attribute("range_executed", "true")
        .add_attribute("range_executor", info.sender)
        .add_attribute("range_underlying_contract", params.cl_vault_address))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Decimal,
    };

    use crate::{
        contract::{execute, instantiate, query},
        msg::{InstantiateMsg, QueryMsg},
        range::execute::RangeExecuteMsg,
        state::{NewRange, PENDING_RANGES},
        ContractError,
    };

    const RANGE_SUBMITTER_ADMIN: &str = "range_submitter_admin";
    const RANGE_EXECUTOR_ADMIN: &str = "range_executor_admin";
    const TEST_CONTRACT: &str = "test_contract";

    #[test]
    fn test_unauthorized_user_can_not_remove_range() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        assert!(instantiate(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            InstantiateMsg {
                range_executor_admin: RANGE_EXECUTOR_ADMIN.to_string(),
                range_submitter_admin: RANGE_SUBMITTER_ADMIN.to_string(),
            }
        )
        .is_ok());

        let err = execute(
            deps.as_mut(),
            env,
            info,
            crate::msg::ExecuteMsg::RangeMsg(RangeExecuteMsg::RemoveRange {
                contract_address: TEST_CONTRACT.to_string(),
            }),
        )
        .unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});
    }

    #[test]
    fn test_range_admin_can_remove_range() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let _ = instantiate(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            InstantiateMsg {
                range_executor_admin: RANGE_EXECUTOR_ADMIN.to_string(),
                range_submitter_admin: RANGE_SUBMITTER_ADMIN.to_string(),
            },
        )
        .unwrap();

        PENDING_RANGES
            .save(
                deps.as_mut().storage,
                Addr::unchecked(TEST_CONTRACT),
                &NewRange {
                    cl_vault_address: TEST_CONTRACT.to_string(),
                    lower_price: Decimal::zero(),
                    upper_price: Decimal::one(),
                },
            )
            .unwrap();
        assert!(query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::RangeQuery(
                crate::range::query::RangeQueryMsg::GetQueuedRangeUpdatesForContract {
                    contract_address: TEST_CONTRACT.to_string()
                }
            )
        )
        .is_ok());

        let info = mock_info(RANGE_SUBMITTER_ADMIN, &[]);
        let _ = execute(
            deps.as_mut(),
            env.clone(),
            info,
            crate::msg::ExecuteMsg::RangeMsg(RangeExecuteMsg::RemoveRange {
                contract_address: TEST_CONTRACT.to_string(),
            }),
        )
        .unwrap();
        assert!(query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::RangeQuery(
                crate::range::query::RangeQueryMsg::GetQueuedRangeUpdatesForContract {
                    contract_address: TEST_CONTRACT.to_string()
                }
            )
        )
        .is_err());
    }
}
