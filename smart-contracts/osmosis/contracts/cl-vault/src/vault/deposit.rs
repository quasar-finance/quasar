use cosmwasm_std::{
    attr, coin, Addr, Coin, Decimal, DepsMut, Env, Fraction, MessageInfo, Response, SubMsg,
    SubMsgResult, Uint128, Uint256,
};

use osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountInResponse;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMint;

use crate::helpers::assert::must_pay_one_or_two;
use crate::helpers::getters::{
    get_asset0_value, get_depositable_tokens, get_swap_amount_and_direction, get_twap_price,
};
use crate::helpers::msgs::{refund_bank_msg, swap_msg};
use crate::msg::DepositType;
use crate::query::query_total_vault_token_supply;
use crate::reply::Replies;
use crate::state::{PoolConfig, CURRENT_SWAP_ANY_DEPOSIT};
use crate::{
    query::query_total_assets,
    state::{POOL_CONFIG, SHARES, VAULT_DENOM},
    ContractError,
};

use super::concentrated_liquidity::{get_cl_pool_info, get_position};
use super::range::SwapDirection;
use super::swap::{SwapCalculationResult, SwapParams};

pub fn execute_deposit(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
    deposit_type: DepositType,
) -> Result<Response, ContractError> {
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let (token0, token1) = must_pay_one_or_two(
        &info,
        (pool_config.token0.clone(), pool_config.token1.clone()),
    )?;

    let (deposit, refund): ((Uint128, Uint128), (Uint128, Uint128)) =
        get_depositable_tokens(deps.branch(), token0.clone(), token1.clone())?;

    // NOTE: both the deposit arms here have some repeated math logic,
    // in case of any math change please consider both arms
    match deposit_type {
        DepositType::Exact => {
            handle_exact_deposit(deps, env, recipient, pool_config, deposit, refund)
        }
        DepositType::Any { max_slippage } => handle_any_deposit(
            deps,
            env,
            recipient,
            pool_config,
            deposit,
            refund,
            max_slippage,
        ),
    }
}

fn handle_exact_deposit(
    deps: DepsMut,
    env: Env,
    recipient: Addr,
    pool_config: PoolConfig,
    deposit: (Uint128, Uint128),
    refund: (Uint128, Uint128),
) -> Result<Response, ContractError> {
    let total_assets = query_total_assets(deps.as_ref(), env.clone())?;
    let total_assets_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        total_assets.token0.amount,
        total_assets.token1.amount,
    )?;

    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total_vault_shares: Uint256 = query_total_vault_token_supply(deps.as_ref())?.total.into();

    let user_value = get_asset0_value(deps.storage, &deps.querier, deposit.0, deposit.1)?;
    let refund_value = get_asset0_value(deps.storage, &deps.querier, refund.0, refund.1)?;

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

    let mint_msg = MsgMint {
        sender: env.clone().contract.address.to_string(),
        amount: Some(coin(user_shares.into(), vault_denom).into()),
        mint_to_address: env.clone().contract.address.to_string(),
    };

    let mut resp = Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "exact_deposit")
        .add_attribute("amount0", deposit.0)
        .add_attribute("amount1", deposit.1)
        .add_message(mint_msg)
        .add_attribute("mint_shares_amount", user_shares)
        .add_attribute("receiver", recipient.as_str());

    if let Some((bank_msg, bank_attr)) = refund_bank_msg(
        recipient,
        Some(coin(refund.0.u128(), pool_config.token0)),
        Some(coin(refund.1.u128(), pool_config.token1)),
    )? {
        resp = resp.add_message(bank_msg).add_attributes(bank_attr);
    }

    Ok(resp)
}

