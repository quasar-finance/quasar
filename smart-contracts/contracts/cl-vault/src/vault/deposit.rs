use cosmwasm_std::{
    attr, coin, Addr, Attribute, BankMsg, Coin, Decimal, DepsMut, Env, Fraction, MessageInfo,
    QuerierWrapper, Response, Storage, Uint128, Uint256,
};

use osmosis_std::{
    try_proto_to_cosmwasm_coins,
    types::{
        cosmos::bank::v1beta1::BankQuerier,
        osmosis::{poolmanager::v1beta1::PoolmanagerQuerier, tokenfactory::v1beta1::MsgMint},
    },
};

use crate::{
    debug, helpers::must_pay_one_or_two, query::query_total_assets, state::{POOL_CONFIG, SHARES, VAULT_DENOM}, vault::concentrated_liquidity::get_position, ContractError
};

// execute_any_deposit is a nice to have feature for the cl vault.
// but left out of the current release.
pub(crate) fn _execute_any_deposit(
    _deps: DepsMut,
    _env: Env,
    _info: &MessageInfo,
    _amount: Uint128,
    _recipient: Option<String>,
) -> Result<Response, ContractError> {
    // Unwrap recipient or use caller's address
    unimplemented!()
}

/// Try to deposit as much user funds as we can into the a position and
/// refund the rest to the caller
pub(crate) fn execute_exact_deposit(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    // Unwrap recipient or use caller's address
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let pool = POOL_CONFIG.load(deps.storage)?;
    let (token0, token1) = must_pay_one_or_two(&info, (pool.token0.clone(), pool.token1.clone()))?;

    // get the amount of funds we can deposit from this ratio
    let (deposit, refund): ((Uint128, Uint128), (Uint128, Uint128)) =
        get_depositable_tokens(deps.branch(), token0.clone(), token1.clone())?;

    println!("deposit: {:?}", deposit);

    // ----- debug

    let pm_querier = PoolmanagerQuerier::new(&deps.querier);
    let spot_price: Decimal = pm_querier
        .spot_price(pool.pool_id, pool.clone().token0, pool.clone().token1)?
        .spot_price
        .parse()?;
    debug!(deps, "spot price", spot_price);
    
    // -----

    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total_vault_shares: Uint256 = BankQuerier::new(&deps.querier)
        .supply_of(vault_denom.clone())?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();

    let user_value = get_asset0_value(deps.storage, &deps.querier, deposit.0, deposit.1)?;

        // calculate the amount of shares we can mint for this
        let total_assets = query_total_assets(deps.as_ref(), env.clone())?;
        let total_assets_value = get_asset0_value(
            deps.storage,
            &deps.querier,
            total_assets.token0.amount,
            total_assets.token1.amount,
        )?.checked_sub(user_value)?;

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

    let mut resp = Response::new()
        .add_attribute("method", "exact_deposit")
        .add_attribute("action", "exact_deposit")
        .add_attribute("amount0", token0.amount)
        .add_attribute("amount1", token1.amount)
        .add_message(mint_msg)
        .add_attributes(mint_attrs);

    if let Some((bank_msg, bank_attr)) = refund_bank_msg(
        recipient,
        Some(coin(refund.0.u128(), pool.token0)),
        Some(coin(refund.1.u128(), pool.token1)),
    )? {
        resp = resp.add_message(bank_msg).add_attributes(bank_attr);
    }

    Ok(resp)
}

/// Calculate the total value of two assets in asset0
fn get_asset0_value(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    token0: Uint128,
    token1: Uint128,
) -> Result<Uint128, ContractError> {
    let pool_config = POOL_CONFIG.load(storage)?;

    let pm_querier = PoolmanagerQuerier::new(querier);
    let spot_price: Decimal = pm_querier
        .spot_price(pool_config.pool_id, pool_config.token0, pool_config.token1)?
        .spot_price
        .parse()?;

    let total = token0
        .checked_add(token1.multiply_ratio(spot_price.denominator(), spot_price.numerator()))?;

    Ok(total)
}

