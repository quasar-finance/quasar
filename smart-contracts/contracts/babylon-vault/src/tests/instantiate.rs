use crate::tests::setup::{OWNER, USER};
use crate::{contract::instantiate, msg::InstantiateMsg};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    let result = instantiate(
        deps.as_mut(),
        env,
        info,
        InstantiateMsg {
            owner: OWNER.to_string(),
        },
    );
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.messages.len(), 0);
}
