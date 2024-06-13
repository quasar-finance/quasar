use cosmwasm_std::{
    coin, BankMsg, CosmosMsg, Decimal256, DepsMut, Env, MessageInfo, Response, SubMsg,
    SubMsgResult, Uint128, Uint256,
};
use osmosis_std::types::{
    cosmos::bank::v1beta1::BankQuerier,
    osmosis::{
        concentratedliquidity::v1beta1::{MsgWithdrawPosition, MsgWithdrawPositionResponse},
        tokenfactory::v1beta1::MsgBurn,
    },
};

use crate::query::query_total_vault_token_supply;
use crate::{
    helpers::{get_asset0_value, get_unused_balances, sort_tokens},
    query::query_total_assets,
    reply::Replies,
    state::{
        CURRENT_WITHDRAWER, CURRENT_WITHDRAWER_DUST, MAIN_POSITION, POOL_CONFIG, SHARES,
        VAULT_DENOM,
    },
    vault::concentrated_liquidity::withdraw_from_position,
    ContractError,
};

use super::concentrated_liquidity::{get_parsed_position, get_positions};

// any locked shares are sent in amount, due to a lack of tokenfactory hooks during development
// currently that functions as a bandaid
pub fn execute_withdraw(
    deps: DepsMut,
    env: &Env,
    info: MessageInfo,
    recipient: Option<String>,
    shares_to_withdraw: Uint256,
) -> Result<Response, ContractError> {
    assert!(
        shares_to_withdraw > Uint256::zero(),
        "amount to withdraw must be greater than zero"
    );

    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    // get the sent along shares
    // let shares = must_pay(&info, vault_denom.as_str())?;

    // get the amount from SHARES state
    let user_shares: Uint256 = SHARES.load(deps.storage, info.sender.clone())?.into();

    let left_over = user_shares
        .checked_sub(shares_to_withdraw)
        .map_err(|_| ContractError::InsufficientFunds)?;
    SHARES.save(deps.storage, info.sender, &left_over.try_into()?)?;

    let total_shares: Uint256 = query_total_vault_token_supply(deps.as_ref())?.total.into();

    // get the dust amounts belonging to the user
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    // TODO replace dust with queries for balance
    let unused_balances = get_unused_balances(&deps.querier, env)?;
    let dust0: Uint256 = unused_balances
        .find_coin(pool_config.token0.clone())
        .amount
        .into();
    let dust1: Uint256 = unused_balances.find_coin(pool_config.token1).amount.into();

    let user_dust0: Uint128 = dust0
        .checked_mul(shares_to_withdraw)?
        .checked_div(total_shares)?
        .try_into()?;
    let user_dust1 = dust1
        .checked_mul(shares_to_withdraw)?
        .checked_div(total_shares)?
        .try_into()?;
    // save the new total amount of dust available for other actions

    CURRENT_WITHDRAWER_DUST.save(deps.storage, &(user_dust0, user_dust1))?;

    let shares_to_withdraw_u128: Uint128 = shares_to_withdraw.try_into()?;
    // burn the shares
    let burn_coin = coin(shares_to_withdraw_u128.u128(), vault_denom);
    let burn_msg: CosmosMsg = MsgBurn {
        sender: env.contract.address.clone().into_string(),
        amount: Some(burn_coin.into()),
        burn_from_address: env.contract.address.clone().into_string(),
    }
    .into();

    CURRENT_WITHDRAWER.save(deps.storage, &recipient)?;

    let assets = query_total_assets(deps.as_ref(), env)?;
    let total_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        assets.token0.amount,
        assets.token1.amount,
    )?;
    let total_supply = query_total_vault_token_supply(deps.as_ref())?.total;

    let user_value = Decimal256::from_ratio(user_shares, 1_u128)
        .checked_mul(Decimal256::from_ratio(total_value, 1_u128))?
        .checked_div(Decimal256::from_ratio(total_supply, 1_u128))?;

    let main_position_id = MAIN_POSITION.load(deps.storage)?;
    let main_position = get_parsed_position(&deps.querier, main_position_id)?;
    let main_postion_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        main_position.asset0.amount,
        main_position.asset1.amount,
    )?;

    // withdraw the user's funds from the position
    // TODO this should have a seperate test to ensure proper value return if between a main position and multiple positions
    let withdraw_msg = if user_value.to_uint_ceil() < main_postion_value.into() {
        vec![withdraw_from_main(deps, env, user_shares.try_into()?)?]
    } else {
        withdraw_pro_rato(deps, env, user_shares.try_into()?)?
    };

    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "withdraw")
        .add_attribute("user_amount", user_value.to_string())
        .add_attribute("share_amount", shares_to_withdraw)
        .add_message(burn_msg)
        .add_submessages(
            withdraw_msg
                .into_iter()
                .map(|m| SubMsg::reply_on_success(m, Replies::WithdrawUser as u64)),
        ))
}

