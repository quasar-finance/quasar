use cosmwasm_std::{
    to_binary, Addr, Env, IbcBasicResponse, IbcTimeout, Storage, SubMsg, Uint128, WasmMsg,
};

use osmosis_std::types::{cosmos::base::v1beta1::Coin, osmosis::lockup::MsgBeginUnlocking};
use quasar_types::{
    callback::{Callback, StartUnbondResponse},
    ica::packet::ica_send,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ContractError,
    helpers::get_total_shares,
    helpers::{create_ibc_ack_submsg, get_ica_address, IbcMsgKind, IcaMessages},
    ibc_lock::Lock,
    state::{
        PendingSingleUnbond, Unbond, CONFIG, IBC_LOCK, ICA_CHANNEL, OSMO_LOCK, SHARES,
        START_UNBOND_QUEUE, UNBONDING_CLAIMS,
    },
};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StartUnbond {
    pub owner: Addr,
    pub id: String,
    pub primitive_shares: Uint128,
}

pub fn do_start_unbond(
    storage: &mut dyn Storage,
    unbond: StartUnbond,
) -> Result<(), ContractError> {
    if UNBONDING_CLAIMS.has(storage, (unbond.owner.clone(), unbond.id.clone())) {
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

    if START_UNBOND_QUEUE.is_empty(storage)? {
        return Ok(None);
    }

    while !START_UNBOND_QUEUE.is_empty(storage)? {
        let unbond =
            START_UNBOND_QUEUE
                .pop_front(storage)?
                .ok_or(ContractError::QueueItemNotFound {
                    queue: "start_unbond".to_string(),
                })?;
        let lp_shares = single_unbond(storage, env, &unbond, total_lp_shares)?;
        to_unbond = to_unbond.checked_add(lp_shares)?;
        unbonds.push(PendingSingleUnbond {
            lp_shares,
            primitive_shares: unbond.primitive_shares,
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
    unbonds: &mut Vec<PendingSingleUnbond>,
) -> Result<IbcBasicResponse, ContractError> {
    let mut msgs: Vec<WasmMsg> = Vec::new();
    for unbond in unbonds {
        let msg = start_internal_unbond(storage, env, unbond)?;
        msgs.push(msg);
    }

    IBC_LOCK.update(storage, |lock| -> Result<Lock, ContractError> {
        Ok(lock.unlock_start_unbond())
    })?;

    Ok(IbcBasicResponse::new()
        .add_attribute("start-unbond", "succes")
        .add_attribute("callback-msgs", msgs.len().to_string())
        .add_messages(msgs))
}

// in single_unbond, we change from using internal primitive to an actual amount of lp-shares that we can unbond
fn single_unbond(
    storage: &mut dyn Storage,
    _env: &Env,
    unbond: &StartUnbond,
    total_lp_shares: Uint128,
) -> Result<Uint128, ContractError> {
    let total_shares = get_total_shares(storage)?;
    Ok(unbond
        .primitive_shares
        .checked_mul(total_lp_shares)?
        .checked_div(total_shares)?)
}

// unbond starts unbonding an amount of lp shares
fn start_internal_unbond(
    storage: &mut dyn Storage,
    env: &Env,
    unbond: &PendingSingleUnbond,
) -> Result<WasmMsg, ContractError> {
    // check that we can create a new unbond
    if !UNBONDING_CLAIMS.has(storage, (unbond.owner.clone(), unbond.id.clone())) {
        return Err(ContractError::DuplicateKey);
    }

    // remove amount of shares
    let left = SHARES
        .load(storage, unbond.owner.clone())?
        .checked_sub(unbond.primitive_shares)?;
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
            lp_shares: unbond.lp_shares,
            unlock_time,
            id: unbond.id.clone(),
            owner: unbond.owner.clone(),
        },
    )?;

    let msg = Callback::StartUnbondResponse(StartUnbondResponse {
        unbond_id: unbond.id.clone(),
        unlock_time,
    });

    Ok(WasmMsg::Execute {
        contract_addr: unbond.owner.to_string(),
        msg: to_binary(&msg)?,
        funds: vec![],
    })
}
