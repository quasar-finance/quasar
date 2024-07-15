use cl_vault::{
    msg::{
        ClQueryMsg, ExecuteMsg as VaultExecuteMsg, ModifyRange, MovePosition,
        QueryMsg as VaultQueryMsg,
    },
    query::PoolResponse,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Decimal, DepsMut, Env, MessageInfo, Response, WasmMsg,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::{
    range::helpers::is_range_executor_admin,
    state::{NewRange, RangeUpdates, UpdateActions, PENDING_RANGES},
    ContractError,
};

use super::helpers::is_range_submitter_admin;

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum RangeExecuteMsg {
    /// Submit a range to the range middleware
    SubmitNewRange { new_ranges: RangeUpdates },
    /// Execute a new range
    PopRangeUpdate {
        vault_address: String,
        params: Option<RangeExecutionParams>,
    },
}

#[cw_serde]
pub struct RangeExecutionParams {
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
        RangeExecuteMsg::SubmitNewRange { new_ranges } => {
            submit_new_range(deps, env, info, new_ranges)
        }
        RangeExecuteMsg::PopRangeUpdate {
            vault_address,
            params,
        } => execute_pop_update(deps, env, info, vault_address, params),
    }
}

pub fn submit_new_range(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_ranges: RangeUpdates,
) -> Result<Response, ContractError> {
    is_range_submitter_admin(deps.storage, &info.sender)?;

    // get validated address
    let vault_address = deps.api.addr_validate(&new_ranges.cl_vault_address)?;

    // make sure it is a contract
    let contract_info_result = deps
        .querier
        .query_wasm_contract_info(new_ranges.cl_vault_address.clone());
    if contract_info_result.is_err() {
        return Err(ContractError::InvalidContractAddress {
            address: vault_address.to_string(),
        });
    }

    // TODO this query can be done via CW2, which allows us to also include version checks
    // try to query the contract to make sure it is a cl contract
    let pool_response_result: Result<PoolResponse, _> = deps.querier.query_wasm_smart(
        vault_address.to_string(),
        &VaultQueryMsg::VaultExtension(cl_vault::msg::ExtensionQueryMsg::ConcentratedLiquidity(
            ClQueryMsg::Pool {},
        )),
    );
    if pool_response_result.is_err() {
        return Err(ContractError::ClExpectedQueryFailed {
            address: vault_address.to_string(),
        });
    }

    // overwrite any previous submission
    PENDING_RANGES.save(deps.storage, vault_address.clone(), &new_ranges)?;

    Ok(Response::new()
        .add_attribute("action", "submit_new_range")
        .add_attribute("range_submitted", "true")
        .add_attribute("range_submitter", info.sender)
        .add_attribute("range_underlying_contract", vault_address))
}

// TODO the optional params are not super nice here, a different solution would be nice
fn execute_pop_update(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    vault_address: String,
    params: Option<RangeExecutionParams>,
) -> Result<Response, ContractError> {
    is_range_executor_admin(deps.storage, &info.sender)?;

    let vault_address = deps.api.addr_validate(&vault_address)?;

    // TODO make this a may load and error nicely on non existing
    let mut ranges = PENDING_RANGES.load(deps.storage, vault_address.clone())?;

    let next = ranges.updates.pop_front();
    if let Some(next) = next {
        let response = match next {
            UpdateActions::CreatePosition(msg) => do_create_position(&vault_address, msg)?,
            UpdateActions::DeletePosition(msg) => do_delete_position(&vault_address, msg)?,
            UpdateActions::DecreaseFunds(msg) => do_decrease_funds(&vault_address, msg)?,
            UpdateActions::IncreaseFunds(msg) => do_increase_funds(&vault_address, msg)?,
            UpdateActions::NewRange(msg) => {
                // if the range was fully executed
                let params = params.ok_or(ContractError::NoRangeExecutionParams {
                    address: vault_address.to_string(),
                })?;

                // TODO This should be refactored somewhat to allow for better reading, refactoring this requires
                // swap code to be refactored too

                // TODO this needs a good comment to explain how this works together
                // if the entire ratio was not executed, repeat the action to partially funds
                if params.ratio_of_swappable_funds_to_use < Decimal::one() {
                    ranges
                        .updates
                        .push_front(UpdateActions::NewRange(msg.clone()))
                }

                do_move_position(params, msg, vault_address.clone())?
            }
        };

        if ranges.updates.is_empty() {
            PENDING_RANGES.remove(deps.storage, vault_address.clone());
            Ok(response
                .add_attribute("range_executed", "true")
                .add_attribute("range_executor", info.sender)
                .add_attribute("range_underlying_contract", vault_address)
                .add_attribute("action", "execute_new_range")
                .add_attribute("status", "finished"))
        } else {
            PENDING_RANGES.save(deps.storage, vault_address.clone(), &ranges)?;

            Ok(response
                .add_attribute("range_executed", "true")
                .add_attribute("range_executor", info.sender)
                .add_attribute("range_underlying_contract", vault_address)
                .add_attribute("action", "execute_new_range")
                .add_attribute("status", "ongoing"))
        }
    } else {
        PENDING_RANGES.remove(deps.storage, vault_address.clone());
        Ok(Response::new().add_attribute("range_finished", vault_address))
    }
}

