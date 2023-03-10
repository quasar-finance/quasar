use cosmwasm_std::{
    Addr, BankMsg, Decimal, DepsMut, Env, Fraction, MessageInfo, Response, Timestamp, Uint128,
};
use quasar_types::callback::{BondResponse, UnbondResponse};

use crate::{
    state::{
        Unbond, BONDING_SEQ_TO_ADDR, BOND_STATE, DEBUG_TOOL, INVESTMENT, PENDING_BOND_IDS,
        PENDING_UNBOND_IDS, TOTAL_SUPPLY, UNBOND_STATE,
    },
    ContractError,
};

pub fn on_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    share_amount: Uint128,
    bond_id: String,
) -> Result<Response, ContractError> {
    DEBUG_TOOL.save(
        deps.storage,
        &format!("We hit on_unbond with bond_id: {}", bond_id),
    )?;

    // load investment info
    let invest = INVESTMENT.load(deps.storage)?;
    let mut bond_stubs = BOND_STATE.load(deps.storage, bond_id.clone())?;

    // lets find the primitive for this response
    let primitive_config = invest.primitives.iter().find(|p| p.address == info.sender);

    // if we don't find a primitive, this is an unauthorized call
    if primitive_config.is_none() {
        return Err(ContractError::Unauthorized {});
    }

    // update deposit state here before doing anything else & save!
    for s in bond_stubs.iter_mut() {
        if s.address == info.sender {
            s.bond_response = Option::Some(BondResponse {
                share_amount,
                bond_id: bond_id.clone(),
            });
        }
    }
    BOND_STATE.save(deps.storage, bond_id.clone(), &bond_stubs)?;

    // if still waiting on successful bonds, then return
    if bond_stubs.iter().any(|s| s.bond_response.is_none()) {
        return Ok(Response::new()
            .add_attribute("action", "on_bond")
            .add_attribute(
                "state",
                bond_stubs
                    .iter()
                    .fold(0u32, |acc, stub| {
                        if stub.bond_response.is_none() {
                            acc + 1
                        } else {
                            acc
                        }
                    })
                    .to_string()
                    + "pending bonds",
            ));
    }

    let user_address = BONDING_SEQ_TO_ADDR.load(deps.storage, bond_id.clone())?;

    // at this point we know that the deposit has succeeded fully, and we can mint shares
    // lets updated all pending deposit info
    PENDING_BOND_IDS.update(
        deps.storage,
        deps.api.addr_validate(&user_address)?,
        |ids| match ids {
            Some(mut bond_ids) => {
                let bond_index = bond_ids.iter().position(|id| id.eq(&bond_id)).ok_or(
                    ContractError::IncorrectCallbackId {
                        expected: bond_id.clone(),
                        ids: bond_ids.clone(),
                    },
                )?;
                bond_ids.remove(bond_index);
                Ok::<Vec<String>, ContractError>(bond_ids)
            }
            None => Err(ContractError::IncorrectCallbackId {
                expected: "Some".to_string(),
                ids: vec!["None".to_string()],
            }), // todo: should this error? we should never be here
        },
    )?;
    // todo: this should save a claim for unlockable_at? will be improved during withdrawal impl
    BOND_STATE.save(deps.storage, bond_id, &bond_stubs)?;

    let total_weight = invest
        .primitives
        .iter()
        .fold(Decimal::zero(), |acc, p| acc.checked_add(p.weight).unwrap());

    // calculate shares to mint
    let shares_to_mint =
        bond_stubs
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
    let _sub_info = MessageInfo {
        sender: env.contract.address,
        funds: vec![],
    };

    // use cw20_base::contract::execute_mint;
    //
    // execute_mint(
    //     deps,
    //     env,
    //     sub_info,
    //     user_address.to_string(),
    //     shares_to_mint,
    // )?;

    // bond them to the validator
    let res = Response::new()
        .add_attribute("action", "bond")
        .add_attribute("from", info.sender)
        .add_attribute("minted", shares_to_mint);
    Ok(res)
}

pub fn on_start_unbond(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    unbond_id: String,
    unlock_time: Timestamp,
) -> Result<Response, ContractError> {
    // load info.sender -> [..., unbond_id], and unbond_id -> [..., { address, unlock_time }]
    // also i guess if unlock_time is now or earlier then we can send right now
    UNBOND_STATE.update(
        deps.storage,
        unbond_id.clone(),
        |s: Option<Unbond>| -> Result<Unbond, ContractError> {
            // TODO change this to ok_or() or unwrap_or()
            // TODO update could/should be more efficient, shouldn't be a need to copy s
            let mut unbond = s.unwrap();
            // update the stub where the address is the same as message sender with the unlock time

            unbond
                .stub
                .iter_mut()
                .find(|s| s.address == info.sender)
                .unwrap()
                .unlock_time = Option::Some(unlock_time);
            Ok(Unbond {
                stub: unbond.stub,
                shares: unbond.shares,
            })
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "start_unbond")
        .add_attribute("unbond_id", unbond_id)
        .add_attribute("unlock_time", unlock_time.to_string()))
}

pub fn on_unbond(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    unbond_id: String,
) -> Result<Response, ContractError> {
    DEBUG_TOOL.save(
        deps.storage,
        &format!(
            "We hit on_unbond with unbond_id: {} and funds: {}",
            unbond_id, info.funds[0]
        ),
    )?;

    let mut unbond_stubs = UNBOND_STATE.load(deps.storage, unbond_id.clone())?;
    let _invest = INVESTMENT.load(deps.storage)?;

    // edit and save the stub where the address is the same as message sender with the unbond response
    let mut unbonding_stub = unbond_stubs
        .stub
        .iter_mut()
        .find(|s| s.address == info.sender)
        .unwrap();

    // update info
    unbonding_stub.unbond_response = Option::Some(UnbondResponse {
        unbond_id: unbond_id.clone(),
    });
    unbonding_stub.unbond_funds = info.funds;

    UNBOND_STATE.save(deps.storage, unbond_id.clone(), &unbond_stubs)?;

    // if still waiting on successful unbonds, then return
    if unbond_stubs
        .stub
        .iter()
        .any(|s| s.unbond_response.is_none())
    {
        return Ok(Response::new());
    }

    // Construct message to return these funds to the user
    let mut return_msgs = Vec::new();
    let user_address = BONDING_SEQ_TO_ADDR.load(deps.storage, unbond_id.clone())?;
    for s in &unbond_stubs.stub {
        return_msgs.push(BankMsg::Send {
            to_address: user_address.to_string(),
            amount: s.unbond_funds.clone(),
        });
    }

    // delete this pending unbond id from the state
    UNBOND_STATE.remove(deps.storage, unbond_id.clone());

    // todo: also need to remove the unbond id from the user's list of pending unbonds
    PENDING_UNBOND_IDS.update(
        deps.storage,
        Addr::unchecked(user_address),
        |ids| -> Result<Vec<String>, ContractError> {
            Ok(ids
                .unwrap()
                .into_iter()
                .filter(|id| id != &unbond_id)
                .collect())
        },
    )?;

    Ok(Response::new()
        .add_messages(return_msgs)
        .add_attribute("action", "on_unbond")
        .add_attribute("unbond_id", unbond_id))
}
