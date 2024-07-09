use crate::contract::{execute, instantiate, query};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::Claim;
use crate::tests::util::{
    get_fund_denom, get_init_msg, mock_wasm_querier_with_lst_adapter, CREATOR, DEPOSIT_DENOM,
    TEST_LST_ADAPTER, TEST_UNBONDING_PERIOD, USER,
};
use cosmwasm_std::testing::{
    mock_dependencies, mock_dependencies_with_balances, mock_env, mock_info, MOCK_CONTRACT_ADDR,
};
use cosmwasm_std::{coin, from_json, to_json_binary, BankMsg, Coin, WasmMsg};
use lst_adapter_osmosis::msg::LstAdapterExecuteMsg;
use osmosis_std::types::cosmos::base::v1beta1::Coin as ProtoCoin;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgBurn;
use quasar_types::error::FundsError;

#[test]
fn withdraw_with_wrong_denom_fails() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let info = mock_info(CREATOR, &[coin(1, "other_denom".to_string())]);
    let err = execute(deps.as_mut(), env, info, ExecuteMsg::Withdraw {}).unwrap_err();
    assert_eq!(
        err,
        ContractError::Funds(FundsError::WrongDenom(get_fund_denom().to_string()))
    );
}

#[test]
fn withdraw_with_more_than_one_asset_fails() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let fund_denom = get_fund_denom();
    let info = mock_info(
        CREATOR,
        &[
            coin(1, fund_denom.to_string()),
            coin(1, "other_denom".to_string()),
        ],
    );
    let err = execute(deps.as_mut(), env, info, ExecuteMsg::Withdraw {}).unwrap_err();
    assert_eq!(err, ContractError::Funds(FundsError::InvalidAssets(1)));
}

#[test]
fn withdraw_registers_pending_claim() {
    let deposits = 10000u128;
    let fund_shares = 50000u64;
    let fund_denom = get_fund_denom();
    let mut deps = mock_dependencies_with_balances(&[
        (USER, &[coin(fund_shares.into(), fund_denom.clone())]),
        (MOCK_CONTRACT_ADDR, &[coin(deposits, DEPOSIT_DENOM)]),
    ]);
    deps.querier.update_wasm(mock_wasm_querier_with_lst_adapter(
        TEST_LST_ADAPTER.to_owned(),
        0,
        0,
    ));
    let env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let withdraw_amount = 10000;
    let info = mock_info(USER, &[coin(withdraw_amount, fund_denom.to_string())]);
    let response = execute(deps.as_mut(), env.clone(), info, ExecuteMsg::Withdraw {}).unwrap();
    assert_eq!(response.messages.len(), 1);
    assert_eq!(
        response.messages[0].msg,
        MsgBurn {
            sender: MOCK_CONTRACT_ADDR.to_string(),
            amount: Some(ProtoCoin {
                amount: withdraw_amount.to_string(),
                denom: fund_denom,
            }),
            burn_from_address: MOCK_CONTRACT_ADDR.to_string(),
        }
        .into()
    );

    let pending: Vec<Claim> = from_json(
        query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::Pending {
                address: USER.to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].amount.u128(), 2000);
    assert_eq!(
        pending[0].expiration,
        env.block.time.plus_seconds(TEST_UNBONDING_PERIOD)
    );
}

#[test]
fn claim_fails_before_expiration() {
    let deposits = 10000u128;
    let fund_shares = 50000u64;
    let fund_denom = get_fund_denom();
    let mut deps = mock_dependencies_with_balances(&[
        (USER, &[coin(fund_shares.into(), fund_denom.clone())]),
        (MOCK_CONTRACT_ADDR, &[coin(deposits, DEPOSIT_DENOM)]),
    ]);
    deps.querier.update_wasm(mock_wasm_querier_with_lst_adapter(
        TEST_LST_ADAPTER.to_owned(),
        0,
        0,
    ));
    let env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let withdraw_amount = 10000;
    let info = mock_info(USER, &[coin(withdraw_amount, fund_denom.to_string())]);
    let _ = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::Withdraw {},
    )
    .unwrap();
    let err = execute(deps.as_mut(), env.clone(), info, ExecuteMsg::Claim {}).unwrap_err();
    assert_eq!(err, ContractError::NothingToClaim {});
}

