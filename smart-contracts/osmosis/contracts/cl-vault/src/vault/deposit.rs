use crate::{
    error::assert_deposits,
    helpers::{
        getters::{
            get_depositable_tokens, get_single_sided_deposit_0_to_1_swap_amount,
            get_single_sided_deposit_1_to_0_swap_amount, get_twap_price, get_unused_pair,
            get_value_wrt_asset0, DepositInfo,
        },
        msgs::refund_bank_msg,
    },
    query::{query_total_assets, query_total_vault_token_supply},
    reply::Replies,
    state::{CurrentSwap, CURRENT_SWAP_INFO, POOL_CONFIG, SHARES, VAULT_CONFIG, VAULT_DENOM},
    vault::{
        concentrated_liquidity::{get_cl_pool_info, get_position},
        swap::{estimate_swap_min_out_amount, swap_msg},
    },
    ContractError,
};
use cosmwasm_std::{
    attr, coin, Addr, Decimal, DepsMut, Env, Fraction, MessageInfo, Response, SubMsg, SubMsgResult,
    Uint128, Uint256,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMint;
use quasar_types::pool_pair::PoolPair;

pub(crate) fn execute_exact_deposit(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    assert_deposits(&info.funds, &pool_config)?;
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;
    let deposit_info = get_depositable_tokens(&deps.as_ref(), info.funds, &pool_config)?;

    execute_deposit(&mut deps, env, recipient, deposit_info)
}

pub(crate) fn execute_any_deposit(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
    max_slippage: Decimal,
) -> Result<Response, ContractError> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    assert_deposits(&info.funds, &pool_config)?;
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;
    let position = get_position(deps.storage, &deps.querier)?
        .position
        .ok_or(ContractError::MissingPosition {})?;

    let deposit_info = get_depositable_tokens(&deps.as_ref(), info.funds, &pool_config)?;
    if deposit_info.base_refund.amount.is_zero() && deposit_info.quote_refund.amount.is_zero() {
        return execute_deposit(&mut deps, env, recipient, deposit_info);
    }

    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let twap_price = get_twap_price(
        &deps.querier,
        env.block.time,
        vault_config.twap_window_seconds,
        pool_config.pool_id,
        pool_config.clone().token0,
        pool_config.clone().token1,
    )?;
    let (token_in, out_denom, price) = if !deposit_info.base_refund.amount.is_zero() {
        let token_in_amount = if pool_details.current_tick > position.upper_tick {
            deposit_info.base_refund.amount
        } else {
            get_single_sided_deposit_0_to_1_swap_amount(
                deposit_info.base_refund.amount,
                position.lower_tick,
                pool_details.current_tick,
                position.upper_tick,
            )?
        };
        let token_in = coin(token_in_amount.into(), pool_config.token0.clone());
        (token_in, pool_config.token1.clone(), twap_price)
    } else {
        let token_in_amount = if pool_details.current_tick < position.lower_tick {
            deposit_info.quote_refund.amount
        } else {
            get_single_sided_deposit_1_to_0_swap_amount(
                deposit_info.quote_refund.amount,
                position.lower_tick,
                pool_details.current_tick,
                position.upper_tick,
            )?
        };
        let token_in = coin(token_in_amount.into(), pool_config.token1.clone());
        (
            token_in,
            pool_config.token0.clone(),
            twap_price.inv().expect("Invalid price"),
        )
    };
    let unused = get_unused_pair(&deps.as_ref(), &env.contract.address, &pool_config)?;
    let base_funds = deposit_info
        .base_deposit
        .checked_add(deposit_info.base_refund.amount)?;
    let quote_funds = deposit_info
        .quote_deposit
        .checked_add(deposit_info.quote_refund.amount)?;
    CURRENT_SWAP_INFO.save(
        deps.storage,
        &CurrentSwap {
            recipient,
            vault_balance: PoolPair::new(
                coin(
                    unused.base.amount.checked_sub(base_funds)?.into(),
                    unused.base.denom,
                ),
                coin(
                    unused.quote.amount.checked_sub(quote_funds)?.into(),
                    unused.quote.denom,
                ),
            ),
        },
    )?;

    let token_out_min_amount = estimate_swap_min_out_amount(token_in.amount, price, max_slippage)?;

    let swap_msg = swap_msg(
        vault_config.dex_router,
        token_in.clone(),
        coin(token_out_min_amount.into(), out_denom.clone()),
        None, // TODO: check this None
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            swap_msg,
            Replies::AnyDepositSwap.into(),
        ))
        .add_attributes(vec![
            attr("method", "execute"),
            attr("action", "any_deposit"),
            attr("token_in", format!("{}", token_in)),
            attr("token_out_min_amount", format!("{}", token_out_min_amount)),
        ]))
}

