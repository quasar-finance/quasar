use apollo_cw_asset::AssetInfoBase;
use cosmwasm_std::{
    attr, coin, Coin, DepsMut, Env, Fraction, MessageInfo, Response, SubMsg, SubMsgResult, Uint128,
    Uint256,
};
use cw_dex::osmosis::OsmosisPool;
use cw_dex_router::operations::{SwapOperationBase, SwapOperationsListUnchecked};

use osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountInResponse;
use osmosis_std::types::{
    cosmos::bank::v1beta1::BankQuerier, osmosis::tokenfactory::v1beta1::MsgMint,
};

use crate::helpers::{
    get_single_sided_deposit_0_to_1_swap_amount, get_single_sided_deposit_1_to_0_swap_amount,
    get_twap_price,
};
use crate::reply::Replies;
use crate::state::CURRENT_SWAP_ANY_DEPOSIT;
use crate::vault::concentrated_liquidity::get_cl_pool_info;
use crate::vault::exact_deposit::{get_asset0_value, get_depositable_tokens};
use crate::vault::range::SwapDirection;
use crate::vault::swap::{swap, SwapParams};
use crate::{
    helpers::must_pay_one_or_two,
    query::query_total_assets,
    state::{POOL_CONFIG, SHARES, VAULT_DENOM},
    vault::concentrated_liquidity::get_position,
    ContractError,
};

// execute_any_deposit is a nice to have feature for the cl vault.
// but left out of the current release.
pub(crate) fn execute_any_deposit(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    // Unwrap recipient or use caller's address
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let pool = POOL_CONFIG.load(deps.storage)?;
    let pool_details = get_cl_pool_info(&deps.querier, pool.pool_id)?;
    let position_breakdown = get_position(deps.storage, &deps.querier)?;
    let position = position_breakdown.position.clone().unwrap();
    let (token0, token1) = must_pay_one_or_two(&info, (pool.token0.clone(), pool.token1.clone()))?;

    // get the amount of funds we can deposit from this ratio
    let (deposit_amount_in_ratio, swappable_amount): ((Uint128, Uint128), (Uint128, Uint128)) =
        get_depositable_tokens(deps.branch(), token0.clone(), token1.clone())?;

    // write swap logic here
    let (swap_amount, swap_direction) = if !swappable_amount.0.is_zero() {
        (
            // range is above current tick
            if pool_details.current_tick > position.upper_tick {
                swappable_amount.0
            } else {
                get_single_sided_deposit_0_to_1_swap_amount(
                    swappable_amount.0,
                    position.lower_tick,
                    pool_details.current_tick,
                    position.upper_tick,
                )?
            },
            SwapDirection::ZeroToOne,
        )
    } else if !swappable_amount.1.is_zero() {
        (
            // current tick is above range
            if pool_details.current_tick < position.lower_tick {
                swappable_amount.1
            } else {
                get_single_sided_deposit_1_to_0_swap_amount(
                    swappable_amount.1,
                    position.lower_tick,
                    pool_details.current_tick,
                    position.upper_tick,
                )?
            },
            SwapDirection::OneToZero,
        )
    } else {
        // calculate the amount of shares we can mint for this
        let total_assets = query_total_assets(deps.as_ref(), env.clone())?;
        let total_assets_value = get_asset0_value(
            deps.storage,
            &deps.querier,
            total_assets.token0.amount,
            total_assets.token1.amount,
        )?;

        let vault_denom = VAULT_DENOM.load(deps.storage)?;
        let total_vault_shares: Uint256 = BankQuerier::new(&deps.querier)
            .supply_of(vault_denom.clone())?
            .amount
            .unwrap()
            .amount
            .parse::<u128>()?
            .into();

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

        let mint_attrs = vec![
            attr("mint_shares_amount", user_shares),
            attr("receiver", recipient.as_str()),
        ];

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

        return Ok(Response::new()
            .add_attribute("method", "reply")
            .add_attribute("action", "any_deposit")
            .add_attribute("amount0", deposit_amount_in_ratio.0)
            .add_attribute("amount1", deposit_amount_in_ratio.1)
            .add_message(mint_msg)
            .add_attributes(mint_attrs));
    };

    // todo check that this math is right with spot price (numerators, denominators) if taken by legacy gamm module instead of poolmanager
    // todo check on the twap_window_seconds (taking hardcoded value for now)
    let twap_price = get_twap_price(deps.storage, &deps.querier, &env, 24u64)?;
    let (token_in_denom, token_out_denom, token_out_ideal_amount, left_over_amount) =
        match swap_direction {
            SwapDirection::ZeroToOne => (
                pool.token0,
                pool.token1,
                swap_amount
                    .checked_multiply_ratio(twap_price.numerator(), twap_price.denominator()),
                swappable_amount.0.checked_sub(swap_amount)?,
            ),
            SwapDirection::OneToZero => (
                pool.token1,
                pool.token0,
                swap_amount
                    .checked_multiply_ratio(twap_price.denominator(), twap_price.numerator()),
                swappable_amount.1.checked_sub(swap_amount)?,
            ),
        };

    CURRENT_SWAP_ANY_DEPOSIT.save(
        deps.storage,
        &(
            swap_direction,
            left_over_amount,
            recipient,
            deposit_amount_in_ratio,
        ),
    )?;

    // todo change this later as it hardcoded as of now
    let token_out_min_amount =
        token_out_ideal_amount?.checked_multiply_ratio(Uint128::new(197), Uint128::new(200))?;

    // generate a swap message with recommended path as the current
    // pool on which the vault is running
    let swap_msg = swap(
        deps,
        &env,
        SwapParams {
            token_in_amount: swap_amount,
            token_out_min_amount,
            token_in_denom: token_in_denom.clone(),
            token_out_denom: token_out_denom.clone(),
            recommended_swap_route: Option::from(SwapOperationsListUnchecked::new(vec![
                SwapOperationBase {
                    pool: cw_dex::Pool::Osmosis(OsmosisPool::unchecked(pool.pool_id)),
                    offer_asset_info: AssetInfoBase::Native(token_in_denom.clone()),
                    ask_asset_info: AssetInfoBase::Native(token_out_denom.clone()),
                },
            ])),
            force_swap_route: false,
        },
    )?;

    // rest minting logic remains same
    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            swap_msg,
            Replies::AnyDepositSwap.into(),
        ))
        .add_attribute("method", "reply")
        .add_attribute("action", "any_deposit_swap")
        .add_attribute("token_in", format!("{}{}", swap_amount, token_in_denom))
        .add_attribute("token_out_min", format!("{}", token_out_min_amount)))
}

