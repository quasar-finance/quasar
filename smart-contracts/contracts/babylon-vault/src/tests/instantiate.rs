use crate::tests::setup::{OWNER, SUBDENOM, USER};
use crate::{contract::instantiate, msg::InstantiateMsg};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    let result = instantiate(
        deps.as_mut(),
        env.clone(),
        info,
        InstantiateMsg {
            owner: OWNER.to_string(),
            subdenom: SUBDENOM.to_string(),
        },
    );
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.messages.len(), 1);
    assert_eq!(
        response.messages[0].msg,
        MsgCreateDenom {
            sender: env.contract.address.to_string(),
            subdenom: SUBDENOM.to_string(),
        }
        .into()
    );
}
