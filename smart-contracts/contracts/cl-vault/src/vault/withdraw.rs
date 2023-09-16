use cosmwasm_std::{
    attr, coin, BankMsg, CosmosMsg, Decimal256, DepsMut, Env, MessageInfo, Response, SubMsg,
    SubMsgResult, Uint128,
};
use osmosis_std::types::{
    cosmos::bank::v1beta1::BankQuerier,
    osmosis::{
        concentratedliquidity::v1beta1::{MsgWithdrawPosition, MsgWithdrawPositionResponse},
        tokenfactory::v1beta1::MsgBurn,
    },
};

use crate::{
    helpers::{get_unused_balances, sort_tokens},
    reply::Replies,
    state::{CURRENT_WITHDRAWER, CURRENT_WITHDRAWER_DUST, POOL_CONFIG, SHARES, VAULT_DENOM},
    vault::concentrated_liquidity::{get_position, withdraw_from_position},
    ContractError,
};

// any locked shares are sent in amount, due to a lack of tokenfactory hooks during development
// currently that functions as a bandaid
pub fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    // get the sent along shares
    // let shares = must_pay(&info, vault_denom.as_str())?;

    // get the amount from SHARES state
    let user_shares = SHARES.load(deps.storage, info.sender.clone())?;
    let left_over = user_shares
        .checked_sub(amount)
        .map_err(|_| ContractError::InsufficientFunds)?;
    SHARES.save(deps.storage, info.sender, &left_over)?;

    let total_shares = BankQuerier::new(&deps.querier)
        .supply_of(vault_denom.clone())?
        .amount
        .unwrap()
        .amount
        .parse()?;

    // get the dust amounts belonging to the user
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    // TODO replace dust with queries for balance
    let unused_balances = get_unused_balances(deps.storage, &deps.querier, &env)?;
    let dust0 = unused_balances.find_coin(pool_config.token0.clone()).amount;
    let dust1 = unused_balances.find_coin(pool_config.token1.clone()).amount;
    let user_dust0 = dust0.checked_mul(amount)?.checked_div(total_shares)?;
    let user_dust1 = dust1.checked_mul(amount)?.checked_div(total_shares)?;
    // save the new total amount of dust available for other actions

    CURRENT_WITHDRAWER_DUST.save(deps.storage, &(user_dust0, user_dust1))?;

    // burn the shares
    let burn_coin = coin(amount.u128(), vault_denom);
    let burn_msg: CosmosMsg = MsgBurn {
        sender: env.contract.address.clone().into_string(),
        amount: Some(burn_coin.into()),
        burn_from_address: env.contract.address.clone().into_string(),
    }
    .into();

    CURRENT_WITHDRAWER.save(deps.storage, &recipient)?;

    // withdraw the user's funds from the position
    let withdraw_msg = withdraw(deps, &env, amount)?; // TODOSN: Rename this function name to something more explicative

    Ok(Response::new()
        .add_attribute("method", "withdraw")
        .add_attribute("action", "withdraw")
        .add_attribute("liquidity_amount", withdraw_msg.liquidity_amount.as_str())
        .add_attribute("share_amount", amount)
        .add_message(burn_msg)
        .add_submessage(SubMsg::reply_on_success(
            withdraw_msg,
            Replies::WithdrawUser as u64,
        )))
}

fn withdraw(
    deps: DepsMut,
    env: &Env,
    user_shares: Uint128,
) -> Result<MsgWithdrawPosition, ContractError> {
    let existing_position = get_position(deps.storage, &deps.querier, env)?;
    let existing_liquidity: Decimal256 = existing_position
        .position
        .ok_or(ContractError::PositionNotFound)?
        .liquidity
        .parse()?;

    let bq = BankQuerier::new(&deps.querier);
    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    let total_vault_shares: Uint128 = bq
        .supply_of(vault_denom)?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();

    let user_liquidity = Decimal256::from_ratio(user_shares, 1_u128)
        .checked_mul(existing_liquidity)?
        .checked_div(Decimal256::from_ratio(total_vault_shares, 1_u128))?;

    withdraw_from_position(deps.storage, env, user_liquidity)
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

    let withdraw_attrs = vec![
        attr("token0_amount", coin0.amount),
        attr("token1_amount", coin1.amount),
    ];

    // send the funds to the user
    let msg = BankMsg::Send {
        to_address: user.to_string(),
        amount: sort_tokens(vec![coin0, coin1]),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "withdraw_position_reply")
        .add_attribute("action", "withdraw")
        .add_attributes(withdraw_attrs))
}

