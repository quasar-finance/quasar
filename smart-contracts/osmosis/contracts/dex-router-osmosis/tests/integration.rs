pub mod initialize;
use cosmwasm_std::{Coin, Uint128};
use dex_router_osmosis::msg::{BestPathForPairResponse, ExecuteMsg, QueryMsg};
use initialize::{
    init_test_contract, perform_swap, query_paths, setup_paths, single_cl_pool_fixture,
    single_gamm_pool_fixture, PoolWithDenoms, ADMIN_BALANCE_AMOUNT, DENOM_BASE, DENOM_QUOTE,
    FEE_DENOM, INTERMEDIATE_QUOTE, TESTUBE_BINARY,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use osmosis_test_tube::{Module, OsmosisTestApp, SigningAccount, Wasm};

#[test]
fn default_init_works() {
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

    let resp: BestPathForPairResponse = wasm
        .query(
            contract_address.as_ref(),
            &QueryMsg::BestPathForPair {
                offer: Coin::new(
                    Uint128::from(100000000u128).into(),
                    pools.first().unwrap().denom0.clone(),
                ),
                ask_denom: pools.first().unwrap().denom1.clone(),
            },
        )
        .unwrap();

    let mut iter = resp.path.into_iter();
    // under the default setup, we expect the best path to route over pool 1
    assert_eq!(iter.next().unwrap().pool_id, 1);
    assert!(iter.next().is_none());
}

#[test]
fn test_set_path_works() {
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
    pools = single_cl_pool_fixture(
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

    let resp: Vec<Vec<SwapAmountInRoute>> = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::PathsForPair {
                offer_denom: pools.first().unwrap().denom0.clone(),
                ask_denom: pools.first().unwrap().denom1.clone(),
            },
        )
        .unwrap();

    // Assert that the set path is included in the response
    let expected_path = SwapAmountInRoute {
        pool_id: pools.first().unwrap().pool,
        token_out_denom: pools.first().unwrap().denom1.clone(),
    };

    let paths_contain_expected = resp.iter().any(|path| path.contains(&expected_path));

    assert!(
        paths_contain_expected,
        "Expected path was not found in the response"
    );
}

#[test]
fn test_set_and_remove_path() {
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
    let _ = wasm
        .execute(
            contract_address.as_ref(),
            &ExecuteMsg::RemovePath {
                path: vec![pools.first().unwrap().pool],
                bidirectional: true,
                offer_denom: pools.first().unwrap().denom0.clone(),
                ask_denom: pools.first().unwrap().denom1.clone(),
            },
            &[],
            &admin,
        )
        .unwrap();

    let resp: Result<Vec<Vec<SwapAmountInRoute>>, osmosis_test_tube::RunnerError> = wasm.query(
        contract_address.as_str(),
        &QueryMsg::PathsForPair {
            offer_denom: pools.first().unwrap().denom0.clone(),
            ask_denom: pools.first().unwrap().denom1.clone(),
        },
    );

    assert!(resp.is_err(), "Path not found");
}

#[test]
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
