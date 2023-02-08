use cosmwasm_std::{Decimal, DepsMut, Env, Fraction, MessageInfo, Response, Uint128};
use cw20_base::contract::execute_mint;
use quasar_types::callback::{BondResponse, Callback};

use crate::{
    state::{BondingStub, DEPOSIT_STATE, INVESTMENT, PENDING_BOND_IDS, TOTAL_SUPPLY},
    ContractError,
};

pub fn handle_callback(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    callback_msg: Callback,
) -> Result<Response, ContractError> {
    match callback_msg {
        Callback::BondResponse(bond_response) => on_bond(
            deps,
            env,
            info,
            bond_response.share_amount,
            bond_response.bond_id,
        ),
        Callback::StartUnbondResponse(_start_unbond_response) => todo!(),
        Callback::UnbondResponse(_unbond_response) => todo!(),
    }
}

pub fn on_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    share_amount: Uint128,
    bond_id: String,
) -> Result<Response, ContractError> {
    // load investment info
    let invest = INVESTMENT.load(deps.storage)?;
    let mut deposit_stubs = DEPOSIT_STATE.load(deps.storage, bond_id.clone())?;

    // lets find the primitive for this response
    let primitive_config = invest.primitives.iter().find(|p| p.address == info.sender);

    // if we don't find a primitive, this is an unauthorized call
    if primitive_config.is_none() {
        return Err(ContractError::Unauthorized {});
    }

    // update deposit state here before doing anything else & save!
    deposit_stubs = deposit_stubs
        .iter()
        .map(|s| {
            if s.address == info.sender {
                BondingStub {
                    address: s.address.clone(),
                    bond_response: Option::Some(BondResponse {
                        share_amount,
                        bond_id: bond_id.clone(),
                    }),
                }
            } else {
                s.to_owned()
            }
        })
        .collect();
    DEPOSIT_STATE.save(deps.storage, bond_id.clone(), &deposit_stubs)?;

    // if still waiting on successful bonds, then return
    if deposit_stubs.iter().any(|s| s.bond_response.is_none()) {
        return Ok(Response::new());
    }

    // at this point we know that the deposit has succeeded fully, and we can mint shares
    // lets updated all pending deposit info
    PENDING_BOND_IDS.update(deps.storage, info.sender.clone(), |ids| match ids {
        Some(mut bond_ids) => {
            let bond_index = bond_ids.iter().position(|id| id == &bond_id).unwrap();
            bond_ids.remove(bond_index);
            Ok::<Vec<String>, ContractError>(bond_ids)
        }
        None => Ok(vec![]), // todo: should this error? we should never be here
    })?;
    // todo: this should save a claim for unlockable_at? will be improved during withdrawal impl
    DEPOSIT_STATE.save(deps.storage, bond_id.to_string(), &deposit_stubs)?;

    let total_weight = invest
        .primitives
        .iter()
        .fold(Decimal::zero(), |acc, p| acc.checked_add(p.weight).unwrap());

    // calculate shares to mint
    let shares_to_mint =
        deposit_stubs
            .iter()
            .zip(invest.primitives.iter())
            .fold(Uint128::zero(), |acc, (s, pc)| {
                acc.checked_add(
                    // omfg pls dont look at this code, i will make it cleaner
                    s.bond_response
                        .as_ref()
                        .unwrap()
                        .share_amount
                        .checked_multiply_ratio(
                            pc.weight.numerator(),
                            total_weight
                                .numerator()
                                .checked_multiply_ratio(
                                    pc.weight.denominator(),
                                    total_weight.denominator(),
                                )
                                .unwrap(),
                        )
                        .unwrap(),
                )
                .unwrap()
            });

    // update total supply
    let mut supply = TOTAL_SUPPLY.load(deps.storage)?;

    // todo: i think supply structure needs to be simplified or augmented
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
