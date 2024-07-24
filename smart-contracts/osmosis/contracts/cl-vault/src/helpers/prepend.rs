use cosmwasm_std::{to_json_binary, CosmosMsg, Env, Response, SubMsg};

use crate::{msg::ExecuteMsg, ContractError};

/// Prepends a callback to the contract to claim any rewards, used to
/// enforce the claiming of rewards before any action that might
/// cause Osmosis to collect rewards anyway, such as fully withdrawing a position
/// or adding funds into a position
pub fn prepend_claim_msg(env: &Env, response: Response) -> Result<Response, ContractError> {
    let claim_msg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_json_binary(&ExecuteMsg::VaultExtension(
            crate::msg::ExtensionExecuteMsg::CollectRewards {},
        ))?,
        funds: vec![],
    });

    Ok(prepend_msg(response, SubMsg::new(claim_msg)))
}

/// Prepend a msg to the start of the messages in a response
fn prepend_msg(mut response: Response, msg: SubMsg) -> Response {
    response.messages.splice(0..0, vec![msg]);
    response
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, testing::mock_env, BankMsg};

    use super::*;

    #[test]
    fn test_prepend_msg_with_empty_response() {
        let response = Response::default();
        let msg = CosmosMsg::Bank(BankMsg::Burn {
            amount: vec![coin(100, "stake")],
        });

        let updated_response = prepend_msg(response, SubMsg::new(msg.clone()));
        assert_eq!(updated_response.messages.len(), 1);
        assert_eq!(updated_response.messages[0].msg, msg);
    }

    #[test]
    fn test_prepend_msg_with_non_empty_response() {
        let existing_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: "bob".to_string(),
            amount: vec![coin(100, "stake")],
        });
        let new_msg = CosmosMsg::Bank(BankMsg::Burn {
            amount: vec![coin(100, "stake")],
        });

        let response = Response::new().add_message(existing_msg.clone());

        let updated_response = prepend_msg(response.clone(), SubMsg::new(new_msg.clone()));
        assert_eq!(updated_response.messages.len(), 2);
        assert_eq!(updated_response.messages[0].msg, new_msg);
        assert_eq!(updated_response.messages[1].msg, existing_msg);
    }

    #[test]
    fn test_prepend_claim_msg_normal_operation() {
        let env = mock_env();
        let msg = CosmosMsg::Bank(BankMsg::Burn {
            amount: vec![coin(100, "stake")],
        });
        let response = Response::new().add_message(msg.clone());

        let claim_msg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_json_binary(&ExecuteMsg::VaultExtension(
                crate::msg::ExtensionExecuteMsg::CollectRewards {},
            ))
            .unwrap(),
            funds: vec![],
        });

        let updated_response = prepend_claim_msg(&env, response).unwrap();
        assert_eq!(updated_response.messages.len(), 2);
        assert_eq!(updated_response.messages[0].msg, claim_msg);
        assert_eq!(updated_response.messages[1].msg, msg);
    }
}
