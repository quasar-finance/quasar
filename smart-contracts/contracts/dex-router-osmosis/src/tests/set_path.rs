use crate::contract::{execute, instantiate};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

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

