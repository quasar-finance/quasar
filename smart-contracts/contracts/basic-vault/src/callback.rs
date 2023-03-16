use cosmwasm_std::{
    Addr, BankMsg, Decimal, DepsMut, Env, Fraction, MessageInfo, OverflowError, Response,
    Timestamp, Uint128,
};
use cw20_base::contract::execute_mint;
use quasar_types::callback::{BondResponse, UnbondResponse};

use crate::{
    msg::PrimitiveConfig,
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
        &format!("We hit on_unbond with bond_id: {bond_id}"),
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
    bond_stubs.iter_mut().for_each(|s| {
        if s.address == info.sender {
            s.bond_response = Option::Some(BondResponse {
                share_amount,
                bond_id: bond_id.clone(),
            });
        }
    });

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
    // at this point we know that the deposit has succeeded fully, and we can mint shares

    let user_address = BONDING_SEQ_TO_ADDR.load(deps.storage, bond_id.clone())?;
    // lets updated all pending deposit info
    PENDING_BOND_IDS.update(
        deps.storage,
        deps.api.addr_validate(&user_address)?,
        |ids| {
            if let Some(mut bond_ids) = ids {
                let bond_index = bond_ids.iter().position(|id| id.eq(&bond_id)).ok_or(
                    ContractError::IncorrectCallbackId {
                        expected: bond_id.clone(),
                        ids: bond_ids.clone(),
                    },
                )?;
                bond_ids.remove(bond_index);
                Ok::<Vec<String>, ContractError>(bond_ids)
            } else {
                Ok(vec![])
            }
        },
    )?;

    BOND_STATE.save(deps.storage, bond_id, &bond_stubs)?;

    let total_weight = invest.primitives.iter().try_fold(
        Decimal::zero(),
        |acc: Decimal, p: &PrimitiveConfig| -> Result<Decimal, OverflowError> {
            acc.checked_add(p.weight)
        },
    )?;

    // calculate shares to mint
    let shares_to_mint = bond_stubs.iter().zip(invest.primitives.iter()).try_fold(
        Uint128::zero(),
        |acc, (s, pc)| -> Result<Uint128, ContractError> {
            Ok(acc.checked_add(
                // omfg pls dont look at this code, i will make it cleaner -> cleaner but still ugly :D
                s.bond_response
                    .as_ref()
                    .ok_or(ContractError::BondResponseIsEmpty {})?
                    .share_amount
                    .checked_multiply_ratio(
                        pc.weight.numerator(),
                        total_weight.numerator().checked_multiply_ratio(
                            pc.weight.denominator(),
                            total_weight.denominator(),
                        )?,
                    )?,
            )?)
        },
    )?;

    // update total supply
    let mut supply = TOTAL_SUPPLY.load(deps.storage)?;

    supply.issued += shares_to_mint;
    TOTAL_SUPPLY.save(deps.storage, &supply)?;

    // call into cw20-base to mint the token, call as self as no one else is allowed
    let sub_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };

    execute_mint(deps, env, sub_info, user_address, shares_to_mint)?;

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
    UNBOND_STATE.update(
        deps.storage,
        unbond_id.clone(),
        |s: Option<Unbond>| -> Result<Unbond, ContractError> {
            let mut unbond = s.ok_or(ContractError::UnbondIsEmpty {})?;
            // update the stub where the address is the same as message sender with the unlock time

            unbond
                .stub
                .iter_mut()
                .find(|s| s.address == info.sender)
                .ok_or(ContractError::UnbondStubIsEmpty {})?
                .unlock_time = Option::Some(unlock_time);
            Ok(Unbond {
                stub: unbond.stub,
                shares: unbond.shares,
            })
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "on_start_unbond")
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
        .ok_or(ContractError::UnbondStubIsEmpty {})?;

    // update info
    unbonding_stub.unbond_response = Option::Some(UnbondResponse {
        unbond_id: unbond_id.clone(),
    });
    unbonding_stub.unbond_funds = info.funds;

    UNBOND_STATE.save(deps.storage, unbond_id.clone(), &unbond_stubs)?;

    // if still waiting on successful unbonds, then return
    // todo: should we eagerly send back funds?
    if unbond_stubs
        .stub
        .iter()
        .any(|s| s.unbond_response.is_none())
    {
        return Ok(Response::new());
    }

    let user_address = BONDING_SEQ_TO_ADDR.load(deps.storage, unbond_id.clone())?;
    // Construct message to return these funds to the user
    let return_msgs: Vec<BankMsg> = unbond_stubs
        .stub
        .iter()
        .map(|s| BankMsg::Send {
            to_address: user_address.to_string(),
            amount: s.unbond_funds.clone(),
        })
        .collect();

    // delete this pending unbond id from the state
    UNBOND_STATE.remove(deps.storage, unbond_id.clone());
    PENDING_UNBOND_IDS.update(
        deps.storage,
        Addr::unchecked(user_address),
        |ids| -> Result<Vec<String>, ContractError> {
            Ok(ids
                .ok_or(ContractError::NoPendingUnbonds {})?
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
