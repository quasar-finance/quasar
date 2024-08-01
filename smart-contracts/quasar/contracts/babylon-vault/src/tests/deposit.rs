use cosmwasm_std::{
    coin, coins,
    testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR},
    Decimal,
};
use quasar_std::quasarlabs::quasarnode::tokenfactory::v1beta1::MsgMint;

use crate::{
    contract::execute,
    msg::ExecuteMsg,
    tests::setup::{
        mock_wasm_querier, setup, setup_with_balances, DEPOSIT_DENOM, OTHER_DEPOSIT_DENOM, OWNER,
        USER, VAULT_DENOM,
    },
    VaultError,
};

#[test]
fn deposits_with_wrong_denom_fails() {
    let mut deps = setup();
    let env = mock_env();

    let info = mock_info(USER, &coins(1, DEPOSIT_DENOM));
    let err = execute(deps.as_mut(), env, info, ExecuteMsg::Deposit {}).unwrap_err();
    assert_eq!(
        err,
        VaultError::DenomNotFound {
            denom: DEPOSIT_DENOM.to_string()
        }
    );
}

#[test]
fn deposits_without_funds_fails() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    let err = execute(deps.as_mut(), env, info, ExecuteMsg::Deposit {}).unwrap_err();
    assert_eq!(err, VaultError::InvalidFunds {});
}

#[test]
fn deposits_with_more_than_one_asset_fails() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(
        USER,
        &[coin(1, DEPOSIT_DENOM), coin(1, OTHER_DEPOSIT_DENOM)],
    );

    let err = execute(deps.as_mut(), env, info, ExecuteMsg::Deposit {}).unwrap_err();
    assert_eq!(err, VaultError::InvalidFunds {});
}

#[test]
fn first_successful_deposit_mints_fund_tokens_according_to_first_provided_asset() {
    let mut deps = setup();
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

    let deposit_amount = 1;
    let info = mock_info(USER, &[coin(deposit_amount, DEPOSIT_DENOM.to_string())]);
    let response = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::Deposit {},
    )
    .unwrap();
    assert_eq!(response.messages.len(), 1);

    assert_eq!(
        response.messages[0].msg,
        MsgMint {
            sender: env.contract.address.to_string(),
            amount: Some(cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
                amount: deposit_amount.to_string(),
                denom: VAULT_DENOM.to_string(),
            }),
            mint_to_address: USER.to_string(),
        }
        .into()
    );
}

#[test]
fn second_successful_deposit_mints_fund_tokens_according_to_share_of_assets() {
    let deposits = 10000u128;
    let fund_shares = 50000u64;
    let new_deposit = 2500;

    let mut deps = setup_with_balances(&[
        ("some_wallet", &[coin(fund_shares.into(), VAULT_DENOM)]),
        (
            MOCK_CONTRACT_ADDR,
            &[
                coin(deposits, DEPOSIT_DENOM),
                coin(new_deposit, OTHER_DEPOSIT_DENOM),
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

    let info = mock_info(USER, &[coin(new_deposit, OTHER_DEPOSIT_DENOM.to_string())]);
    let response = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::Deposit {},
    )
    .unwrap();
    assert_eq!(response.messages.len(), 1);

    let expected_minted_tokens = 25000;
    assert_eq!(
        response.messages[0].msg,
        MsgMint {
            sender: env.contract.address.to_string(),
            amount: Some(cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
                amount: expected_minted_tokens.to_string(),
                denom: VAULT_DENOM.to_string(),
            }),
            mint_to_address: USER.to_string(),
        }
        .into()
    );
}