#[test]
fn claim_succeeds_after_expiration() {
    let deposits = 10000u128;
    let fund_shares = 50000u64;
    let fund_denom = get_fund_denom();
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
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let withdraw_amount = 10000;
    let info = mock_info(USER, &[coin(withdraw_amount, fund_denom.to_string())]);
    let _ = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::Withdraw {},
    )
    .unwrap();
    env.block.time = env.block.time.plus_seconds(TEST_UNBONDING_PERIOD);
    let response = execute(deps.as_mut(), env.clone(), info, ExecuteMsg::Claim {}).unwrap();
    assert_eq!(response.messages.len(), 1);
    assert_eq!(
        response.messages[0].msg,
        BankMsg::Send {
            to_address: USER.to_string(),
            amount: vec![Coin::new(2000, fund_denom)]
        }
        .into()
    );

    let pending: Vec<Claim> = from_json(
        query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::Pending {
                address: USER.to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(pending.len(), 0);
}

#[test]
fn claim_triggers_claim_from_lst_adapter_if_necessary() {
    let deposits = 1000u128;
    let lst_pending = 9000u128;
    let lst_claimable = 2000u128;
    let fund_shares = 50000u64;
    let fund_denom = get_fund_denom();
    let mut deps = mock_dependencies_with_balances(&[
        (USER, &[coin(fund_shares.into(), fund_denom.clone())]),
        (MOCK_CONTRACT_ADDR, &[coin(deposits, DEPOSIT_DENOM)]),
    ]);
    deps.querier.update_wasm(mock_wasm_querier_with_lst_adapter(
        TEST_LST_ADAPTER.to_owned(),
        lst_pending,
        lst_claimable,
    ));
    let mut env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let withdraw_amount = 10000;
    let info = mock_info(USER, &[coin(withdraw_amount, fund_denom.to_string())]);
    let _ = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::Withdraw {},
    )
    .unwrap();
    env.block.time = env.block.time.plus_seconds(TEST_UNBONDING_PERIOD);
    let response = execute(deps.as_mut(), env.clone(), info, ExecuteMsg::Claim {}).unwrap();
    assert_eq!(response.messages.len(), 2);
    assert_eq!(
        response.messages[0].msg,
        WasmMsg::Execute {
            contract_addr: TEST_LST_ADAPTER.to_string(),
            msg: to_json_binary(&LstAdapterExecuteMsg::Claim {}).unwrap(),
            funds: vec![]
        }
        .into()
    );
    assert_eq!(
        response.messages[1].msg,
        BankMsg::Send {
            to_address: USER.to_string(),
            amount: vec![Coin::new(2000, fund_denom)]
        }
        .into()
    );
}

#[test]
fn claim_fails_if_insufficent_funds_are_available() {
    let deposits = 1000u128;
    let lst_pending = 9000u128;
    let lst_claimable = 500u128;
    let fund_shares = 50000u64;
    let fund_denom = get_fund_denom();
    let mut deps = mock_dependencies_with_balances(&[
        (USER, &[coin(fund_shares.into(), fund_denom.clone())]),
        (MOCK_CONTRACT_ADDR, &[coin(deposits, DEPOSIT_DENOM)]),
    ]);
    deps.querier.update_wasm(mock_wasm_querier_with_lst_adapter(
        TEST_LST_ADAPTER.to_owned(),
        lst_pending,
        lst_claimable,
    ));
    let mut env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let withdraw_amount = 10000;
    let info = mock_info(USER, &[coin(withdraw_amount, fund_denom.to_string())]);
    let _ = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::Withdraw {},
    )
    .unwrap();
    env.block.time = env.block.time.plus_seconds(TEST_UNBONDING_PERIOD);
    let err = execute(deps.as_mut(), env.clone(), info, ExecuteMsg::Claim {}).unwrap_err();
    assert_eq!(err, ContractError::InsufficientFunds {});
}
