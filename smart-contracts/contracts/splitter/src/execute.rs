use cosmwasm_std::{attr, coin, Attribute, BankMsg, CosmosMsg, Deps, Env, Fraction, Response};

use crate::{state::RECEIVERS, ContractError};
#[cfg(claim)]
use crate::msg::Claim;

/// Split the current contract balance between all receivers
pub fn execute_split(deps: Deps, env: Env) -> Result<Response, ContractError> {
    let receivers = RECEIVERS.load(deps.storage)?;
    let balance = deps.querier.query_all_balances(env.contract.address)?;

    let to_send = receivers
        .iter()
        .map(|recv| {
            (
                recv.address.as_str(),
                balance.iter().map(|c| {
                    coin(
                        c.amount
                            .multiply_ratio(recv.share.numerator(), recv.share.denominator())
                            .u128(),
                        c.denom.clone(),
                    )
                }),
            )
        })
        .map(|(addr, coins)| BankMsg::Send {
            to_address: addr.to_string(),
            amount: coins.collect(),
        })
        .map(CosmosMsg::Bank);

    Ok(Response::new().add_messages(to_send))
}

/// Claim any funds through the fee splitter contract, this is needed for any strategy 
/// This also means that this contract should not be receiving any CW20s
#[cfg(claim)]
pub fn execute_claim(claims: Vec<Claim>) -> Result<Response, ContractError> {
    let (attrs, msgs): (Vec<Attribute>, Vec<CosmosMsg>) = claims
        .into_iter()
        .map(|c| {
            (
                attr("claim_address", &c.address),
                CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                    contract_addr: c.address,
                    msg: c.msg,
                    funds: vec![],
                }),
            )
        })
        .unzip();

    Ok(Response::new().add_messages(msgs).add_attributes(attrs))
}
#[cfg(claim)]
#[cfg(test)]
mod tests {
    use cl_vault::msg::AdminExtensionExecuteMsg;
    use cosmwasm_std::to_json_binary;

    use super::*;

    #[test]
    fn test_claim_works() {
        let claim_msg = cw_vault_multi_standard::VaultStandardExecuteMsg::VaultExtension(
            AdminExtensionExecuteMsg::ClaimStrategistRewards {},
        );
        let claim = Claim {
            address: "vault".to_string(),
            msg: to_json_binary(&claim_msg).unwrap(),
        };

        let response = execute_claim(vec![claim]).unwrap();
        assert_eq!(
            response.messages[0].msg,
            CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                contract_addr: "vault".to_string(),
                msg: to_json_binary(&claim_msg).unwrap(),
                funds: vec![]
            })
        )
    }
}
