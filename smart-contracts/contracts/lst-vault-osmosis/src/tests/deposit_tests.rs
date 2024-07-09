use crate::contract::{execute, instantiate};
use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::tests::util::{
    get_fund_denom, get_init_msg, mock_wasm_querier_with_lst_adapter, CREATOR, DEPOSIT_DENOM,
    TEST_LST_ADAPTER,
};
use cosmwasm_std::coin;
use cosmwasm_std::testing::{
    mock_dependencies, mock_dependencies_with_balances, mock_env, mock_info, MOCK_CONTRACT_ADDR,
};
use osmosis_std::types::cosmos::base::v1beta1::Coin as ProtoCoin;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMint;
use quasar_types::error::FundsError;

#[test]
fn deposits_with_wrong_denom_fails() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let info = mock_info(CREATOR, &[coin(1, "other_denom".to_string())]);
    let err = execute(deps.as_mut(), env, info, ExecuteMsg::Deposit {}).unwrap_err();
    assert_eq!(
        err,
        ContractError::Funds(FundsError::WrongDenom(DEPOSIT_DENOM.to_string()))
    );
}

#[test]
fn deposits_with_more_than_one_asset_fails() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let info = mock_info(
        CREATOR,
        &[
            coin(1, DEPOSIT_DENOM.to_string()),
            coin(1, "other_denom".to_string()),
        ],
    );
    let err = execute(deps.as_mut(), env, info, ExecuteMsg::Deposit {}).unwrap_err();
    assert_eq!(err, ContractError::Funds(FundsError::InvalidAssets(1)));
}

#[test]
fn first_successful_deposit_mints_fund_tokens_according_to_first_provided_asset() {
    let mut deps = mock_dependencies();
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

    let deposit_amount = 1;
    let depositor = "depositor";
    let info = mock_info(
        depositor,
        &[coin(deposit_amount, DEPOSIT_DENOM.to_string())],
    );
    let response = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::Deposit {},
    )
    .unwrap();
    assert_eq!(response.messages.len(), 1);

    let fund_denom = get_fund_denom();
    assert_eq!(
        response.messages[0].msg,
        MsgMint {
            sender: env.contract.address.to_string(),
            amount: Some(ProtoCoin {
                amount: deposit_amount.to_string(),
                denom: fund_denom.clone(),
            }),
            mint_to_address: depositor.to_string(),
        }
        .into()
    );
}

#[test]
fn second_successful_deposit_mints_fund_tokens_according_to_share_of_assets() {
    let deposits = 10000u128;
    let env = mock_env();
    let fund_shares = 50000u64;
    let fund_denom = get_fund_denom();
    let mut deps = mock_dependencies_with_balances(&[
        ("some_wallet", &[coin(fund_shares.into(), fund_denom)]),
        (MOCK_CONTRACT_ADDR, &[coin(deposits, DEPOSIT_DENOM)]),
    ]);

    deps.querier.update_wasm(mock_wasm_querier_with_lst_adapter(
        TEST_LST_ADAPTER.to_owned(),
        0,
        0,
    ));

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let deposit_amount = 5000;
    let depositor = "depositor";
    let info = mock_info(
        depositor,
        &[coin(deposit_amount, DEPOSIT_DENOM.to_string())],
    );
    let response = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::Deposit {},
    )
    .unwrap();
    assert_eq!(response.messages.len(), 1);

    let expected_minted_tokens = 25000;
    let fund_denom = get_fund_denom();
    assert_eq!(
        response.messages[0].msg,
        MsgMint {
            sender: env.contract.address.to_string(),
            amount: Some(ProtoCoin {
                amount: expected_minted_tokens.to_string(),
                denom: fund_denom.clone(),
            }),
            mint_to_address: depositor.to_string(),
        }
        .into()
    );
}