fn handle_any_deposit(
    deps: DepsMut,
    env: Env,
    recipient: Addr,
    pool_config: PoolConfig,
    deposit: (Uint128, Uint128),
    refund: (Uint128, Uint128),
    max_slippage: Decimal,
) -> Result<Response, ContractError> {
    let position = get_position(deps.storage, &deps.querier)?
        .position
        .ok_or(ContractError::MissingPosition {})?;

    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;
    if !refund.0.is_zero() || !refund.1.is_zero() {
        let (swap_amount, swap_direction) = get_swap_amount_and_direction(
            refund.0,
            refund.1,
            pool_details.current_tick,
            position.lower_tick,
            position.upper_tick,
        )
        .unwrap();

        let swap_calc_result = calculate_swap_amount(
            deps,
            &env,
            pool_config,
            swap_direction,
            swap_amount,
            refund,
            deposit,
            max_slippage,
            &recipient,
        )?;

        return Ok(Response::new()
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
            ]));
    }

    let (mint_msg, user_shares) = mint_msg_user_shares(deps, &env, &deposit, &recipient)?;

    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "any_deposit")
        .add_attribute("amount0", deposit.0)
        .add_attribute("amount1", deposit.1)
        .add_message(mint_msg)
        .add_attribute("mint_shares_amount", user_shares)
        .add_attribute("receiver", recipient.as_str()))
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
    swappable_amount: (Uint128, Uint128),
    deposit_amount_in_ratio: (Uint128, Uint128),
    max_slippage: Decimal,
    recipient: &Addr,
) -> Result<SwapCalculationResult, ContractError> {
    let twap_price = get_twap_price(deps.storage, &deps.querier, env, 24u64)?;
    let (token_in_denom, token_out_denom, token_out_ideal_amount, left_over_amount) =
        match swap_direction {
            SwapDirection::ZeroToOne => (
                &pool_config.token0,
                &pool_config.token1,
                token_in_amount
                    .checked_multiply_ratio(twap_price.numerator(), twap_price.denominator()),
                swappable_amount.0.checked_sub(token_in_amount)?,
            ),
            SwapDirection::OneToZero => (
                &pool_config.token1,
                &pool_config.token0,
                token_in_amount
                    .checked_multiply_ratio(twap_price.denominator(), twap_price.numerator()),
                swappable_amount.1.checked_sub(token_in_amount)?,
            ),
        };

    CURRENT_SWAP_ANY_DEPOSIT.save(
        deps.storage,
        &(
            swap_direction,
            left_over_amount,
            recipient.clone(),
            deposit_amount_in_ratio,
        ),
    )?;

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
        position_id: None,
    })
}

#[cfg(test)]
mod tests {
    use std::{marker::PhantomData, str::FromStr};

