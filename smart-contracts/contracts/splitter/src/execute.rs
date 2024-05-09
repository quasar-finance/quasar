use cosmwasm_std::{attr, coin, Attribute, BankMsg, CosmosMsg, Deps, Env, Fraction, Response};

use crate::{msg::Claim, state::RECEIVERS, ContractError};

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
        .map(|b| CosmosMsg::Bank(b));

    Ok(Response::new().add_messages(to_send))
}

// TODO is it save to just execute user given binary messages? Since we send no funds along I'd assume it's save except if this contract is used as the receiver for CW20's
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
