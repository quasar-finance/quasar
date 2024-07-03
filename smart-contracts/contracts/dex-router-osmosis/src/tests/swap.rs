use crate::contract::{execute, instantiate};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coin, Coin, Uint128};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use osmosis_test_tube::{Module, Wasm};
use quasar_types::error::FundsError;

use super::initialize::{single_cl_pool_fixture, single_gamm_pool_fixture};

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
fn test_swap_over_singl_cl_route() {
    let (app, contract_address, pools, admin) = single_cl_pool_fixture();
    let wasm = Wasm::new(&app);

    for pool in pools.clone() {
        let _ = wasm
            .execute(
                &contract_address.to_string(),
                &ExecuteMsg::SetPath {
                    path: vec![pool.pool],
                    bidirectional: true,
                    offer_denom: pool.denom0.clone(),
                    ask_denom: pool.denom1.clone(),
                },
                &[],
                &admin,
            )
            .unwrap();
    }

    let resp: Vec<Vec<SwapAmountInRoute>> = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::PathsForPair {
                offer_denom: pools.first().unwrap().denom0.clone(),
                ask_denom: pools.first().unwrap().denom1.clone(),
            },
        )
        .unwrap();

    let _ = wasm
        .execute(
            &contract_address.to_string(),
            &ExecuteMsg::Swap {
                out_denom: pools.first().unwrap().denom1.clone(),
                path: Some(resp.first().unwrap().clone()),
                minimum_receive: Some(Uint128::new(9500)),
            },
            &[Coin::new(10000u128, pools.first().unwrap().denom0.clone())],
            &admin,
        )
        .unwrap();
}

#[test]
#[ignore]
fn test_swap_over_singl_gamm_route() {
    let (app, contract_address, pools, admin) = single_gamm_pool_fixture();
    let wasm = Wasm::new(&app);

    for pool in pools.clone() {
        let _ = wasm
            .execute(
                &contract_address.to_string(),
                &ExecuteMsg::SetPath {
                    path: vec![pool.pool],
                    bidirectional: true,
                    offer_denom: pool.denom0.clone(),
                    ask_denom: pool.denom1.clone(),
                },
                &[],
                &admin,
            )
            .unwrap();
    }

    let resp: Vec<Vec<SwapAmountInRoute>> = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::PathsForPair {
                offer_denom: pools.first().unwrap().denom0.clone(),
                ask_denom: pools.first().unwrap().denom1.clone(),
            },
        )
        .unwrap();

    let _ = wasm
        .execute(
            &contract_address.to_string(),
            &ExecuteMsg::Swap {
                out_denom: pools.first().unwrap().denom1.clone(),
                path: Some(resp.first().unwrap().clone()),
                minimum_receive: Some(Uint128::new(9500)),
            },
            &[Coin::new(10000u128, pools.first().unwrap().denom0.clone())],
            &admin,
        )
        .unwrap();
}
