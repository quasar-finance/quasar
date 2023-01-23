use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw20_base::contract::execute_mint;
use quasar_types::callback::BondResponse;

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
        CallbackMsg::OnBond(bond_response) => on_bond(
            deps,
            env,
            info,
            bond_response.share_amount,
            bond_response.bond_id,
        ),
    }
}

pub fn on_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    shares_amount: Uint128,
    bond_id: u64,
) -> Result<Response, ContractError> {
    // load investment info
    let invest = INVESTMENT.load(deps.storage)?;
    let mut deposit_stubs = DEPOSIT_STATE.load(deps.storage, bond_id)?;

    // lets save this primitive response
    let primitive_config = invest
        .primitives
        .iter()
        .find(|p| p.address == info.sender)
        .unwrap();

    // update deposit state here before doing anything else & save!
    deposit_stubs = deposit_stubs.iter().map(|s| {
        if (s.address == info.sender) {
            s.bond_response = Option::Some(BondResponse {
                share_amount,
                bond_id,
            });
        }
        s
    });
    DEPOSIT_STATE.save(deps.storage, bond_id, &deposit_stubs);

    // if still waiting on successful bonds, then return
    if (deposit_stubs.iter().any(|s| s.bond_response.is_none())) {
        return Ok(Response::new());
    }

    let total_weight = invest
        .primitives
        .iter()
        .fold(Uint128::zero(), |acc, p| acc.checked_add(p.weight).unwrap());

    // calculate shares to mint
    let shares_to_mint =
        deposit_stubs
            .iter()
            .zip(invest.primitives.iter())
            .fold(Uint128::zero(), |acc, (s, pc)| {
                acc.checked_add(
                    s.bond_response
                        .unwrap()
                        .share_amount
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
        .add_attribute("from", info.sender);
    // .add_attribute("bonded", payment.amount)
    // .add_attribute("minted", to_mint);
    Ok(res)
}
