use cosmwasm_std::{Addr, Env, IbcBasicResponse, IbcTimeout, Storage, SubMsg, Uint128};
use cw_storage_plus::DequeIter;
use osmosis_std::types::{cosmos::base::v1beta1::Coin, osmosis::lockup::MsgBeginUnlocking};
use quasar_types::ica::packet::ica_send;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ContractError,
    helpers::get_total_shares,
    helpers::{create_ibc_ack_submsg, get_ica_address, IbcMsgKind, IcaMessages},
    ibc_lock::{IbcLock, Lock},
    icq::try_icq,
    state::{
        PendingSingleUnbond, Unbond, CONFIG, ICA_CHANNEL, OSMO_LOCK, SHARES, START_UNBOND_QUEUE,
        UNBONDING_CLAIMS, IBC_LOCK,
    },
};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StartUnbond {
    pub owner: Addr,
    pub id: String,
    pub shares: Uint128,
}

pub fn do_start_unbond(
    storage: &mut dyn Storage,
    unbond: StartUnbond,
) -> Result<(), ContractError> {
    if !UNBONDING_CLAIMS.has(storage, (unbond.owner.clone(), unbond.id.clone())) {
        return Err(ContractError::DuplicateKey);
    }

    Ok(START_UNBOND_QUEUE.push_back(storage, &unbond)?)
}

// batch unbond tries to unbond a batch of unbondings, should be called after the icq query has returned for deposits
pub fn batch_start_unbond(
    storage: &mut dyn Storage,
    env: &Env,
    total_lp_shares: Uint128,
) -> Result<Option<SubMsg>, ContractError> {
    let mut to_unbond = Uint128::zero();
    let mut unbonds: Vec<PendingSingleUnbond> = vec![];

    let empty = START_UNBOND_QUEUE.is_empty(storage)?;

    if empty {
        return Ok(None)
    }

    while !empty {
        let unbond = START_UNBOND_QUEUE
            .pop_front(storage)?
            .ok_or(ContractError::QueueItemNotFound)?;
        let lp_shares = single_unbond(storage, &env, &unbond, total_lp_shares)?;
        to_unbond = to_unbond.checked_add(lp_shares)?;
        unbonds.push(PendingSingleUnbond {
            amount: lp_shares,
            owner: unbond.owner,
            id: unbond.id,
        })
    }

    let config = CONFIG.load(storage)?;
    let ica_address = get_ica_address(storage, ICA_CHANNEL.load(storage)?)?;

    let msg = MsgBeginUnlocking {
        owner: ica_address,
        id: OSMO_LOCK.load(storage)?,
        coins: vec![Coin {
            denom: config.pool_denom,
            amount: to_unbond.to_string(),
        }],
    };

    let pkt = ica_send::<MsgBeginUnlocking>(
        msg,
        ICA_CHANNEL.load(storage)?,
        IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
    )?;

    Ok(Some(create_ibc_ack_submsg(
        storage,
        &IbcMsgKind::Ica(IcaMessages::BeginUnlocking(unbonds)),
        pkt,
    )?))
}

pub fn handle_start_unbond_ack(
    storage: &mut dyn Storage,
    env: &Env,
    unbonds: Vec<PendingSingleUnbond>,
) -> Result<IbcBasicResponse, ContractError> {
    for unbond in unbonds {
        start_internal_unbond(storage, env, &unbond)?
    }

    IBC_LOCK.update(storage, |lock| -> Result<Lock, ContractError> {Ok(lock.unlock_start_unbond())})?;
    
    Ok(IbcBasicResponse::new().add_attribute("start-unbond", "succes"))
}

// in single_unbond, we change from using internal primitive to an actual amount of lp-shares that we can unbond
fn single_unbond(
    storage: &mut dyn Storage,
    env: &Env,
    unbond: &StartUnbond,
    total_lp_shares: Uint128,
) -> Result<Uint128, ContractError> {
    let total_shares = get_total_shares(storage)?;
    Ok(unbond
        .shares
        .checked_mul(total_lp_shares)?
        .checked_div(total_shares)?)
}

// unbond starts unbonding an amount of lp shares
fn start_internal_unbond(
    storage: &mut dyn Storage,
    env: &Env,
    unbond: &PendingSingleUnbond,
) -> Result<(), ContractError> {
    // check that we can create a new unbond
    if !UNBONDING_CLAIMS.has(storage, (unbond.owner.clone(), unbond.id.clone())) {
        return Err(ContractError::DuplicateKey);
    }

    // remove amount of shares
    let left = SHARES
        .load(storage, unbond.owner.clone())?
        .checked_sub(unbond.amount)?;
    // subtracting below zero here should trigger an error in check_sub
    if left.is_zero() {
        SHARES.remove(storage, unbond.owner.clone());
    } else {
        SHARES.save(storage, unbond.owner.clone(), &left)?;
    }

    // todo verify logic of unlock times
    let unlock_time = env
        .block
        .time
        .plus_seconds(CONFIG.load(storage)?.lock_period);

    // add amount of unbonding claims
    UNBONDING_CLAIMS.save(
        storage,
        (unbond.owner.clone(), unbond.id.clone()),
        &Unbond {
            lp_shares: unbond.amount,
            unlock_time,
            id: unbond.id.clone(),
            owner: unbond.owner.clone(),
        },
    )?;
    Ok(())
}
