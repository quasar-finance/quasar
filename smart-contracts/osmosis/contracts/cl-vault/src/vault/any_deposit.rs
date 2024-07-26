use cosmwasm_std::{
    attr, coin, Addr, Coin, Decimal, DepsMut, Env, Fraction, MessageInfo, Response, SubMsg,
    SubMsgResult, Uint128, Uint256,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountInResponse;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMint;

use crate::{
    helpers::{
        assert::must_pay_one_or_two,
        getters::{
            get_asset0_value, get_depositable_tokens, get_single_sided_deposit_0_to_1_swap_amount,
            get_single_sided_deposit_1_to_0_swap_amount, get_twap_price,
        },
        msgs::swap_msg,
    },
    query::{query_total_assets, query_total_vault_token_supply},
    reply::Replies,
    state::{PoolConfig, CURRENT_SWAP_ANY_DEPOSIT, POOL_CONFIG, SHARES, VAULT_DENOM},
    vault::{
        concentrated_liquidity::{get_cl_pool_info, get_position},
        range::SwapDirection,
        swap::{SwapCalculationResult, SwapParams},
    },
    ContractError,
};

pub fn execute_any_deposit(
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

    let (token0, token1) = must_pay_one_or_two(
        &info,
        (pool_config.token0.clone(), pool_config.token1.clone()),
    )?;

    // get the amount of funds we can deposit from this ratio
    let (deposit_amount_in_ratio, swappable_amount): ((Uint128, Uint128), (Uint128, Uint128)) =
        get_depositable_tokens(deps.branch(), token0.clone(), token1.clone())?;

    if swappable_amount.0.is_zero() && swappable_amount.1.is_zero() {
        let (mint_msg, user_shares) =
            mint_msg_user_shares(deps, &env, &deposit_amount_in_ratio, &recipient)?;

        return Ok(Response::new()
            .add_attribute("method", "execute")
            .add_attribute("action", "any_deposit")
            .add_attribute("amount0", deposit_amount_in_ratio.0)
            .add_attribute("amount1", deposit_amount_in_ratio.1)
            .add_message(mint_msg)
            .add_attribute("mint_shares_amount", user_shares)
            .add_attribute("receiver", recipient.as_str()));
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

    let pool_config = POOL_CONFIG.load(deps.storage)?;

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

    // Create the tuple for minting coins
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

    let (mint_msg, user_shares) = mint_msg_user_shares(
        deps.branch(),
        &env,
        &(coins_to_mint_for.0.amount, coins_to_mint_for.1.amount),
        &recipient,
    )?;

    CURRENT_SWAP_ANY_DEPOSIT.remove(deps.storage);

    Ok(Response::new()
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_any_deposit_swap")
        .add_attribute("amount0", balance0)
        .add_attribute("amount1", balance1)
        .add_message(mint_msg)
        .add_attribute("mint_shares_amount", user_shares)
        .add_attribute("receiver", recipient.as_str()))
}

fn mint_msg_user_shares(
    deps: DepsMut,
    env: &Env,
    deposit_amount_in_ratio: &(Uint128, Uint128),
    recipient: &Addr,
) -> Result<(MsgMint, Uint128), ContractError> {
    // calculate the amount of shares we can mint for this
    let total_assets = query_total_assets(deps.as_ref(), env.clone())?;
    let total_assets_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        total_assets.token0.amount,
        total_assets.token1.amount,
    )?;

    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total_vault_shares: Uint256 = query_total_vault_token_supply(deps.as_ref())?.total.into();

    let user_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        deposit_amount_in_ratio.0,
        deposit_amount_in_ratio.1,
    )?;

    // total_vault_shares.is_zero() should never be zero. This should ideally always enter the else and we are just sanity checking.
    let user_shares: Uint128 = if total_vault_shares.is_zero() {
        user_value
    } else {
        total_vault_shares
            .checked_mul(user_value.into())?
            .checked_div(total_assets_value.into())?
            .try_into()?
    };

    // save the shares in the user map
    SHARES.update(
        deps.storage,
        recipient.clone(),
        |old| -> Result<Uint128, ContractError> {
            if let Some(existing_user_shares) = old {
                Ok(user_shares + existing_user_shares)
            } else {
                Ok(user_shares)
            }
        },
    )?;

    // TODO the locking of minted shares is a band-aid for giving out rewards to users,
    // once tokenfactory has send hooks, we can remove the lockup and have the users
    // own the shares in their balance
    // we mint shares to the contract address here, so we can lock those shares for the user later in the same call
    // this is blocked by Osmosis v17 update
    let mint_msg = MsgMint {
        sender: env.clone().contract.address.to_string(),
        amount: Some(coin(user_shares.into(), vault_denom).into()),
        mint_to_address: env.clone().contract.address.to_string(),
    };

    Ok((mint_msg, user_shares))
}

#[allow(clippy::too_many_arguments)]
fn calculate_swap_amount(
    deps: DepsMut,
    env: &Env,
    pool_config: PoolConfig,
    swap_direction: SwapDirection,
    token_in_amount: Uint128,
    max_slippage: Decimal,
) -> Result<SwapCalculationResult, ContractError> {
    // TODO check that this math is right with spot price (numerators, denominators) if taken by legacy gamm module instead of poolmanager
    // TODO check on the twap_window_seconds (taking hardcoded value for now)
    let twap_price = get_twap_price(deps.storage, &deps.querier, env, 24u64)?;
    let (token_in_denom, token_out_denom, token_out_ideal_amount) = match swap_direction {
        SwapDirection::ZeroToOne => (
            &pool_config.token0,
            &pool_config.token1,
            token_in_amount
                .checked_multiply_ratio(twap_price.numerator(), twap_price.denominator()),
        ),
        SwapDirection::OneToZero => (
            &pool_config.token1,
            &pool_config.token0,
            token_in_amount
                .checked_multiply_ratio(twap_price.denominator(), twap_price.numerator()),
        ),
    };

    let token_out_min_amount = token_out_ideal_amount?
        .checked_multiply_ratio(max_slippage.numerator(), max_slippage.denominator())?;

    if !pool_config.pool_contains_token(token_in_denom) {
        return Err(ContractError::BadTokenForSwap {
            base_token: pool_config.token0,
            quote_token: pool_config.token1,
        });
    }

    // generate a swap message with recommended path as the current
    // pool on which the vault is running
    let swap_msg = swap_msg(
        &deps,
        env,
        SwapParams {
            pool_id: pool_config.pool_id,
            token_in_amount,
            token_out_min_amount,
            token_in_denom: token_in_denom.clone(),
            token_out_denom: token_out_denom.clone(),
            forced_swap_route: None, // TODO: check this None
        },
    )?;

    Ok(SwapCalculationResult {
        swap_msg,
        token_in_denom: token_in_denom.to_string(),
        token_out_min_amount,
        token_in_amount,
    })
}
