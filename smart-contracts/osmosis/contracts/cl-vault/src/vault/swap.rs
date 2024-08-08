use cosmwasm_std::{coin, Addr, Coin, CosmosMsg, Decimal, DepsMut, MessageInfo, Response, Uint128};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::{
    helpers::{assert::assert_range_admin, msgs::swap_msg},
    msg::SwapOperation,
    state::{PoolConfig, DEX_ROUTER, POOL_CONFIG, VAULT_CONFIG},
    ContractError,
};

#[cosmwasm_schema::cw_serde]
pub enum SwapDirection {
    ZeroToOne,
    OneToZero,
}

/// SwapCalculationResult holds the result of a swap calculation
pub struct SwapCalculationResult {
    pub swap_msg: CosmosMsg,
    pub offer: Coin,
    pub token_out_min_amount: Uint128,
}

pub fn execute_swap_non_vault_funds(
    deps: DepsMut,
    contract_address: Addr,
    info: MessageInfo,
    swap_operations: Vec<SwapOperation>,
) -> Result<Response, ContractError> {
    assert_range_admin(deps.storage, &info.sender)?;

    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    if swap_operations.is_empty() {
        return Err(ContractError::EmptySwapOperations {});
    }

    let mut swap_msgs: Vec<CosmosMsg> = vec![];

    for swap_operation in swap_operations {
        let token_in_denom = swap_operation.token_in_denom.clone();
        let pool_token_0 = pool_config.token0.clone();
        let pool_token_1 = pool_config.token1.clone();

        // Assert that no BASE_DENOM or QUOTE_DENOM is trying to be swapped as token_in
        if token_in_denom == pool_token_0 || token_in_denom == pool_token_1 {
            return Err(ContractError::InvalidSwapAssets {});
        }

        let balance_in_contract = deps
            .querier
            .query_balance(
                contract_address.clone(),
                swap_operation.clone().token_in_denom,
            )?
            .amount;

        // TODO_FUTURE: This could be a <= condition against a threshold value mayube in dollars to avoid dust swaps
        if balance_in_contract == Uint128::zero() {
            // TODO: Use InsufficientFundsForSwap instead, this has been removed after STRATEGIST_REWARDS state eval removal
            return Err(ContractError::InsufficientFunds {});
        }

        // TODO_FUTURE: We could be swapping into the actual vault balance so we could prepend_swap() the autocompound entrypoint.
        let part_0_amount = balance_in_contract.checked_div(Uint128::new(2))?;
        let part_1_amount = balance_in_contract
            .checked_add(Uint128::new(1))?
            .checked_div(Uint128::new(2))?;

        // TODO_FUTURE: We should be passing the max_slippage from outside as we do during ModifyRange
        let token_out_min_amount_0 =
            part_0_amount.checked_mul_floor(vault_config.swap_max_slippage)?;
        let token_out_min_amount_1 =
            part_1_amount.checked_mul_floor(vault_config.swap_max_slippage)?;

        let dex_router = DEX_ROUTER.may_load(deps.storage)?;
        swap_msgs.push(swap_msg(
            contract_address.clone(),
            swap_operation.pool_id_0,
            coin(part_0_amount.into(), token_in_denom.clone()),
            coin(token_out_min_amount_0.into(), pool_token_0),
            swap_operation.forced_swap_route_token_0,
            dex_router.clone(),
        )?);
        swap_msgs.push(swap_msg(
            contract_address.clone(),
            swap_operation.pool_id_1,
            coin(part_1_amount.into(), token_in_denom.clone()),
            coin(token_out_min_amount_1.into(), pool_token_1),
            swap_operation.forced_swap_route_token_1,
            dex_router,
        )?);
    }

    Ok(Response::new()
        .add_messages(swap_msgs)
        .add_attribute("method", "execute")
        .add_attribute("action", "swap_non_vault_funds"))
}

#[allow(clippy::too_many_arguments)]
pub fn calculate_swap_amount(
    contract_address: Addr,
    pool_config: PoolConfig,
    swap_direction: SwapDirection,
    token_in_amount: Uint128,
    max_slippage: Decimal,
    forced_swap_route: Option<Vec<SwapAmountInRoute>>,
    twap_price: Decimal,
    dex_router: Option<Addr>,
) -> Result<SwapCalculationResult, ContractError> {
    let (token_in_denom, token_out_denom, token_out_ideal_amount) = match swap_direction {
        SwapDirection::ZeroToOne => (
            &pool_config.token0,
            &pool_config.token1,
            token_in_amount.checked_mul_floor(twap_price),
        ),
        SwapDirection::OneToZero => (
            &pool_config.token1,
            &pool_config.token0,
            token_in_amount.checked_div_floor(twap_price),
        ),
    };

    let token_out_min_amount = token_out_ideal_amount?.checked_mul_floor(max_slippage)?;
    let swap_msg = swap_msg(
        contract_address,
        pool_config.pool_id,
        coin(token_in_amount.into(), token_in_denom.clone()),
        coin(token_out_min_amount.into(), token_out_denom.clone()),
        forced_swap_route,
        dex_router,
    )?;

    Ok(SwapCalculationResult {
        swap_msg,
        offer: coin(token_in_amount.into(), token_in_denom.clone()),
        token_out_min_amount,
    })
}