pub fn handle_withdraw_user_reply(
    deps: DepsMut,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    // parse the reply and instantiate the funds we want to send
    let response: MsgWithdrawPositionResponse = data.try_into()?;
    let user = CURRENT_WITHDRAWER.load(deps.storage)?;
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    let (user_dust0, user_dust1) = CURRENT_WITHDRAWER_DUST.load(deps.storage)?;
    let amount0 = Uint128::new(response.amount0.parse()?).checked_add(user_dust0)?;
    let amount1 = Uint128::new(response.amount1.parse()?).checked_add(user_dust1)?;

    let coin0 = coin(amount0.u128(), pool_config.token0);
    let coin1 = coin(amount1.u128(), pool_config.token1);

    // send the funds to the user
    let msg = BankMsg::Send {
        to_address: user.to_string(),
        amount: sort_tokens(vec![coin0.clone(), coin1.clone()]),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_withdraw_user")
        .add_attribute("amount0", coin0.clone().amount)
        .add_attribute("amount1", coin1.clone().amount))
}

fn withdraw_from_main(
    deps: DepsMut,
    env: &Env,
    user_shares: Uint128,
) -> Result<MsgWithdrawPosition, ContractError> {
    let assets = query_total_assets(deps.as_ref(), &env)?;
    let total_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        assets.token0.amount,
        assets.token1.amount,
    )?;
    let total_supply = query_total_vault_token_supply(deps.as_ref())?.total;

    let user_value = Decimal256::from_ratio(user_shares, 1_u128)
        .checked_mul(Decimal256::from_ratio(total_value, 1_u128))?
        .checked_div(Decimal256::from_ratio(total_supply, 1_u128))?;

    let main_position_id = MAIN_POSITION.load(deps.storage)?;
    let main_position = get_parsed_position(&deps.querier, main_position_id)?;
    let main_postion_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        main_position.asset0.amount,
        main_position.asset1.amount,
    )?;

    // User value * Main position liquidity / Main position value = user value
    let withdraw_liquidity = (user_value * main_position.position.liquidity)
        / Decimal256::from_ratio(main_postion_value, 1_u128);

    withdraw_from_position(&env, main_position_id, withdraw_liquidity)
}