    use cosmwasm_std::{
        testing::{mock_env, MockApi, MockStorage, MOCK_CONTRACT_ADDR},
        Addr, BankMsg, Coin, Decimal256, Empty, Fraction, OwnedDeps, Uint256,
    };

    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::concentratedliquidity::v1beta1::{
            FullPositionBreakdown, Position as OsmoPosition,
        },
    };

    use crate::{
        helpers::{getters::get_depositable_tokens, msgs::refund_bank_msg},
        state::{Position, POSITION},
        test_helpers::{mock_deps_with_querier, QuasarQuerier},
    };

    use super::*;

    #[test]
    fn test_position_in_both_asset() {
        let token0 = Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(1_000_000_000u128),
        };
        let token1 = Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100_000_000_000_000_000_000_000_000_000u128),
        };

        let mut deps = mock_deps_with_position(Some(token0.clone()), Some(token1.clone()));
        let mutdeps = deps.as_mut();

        let result = get_depositable_tokens(mutdeps, token0, token1).unwrap();
        assert_eq!(
            result,
            (
                (
                    Uint128::zero(),
                    Uint128::new(100_000_000_000_000_000_000_000_000_000u128)
                ),
                (Uint128::new(1_000_000_000u128), Uint128::zero())
            )
        );
    }

    #[test]
    fn test_position_in_asset1_only() {
        let token0 = Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(50),
        };
        let token1 = Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        };

        // Osmosis is not using None for missing assets, but Some with amount 0, so we need to mimic that here
        let mut deps = mock_deps_with_position(
            Some(Coin {
                denom: "token0".to_string(),
                amount: Uint128::zero(),
            }),
            Some(token1.clone()),
        );

        let result = get_depositable_tokens(deps.as_mut(), token0, token1).unwrap();
        assert_eq!(
            result,
            (
                (Uint128::zero(), Uint128::new(100)),
                (Uint128::new(50), Uint128::zero())
            )
        );
    }

    #[test]
    fn test_position_in_asset0_only() {
        let token0 = Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(50),
        };
        let token1 = Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        };

        // Osmosis is not using None for missing assets, but Some with amount 0, so we need to mimic that here
        let mut deps = mock_deps_with_position(
            Some(token0.clone()),
            Some(Coin {
                denom: "token1".to_string(),
                amount: Uint128::zero(),
            }),
        );

        let result = get_depositable_tokens(deps.as_mut(), token0, token1).unwrap();
        assert_eq!(
            result,
            (
                (Uint128::new(50), Uint128::zero()),
                (Uint128::zero(), Uint128::new(100))
            )
        );
    }

    #[test]
    fn test_both_assets_present_token0_limiting() {
        let token0 = Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(50),
        };
        let token1 = Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        };

        // we use a ratio of 1/2
        let mut deps = mock_deps_with_position(Some(token0.clone()), Some(token1.clone()));

        let result =
            get_depositable_tokens(deps.as_mut(), coin(2000, "token0"), coin(5000, "token1"))
                .unwrap();
        assert_eq!(
            result,
            (
                (Uint128::new(2000), Uint128::new(4000)),
                (Uint128::zero(), Uint128::new(1000))
            )
        );
    }

    #[test]
    fn test_both_assets_present_token1_limiting() {
        let token0 = Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(50),
        };
        let token1 = Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        };

        // we use a ratio of 1/2
        let mut deps = mock_deps_with_position(Some(token0.clone()), Some(token1.clone()));
        let mutdeps = deps.as_mut();

        let result =
            get_depositable_tokens(mutdeps, coin(2000, "token0"), coin(3000, "token1")).unwrap();
        assert_eq!(
            result,
            (
                (Uint128::new(1500), Uint128::new(3000)),
                (Uint128::new(500), Uint128::zero())
            )
        );
    }

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

        execute_deposit(
            deps.as_mut(),
            env,
            MessageInfo {
                sender: sender.clone(),
                funds: vec![coin(100, "token0"), coin(100, "token1")],
            },
            None,
            DepositType::Exact,
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

    fn mock_deps_with_position(
        token0: Option<Coin>,
        token1: Option<Coin>,
    ) -> OwnedDeps<MockStorage, MockApi, QuasarQuerier, Empty> {
        let position_id = 2;

        let mut deps = OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: QuasarQuerier::new(
                FullPositionBreakdown {
                    position: Some(OsmoPosition {
                        position_id,
                        address: MOCK_CONTRACT_ADDR.to_string(),
                        pool_id: 1,
                        lower_tick: 100,
                        upper_tick: 1000,
                        join_time: None,
                        liquidity: "1000000.2".to_string(),
                    }),
                    asset0: token0.map(|c| c.into()),
                    asset1: token1.map(|c| c.into()),
                    claimable_spread_rewards: vec![
                        OsmoCoin {
                            denom: "token0".to_string(),
                            amount: "100".to_string(),
                        },
                        OsmoCoin {
                            denom: "token1".to_string(),
                            amount: "100".to_string(),
                        },
                    ],
                    claimable_incentives: vec![],
                    forfeited_incentives: vec![],
                },
                500,
            ),
            custom_query_type: PhantomData,
        };
        POSITION
            .save(
                deps.as_mut().storage,
                &Position {
                    position_id,
                    join_time: 0,
                    claim_after: None,
                },
            )
            .unwrap();
        deps
    }
}
