use crate::tests::setup::{OWNER, SUBDENOM, USER, VAULT_DENOM};
use crate::{
    contract::{instantiate, query, reply, CREATE_DENOM_REPLY_ID},
    msg::{InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{
    from_json,
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

    assert!(reply(
        deps.as_mut(),
        env.clone(),
        Reply {
            id: CREATE_DENOM_REPLY_ID,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: Some(
                    MsgCreateDenomResponse {
                        new_token_denom: VAULT_DENOM.to_string(),
                    }
                    .encode_to_vec()
                    .into()
                ),
            })
        }
    )
    .is_ok());

    let vault_denom =
        from_json::<String>(&query(deps.as_ref(), env, QueryMsg::Denom {}).unwrap()).unwrap();
    assert_eq!(vault_denom, VAULT_DENOM);
}