fn withdraw_pro_rato(
    deps: DepsMut,
    env: &Env,
    user_shares: Uint128,
) -> Result<Vec<MsgWithdrawPosition>, ContractError> {
    let bq = BankQuerier::new(&deps.querier);
    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    let total_vault_shares: Uint128 = bq
        .supply_of(vault_denom)?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();

    let positions = get_positions(deps.storage, &deps.querier);

    let withdraws: Result<Vec<MsgWithdrawPosition>, ContractError> = positions?
        .into_iter()
        .map(|(p, fp)| {
            let existing_liquidity: Decimal256 = fp.position.liquidity;

            let user_liquidity = Decimal256::from_ratio(user_shares, 1_u128)
                .checked_mul(existing_liquidity)?
                .checked_div(Decimal256::from_ratio(total_vault_shares, 1_u128))?;

            withdraw_from_position(env, p.position_id, user_liquidity)
        })
        .collect();

    withdraws
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[allow(deprecated)]
    use crate::{
        // rewards::CoinList,
        rewards::CoinList,
        state::{PoolConfig, STRATEGIST_REWARDS},
        test_helpers::mock_deps_with_querier_with_balance,
    };
    use crate::{
        test_helpers::{mock_deps_with_querier_with_balance_with_positions, FullPositionBuilder},
        vault::concentrated_liquidity::{
            FullPositionParsed, FullPositionParsedBuilder, PositionParsed, PositionParsedBuilder,
        },
    };
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR},
        Addr, CosmosMsg, SubMsgResponse, Timestamp,
    };

    use super::*;

    #[test]
    fn execute_withdraw_works_no_rewards() {
        let info = mock_info("bolice", &[]);
        let mut deps = mock_deps_with_querier_with_balance(
            &info,
            &[(
                MOCK_CONTRACT_ADDR,
                &[coin(2000, "token0"), coin(3000, "token1")],
            )],
        );
        let env = mock_env();

        // TODO: We should remove this in the next patch or just adjust now accordingly as we depcreate this state
        #[allow(deprecated)]
        STRATEGIST_REWARDS
            .save(deps.as_mut().storage, &CoinList::new())
            .unwrap();
        VAULT_DENOM
            .save(deps.as_mut().storage, &"share_token".to_string())
            .unwrap();
        SHARES
            .save(
                deps.as_mut().storage,
                Addr::unchecked("bolice"),
                &Uint128::new(1000),
            )
            .unwrap();

        let _res =
            execute_withdraw(deps.as_mut(), &env, info, None, Uint128::new(1000).into()).unwrap();
        // our querier returns a total supply of 100_000, this user unbonds 1000, or 1%. The Dust saved should be one lower
        assert_eq!(
            CURRENT_WITHDRAWER_DUST.load(deps.as_ref().storage).unwrap(),
            (Uint128::new(20), Uint128::new(30))
        )
    }

    #[test]
    fn withdraw_from_main_works() {
        let info = mock_info("bolice", &[]);
        let main_position = FullPositionParsedBuilder::default()
            .position(
                PositionParsedBuilder::default()
                    .address(MOCK_CONTRACT_ADDR.to_string())
                    .position_id(1)
                    .pool_id(1)
                    .lower_tick(-10000)
                    .upper_tick(1000)
                    .liquidity(Decimal256::from_str("1000.1").unwrap())
                    .join_time(Timestamp::from_seconds(1))
                    .build()
                    .unwrap(),
            )
            .asset0(coin(1000, "token0"))
            .asset1(coin(1000, "token1"))
            .claimable_spread_rewards((vec![coin(100, "token0"), coin(100, "token1")]))
            .claimable_incentives(vec![])
            .forfeited_incentives(vec![])
            .build()
            .unwrap();

        let secondary_positions = vec![
            FullPositionParsedBuilder::default()
            .position(
                PositionParsedBuilder::default()
                    .address(MOCK_CONTRACT_ADDR.to_string())
                    .position_id(2)
                    .pool_id(1)
                    .lower_tick(-1000)
                    .upper_tick(100)
                    .liquidity(Decimal256::from_str("100.1").unwrap())
                    .join_time(Timestamp::from_seconds(1))
                    .build()
                    .unwrap(),
            )
            .asset0(coin(100, "token0"))
            .asset1(coin(100, "token1"))
            .claimable_spread_rewards((vec![coin(100, "token0"), coin(100, "token1")]))
            .claimable_incentives(vec![])
            .forfeited_incentives(vec![])
            .build()
            .unwrap(),
            FullPositionParsedBuilder::default()
            .position(
                PositionParsedBuilder::default()
                    .address(MOCK_CONTRACT_ADDR.to_string())
                    .position_id(3)
                    .pool_id(1)
                    .lower_tick(-1000)
                    .upper_tick(2000)
                    .liquidity(Decimal256::from_str("10.1").unwrap())
                    .join_time(Timestamp::from_seconds(1))
                    .build()
                    .unwrap(),
            )
            .asset0(coin(10, "token0"))
            .asset1(coin(20, "token1"))
            .claimable_spread_rewards((vec![coin(100, "token0"), coin(100, "token1")]))
            .claimable_incentives(vec![])
            .forfeited_incentives(vec![])
            .build()
            .unwrap(),
        ];

        // The QuasarQuerier hard mocks total shares to 100000
        let total_shares = 100000;
        let user_shares  = 1000;

        let mut deps = mock_deps_with_querier_with_balance_with_positions(
            &info,
            &[(
                MOCK_CONTRACT_ADDR,
                &[
                    coin(2000, "token0"),
                    coin(3000, "token1"),
                    coin(total_shares, "shares"),
                ],
            )],
            main_position.clone().into(),
            secondary_positions.into_iter().map(|p| p.into()).collect(),
        );
        VAULT_DENOM
            .save(deps.as_mut().storage, &"shares".to_string())
            .unwrap();

        let env = mock_env();
        let total_value = query_total_assets(deps.as_ref(), &env).unwrap();

        let total_asset0_value = get_asset0_value(
            deps.as_ref().storage,
            &deps.as_ref().querier,
            total_value.token0.amount,
            total_value.token1.amount,
        )
        .unwrap();

        // our users asset0 value
        let user_value = user_shares * total_asset0_value.u128() / total_shares;

        let main_position_asset0_value = get_asset0_value(
            deps.as_ref().storage,
            &deps.as_ref().querier,
            main_position.asset0.amount,
            main_position.asset1.amount,
        )
        .unwrap();

        // expected liquidity is the % equivalent of asset_0 value we are withdrawing compared to asset0 value of the main position
        let liquidity_ratio = Decimal256::from_ratio(user_value, main_position_asset0_value);
        let expected_liquidity = main_position.position.liquidity * liquidity_ratio;

        let res = withdraw_from_main(deps.as_mut(), &env, Uint128::new(user_shares)).unwrap();
        assert_eq!(Decimal256::from_atomics(Uint256::from_str(res.liquidity_amount.as_str()).unwrap(), 18).unwrap().to_uint_floor(), expected_liquidity.to_uint_floor())
    }

    // #[test]
    // fn execute_withdraw_works_user_rewards() {
    //     let info = mock_info("bolice", &[]);
    //     let mut deps = mock_deps_with_querier_with_balance(
    //         &info,
    //         &[(
    //             MOCK_CONTRACT_ADDR,
    //             &[coin(2000, "token0"), coin(3000, "token1")],
    //         )],
    //     );
    //     let env = mock_env();

    //     STRATEGIST_REWARDS
    //         .save(deps.as_mut().storage, &CoinList::new())
    //         .unwrap();
    //     VAULT_DENOM
    //         .save(deps.as_mut().storage, &"share_token".to_string())
    //         .unwrap();
    //     SHARES
    //         .save(
    //             deps.as_mut().storage,
    //             Addr::unchecked("bolice"),
    //             &Uint128::new(1000),
    //         )
    //         .unwrap();

    // USER_REWARDS
    //     .save(
    //         deps.as_mut().storage,
    //         Addr::unchecked("alice"),
    //         &CoinList::from_coins(vec![coin(100, "token0"), coin(175, "token1")]),
    //     )
    //     .unwrap();
    // USER_REWARDS
    //     .save(
    //         deps.as_mut().storage,
    //         Addr::unchecked("bob"),
    //         &CoinList::from_coins(vec![coin(50, "token0"), coin(125, "token1")]),
    //     )
    //     .unwrap();

    //     let _res =
    //         execute_withdraw(deps.as_mut(), env, info, None, Uint128::new(1000).into()).unwrap();
    //     // our querier returns a total supply of 100_000, this user unbonds 1000, or 1%. The Dust saved should be one lower
    //     assert_eq!(
    //         CURRENT_WITHDRAWER_DUST.load(deps.as_ref().storage).unwrap(),
    //         (Uint128::new(18), Uint128::new(27))
    //     )
    // }

    // #[test]
    // fn execute_withdraw_works_user_strategist_rewards() {
    //     let info = mock_info("bolice", &[]);
    //     let mut deps = mock_deps_with_querier_with_balance(
    //         &info,
    //         &[(
    //             MOCK_CONTRACT_ADDR,
    //             &[coin(200000, "token0"), coin(300000, "token1")],
    //         )],
    //     );
    //     let env = mock_env();

    //     STRATEGIST_REWARDS
    //         .save(
    //             deps.as_mut().storage,
    //             &CoinList::from_coins(vec![coin(50, "token0"), coin(50, "token1")]),
    //         )
    //         .unwrap();
    //     VAULT_DENOM
    //         .save(deps.as_mut().storage, &"share_token".to_string())
    //         .unwrap();
    //     SHARES
    //         .save(
    //             deps.as_mut().storage,
    //             Addr::unchecked("bolice"),
    //             &Uint128::new(1000),
    //         )
    //         .unwrap();

    // USER_REWARDS
    //     .save(
    //         deps.as_mut().storage,
    //         Addr::unchecked("alice"),
    //         &CoinList::from_coins(vec![coin(200, "token0"), coin(300, "token1")]),
    //     )
    //     .unwrap();
    // USER_REWARDS
    //     .save(
    //         deps.as_mut().storage,
    //         Addr::unchecked("bob"),
    //         &CoinList::from_coins(vec![coin(400, "token0"), coin(100, "token1")]),
    //     )
    //     .unwrap();

    //     let _res =
    //         execute_withdraw(deps.as_mut(), env, info, None, Uint128::new(1000).into()).unwrap();
    //     // our querier returns a total supply of 100_000, this user unbonds 1000, or 1%. The Dust saved should be one lower
    //     // user dust should be 1% of 200000 - 650 (= 1993.5) and 1% of 300000 - 450 (= 2995.5)
    //     assert_eq!(
    //         CURRENT_WITHDRAWER_DUST.load(deps.as_ref().storage).unwrap(),
    //         (Uint128::new(1993), Uint128::new(2995))
    //     )
    // }

    // the execute withdraw flow should be easiest to test in test-tube, since it requires quite a bit of Osmsosis specific information
    // we just test the handle withdraw implementation here
    #[test]
    fn handle_withdraw_user_reply_works() {
        let mut deps = mock_dependencies();
        let to_address = Addr::unchecked("bolice");
        CURRENT_WITHDRAWER
            .save(deps.as_mut().storage, &to_address)
            .unwrap();
        CURRENT_WITHDRAWER_DUST
            .save(
                deps.as_mut().storage,
                &(Uint128::new(123), Uint128::new(234)),
            )
            .unwrap();

        POOL_CONFIG
            .save(
                deps.as_mut().storage,
                &PoolConfig {
                    pool_id: 1,
                    token0: "uosmo".into(),
                    token1: "uatom".into(),
                },
            )
            .unwrap();

        let msg = MsgWithdrawPositionResponse {
            amount0: "1000".to_string(),
            amount1: "1000".to_string(),
        }
        .try_into()
        .unwrap();

        let response = handle_withdraw_user_reply(
            deps.as_mut(),
            SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: Some(msg),
            }),
        )
        .unwrap();
        assert_eq!(
            response.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: to_address.to_string(),
                amount: sort_tokens(vec![coin(1123, "uosmo"), coin(1234, "uatom")])
            })
        )
    }
}
