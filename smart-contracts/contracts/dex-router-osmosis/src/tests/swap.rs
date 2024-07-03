use crate::contract::{execute, instantiate};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::tests::initialize::{
    perform_swap, query_paths, setup_paths, ADMIN_BALANCE_AMOUNT, DENOM_BASE, DENOM_QUOTE,
    FEE_DENOM, INTERMEDIATE_QUOTE, TESTUBE_BINARY,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coin, Coin};
use osmosis_test_tube::{Module, OsmosisTestApp, SigningAccount, Wasm};
use quasar_types::error::FundsError;

use super::initialize::{
    init_test_contract, single_cl_pool_fixture, single_gamm_pool_fixture, PoolWithDenoms,
};

#[test]
fn test_swap_without_funds_throws() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = InstantiateMsg {};
    assert!(instantiate(deps.as_mut(), env.clone(), info, msg).is_ok());

    let msg = ExecuteMsg::Swap {
        path: None,
        out_denom: "test".to_string(),
        minimum_receive: None,
    };

    let info = mock_info("user", &[]);
    let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    assert_eq!(err, ContractError::Funds(FundsError::InvalidAssets(1)))
}

#[test]
fn test_swap_with_too_many_funds_throws() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = InstantiateMsg {};
    assert!(instantiate(deps.as_mut(), env.clone(), info, msg).is_ok());

    let msg = ExecuteMsg::Swap {
        path: None,
        out_denom: "test".to_string(),
        minimum_receive: None,
    };

    let info = mock_info("user", &[coin(1000, "uosmo"), coin(1000, "uatom")]);
    let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    assert_eq!(err, ContractError::Funds(FundsError::InvalidAssets(1)))
}

#[test]
#[ignore]
fn test_swap_over_single_cl_route() {
    let app = OsmosisTestApp::new();

    // Create new account with initial funds
    let admin = app
        .init_account(&[
            Coin::new(ADMIN_BALANCE_AMOUNT, FEE_DENOM),
            Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
            Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
        ])
        .unwrap();
    let wasm = Wasm::new(&app);

    let mut pools: Vec<PoolWithDenoms> = vec![];
    pools = single_gamm_pool_fixture(
        &app,
        &admin,
        vec![DENOM_BASE.to_string(), DENOM_QUOTE.to_string()],
        pools,
    );

    let contract_address = init_test_contract(&app, &admin, TESTUBE_BINARY);

    setup_paths(
        &wasm,
        &contract_address,
        vec![pools[0].pool],
        pools[0].denom0.clone(),
        pools[0].denom1.clone(),
        &admin,
    );
    let queried_path = query_paths(
        &wasm,
        &contract_address,
        pools[0].denom0.clone(),
        pools[0].denom1.clone(),
    )
    .unwrap();

    assert!(perform_swap(
        &wasm,
        &contract_address,
        pools[0].denom0.clone(),
        pools[0].denom1.clone(),
        queried_path,
        &admin
    )
    .is_ok());
}

#[test]
#[ignore]
fn test_swap_over_single_gamm_route() {
    let app = OsmosisTestApp::new();

    // Create new account with initial funds
    let admin = app
        .init_account(&[
            Coin::new(ADMIN_BALANCE_AMOUNT, FEE_DENOM),
            Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
            Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
        ])
        .unwrap();
    let wasm = Wasm::new(&app);
    let mut pools: Vec<PoolWithDenoms> = vec![];
    pools = single_gamm_pool_fixture(
        &app,
        &admin,
        vec![DENOM_BASE.to_string(), DENOM_QUOTE.to_string()],
        pools,
    );

    let contract_address = init_test_contract(&app, &admin, TESTUBE_BINARY);

    setup_paths(
        &wasm,
        &contract_address,
        vec![pools[0].pool],
        pools[0].denom0.clone(),
        pools[0].denom1.clone(),
        &admin,
    );
    let queried_path = query_paths(
        &wasm,
        &contract_address,
        pools[0].denom0.clone(),
        pools[0].denom1.clone(),
    )
    .unwrap();

    assert!(perform_swap(
        &wasm,
        &contract_address,
        pools[0].denom0.clone(),
        pools[0].denom1.clone(),
        queried_path,
        &admin
    )
    .is_ok());
}

#[test]
#[ignore]
fn test_multihop_swap() {
    // Case 1: CL / CL
    run_test_case(single_cl_pool_fixture, single_cl_pool_fixture);

    // Case 2: GAMM / GAMM
    run_test_case(single_gamm_pool_fixture, single_gamm_pool_fixture);

    // Case 3: CL / GAMM
    run_test_case(single_cl_pool_fixture, single_gamm_pool_fixture);

    // Case 4: GAMM / CL
    run_test_case(single_gamm_pool_fixture, single_cl_pool_fixture);
}

fn run_test_case(
    fixture1: fn(
        &OsmosisTestApp,
        &SigningAccount,
        Vec<String>,
        Vec<PoolWithDenoms>,
    ) -> Vec<PoolWithDenoms>,
    fixture2: fn(
        &OsmosisTestApp,
        &SigningAccount,
        Vec<String>,
        Vec<PoolWithDenoms>,
    ) -> Vec<PoolWithDenoms>,
) {
    let app = OsmosisTestApp::new();

    // Create new account with initial funds
    let admin = app
        .init_account(&[
            Coin::new(ADMIN_BALANCE_AMOUNT, FEE_DENOM),
            Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
            Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
            Coin::new(ADMIN_BALANCE_AMOUNT, INTERMEDIATE_QUOTE),
        ])
        .unwrap();
    let wasm = Wasm::new(&app);
    let mut pools: Vec<PoolWithDenoms> = vec![];
    pools = fixture1(
        &app,
        &admin,
        vec![DENOM_QUOTE.to_string(), INTERMEDIATE_QUOTE.to_string()],
        pools,
    );
    pools = fixture2(
        &app,
        &admin,
        vec![INTERMEDIATE_QUOTE.to_string(), DENOM_BASE.to_string()],
        pools,
    );

    let contract_address = init_test_contract(&app, &admin, TESTUBE_BINARY);

    setup_paths(
        &wasm,
        &contract_address,
        vec![pools[0].pool, pools[1].pool],
        pools[0].denom0.clone(),
        pools[1].denom1.clone(),
        &admin,
    );
    let queried_path = query_paths(
        &wasm,
        &contract_address,
        pools[0].denom0.clone(),
        pools[1].denom1.clone(),
    )
    .unwrap();

    assert!(perform_swap(
        &wasm,
        &contract_address,
        pools[0].denom0.clone(),
        pools[1].denom1.clone(),
        queried_path,
        &admin
    )
    .is_ok());
}
