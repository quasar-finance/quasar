use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw20_base::contract::execute_mint;

use crate::{
    msg::CallbackMsg,
    state::{DEPOSIT_STATE, INVESTMENT, TOTAL_SUPPLY},
    ContractError,
};

pub fn handle_callback(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    callback_msg: CallbackMsg,
) -> Result<Response, ContractError> {
    match callback_msg {
        CallbackMsg::OnBond {
            shares_out,
            deposit_id,
        } => on_bond(deps, env, info, shares_out, deposit_id),
    }
}

pub fn on_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    shares_out: Uint128,
    deposit_id: u64,
) -> Result<Response, ContractError> {
    // load investment info
    let invest = INVESTMENT.load(deps.storage)?;
    let deposit_state = DEPOSIT_STATE.load(deps.storage, deposit_id)?;

    // lets save this primitive response
    let primitive_config = invest
        .primitives
        .iter()
        .find(|p| p.address == info.sender)
        .unwrap();

    // update deposit state here before doing anything else & save!

    // if still waiting on successful bonds, then return
    if (deposit_state.iter().any(|s| s.success == false)) {
        return Ok(Response::new());
    }

    // todo: iff all primitives succeeded, then we can mint the tokens
    // (aka surround below in an if statement, else do nothing)

    let total_weight = invest
        .primitives
        .iter()
        .fold(Uint128::zero(), |acc, p| acc.checked_add(p.weight).unwrap());

    let zipped = deposit_state.iter().zip(invest.primitives.iter());
  
    // calculate shares to mint
    let shares_to_mint = zipped
        .fold(Uint128::zero(), |acc, (s, pc)| {
            acc.checked_add(
                s.shares_out
                    .checked_multiply_ratio(pc.weight, total_weight)
                    .unwrap(),
            )
            .unwrap()
        });


    // update total supply
    let mut supply = TOTAL_SUPPLY.load(deps.storage)?;

    supply.issued += shares_to_mint;
    // TODO: this is just a safety assertion - do we keep it, or remove caching?
    // in the end supply is just there to cache the (expected) results of get_bonded() so we don't
    // have expensive queries everywhere
    // assert_bonds(&supply, bonded)?;
    // let to_mint = if supply.issued.is_zero() || bonded.is_zero() {
    //     FALLBACK_RATIO * payment.amount
    // } else {
    //     payment.amount.multiply_ratio(supply.issued, bonded)
    // };
    // supply.bonded = bonded + payment.amount;
    // supply.issued += to_mint;
    TOTAL_SUPPLY.save(deps.storage, &supply)?;

    // call into cw20-base to mint the token, call as self as no one else is allowed
    let sub_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };
    execute_mint(deps, env, sub_info, info.sender.to_string(), shares_to_mint)?;

    // bond them to the validator
    let res = Response::new()
        .add_attribute("action", "bond")
        .add_attribute("from", info.sender)
        // .add_attribute("bonded", payment.amount)
        // .add_attribute("minted", to_mint);
    Ok(res)
}
