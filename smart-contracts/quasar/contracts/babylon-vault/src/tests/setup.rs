use crate::{contract::instantiate, msg::InstantiateMsg};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Empty, OwnedDeps,
};

pub const OWNER: &str = "owner";
pub const USER: &str = "user";

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    assert!(instantiate(
        deps.as_mut(),
        env,
        info,
        InstantiateMsg {
            owner: OWNER.to_string(),
        },
    )
    .is_ok());
    deps
}