#[cfg(test)]
mod tests {
    use crate::{
        rewards::CoinList,
        state::{PoolConfig, STRATEGIST_REWARDS, USER_REWARDS},
        test_helpers::{mock_deps_with_querier, mock_deps_with_querier_with_balance},
    };
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR},
        Addr, CosmosMsg, SubMsgResponse,
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

        let res = execute_withdraw(deps.as_mut(), env, info, None, Uint128::new(1000)).unwrap();
        // our querier returns a total supply of 100_000, this user unbonds 1000, or 1%. The Dust saved should be one lower
        assert_eq!(
            CURRENT_WITHDRAWER_DUST.load(deps.as_ref().storage).unwrap(),
            (Uint128::new(20), Uint128::new(30))
        )
    }

    #[test]
    fn execute_withdraw_works_user_rewards() {
        let info = mock_info("bolice", &[]);
        let mut deps = mock_deps_with_querier_with_balance(
            &info,
            &[(
                MOCK_CONTRACT_ADDR,
                &[coin(2000, "token0"), coin(3000, "token1")],
            )],
        );
        let env = mock_env();

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

        USER_REWARDS
            .save(
                deps.as_mut().storage,
                Addr::unchecked("alice"),
                &CoinList::from_coins(vec![coin(100, "token0"), coin(175, "token1")]),
            )
            .unwrap();
        USER_REWARDS
            .save(
                deps.as_mut().storage,
                Addr::unchecked("bob"),
                &CoinList::from_coins(vec![coin(50, "token0"), coin(125, "token1")]),
            )
            .unwrap();

        let res = execute_withdraw(deps.as_mut(), env, info, None, Uint128::new(1000)).unwrap();
        // our querier returns a total supply of 100_000, this user unbonds 1000, or 1%. The Dust saved should be one lower
        assert_eq!(
            CURRENT_WITHDRAWER_DUST.load(deps.as_ref().storage).unwrap(),
            (Uint128::new(18), Uint128::new(27))
        )
    }

    #[test]
    fn execute_withdraw_works_user_strategist_rewards() {
        let info = mock_info("bolice", &[]);
        let mut deps = mock_deps_with_querier_with_balance(
            &info,
            &[(
                MOCK_CONTRACT_ADDR,
                &[coin(200000, "token0"), coin(300000, "token1")],
            )],
        );
        let env = mock_env();

        STRATEGIST_REWARDS
            .save(
                deps.as_mut().storage,
                &CoinList::from_coins(vec![coin(50, "token0"), coin(50, "token1")]),
            )
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

        USER_REWARDS
            .save(
                deps.as_mut().storage,
                Addr::unchecked("alice"),
                &CoinList::from_coins(vec![coin(200, "token0"), coin(300, "token1")]),
            )
            .unwrap();
        USER_REWARDS
            .save(
                deps.as_mut().storage,
                Addr::unchecked("bob"),
                &CoinList::from_coins(vec![coin(400, "token0"), coin(100, "token1")]),
            )
            .unwrap();

        let res = execute_withdraw(deps.as_mut(), env, info, None, Uint128::new(1000)).unwrap();
        // our querier returns a total supply of 100_000, this user unbonds 1000, or 1%. The Dust saved should be one lower
        // user dust should be 1% of 200000 - 650 (= 1993.5) and 1% of 300000 - 450 (= 2995.5)
        assert_eq!(
            CURRENT_WITHDRAWER_DUST.load(deps.as_ref().storage).unwrap(),
            (Uint128::new(1993), Uint128::new(2995))
        )
    }

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
