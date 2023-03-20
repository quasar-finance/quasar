use cosmwasm_std::{Addr, DepsMut, Reply, Response, SubMsgResponse};
use cosmwasm_std::{StdError, Uint128};
use quasar_types::callback::{BondResponse, Callback};

use crate::error::{ContractError, Trap};
use crate::helpers::{parse_seq, unlock_on_error, ContractCallback, IbcMsgKind};
use crate::state::{
    FundPath, CLAIMABLE_FUNDS, PENDING_ACK, REPLIES, RETURNING, TRAPS, UNBONDING_CLAIMS,
};

pub fn handle_ibc_reply(
    deps: DepsMut,
    msg: Reply,
    pending: IbcMsgKind,
) -> Result<Response, ContractError> {
    let data = msg
        .result
        .into_result()
        .map_err(|msg| StdError::GenericErr {
            msg: format!("submsg error: {msg:?}"),
        })?
        .data
        .ok_or(ContractError::NoReplyData)
        .map_err(|_| StdError::NotFound {
            kind: "reply-data".to_string(),
        })?;

    let seq = parse_seq(data).map_err(|err| StdError::SerializeErr {
        source_type: "protobuf-decode".to_string(),
        msg: err.to_string(),
    })?;

    PENDING_ACK.save(deps.storage, seq, &pending)?;

    // cleanup the REPLIES state item
    REPLIES.remove(deps.storage, msg.id);

    Ok(Response::default()
        .add_attribute("pending-msg", seq.to_string())
        .add_attribute("step", format!("{pending:?}")))
}

pub fn handle_ack_reply(deps: DepsMut, msg: Reply, seq: u64) -> Result<Response, ContractError> {
    let mut resp = Response::new();

    // if we have an error in our Ack execution, the submsg saves the error in TRAPS and (should) rollback
    // the entire state of the ack execution,
    if let Err(error) = msg.result.into_result() {
        let step = PENDING_ACK.load(deps.storage, seq)?;
        unlock_on_error(deps.storage, &step)?;

        // reassignment needed since add_attribute
        resp = resp.add_attribute("trapped-error", error.as_str());

        TRAPS.save(
            deps.storage,
            seq,
            &Trap {
                error,
                step,
                last_succesful: true,
            },
        )?;
    }

    // // cleanup the REPLIES state item
    REPLIES.remove(deps.storage, msg.id);
    Ok(resp)
}

pub fn handle_callback_reply(
    deps: DepsMut,
    msg: Reply,
    callback: ContractCallback,
) -> Result<Response, ContractError> {
    // TODO: if error, add manual withdraws to lp-strategy
    //
    // create in claimable_funds map... Addr, unbond_id -> amount
    // in Callback contract add callbacl(callback, amount)
    let mut res = Response::new();

    if let Err(error) = msg.result.into_result() {
        match callback {
            ContractCallback::Callback {
                callback,
                amount,
                owner,
            } => match callback {
                Callback::UnbondResponse(ur) => {
                    let fund_path = FundPath::Unbond { id: ur.unbond_id };
                    match amount {
                        Some(amount) => {
                            let amt = amount;
                            Ok(amt)
                        }
                        // TODO: final release should not return an error but log
                        None => Err(ContractError::CallbackHasNoAmount {}),
                    }?;
                    CLAIMABLE_FUNDS.save(deps.storage, (owner, fund_path), &amount)?;
                    res.add_attribute("unbond-callback-error", error.as_str());
                }
                _ => {}
            },
            _ => {}
        }
    }

    // Q: should we handle the callback bank message?

    // cleanup the REPLIES state item
    REPLIES.remove(deps.storage, msg.id);
    Ok(res)
}

// test handle callback reply

#[cfg(test)]
#[test]
fn test_handle_callback_reply_is_err() {
    use cosmwasm_std::{testing::mock_dependencies, SubMsgResult};
    use quasar_types::callback::UnbondResponse;

    use crate::helpers::SubMsgKind;

    let mut deps = mock_dependencies();
    let owner = Addr::unchecked("owner");

    let contract_callback = ContractCallback::Callback {
        callback: Callback::UnbondResponse(UnbondResponse {
            unbond_id: "unbond_id".to_string(),
        }),
        amount: Some(Uint128::new(100)),
        owner: owner.to_string(),
    };
    let msg = Reply {
        id: 1,
        result: SubMsgResult::Err("error".to_string()),
    };

    // mocking replies
    REPLIES.save(
        &mut deps.storage,
        msg.id,
        &SubMsgKind::Callback(contract_callback.clone()),
    );

    let res = handle_callback_reply(deps.as_mut(), msg, contract_callback).unwrap();
    assert_eq!(res.attributes.len(), 1);
    assert_eq!(res.attributes[0].key, "callback-error");
    assert_eq!(res.attributes[0].value, "error");

    assert_eq!(
        CLAIMABLE_FUNDS
            .load(
                deps.storage,
                (
                    owner,
                    FundPath::Unbond {
                        id: "unbond_id".to_string(),
                    },
                ),
            )
            .unwrap(),
        Uint128::new(100)
    );

    // after cleanup it should be empty
    assert!(REPLIES.load(&deps.storage, msg.id).is_err());
}

fn test_handle_callback_reply_is_err_empty_amount() {
    use cosmwasm_std::{testing::mock_dependencies, SubMsgResult};
    use quasar_types::callback::UnbondResponse;

    let mut deps = mock_dependencies();
    let owner = Addr::unchecked("owner");

    let contract_callback = ContractCallback::Callback {
        callback: Callback::UnbondResponse(UnbondResponse {
            unbond_id: "unbond_id".to_string(),
        }),
        amount: None,
        owner: owner.to_string(),
    };
    let msg = Reply {
        id: 1,
        result: SubMsgResult::Err("error".to_string()),
    };

    let res = handle_callback_reply(deps.as_mut(), msg, contract_callback).unwrap_err();
    assert_eq!(res, ContractError::CallbackHasNoAmount {});
}

fn test_handle_callback_reply_is_ok() {
    use crate::helpers::SubMsgKind;
    use cosmwasm_std::{testing::mock_dependencies, SubMsgResult};
    use quasar_types::callback::UnbondResponse;

    let mut deps = mock_dependencies();
    let owner = Addr::unchecked("owner");

    let contract_callback = ContractCallback::Callback {
        callback: Callback::UnbondResponse(UnbondResponse {
            unbond_id: "unbond_id".to_string(),
        }),
        amount: Some(Uint128::new(100)),
        owner: owner.to_string(),
    };
    let msg = Reply {
        id: 1,
        result: SubMsgResult::Ok(SubMsgResponse {
            data: None,
            events: vec![],
        }),
    };

    // mocking replies
    REPLIES.save(
        &mut deps.storage,
        msg.id,
        &SubMsgKind::Callback(contract_callback.clone()),
    );

    let res = handle_callback_reply(deps.as_mut(), msg, contract_callback).unwrap();
    assert_eq!(res.attributes.len(), 0);

    // after cleanup it should be empty
    assert!(REPLIES.is_empty(&deps.storage));
}
