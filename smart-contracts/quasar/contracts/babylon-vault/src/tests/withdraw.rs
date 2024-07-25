use cosmwasm_std::{
    coin, coins, from_json,
    testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR},
    BankMsg, Decimal, Uint128,
};
use quasar_std::quasarlabs::quasarnode::tokenfactory::v1beta1::MsgBurn;

use crate::{
    contract::{execute, query},
    msg::{ExecuteMsg, QueryMsg},
    tests::setup::{
        mock_wasm_querier, setup, setup_with_balances, DEPOSIT_DENOM, OTHER_DEPOSIT_DENOM, OWNER,
        USER, VAULT_DENOM,
    },
    VaultError,
};

#[test]
fn withdraw_with_wrong_denom_fails() {
    let mut deps = setup();
    let env = mock_env();

    let info = mock_info(USER, &coins(1, DEPOSIT_DENOM));
    let err = execute(deps.as_mut(), env, info, ExecuteMsg::Withdraw {}).unwrap_err();
    assert_eq!(
        err,
        VaultError::DenomNotFound {
            denom: DEPOSIT_DENOM.to_string()
        }
    );
}

#[test]
fn withdraw_without_funds_fails() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    let err = execute(deps.as_mut(), env, info, ExecuteMsg::Withdraw {}).unwrap_err();
    assert_eq!(err, VaultError::InvalidFunds {});
}

#[test]
fn test_withdrawal() {
    let deposits = 10000u128;
    let fund_shares = 50000u64;

    let mut deps = setup_with_balances(&[
        (USER, &coins(fund_shares.into(), VAULT_DENOM)),
        (MOCK_CONTRACT_ADDR, &coins(deposits, DEPOSIT_DENOM)),
    ]);
    deps.querier.update_wasm(mock_wasm_querier(
        "oracle".to_string(),
        Decimal::percent(123),
        Decimal::percent(123),
    ));

    let env = mock_env();
    let info = mock_info(OWNER, &[]);
    assert!(execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::RegisterLst {
            denom: DEPOSIT_DENOM.to_string(),
            interface: "interface".to_string(),
        }
    )
    .is_ok());

    let value: Uint128 =
        from_json(query(deps.as_ref(), env.clone(), QueryMsg::Value {}).unwrap()).unwrap();
    assert_eq!(value, Uint128::from(12300u128));

    let withdraw_amount = 10000;
    let info = mock_info(USER, &[coin(withdraw_amount, VAULT_DENOM.to_string())]);
    let response = execute(deps.as_mut(), env.clone(), info, ExecuteMsg::Withdraw {}).unwrap();
    assert_eq!(response.messages.len(), 2);
    assert_eq!(
        response.messages[0].msg,
        MsgBurn {
            sender: MOCK_CONTRACT_ADDR.to_string(),
            amount: Some(cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
                amount: withdraw_amount.to_string(),
                denom: VAULT_DENOM.to_string(),
            }),
            burn_from_address: MOCK_CONTRACT_ADDR.to_string(),
        }
        .into()
    );
    assert_eq!(
        response.messages[1].msg,
        BankMsg::Send {
            to_address: USER.to_string(),
            amount: coins(2000, DEPOSIT_DENOM)
        }
        .into()
    );
}

#[test]
fn test_withdrawal_with_two_registered_lsts() {
    let deposits = 10000u128;
    let other_deposits = 20000u128;
    let fund_shares = 50000u64;

    let mut deps = setup_with_balances(&[
        (USER, &coins(fund_shares.into(), VAULT_DENOM)),
        (
            MOCK_CONTRACT_ADDR,
            &[
                coin(deposits, DEPOSIT_DENOM),
                coin(other_deposits, OTHER_DEPOSIT_DENOM),
            ],
        ),
    ]);
    deps.querier.update_wasm(mock_wasm_querier(
        "oracle".to_string(),
        Decimal::percent(123),
        Decimal::percent(246),
    ));

    let env = mock_env();
    let info = mock_info(OWNER, &[]);
    assert!(execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::RegisterLst {
            denom: DEPOSIT_DENOM.to_string(),
            interface: "interface".to_string(),
        }
    )
    .is_ok());
    assert!(execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::RegisterLst {
            denom: OTHER_DEPOSIT_DENOM.to_string(),
            interface: "interface".to_string(),
        }
    )
    .is_ok());

    let value: Uint128 =
        from_json(query(deps.as_ref(), env.clone(), QueryMsg::Value {}).unwrap()).unwrap();
    assert_eq!(value, Uint128::from(61500u128));

    let withdraw_amount = 10000;
    let info = mock_info(USER, &[coin(withdraw_amount, VAULT_DENOM.to_string())]);
    let response = execute(deps.as_mut(), env.clone(), info, ExecuteMsg::Withdraw {}).unwrap();
    assert_eq!(response.messages.len(), 2);
    assert_eq!(
        response.messages[0].msg,
        MsgBurn {
            sender: MOCK_CONTRACT_ADDR.to_string(),
            amount: Some(cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
                amount: withdraw_amount.to_string(),
                denom: VAULT_DENOM.to_string(),
            }),
            burn_from_address: MOCK_CONTRACT_ADDR.to_string(),
        }
        .into()
    );
    assert_eq!(
        response.messages[1].msg,
        BankMsg::Send {
            to_address: USER.to_string(),
            amount: vec![coin(2000, DEPOSIT_DENOM), coin(4000, OTHER_DEPOSIT_DENOM)]
        }
        .into()
    );
}