pub fn handle_any_deposit_swap_reply(
    mut deps: DepsMut,
    env: Env,
    _data: SubMsgResult,
) -> Result<Response, ContractError> {
    let info: CurrentSwap = CURRENT_SWAP_INFO.load(deps.storage)?;
    CURRENT_SWAP_INFO.remove(deps.storage);

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let balances = get_unused_pair(&deps.as_ref(), &env.contract.address, &pool_config)?;
    let user_balances = balances.checked_sub(&info.vault_balance)?;

    execute_deposit(
        &mut deps,
        env,
        info.recipient,
        DepositInfo {
            base_deposit: user_balances.base.amount,
            quote_deposit: user_balances.quote.amount,
            base_refund: coin(0u128, pool_config.token0),
            quote_refund: coin(0u128, pool_config.token1),
        },
    )
}

/// Try to deposit as much user funds as we can in the current ratio of the vault and
/// refund the rest to the caller.
fn execute_deposit(
    deps: &mut DepsMut,
    env: Env,
    recipient: Addr,
    deposit_info: DepositInfo,
) -> Result<Response, ContractError> {
    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total_vault_shares: Uint256 = query_total_vault_token_supply(deps.as_ref())?.total.into();

    let user_value = get_value_wrt_asset0(
        deps.storage,
        &deps.querier,
        deposit_info.base_deposit,
        deposit_info.quote_deposit,
    )?;
    let refund_value = get_value_wrt_asset0(
        deps.storage,
        &deps.querier,
        deposit_info.base_refund.amount,
        deposit_info.quote_refund.amount,
    )?;

    // calculate the amount of shares we can mint for this
    let total_assets = query_total_assets(deps.as_ref(), env.clone())?;
    let total_assets_value = get_value_wrt_asset0(
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
        .add_attribute("amount0", deposit_info.base_deposit)
        .add_attribute("amount1", deposit_info.quote_deposit)
        .add_message(mint_msg)
        .add_attribute("mint_shares_amount", user_shares)
        .add_attribute("receiver", recipient.as_str());

    if let Some((bank_msg, bank_attr)) = refund_bank_msg(
        recipient,
        Some(deposit_info.base_refund),
        Some(deposit_info.quote_refund),
    )? {
        resp = resp.add_message(bank_msg).add_attributes(bank_attr);
    }

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{
        testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR},
        Addr, BankMsg, CosmosMsg, Decimal256, Fraction, Reply, SubMsgResponse, Uint256, WasmMsg,
    };

    use crate::{
        contract::{execute, reply},
        helpers::msgs::refund_bank_msg,
        msg::ExecuteMsg,
        test_helpers::{
            instantiate_contract, mock_deps_with_querier, mock_deps_with_querier_with_balance,
            BASE_DENOM, QUOTE_DENOM, TEST_VAULT_DENOM, TEST_VAULT_TOKEN_SUPPLY,
        },
    };

    use super::*;

    #[test]
    fn execute_exact_deposit_works() {
        let mut deps = mock_deps_with_querier();
        let env = mock_env();
        let sender = "alice";

        instantiate_contract(deps.as_mut(), env.clone(), sender);

        let info = mock_info(sender, &[coin(100, BASE_DENOM), coin(100, QUOTE_DENOM)]);
        execute_exact_deposit(deps.as_mut(), env, info, None).unwrap();

        // we currently have 100_000 total_vault_shares outstanding and the equivalent of 1999500token0, the user deposits the equivalent of 199token0, thus shares are
        // 199 * 100000 / 1999500 = 9.95, which we round down. Thus we expect 9 shares in this example
        assert_eq!(
            SHARES
                .load(deps.as_ref().storage, Addr::unchecked(sender))
                .unwrap(),
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

    const ADMIN: &str = "admin";
    const SENDER: &str = "sender";

    #[test]
    fn exact_deposit_with_single_wrong_denom_fails() {
        let mut deps = mock_deps_with_querier();
        let env = mock_env();
        instantiate_contract(deps.as_mut(), env.clone(), ADMIN);

        let info = mock_info(SENDER, &[coin(1, "other_denom".to_string())]);
        let err = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::ExactDeposit { recipient: None },
        )
        .unwrap_err();
        assert_eq!(err, ContractError::IncorrectDepositFunds);
    }

    #[test]
    fn exact_deposit_with_more_than_two_assets_fails() {
        let mut deps = mock_deps_with_querier();
        let env = mock_env();
        instantiate_contract(deps.as_mut(), env.clone(), ADMIN);

        let info = mock_info(
            SENDER,
            &[
                coin(1, BASE_DENOM.to_string()),
                coin(1, QUOTE_DENOM.to_string()),
                coin(1, "other_denom".to_string()),
            ],
        );
        let err = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::ExactDeposit { recipient: None },
        )
        .unwrap_err();
        assert_eq!(err, ContractError::IncorrectDepositFunds);
    }

    #[test]
    fn successful_exact_deposit_mints_fund_tokens_according_to_share_of_assets() {
        let current_deposit_amount = 100u128;
        let deposit_amount = 50;
        let env = mock_env();
        let fund_shares = 50000u64;
        let mut deps = mock_deps_with_querier_with_balance(
            100,
            100,
            0,
            100,
            1000,
            &[(
                MOCK_CONTRACT_ADDR,
                &[
                    coin(current_deposit_amount + deposit_amount, BASE_DENOM),
                    coin(current_deposit_amount + deposit_amount, QUOTE_DENOM),
                    coin(fund_shares.into(), TEST_VAULT_DENOM),
                ],
            )],
        );

        instantiate_contract(deps.as_mut(), env.clone(), ADMIN);

        let info = mock_info(
            SENDER,
            &[
                coin(deposit_amount, BASE_DENOM.to_string()),
                coin(deposit_amount, QUOTE_DENOM),
            ],
        );
        let response = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::ExactDeposit { recipient: None },
        )
        .unwrap();
        assert_eq!(response.messages.len(), 1);

        let expected_minted_tokens = TEST_VAULT_TOKEN_SUPPLY / 4;
        let msg = response.messages[0].msg.clone();
        match msg {
            CosmosMsg::Stargate { type_url: _, value } => {
                let m: MsgMint = value.try_into().unwrap();
                assert_eq!(m.sender, env.contract.address.to_string());
                assert_eq!(
                    m.amount.as_ref().unwrap().amount,
                    expected_minted_tokens.to_string()
                );
                assert_eq!(
                    m.amount.as_ref().unwrap().denom,
                    TEST_VAULT_DENOM.to_string()
                );
                assert_eq!(m.mint_to_address, env.contract.address.to_string());
            }
            _ => panic!("unreachable"),
        }
    }

    #[test]
    fn any_deposit_with_single_wrong_denom_fails() {
        let mut deps = mock_deps_with_querier();
        let env = mock_env();
        instantiate_contract(deps.as_mut(), env.clone(), ADMIN);

        let info = mock_info(SENDER, &[coin(1, "other_denom".to_string())]);
        let err = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::AnyDeposit {
                amount: Uint128::zero(),
                asset: String::default(),
                recipient: None,
                max_slippage: Decimal::percent(90),
            },
        )
        .unwrap_err();
        assert_eq!(err, ContractError::IncorrectDepositFunds);
    }

    #[test]
    fn any_deposit_with_more_than_two_assets_fails() {
        let mut deps = mock_deps_with_querier();
        let env = mock_env();
        instantiate_contract(deps.as_mut(), env.clone(), ADMIN);

        let info = mock_info(
            SENDER,
            &[
                coin(1, BASE_DENOM.to_string()),
                coin(1, QUOTE_DENOM.to_string()),
                coin(1, "other_denom".to_string()),
            ],
        );
        let err = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::AnyDeposit {
                amount: Uint128::zero(),
                asset: String::default(),
                recipient: None,
                max_slippage: Decimal::percent(90),
            },
        )
        .unwrap_err();
        assert_eq!(err, ContractError::IncorrectDepositFunds);
    }

    #[test]
    fn successful_exact_any_deposit_mints_fund_tokens_according_to_share_of_assets() {
        let current_deposit_amount = 100u128;
        let deposit_amount = 50;
        let env = mock_env();
        let mut deps = mock_deps_with_querier_with_balance(
            100,
            100,
            0,
            100,
            1000,
            &[(
                MOCK_CONTRACT_ADDR,
                &[
                    coin(current_deposit_amount + deposit_amount, BASE_DENOM),
                    coin(current_deposit_amount + deposit_amount, QUOTE_DENOM),
                    coin(TEST_VAULT_TOKEN_SUPPLY, TEST_VAULT_DENOM),
                ],
            )],
        );

        instantiate_contract(deps.as_mut(), env.clone(), ADMIN);

        let info = mock_info(
            SENDER,
            &[
                coin(deposit_amount, BASE_DENOM.to_string()),
                coin(deposit_amount, QUOTE_DENOM),
            ],
        );
        let response = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::AnyDeposit {
                amount: Uint128::zero(),
                asset: String::default(),
                recipient: None,
                max_slippage: Decimal::percent(90),
            },
        )
        .unwrap();
        assert_eq!(response.messages.len(), 1);

        let expected_minted_tokens = TEST_VAULT_TOKEN_SUPPLY / 4;
        let msg = response.messages[0].msg.clone();
        match msg {
            CosmosMsg::Stargate { type_url: _, value } => {
                let m: MsgMint = value.try_into().unwrap();
                assert_eq!(m.sender, env.contract.address.to_string());
                assert_eq!(
                    m.amount.as_ref().unwrap().amount,
                    expected_minted_tokens.to_string()
                );
                assert_eq!(
                    m.amount.as_ref().unwrap().denom,
                    TEST_VAULT_DENOM.to_string()
                );
                assert_eq!(m.mint_to_address, env.contract.address.to_string());
            }
            _ => panic!("unreachable"),
        }
    }

    #[test]
    fn successful_inexact_any_deposit_mints_fund_tokens_according_to_share_of_assets() {
        let current_vault_base_balance = 150u128;
        let current_vault_quote_balance = 100u128;
        let base_deposit_amount = 150;
        let quote_deposit_amount = 100;
        let current_price = Decimal::percent(200);
        let env = mock_env();
        let mut deps = mock_deps_with_querier_with_balance(
            100,
            200,
            1_000_000,
            900_000,
            1_101_000,
            &[(
                MOCK_CONTRACT_ADDR,
                &[
                    coin(current_vault_base_balance + base_deposit_amount, BASE_DENOM),
                    coin(
                        current_vault_quote_balance + quote_deposit_amount,
                        QUOTE_DENOM,
                    ),
                    coin(TEST_VAULT_TOKEN_SUPPLY, TEST_VAULT_DENOM),
                ],
            )],
        );

        instantiate_contract(deps.as_mut(), env.clone(), ADMIN);

        let info = mock_info(
            SENDER,
            &[
                coin(base_deposit_amount, BASE_DENOM.to_string()),
                coin(quote_deposit_amount, QUOTE_DENOM),
            ],
        );
        let response = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::AnyDeposit {
                amount: Uint128::zero(),
                asset: String::default(),
                recipient: None,
                max_slippage: Decimal::percent(90),
            },
        )
        .unwrap();
        assert_eq!(response.messages.len(), 1);

        let expected_swap_amount = 50u128;
        let msg = response.messages[0].msg.clone();
        match msg {
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: _,
                msg: _,
                funds,
            }) => {
                assert_eq!(funds.len(), 1);
                assert_eq!(funds[0].denom, BASE_DENOM);
                assert_eq!(funds[0].amount.u128(), expected_swap_amount);
            }
            _ => panic!("unreachable"),
        }
        let swap_receive_amount: u128 = Uint128::from(expected_swap_amount)
            .checked_mul_floor(current_price)
            .unwrap()
            .into();
        deps.querier.update_balances(&[(
            MOCK_CONTRACT_ADDR,
            &[
                coin(
                    current_vault_base_balance + base_deposit_amount - expected_swap_amount,
                    BASE_DENOM,
                ),
                coin(
                    current_vault_quote_balance + quote_deposit_amount + swap_receive_amount,
                    QUOTE_DENOM,
                ),
                coin(TEST_VAULT_TOKEN_SUPPLY, TEST_VAULT_DENOM),
            ],
        )]);

        let response = reply(
            deps.as_mut(),
            env.clone(),
            Reply {
                id: Replies::AnyDepositSwap.into(),
                result: SubMsgResult::Ok(SubMsgResponse {
                    events: vec![],
                    data: None,
                }),
            },
        )
        .unwrap();
        let expected_minted_tokens = 50_000;
        let msg = response.messages[0].msg.clone();
        match msg {
            CosmosMsg::Stargate { type_url: _, value } => {
                let m: MsgMint = value.try_into().unwrap();
                assert_eq!(m.sender, env.contract.address.to_string());
                assert_eq!(
                    m.amount.as_ref().unwrap().amount,
                    expected_minted_tokens.to_string()
                );
                assert_eq!(
                    m.amount.as_ref().unwrap().denom,
                    TEST_VAULT_DENOM.to_string()
                );
                assert_eq!(m.mint_to_address, env.contract.address.to_string());
            }
            _ => panic!("unreachable"),
        }
    }

    #[test]
    fn successful_inexact_any_deposit_mints_fund_tokens_according_to_share_of_assets_one_sided_position_base_only(
    ) {
        let current_vault_base_balance = 100u128;
        let current_vault_quote_balance = 100u128;
        let base_deposit_amount = 50;
        let quote_deposit_amount = 100;
        let position_base_amount = 100;
        let current_price = Decimal::percent(200);
        let env = mock_env();
        let mut deps = mock_deps_with_querier_with_balance(
            position_base_amount,
            0,
            1_000_000,
            10_000_000,
            20_000_000,
            &[(
                MOCK_CONTRACT_ADDR,
                &[
                    coin(current_vault_base_balance + base_deposit_amount, BASE_DENOM),
                    coin(
                        current_vault_quote_balance + quote_deposit_amount,
                        QUOTE_DENOM,
                    ),
                    coin(TEST_VAULT_TOKEN_SUPPLY, TEST_VAULT_DENOM),
                ],
            )],
        );

        instantiate_contract(deps.as_mut(), env.clone(), ADMIN);

        let info = mock_info(
            SENDER,
            &[
                coin(base_deposit_amount, BASE_DENOM.to_string()),
                coin(quote_deposit_amount, QUOTE_DENOM),
            ],
        );
        let response = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::AnyDeposit {
                amount: Uint128::zero(),
                asset: String::default(),
                recipient: None,
                max_slippage: Decimal::percent(90),
            },
        )
        .unwrap();
        assert_eq!(response.messages.len(), 1);

        let expected_swap_amount = 100u128;
        let msg = response.messages[0].msg.clone();
        match msg {
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: _,
                msg: _,
                funds,
            }) => {
                assert_eq!(funds.len(), 1);
                assert_eq!(funds[0].denom, QUOTE_DENOM);
                assert_eq!(funds[0].amount, Uint128::from(expected_swap_amount));
            }
            _ => panic!("unreachable"),
        }

        let swap_receive_amount: u128 = Uint128::from(expected_swap_amount)
            .checked_div_floor(current_price)
            .unwrap()
            .into();
        deps.querier.update_balances(&[(
            MOCK_CONTRACT_ADDR,
            &[
                coin(
                    current_vault_base_balance + base_deposit_amount + swap_receive_amount,
                    BASE_DENOM,
                ),
                coin(
                    current_vault_quote_balance + quote_deposit_amount - expected_swap_amount,
                    QUOTE_DENOM,
                ),
                coin(TEST_VAULT_TOKEN_SUPPLY, TEST_VAULT_DENOM),
            ],
        )]);

        let response = reply(
            deps.as_mut(),
            env.clone(),
            Reply {
                id: Replies::AnyDepositSwap.into(),
                result: SubMsgResult::Ok(SubMsgResponse {
                    events: vec![],
                    data: None,
                }),
            },
        )
        .unwrap();
        let expected_minted_tokens = 40_000;
        let msg = response.messages[0].msg.clone();
        match msg {
            CosmosMsg::Stargate { type_url: _, value } => {
                let m: MsgMint = value.try_into().unwrap();
                assert_eq!(m.sender, env.contract.address.to_string());
                assert_eq!(
                    m.amount.as_ref().unwrap().amount,
                    expected_minted_tokens.to_string()
                );
                assert_eq!(
                    m.amount.as_ref().unwrap().denom,
                    TEST_VAULT_DENOM.to_string()
                );
                assert_eq!(m.mint_to_address, env.contract.address.to_string());
            }
            _ => panic!("unreachable"),
        }
    }

    #[test]
    fn successful_inexact_any_deposit_mints_fund_tokens_according_to_share_of_assets_one_sided_position_quote_only(
    ) {
        let current_vault_base_balance = 100u128;
        let current_vault_quote_balance = 100u128;
        let base_deposit_amount = 50;
        let quote_deposit_amount = 100;
        let position_quote_amount = 100;
        let current_price = Decimal::percent(200);
        let env = mock_env();
        let mut deps = mock_deps_with_querier_with_balance(
            0,
            position_quote_amount,
            1_000_000, // price = 2.0
            100,
            1000,
            &[(
                MOCK_CONTRACT_ADDR,
                &[
                    coin(current_vault_base_balance + base_deposit_amount, BASE_DENOM),
                    coin(
                        current_vault_quote_balance + quote_deposit_amount,
                        QUOTE_DENOM,
                    ),
                    coin(TEST_VAULT_TOKEN_SUPPLY, TEST_VAULT_DENOM),
                ],
            )],
        );

        instantiate_contract(deps.as_mut(), env.clone(), ADMIN);

        let info = mock_info(
            SENDER,
            &[
                coin(base_deposit_amount, BASE_DENOM.to_string()),
                coin(quote_deposit_amount, QUOTE_DENOM),
            ],
        );
        let response = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::AnyDeposit {
                amount: Uint128::zero(),
                asset: String::default(),
                recipient: None,
                max_slippage: Decimal::percent(90),
            },
        )
        .unwrap();
        assert_eq!(response.messages.len(), 1);

        let expected_swap_amount = 50u128;
        let msg = response.messages[0].msg.clone();
        match msg {
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: _,
                msg: _,
                funds,
            }) => {
                assert_eq!(funds.len(), 1);
                assert_eq!(funds[0].denom, BASE_DENOM);
                assert_eq!(funds[0].amount, Uint128::from(expected_swap_amount));
            }
            _ => panic!("unreachable"),
        }

        let swap_receive_amount: u128 = Uint128::from(expected_swap_amount)
            .checked_mul_floor(current_price)
            .unwrap()
            .into();
        deps.querier.update_balances(&[(
            MOCK_CONTRACT_ADDR,
            &[
                coin(
                    current_vault_base_balance + base_deposit_amount - expected_swap_amount,
                    BASE_DENOM,
                ),
                coin(
                    current_vault_quote_balance + quote_deposit_amount + swap_receive_amount,
                    QUOTE_DENOM,
                ),
                coin(TEST_VAULT_TOKEN_SUPPLY, TEST_VAULT_DENOM),
            ],
        )]);

        let response = reply(
            deps.as_mut(),
            env.clone(),
            Reply {
                id: Replies::AnyDepositSwap.into(),
                result: SubMsgResult::Ok(SubMsgResponse {
                    events: vec![],
                    data: None,
                }),
            },
        )
        .unwrap();
        let expected_minted_tokens = 50_000;
        let msg = response.messages[0].msg.clone();
        match msg {
            CosmosMsg::Stargate { type_url: _, value } => {
                let m: MsgMint = value.try_into().unwrap();
                assert_eq!(m.sender, env.contract.address.to_string());
                assert_eq!(
                    m.amount.as_ref().unwrap().amount,
                    expected_minted_tokens.to_string()
                );
                assert_eq!(
                    m.amount.as_ref().unwrap().denom,
                    TEST_VAULT_DENOM.to_string()
                );
                assert_eq!(m.mint_to_address, env.contract.address.to_string());
            }
            _ => panic!("unreachable"),
        }
    }
}