fn get_depositable_tokens(
    deps: DepsMut,
    token0: Coin,
    token1: Coin,
) -> Result<((Uint128, Uint128), (Uint128, Uint128)), ContractError> {
    let position = get_position(deps.storage, &deps.querier)?;
    debug!(deps, "position", position);


    match (position.asset0, position.asset1) {
        (None, _) => Ok((
            (Uint128::zero(), token1.amount),
            (token0.amount, Uint128::zero()),
        )),
        (_, None) => Ok((
            (token0.amount, Uint128::zero()),
            (Uint128::zero(), token1.amount),
        )),
        /*
           Figure out how many of the tokens we can use for a double sided position.
           First we find whether token0 or token0 is the limiting factor for the token by
           dividing token0 by the current amount of token0 in the position and do the same for token1
           for calculating further amounts we start from:
           token0 / token1 = ratio0 / ratio1, where ratio0 / ratio1 are the ratios from the position

           if token0 is limiting, we calculate the token1 amount by
           token1 = token0*ratio1/ratio0

           if token1 is limiting, we calculate the token0 amount by
           token0 = token1 * ratio0 / ratio1
        */
        (Some(asset0), Some(asset1)) => {
            let token0 = token0.amount;
            let token1 = token1.amount;
            let assets = try_proto_to_cosmwasm_coins(vec![asset0, asset1])?;
            let ratio = Decimal::from_ratio(assets[0].amount, assets[1].amount);
            println!("{:?}", ratio);

            // TODO make sure that this works correctly, also
            let zero_usage: Uint128 = ((Uint256::from(token0)
                * Uint256::from_u128(1_000_000_000_000_000_000u128))
                / Uint256::from(ratio.numerator()))
            .try_into()?;
            let one_usage: Uint128 = ((Uint256::from(token1)
                * Uint256::from_u128(1_000_000_000_000_000_000u128))
                / Uint256::from(ratio.denominator()))
            .try_into()?;

            if zero_usage < one_usage {
                let t1: Uint128 = (Uint256::from(token0) * (Uint256::from(ratio.denominator()))
                    / Uint256::from(ratio.numerator()))
                .try_into()?;
                Ok(((token0, t1), (Uint128::zero(), token1.checked_sub(t1)?)))
            } else {
                let t0: Uint128 = ((Uint256::from(token1) * Uint256::from(ratio.numerator()))
                    / Uint256::from(ratio.denominator()))
                .try_into()?;
                Ok(((t0, token1), (token0.checked_sub(t0)?, Uint128::zero())))
            }
        }
    }
}

fn refund_bank_msg(
    receiver: Addr,
    refund0: Option<Coin>,
    refund1: Option<Coin>,
) -> Result<Option<(BankMsg, Vec<Attribute>)>, ContractError> {
    let mut attributes: Vec<Attribute> = vec![];
    let mut coins: Vec<Coin> = vec![];

    if let Some(refund0) = refund0 {
        if refund0.amount > Uint128::zero() {
            attributes.push(attr("refund0_amount", refund0.amount));
            attributes.push(attr("refund0_denom", refund0.denom.as_str()));
            coins.push(refund0)
        }
    }
    if let Some(refund1) = refund1 {
        if refund1.amount > Uint128::zero() {
            attributes.push(attr("refund1_amount", refund1.amount));
            attributes.push(attr("refund1_denom", refund1.denom.as_str()));
            coins.push(refund1)
        }
    }
    let result: Option<(BankMsg, Vec<Attribute>)> = if !coins.is_empty() {
        Some((
            BankMsg::Send {
                to_address: receiver.to_string(),
                amount: coins,
            },
            attributes,
        ))
    } else {
        None
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::{marker::PhantomData, str::FromStr};

    use cosmwasm_std::{
        testing::{mock_env, MockApi, MockStorage, MOCK_CONTRACT_ADDR},
        Addr, Decimal256, Empty, OwnedDeps, Uint256,
    };

    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::concentratedliquidity::v1beta1::{
            FullPositionBreakdown, Position as OsmoPosition,
        },
    };

    use crate::{
        rewards::CoinList,
        state::{PoolConfig, Position, POSITION, STRATEGIST_REWARDS},
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
            (Uint128::new(1_000_000_000u128), Uint128::new(100_000_000_000_000_000_000_000_000_000u128)),
            (Uint128::zero(), Uint128::zero())
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

        let mut deps = mock_deps_with_position(None, Some(token1.clone()));
        let mutdeps = deps.as_mut();

        let result = get_depositable_tokens(mutdeps, token0, token1).unwrap();
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

        let mut deps = mock_deps_with_position(Some(token0.clone()), None);
        let mutdeps = deps.as_mut();

        let result = get_depositable_tokens(mutdeps, token0, token1).unwrap();
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
        let mutdeps = deps.as_mut();

        let result =
            get_depositable_tokens(mutdeps, coin(2000, "token0"), coin(5000, "token1")).unwrap();
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

        STRATEGIST_REWARDS
            .save(deps.as_mut().storage, &CoinList::new())
            .unwrap();
        POOL_CONFIG
            .save(
                deps.as_mut().storage,
                &PoolConfig {
                    pool_id: 1,
                    token0: "token0".to_string(),
                    token1: "token1".to_string(),
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