fn do_create_position(
    vault_address: &Addr,
    msg: cl_vault::msg::CreatePosition,
) -> Result<Response, ContractError> {
    Ok(call_vault(
        vault_address.clone(),
        to_json_binary(&VaultExecuteMsg::VaultExtension(
            cl_vault::msg::ExtensionExecuteMsg::ModifyRange(ModifyRange::CreatePosition(msg)),
        ))?,
    )?
    .add_attribute("update", "create_position"))
}

fn do_delete_position(
    vault_address: &Addr,
    msg: cl_vault::msg::DeletePosition,
) -> Result<Response, ContractError> {
    Ok(call_vault(
        vault_address.clone(),
        to_json_binary(&VaultExecuteMsg::VaultExtension(
            cl_vault::msg::ExtensionExecuteMsg::ModifyRange(ModifyRange::DeletePosition(msg)),
        ))?,
    )?
    .add_attribute("update", "delete_position"))
}

fn do_decrease_funds(
    vault_address: &Addr,
    msg: cl_vault::msg::DecreaseFunds,
) -> Result<Response, ContractError> {
    Ok(call_vault(
        vault_address.clone(),
        to_json_binary(&VaultExecuteMsg::VaultExtension(
            cl_vault::msg::ExtensionExecuteMsg::ModifyRange(ModifyRange::DecreaseFunds(msg)),
        ))?,
    )?
    .add_attribute("update", "decrease_funds"))
}

fn do_increase_funds(
    vault_address: &Addr,
    msg: cl_vault::msg::IncreaseFunds,
) -> Result<Response, ContractError> {
    Ok(call_vault(
        vault_address.clone(),
        to_json_binary(&VaultExecuteMsg::VaultExtension(
            cl_vault::msg::ExtensionExecuteMsg::ModifyRange(ModifyRange::IncreaseFunds(msg)),
        ))?,
    )?
    .add_attribute("update", "increase_funds"))
}
pub fn do_move_position(
    params: RangeExecutionParams,
    new_range: NewRange,
    vault_address: Addr,
) -> Result<Response, ContractError> {
    // construct message to send to cl vault
    let msg = WasmMsg::Execute {
        contract_addr: vault_address.to_string(),
        msg: to_json_binary(&VaultExecuteMsg::VaultExtension(
            cl_vault::msg::ExtensionExecuteMsg::ModifyRange(ModifyRange::MovePosition(MovePosition {
                lower_price: new_range.lower_price,
                upper_price: new_range.upper_price,
                max_slippage: params.max_slippage,
                ratio_of_swappable_funds_to_use: params.ratio_of_swappable_funds_to_use,
                twap_window_seconds: params.twap_window_seconds,
                forced_swap_route: params.forced_swap_route,
                claim_after: params.claim_after,
                position_id: new_range.position_id, })) ,
        ))?,

        funds: vec![],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("update", "move_position"))
}

