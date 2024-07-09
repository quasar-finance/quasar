use crate::contract::{execute, instantiate, reply, SWAP_REPLY_ID};
use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::tests::util::{
    get_fund_denom, get_init_msg, mock_wasm_querier_with_lst_adapter, CREATOR, DEPOSIT_DENOM,
    LST_DENOM, TEST_DEX_ADAPTER, TEST_LST_ADAPTER, TEST_UNBONDING_PERIOD, USER,
};
use cosmwasm_std::testing::{
    mock_dependencies, mock_dependencies_with_balances, mock_env, mock_info, MOCK_CONTRACT_ADDR,
};
use cosmwasm_std::{
    coin, coins, to_json_binary, Reply, SubMsgResponse, SubMsgResult, Uint128, WasmMsg,
};
use lst_adapter_osmosis::msg::LstAdapterExecuteMsg;
use lst_dex_adapter_osmosis::msg::DexAdapterExecuteMsg as DexExecuteMsg;
use quasar_types::abstract_sdk::ExecuteMsg as AbstractExecuteMsg;

#[test]
fn swap_fails_if_insufficient_funds() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let swap_amount = Uint128::from(100u128);
    let info = mock_info(CREATOR, &[coin(1, "other_denom".to_string())]);
    let err = execute(
        deps.as_mut(),
        env,
        info,
        ExecuteMsg::Swap {
            amount: swap_amount,
            slippage: None,
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::InsufficientFunds {});
}

#[test]
fn swap_fails_if_insufficient_funds_due_to_blocked_funds() {
    let deposits = 1_000_000;
    let fund_denom = get_fund_denom();
    let fund_shares = 10000u64;
    let mut deps = mock_dependencies_with_balances(&[
        (USER, &[coin(fund_shares.into(), fund_denom.clone())]),
        (MOCK_CONTRACT_ADDR, &[coin(deposits, DEPOSIT_DENOM)]),
    ]);
    deps.querier.update_wasm(mock_wasm_querier_with_lst_adapter(
        TEST_LST_ADAPTER.to_owned(),
        0,
        0,
    ));
    let mut env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[coin(fund_shares.into(), fund_denom.clone())]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let withdraw_amount = 10000;
    let info = mock_info(USER, &[coin(withdraw_amount, fund_denom.to_string())]);
    let _ = execute(deps.as_mut(), env.clone(), info, ExecuteMsg::Withdraw {}).unwrap();
    env.block.time = env.block.time.plus_seconds(TEST_UNBONDING_PERIOD);
    let swap_amount = Uint128::from(100u128);
    let info = mock_info(CREATOR, &[]);
    let err = execute(
        deps.as_mut(),
        env,
        info,
        ExecuteMsg::Swap {
            amount: swap_amount,
            slippage: None,
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::InsufficientFunds {});
}

// #[test]
// fn swap_succeeds_if_blocked_funds_are_covered_by_claimable_funds() {
//     let deposits = 1_000_000;
//     let lst_claimable = 1_000_000;
//     let fund_denom = get_fund_denom();
//     let fund_shares = 10000u64;
//     let mut deps = mock_dependencies_with_balances(&[
//         (USER, &[coin(fund_shares.into(), fund_denom.clone())]),
//         (MOCK_CONTRACT_ADDR, &[coin(deposits, DEPOSIT_DENOM)]),
//     ]);
//     deps.querier.update_wasm(mock_wasm_querier_with_lst_adapter(
//         TEST_LST_ADAPTER.to_owned(),
//         0,
//         lst_claimable,
//     ));
//     let mut env = mock_env();

//     let msg = get_init_msg();
//     let info = mock_info(CREATOR, &[coin(fund_shares.into(), fund_denom.clone())]);
//     let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
//     assert_eq!(1, res.messages.len());

//     let withdraw_amount = 9000;
//     let info = mock_info(USER, &[coin(withdraw_amount, fund_denom.to_string())]);
//     let _ = execute(deps.as_mut(), env.clone(), info, ExecuteMsg::Withdraw {}).unwrap();
//     env.block.time = env.block.time.plus_seconds(TEST_UNBONDING_PERIOD);
//     let swap_amount = Uint128::from(200_000u128);
//     let info = mock_info(CREATOR, &[]);
//     let _ = execute(
//         deps.as_mut(),
//         env,
//         info,
//         ExecuteMsg::Swap {
//             amount: swap_amount,
//             slippage: None,
//         },
//     )
//     .unwrap();
// }

#[test]
fn after_swap_lst_tokens_are_send_to_the_lst_adapter() {
    let deposits = 1_000_000;
    let lst_amount = 1000;
    let mut deps = mock_dependencies_with_balances(&[(
        MOCK_CONTRACT_ADDR,
        &[coin(deposits, DEPOSIT_DENOM), coin(lst_amount, LST_DENOM)],
    )]);
    let env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let swap_amount = Uint128::from(100u128);
    let info = mock_info(CREATOR, &[]);
    let response = execute(
        deps.as_mut(),
        env.clone(),
        info,
        ExecuteMsg::Swap {
            amount: swap_amount,
            slippage: None,
        },
    )
    .unwrap();
    assert_eq!(response.messages.len(), 1);
    assert_eq!(
        response.messages[0].msg,
        WasmMsg::Execute {
            contract_addr: TEST_DEX_ADAPTER.to_string(),
            msg: to_json_binary(&AbstractExecuteMsg::Module {
                module: DexExecuteMsg::Swap { slippage: None }
            })
            .unwrap(),
            funds: vec![coin(swap_amount.into(), DEPOSIT_DENOM)],
        }
        .into()
    );

    let response = reply(
        deps.as_mut(),
        env,
        Reply {
            id: SWAP_REPLY_ID,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: None,
            }),
        },
    )
    .unwrap();
    assert_eq!(response.messages.len(), 1);
    assert_eq!(
        response.messages[0].msg,
        WasmMsg::Execute {
            contract_addr: TEST_LST_ADAPTER.to_string(),
            msg: to_json_binary(&LstAdapterExecuteMsg::Unbond {}).unwrap(),
            funds: coins(lst_amount, LST_DENOM)
        }
        .into()
    );
}