pub(crate) fn handle_any_deposit_swap_reply(
    deps: DepsMut,
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
    // Create the position after swapped the leftovers based on swap direction
    let mut coins_to_mint_for = vec![];
    if !balance0.is_zero() || !deposit_amount_in_ratio.0.is_zero() {
        coins_to_mint_for.push(Coin {
            denom: pool_config.token0.clone(),
            amount: balance0 + deposit_amount_in_ratio.0,
        });
    }
    if !balance1.is_zero() || !deposit_amount_in_ratio.1.is_zero() {
        coins_to_mint_for.push(Coin {
            denom: pool_config.token1.clone(),
            amount: balance1 + deposit_amount_in_ratio.1,
        });
    }

    // calculate the amount of shares we can mint for this
    let total_assets = query_total_assets(deps.as_ref(), env.clone())?;
    let total_assets_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        total_assets.token0.amount,
        total_assets.token1.amount,
    )?;

    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total_vault_shares: Uint256 = BankQuerier::new(&deps.querier)
        .supply_of(vault_denom.clone())?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();

    let user_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        coins_to_mint_for[0].amount,
        coins_to_mint_for[1].amount,
    )?;

    // total_vault_shares.is_zero() should never be zero. This should ideally always enter the else and we are just sanity checking.
    let user_shares: Uint128 = if total_vault_shares.is_zero() {
        user_value
    } else {
        total_vault_shares
            .checked_mul(user_value.into())?
            .checked_div(total_assets_value.checked_sub(user_value)?.into())?
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

    let mint_attrs = vec![
        attr("mint_shares_amount", user_shares),
        attr("receiver", recipient.as_str()),
    ];

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

    CURRENT_SWAP_ANY_DEPOSIT.remove(deps.storage);

    Ok(Response::new()
        .add_attribute("method", "reply")
        .add_attribute("action", "any_deposit_swap_reply")
        .add_attribute("amount0", balance0)
        .add_attribute("amount1", balance1)
        .add_message(mint_msg)
        .add_attributes(mint_attrs))
}