pub fn call_vault(vault: Addr, msg: Binary) -> Result<Response, ContractError> {
    let msg = WasmMsg::Execute {
        contract_addr: vault.to_string(),
        msg,
        funds: vec![],
    };

    Ok(Response::new().add_message(msg))
}

#[cfg(test)]
mod tests {
    use cl_vault::msg::CreatePosition;
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        Addr, Decimal, MessageInfo,
    };
    use std::collections::VecDeque;

    use crate::state::{
        NewRange, RangeUpdates, UpdateActions, PENDING_RANGES, RANGE_EXECUTOR_ADMIN,
    };

    use super::{execute_pop_update, RangeExecutionParams};

    #[test]
    fn pop_move_position_partial_execution_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let sender = "alice";
        let info = MessageInfo {
            sender: Addr::unchecked(sender),
            funds: vec![],
        };
        RANGE_EXECUTOR_ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked(sender))
            .unwrap();

        let vault_address = "contract1";

        let mut updates = VecDeque::new();
        updates.push_back(UpdateActions::NewRange(NewRange {
            position_id: 1,
            lower_price: Decimal::zero(),
            upper_price: Decimal::one(),
        }));
        updates.push_back(UpdateActions::CreatePosition(CreatePosition {
            lower_price: Decimal::from_ratio(1_u128, 2_u128),
            upper_price: Decimal::one(),
            claim_after: None,
            max_token0: None,
            max_token1: None,
        }));

        let range_update = RangeUpdates {
            updates,
            cl_vault_address: vault_address.to_string(),
        };

        PENDING_RANGES
            .save(
                deps.as_mut().storage,
                Addr::unchecked(vault_address),
                &range_update,
            )
            .unwrap();

        let params = RangeExecutionParams {
            max_slippage: Decimal::percent(99),
            ratio_of_swappable_funds_to_use: Decimal::percent(10),
            twap_window_seconds: 24,
            forced_swap_route: None,
            claim_after: None,
        };

        let _res = execute_pop_update(
            deps.as_mut(),
            env,
            info,
            vault_address.to_string(),
            Some(params),
        )
        .unwrap();

        assert_eq!(
            PENDING_RANGES
                .load(deps.as_mut().storage, Addr::unchecked(vault_address))
                .unwrap(),
            range_update
        )
    }

    #[test]
    fn pop_move_position_complete_execution_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let sender = "alice";
        let info = MessageInfo {
            sender: Addr::unchecked(sender),
            funds: vec![],
        };

        RANGE_EXECUTOR_ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked(sender))
            .unwrap();

        let vault_address = "contract1";

        let mut updates = VecDeque::new();
        updates.push_back(UpdateActions::NewRange(NewRange {
            position_id: 1,
            lower_price: Decimal::zero(),
            upper_price: Decimal::one(),
        }));
        updates.push_back(UpdateActions::CreatePosition(CreatePosition {
            lower_price: Decimal::from_ratio(1_u128, 2_u128),
            upper_price: Decimal::one(),
            claim_after: None,
            max_token0: None,
            max_token1: None,
        }));

        let mut range_update = RangeUpdates {
            updates,
            cl_vault_address: vault_address.to_string(),
        };

        PENDING_RANGES
            .save(
                deps.as_mut().storage,
                Addr::unchecked(vault_address),
                &range_update,
            )
            .unwrap();

        let params = RangeExecutionParams {
            max_slippage: Decimal::percent(99),
            ratio_of_swappable_funds_to_use: Decimal::one(),
            twap_window_seconds: 24,
            forced_swap_route: None,
            claim_after: None,
        };

        let _res = execute_pop_update(
            deps.as_mut(),
            env,
            info,
            vault_address.to_string(),
            Some(params),
        )
        .unwrap();

        // at this point we expect to have fully executed the move position and this expect the front item of updates to be popped
        range_update.updates.pop_front();

        assert_eq!(
            PENDING_RANGES
                .load(deps.as_mut().storage, Addr::unchecked(vault_address))
                .unwrap(),
            range_update
        );
    }
}
