use crate::contract::{execute, instantiate};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::tests::initialize::{
    init_test_contract, setup_paths, PoolWithDenoms, ADMIN_BALANCE_AMOUNT, DENOM_BASE, DENOM_QUOTE,
    FEE_DENOM, TESTUBE_BINARY,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::Coin;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use osmosis_test_tube::{Module, OsmosisTestApp, Wasm};

use super::initialize::single_cl_pool_fixture;

#[test]
fn test_if_not_owner_then_set_path_fails() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = InstantiateMsg {};
    assert!(instantiate(deps.as_mut(), env.clone(), info, msg).is_ok());

    let info = mock_info("user", &[]);

    let msg = ExecuteMsg::SetPath {
        offer_denom: "from".to_string(),
        ask_denom: "to".to_string(),
        path: vec![],
        bidirectional: true,
    };
    let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::Owner(mars_owner::OwnerError::NotOwner {})
    );
}

#[test]
fn test_if_path_is_empty_then_set_path_fails() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = InstantiateMsg {};
    assert!(instantiate(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());

    let msg = ExecuteMsg::SetPath {
        offer_denom: "from".to_string(),
        ask_denom: "to".to_string(),
        path: vec![],
        bidirectional: true,
    };
    let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    assert_eq!(err, ContractError::EmptyPath {});
}

#[test]
#[ignore]
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
