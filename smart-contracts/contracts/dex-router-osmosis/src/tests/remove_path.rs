use crate::contract::{execute, instantiate};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::PATHS;
use crate::tests::initialize::default_init;
use crate::ContractError;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use osmosis_test_tube::{Module, Wasm};

#[test]
fn test_if_not_owner_then_remove_path_fails() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = InstantiateMsg {};
    assert!(instantiate(deps.as_mut(), env.clone(), info, msg).is_ok());

    let info = mock_info("user", &[]);

    let msg = ExecuteMsg::RemovePath {
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
fn test_if_path_is_empty_then_remove_path_fails() {
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
fn test_remove_path_bidirectional_fails_if_reverse_path_does_not_exist() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = InstantiateMsg {};
    let offer_denom = "from".to_string();
    let ask_denom = "to".to_string();
    assert!(instantiate(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let key = (offer_denom.clone(), ask_denom.clone());
    let path = vec![
        SwapAmountInRoute {
            pool_id: 0,
            token_out_denom: "token0".to_string(),
        },
        SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: ask_denom.clone(),
        },
    ];
    PATHS
        .save(deps.as_mut().storage, key.clone(), &vec![path.clone()])
        .unwrap();

    let msg = ExecuteMsg::RemovePath {
        offer_denom: offer_denom.clone(),
        ask_denom: ask_denom.clone(),
        path: path.into_iter().map(|route| route.pool_id).collect(),
        bidirectional: true,
    };
    let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NoPathFound {
            offer: ask_denom,
            ask: offer_denom
        }
    );
}

#[test]
fn test_remove_path_bidirectional() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = InstantiateMsg {};
    let offer_denom = "from".to_string();
    let ask_denom = "to".to_string();
    assert!(instantiate(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let key = (offer_denom.clone(), ask_denom.clone());
    let key_rev = (ask_denom.clone(), offer_denom.clone());
    let path = vec![
        SwapAmountInRoute {
            pool_id: 0,
            token_out_denom: "token0".to_string(),
        },
        SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: ask_denom.clone(),
        },
    ];
    let path_rev = vec![
        SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: "token0".to_string(),
        },
        SwapAmountInRoute {
            pool_id: 0,
            token_out_denom: offer_denom.clone(),
        },
    ];
    PATHS
        .save(deps.as_mut().storage, key.clone(), &vec![path.clone()])
        .unwrap();
    PATHS
        .save(
            deps.as_mut().storage,
            key_rev.clone(),
            &vec![path_rev.clone()],
        )
        .unwrap();

    let msg = ExecuteMsg::RemovePath {
        offer_denom: offer_denom.clone(),
        ask_denom: ask_denom.clone(),
        path: path.into_iter().map(|route| route.pool_id).collect(),
        bidirectional: true,
    };
    assert!(execute(deps.as_mut(), env, info, msg).is_ok());
    assert_eq!(PATHS.may_load(deps.as_mut().storage, key).unwrap(), None);
}

#[test]
fn test_remove_path() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = InstantiateMsg {};
    let offer_denom = "from".to_string();
    let ask_denom = "to".to_string();
    assert!(instantiate(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let key = (offer_denom.clone(), ask_denom.clone());
    let path = vec![
        SwapAmountInRoute {
            pool_id: 0,
            token_out_denom: "token0".to_string(),
        },
        SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: ask_denom.clone(),
        },
    ];
    PATHS
        .save(deps.as_mut().storage, key.clone(), &vec![path.clone()])
        .unwrap();

    let msg = ExecuteMsg::RemovePath {
        offer_denom,
        ask_denom,
        path: path.into_iter().map(|route| route.pool_id).collect(),
        bidirectional: false,
    };
    assert!(execute(deps.as_mut(), env, info, msg).is_ok());
    assert_eq!(PATHS.may_load(deps.as_mut().storage, key).unwrap(), None);
}

#[test]
fn test_remove_one_of_two_paths() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = InstantiateMsg {};
    let offer_denom = "from".to_string();
    let ask_denom = "to".to_string();
    assert!(instantiate(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let key = (offer_denom.clone(), ask_denom.clone());
    let path1 = vec![
        SwapAmountInRoute {
            pool_id: 0,
            token_out_denom: "token0".to_string(),
        },
        SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: ask_denom.clone(),
        },
    ];
    let path2 = vec![
        SwapAmountInRoute {
            pool_id: 2,
            token_out_denom: "token1".to_string(),
        },
        SwapAmountInRoute {
            pool_id: 3,
            token_out_denom: ask_denom.clone(),
        },
    ];
    PATHS
        .save(
            deps.as_mut().storage,
            key.clone(),
            &vec![path1.clone(), path2.clone()],
        )
        .unwrap();

    let msg = ExecuteMsg::RemovePath {
        offer_denom,
        ask_denom,
        path: path1.into_iter().map(|route| route.pool_id).collect(),
        bidirectional: false,
    };
    assert!(execute(deps.as_mut(), env, info, msg).is_ok());
    assert_eq!(
        PATHS.may_load(deps.as_mut().storage, key).unwrap(),
        Some(vec![path2])
    );
}

#[test]
fn test_set_and_remove_path() {
    let (app, contract_address, pools, admin) = default_init();
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

    let _ = wasm
        .execute(
            &contract_address.to_string(),
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
    let expected_path_to_remove = SwapAmountInRoute {
        pool_id: pools.first().unwrap().pool,
        token_out_denom: pools.first().unwrap().denom1.clone(),
    };

    let paths_contain_expected = resp
        .iter()
        .any(|path| path.contains(&expected_path_to_remove));

    assert_eq!(paths_contain_expected, false);
}
