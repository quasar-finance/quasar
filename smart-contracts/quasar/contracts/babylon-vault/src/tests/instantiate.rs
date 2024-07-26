use crate::tests::setup::{OWNER, SUBDENOM, USER};
use crate::{
    contract::{instantiate, reply, CREATE_DENOM_REPLY_ID},
    msg::InstantiateMsg,
    state::VAULT_DENOM,
};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Reply, SubMsgResponse, SubMsgResult,
};
use prost::Message;
use quasar_std::quasarlabs::quasarnode::tokenfactory::v1beta1::{
    MsgCreateDenom, MsgCreateDenomResponse,
};

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

    let new_token = "new_token".to_string();

    assert!(reply(
        deps.as_mut(),
        env,
        Reply {
            id: CREATE_DENOM_REPLY_ID,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: Some(
                    MsgCreateDenomResponse {
                        new_token_denom: new_token.clone(),
                    }
                    .encode_to_vec()
                    .into()
                ),
            })
        }
    )
    .is_ok());

    let vault_token = VAULT_DENOM.load(deps.as_ref().storage).unwrap();
    assert_eq!(vault_token, new_token);
}
