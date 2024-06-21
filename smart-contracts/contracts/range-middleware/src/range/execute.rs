use cl_vault::{
    msg::{ClQueryMsg, ExecuteMsg as VaultExecuteMsg, ModifyRange, MovePosition, QueryMsg as VaultQueryMsg},
    query::PoolResponse,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Addr, Decimal, DepsMut, Env, MessageInfo, Response, WasmMsg};
use cw_dex_router::operations::SwapOperationsListUnchecked;

use crate::{
    range::helpers::is_range_executor_admin,
    state::{NewRange, RangeUpdates, PENDING_RANGES},
    ContractError,
};

use super::helpers::is_range_submitter_admin;

#[cw_serde]
pub enum RangeExecuteMsg {
    /// Submit a range to the range middleware
    SubmitNewRange { new_ranges: RangeUpdates },
    /// Execute a new range
    PopRangeUpdate {
        vault_address: String,
        params: Option<RangeExecutionParams>        
    },
}

#[cw_serde]
pub struct RangeExecutionParams {
    pub max_slippage: Decimal,
    pub ratio_of_swappable_funds_to_use: Decimal,
    pub twap_window_seconds: u64,
    pub recommended_swap_route: SwapOperationsListUnchecked,
    pub force_swap_route: bool,
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
        RangeExecuteMsg::PopRangeUpdate { vault_address, params } => execute_pop_update(deps, env, info, vault_address, params)
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

    let mut ranges = PENDING_RANGES.load(deps.storage, vault_address.clone())?;
    let next = ranges.updates.pop_front();
    if let Some(next) = next {
        // TODO assert that the pending range is properly decremented here
        PENDING_RANGES.save(deps.storage, vault_address.clone(), &ranges)?;

        let response = match next {
            crate::state::UpdateActions::CreatePosition(msg) => todo!(),
            crate::state::UpdateActions::DeletePosition(msg) => todo!(),
            crate::state::UpdateActions::DecreaseFunds(msg) => todo!(),
            crate::state::UpdateActions::IncreaseFunds(msg) => todo!(),
            crate::state::UpdateActions::NewRange(msg) => do_move_position(deps, params.ok_or(ContractError::NoRangeExecutionParams { address: vault_address.to_string() })?, msg, vault_address.clone()),
        };

        Ok(response?.add_attribute("range_executed", "true")
        .add_attribute("range_executor", info.sender)
        .add_attribute("range_underlying_contract", vault_address)
        .add_attribute("action", "execute_new_range"))
    } else {
        PENDING_RANGES.remove(deps.storage, vault_address.clone());
        Ok(Response::new().add_attribute("range_finished", vault_address))
    }
}

pub fn do_move_position(
    deps: DepsMut,
    params: RangeExecutionParams,
    new_range: NewRange,
    vault_address: Addr
) -> Result<Response, ContractError> {

    // TODO this should pop the pending range of the front
    // if range was completed, delete from pending ranges
    if params.ratio_of_swappable_funds_to_use == Decimal::one() {
        PENDING_RANGES.remove(deps.storage, vault_address.clone());
    }

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
                force_swap_route: params.force_swap_route,
                recommended_swap_route: Some(params.recommended_swap_route),
                claim_after: params.claim_after,
                position_id: new_range.position_id, })) ,
        ))?,

        funds: vec![],
    };

    Ok(Response::new()
        .add_message(msg)
)
}
