use crate::{
    helpers::{
        getters::{
            get_asset0_value, get_depositable_tokens, get_single_sided_deposit_0_to_1_swap_amount,
            get_single_sided_deposit_1_to_0_swap_amount, get_twap_price,
        },
        msgs::refund_bank_msg,
    },
    query::{query_total_assets, query_total_vault_token_supply},
    reply::Replies,
    state::{CURRENT_SWAP_ANY_DEPOSIT, DEX_ROUTER, POOL_CONFIG, SHARES, VAULT_DENOM},
    vault::{
        concentrated_liquidity::{get_cl_pool_info, get_position},
        swap::{calculate_swap_amount, SwapDirection},
    },
    ContractError,
};
use cosmwasm_std::{
    attr, coin, Addr, Coin, Decimal, DepsMut, Env, MessageInfo, Response, SubMsg, SubMsgResult,
    Uint128, Uint256,
};
use osmosis_std::types::osmosis::{
    poolmanager::v1beta1::MsgSwapExactAmountInResponse, tokenfactory::v1beta1::MsgMint,
};

pub(crate) fn execute_exact_deposit(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    // get the amount of funds we can deposit from this ratio
    let deposit_info = get_depositable_tokens(&deps, &info.funds, &pool_config)?;

    execute_deposit(
        &mut deps,
        env,
        recipient,
        (deposit_info.base_deposit, deposit_info.quote_deposit),
        (
            coin(deposit_info.base_refund.into(), pool_config.token0),
            coin(deposit_info.quote_refund.into(), pool_config.token1),
        ),
    )
}

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
    // let (deposit_amount_in_ratio, swappable_amount): ((Uint128, Uint128), (Uint128, Uint128)) =
    let deposit_info = get_depositable_tokens(&deps.branch(), &info.funds, &pool_config)?;

    if deposit_info.base_refund.is_zero() && deposit_info.quote_refund.is_zero() {
        return execute_deposit(
            &mut deps,
            env,
            recipient,
            (deposit_info.base_deposit, deposit_info.quote_deposit),
            (
                coin(0u128, pool_config.token0),
                coin(0u128, pool_config.token1),
            ),
        );
    }

    // Swap logic
    // TODO_FUTURE: Optimize this if conditions
    let (swap_amount, swap_direction, left_over_amount) = if !deposit_info.base_refund.is_zero() {
        // range is above current tick
        let swap_amount = if pool_details.current_tick > position.upper_tick {
            deposit_info.base_refund
        } else {
            get_single_sided_deposit_0_to_1_swap_amount(
                deposit_info.base_refund,
                position.lower_tick,
                pool_details.current_tick,
                position.upper_tick,
            )?
        };
        let left_over_amount = deposit_info.base_refund.checked_sub(swap_amount)?;
        (swap_amount, SwapDirection::ZeroToOne, left_over_amount)
    } else {
        // current tick is above range
        let swap_amount = if pool_details.current_tick < position.lower_tick {
            deposit_info.quote_refund
        } else {
            get_single_sided_deposit_1_to_0_swap_amount(
                deposit_info.quote_refund,
                position.lower_tick,
                pool_details.current_tick,
                position.upper_tick,
            )?
        };
        let left_over_amount = deposit_info.quote_refund.checked_sub(swap_amount)?;
        (swap_amount, SwapDirection::OneToZero, left_over_amount)
    };
    CURRENT_SWAP_ANY_DEPOSIT.save(
        deps.storage,
        &(
            swap_direction.clone(),
            left_over_amount,
            recipient.clone(),
            (deposit_info.base_deposit, deposit_info.quote_deposit),
        ),
    )?;

    let dex_router = DEX_ROUTER.may_load(deps.storage)?;
    let twap_price = get_twap_price(deps.storage, &deps.querier, &env, 24u64)?;
    let swap_calc_result = calculate_swap_amount(
        env.contract.address,
        pool_config,
        swap_direction,
        swap_amount,
        max_slippage,
        None, // TODO: check this None
        twap_price,
        dex_router,
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
                format!("{}{}", swap_amount, swap_calc_result.offer.denom),
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

/// Try to deposit as much user funds as we can in the current ratio of the vault and
/// refund the rest to the caller.
fn execute_deposit(
    deps: &mut DepsMut,
    env: Env,
    recipient: Addr,
    deposit: (Uint128, Uint128),
    refund: (Coin, Coin),
) -> Result<Response, ContractError> {
    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total_vault_shares: Uint256 = query_total_vault_token_supply(deps.as_ref())?.total.into();

    let user_value = get_asset0_value(deps.storage, &deps.querier, deposit.0, deposit.1)?;
    let refund_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        refund.0.amount,
        refund.1.amount,
    )?;

    // calculate the amount of shares we can mint for this
    let total_assets = query_total_assets(deps.as_ref(), env.clone())?;
    let total_assets_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        total_assets.token0.amount,
        total_assets.token1.amount,
    )?;

    // total_vault_shares.is_zero() should never be zero. This should ideally always enter the else and we are just sanity checking.
    let user_shares: Uint128 = if total_vault_shares.is_zero() {
        user_value
    } else {
        total_vault_shares
            .checked_mul(user_value.into())?
            .checked_div(
                total_assets_value
                    .checked_sub(user_value)?
                    .checked_sub(refund_value)?
                    .into(),
            )?
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

    let mut resp = Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "deposit")
        .add_attribute("amount0", deposit.0)
        .add_attribute("amount1", deposit.1)
        .add_message(mint_msg)
        .add_attribute("mint_shares_amount", user_shares)
        .add_attribute("receiver", recipient.as_str());

    if let Some((bank_msg, bank_attr)) = refund_bank_msg(recipient, Some(refund.0), Some(refund.1))?
    {
        resp = resp.add_message(bank_msg).add_attributes(bank_attr);
    }

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{testing::mock_env, Addr, BankMsg, Decimal256, Fraction, Uint256};

    use crate::{
        helpers::msgs::refund_bank_msg,
        state::{Position, POSITION},
        test_helpers::mock_deps_with_querier,
    };

    use super::*;

    #[test]
    fn execute_exact_deposit_works() {
        let mut deps = mock_deps_with_querier(&MessageInfo {
            sender: Addr::unchecked("alice"),
            funds: vec![],
        });
        let env = mock_env();
        let sender = Addr::unchecked("alice");
        VAULT_DENOM
            .save(deps.as_mut().storage, &"money".to_string())
            .unwrap();
        POSITION
            .save(
                deps.as_mut().storage,
                &Position {
                    position_id: 1,
                    join_time: 0,
                    claim_after: None,
                },
            )
            .unwrap();

        execute_exact_deposit(
            deps.as_mut(),
            env,
            MessageInfo {
                sender: sender.clone(),
                funds: vec![coin(100, "token0"), coin(100, "token1")],
            },
            None,
        )
        .unwrap();

        // we currently have 100_000 total_vault_shares outstanding and the equivalent of 1999500token0, the user deposits the equivalent of 199token0, thus shares are
        // 199 * 100000 / 1999500 = 9.95, which we round down. Thus we expect 9 shares in this example
        assert_eq!(
            SHARES.load(deps.as_ref().storage, sender).unwrap(),
            Uint128::new(9)
        );
    }

    #[test]
    fn test_shares() {
        let total_shares = Uint256::from(1000000000_u128);
        let total_liquidity = Decimal256::from_str("1000000000").unwrap();
        let liquidity = Decimal256::from_str("5000000").unwrap();

        let _user_shares: Uint128 = if total_shares.is_zero() && total_liquidity.is_zero() {
            liquidity.to_uint_floor().try_into().unwrap()
        } else {
            let _ratio = liquidity.checked_div(total_liquidity).unwrap();
            total_shares
                .multiply_ratio(liquidity.numerator(), liquidity.denominator())
                .multiply_ratio(total_liquidity.denominator(), total_liquidity.numerator())
                .try_into()
                .unwrap()
        };
    }

    #[test]
    fn refund_bank_msg_2_coins() {
        let _env = mock_env();
        let user = Addr::unchecked("alice");

        let refund0 = coin(150, "uosmo");
        let refund1 = coin(250, "uatom");

        let response = refund_bank_msg(user.clone(), Some(refund0), Some(refund1)).unwrap();
        assert!(response.is_some());
        assert_eq!(
            response.unwrap().0,
            BankMsg::Send {
                to_address: user.to_string(),
                amount: vec![coin(150, "uosmo"), coin(250, "uatom")],
            }
        )
    }

    #[test]
    fn refund_bank_msg_token0() {
        let _env = mock_env();
        let user = Addr::unchecked("alice");

        let refund0 = coin(150, "uosmo");
        let refund1 = coin(0, "uatom");

        let response = refund_bank_msg(user.clone(), Some(refund0), Some(refund1)).unwrap();
        assert!(response.is_some());
        assert_eq!(
            response.unwrap().0,
            BankMsg::Send {
                to_address: user.to_string(),
                amount: vec![coin(150, "uosmo")],
            }
        )
    }

    #[test]
    fn refund_bank_msg_token1() {
        let _env = mock_env();
        let user = Addr::unchecked("alice");

        let refund0 = coin(0, "uosmo");
        let refund1 = coin(250, "uatom");

        let response = refund_bank_msg(user.clone(), Some(refund0), Some(refund1)).unwrap();
        assert!(response.is_some());
        assert_eq!(
            response.unwrap().0,
            BankMsg::Send {
                to_address: user.to_string(),
                amount: vec![coin(250, "uatom")],
            }
        )
    }
}
