use cosmwasm_std::{
    attr, coin, Coin, Decimal, DepsMut, Env, MessageInfo, Response, SubMsg, SubMsgResult, Uint128,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountInResponse;

use crate::{
    helpers::getters::{
        get_depositable_tokens, get_single_sided_deposit_0_to_1_swap_amount,
        get_single_sided_deposit_1_to_0_swap_amount,
    },
    reply::Replies,
    state::{CURRENT_SWAP_ANY_DEPOSIT, POOL_CONFIG},
    vault::{
        concentrated_liquidity::{get_cl_pool_info, get_position},
        exact_deposit::execute_deposit,
        swap::{calculate_swap_amount, SwapDirection},
    },
    ContractError,
};

pub(crate) fn execute_any_deposit(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
    max_slippage: Decimal,
) -> Result<Response, ContractError> {
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;
    let position = get_position(deps.storage, &deps.querier)?
        .position
        .ok_or(ContractError::MissingPosition {})?;

    // get the amount of funds we can deposit from this ratio
    let (deposit_amount_in_ratio, swappable_amount): ((Uint128, Uint128), (Uint128, Uint128)) =
        get_depositable_tokens(&deps.branch(), &info.funds, &pool_config)?;

    if swappable_amount.0.is_zero() && swappable_amount.1.is_zero() {
        return execute_deposit(
            &mut deps,
            env,
            recipient,
            deposit_amount_in_ratio,
            (
                coin(0u128, pool_config.token0),
                coin(0u128, pool_config.token1),
            ),
        );
    }

    // Swap logic
    // TODO_FUTURE: Optimize this if conditions
    let (swap_amount, swap_direction, left_over_amount) = if !swappable_amount.0.is_zero() {
        // range is above current tick
        let swap_amount = if pool_details.current_tick > position.upper_tick {
            swappable_amount.0
        } else {
            get_single_sided_deposit_0_to_1_swap_amount(
                swappable_amount.0,
                position.lower_tick,
                pool_details.current_tick,
                position.upper_tick,
            )?
        };
        let left_over_amount = swappable_amount.0.checked_sub(swap_amount)?;
        (swap_amount, SwapDirection::ZeroToOne, left_over_amount)
    } else {
        // current tick is above range
        let swap_amount = if pool_details.current_tick < position.lower_tick {
            swappable_amount.1
        } else {
            get_single_sided_deposit_1_to_0_swap_amount(
                swappable_amount.1,
                position.lower_tick,
                pool_details.current_tick,
                position.upper_tick,
            )?
        };
        let left_over_amount = swappable_amount.1.checked_sub(swap_amount)?;
        (swap_amount, SwapDirection::OneToZero, left_over_amount)
    };
    CURRENT_SWAP_ANY_DEPOSIT.save(
        deps.storage,
        &(
            swap_direction.clone(),
            left_over_amount,
            recipient.clone(),
            deposit_amount_in_ratio,
        ),
    )?;
    let swap_calc_result = calculate_swap_amount(
        deps,
        &env,
        pool_config,
        swap_direction,
        swap_amount,
        max_slippage,
        None, // TODO: check this None
        24u64,
    )?;

    // rest minting logic remains same
    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            swap_calc_result.swap_msg,
            Replies::AnyDepositSwap.into(),
        ))
        .add_attributes(vec![
            attr("method", "execute"),
            attr("action", "any_deposit"),
            attr(
                "token_in",
                format!("{}{}", swap_amount, swap_calc_result.token_in_denom),
            ),
            attr(
                "token_out_min",
                format!("{}", swap_calc_result.token_out_min_amount),
            ),
        ]))
}

pub fn handle_any_deposit_swap_reply(
    mut deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    // Attempt to directly parse the data to MsgSwapExactAmountInResponse outside of the match
    let resp: MsgSwapExactAmountInResponse = data.try_into()?;

    let (swap_direction, left_over_amount, recipient, deposit_amount_in_ratio) =
        CURRENT_SWAP_ANY_DEPOSIT.load(deps.storage)?;
    CURRENT_SWAP_ANY_DEPOSIT.remove(deps.storage);

    // get post swap balances to create positions with
    let (balance0, balance1): (Uint128, Uint128) = match swap_direction {
        SwapDirection::ZeroToOne => (
            left_over_amount,
            Uint128::new(resp.token_out_amount.parse()?),
        ),
        SwapDirection::OneToZero => (
            Uint128::new(resp.token_out_amount.parse()?),
            left_over_amount,
        ),
    };

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let coins_to_mint_for = (
        Coin {
            denom: pool_config.token0.clone(),
            amount: balance0 + deposit_amount_in_ratio.0,
        },
        Coin {
            denom: pool_config.token1.clone(),
            amount: balance1 + deposit_amount_in_ratio.1,
        },
    );

    execute_deposit(
        &mut deps,
        env,
        recipient,
        (coins_to_mint_for.0.amount, coins_to_mint_for.1.amount),
        (
            coin(0u128, pool_config.token0),
            coin(0u128, pool_config.token1),
        ),
    )
}
