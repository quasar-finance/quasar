use crate::contract::{execute, instantiate};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use cosmwasm_std::coin;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use quasar_types::error::FundsError;

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
        to: None,
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
        to: None,
    };

    let info = mock_info("user", &[coin(1000, "uosmo"), coin(1000, "uatom")]);
    let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    assert_eq!(err, ContractError::Funds(FundsError::InvalidAssets(1)))
}
