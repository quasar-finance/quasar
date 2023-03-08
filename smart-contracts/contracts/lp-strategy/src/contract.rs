#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    Uint128,
};
use cw2::set_contract_version;
use cw_utils::must_pay;

use crate::bond::do_bond;
use crate::error::ContractError;
use crate::helpers::parse_seq;
use crate::ibc_lock::Lock;
use crate::ibc_util::{do_ibc_join_pool_swap_extern_amount_in, do_transfer};
use crate::icq::try_icq;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::queries::{
    handle_channels_query, handle_config_query, handle_ica_address_query, handle_ica_balance,
    handle_ica_channel, handle_list_bonding_claims, handle_list_pending_acks,
    handle_list_primitive_shares, handle_list_unbonding_claims, handle_lock,
    handle_lp_shares_query, handle_primitive_shares, handle_trapped_errors_query,
    handle_unbonding_claim_query,
};
use crate::start_unbond::{do_start_unbond, StartUnbond};
use crate::state::{
    Config, OngoingDeposit, RawAmount, CONFIG, IBC_LOCK, ICA_BALANCE, LP_SHARES, PENDING_ACK,
    REPLIES, RETURNING,
};
use crate::unbond::{do_unbond, transfer_batch_unbond, PendingReturningUnbonds, ReturningUnbond};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:lp-strategy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;
    CONFIG.save(
        deps.storage,
        &Config {
            lock_period: msg.lock_period,
            pool_id: msg.pool_id,
            pool_denom: msg.pool_denom,
            base_denom: msg.base_denom,
            local_denom: msg.local_denom,
            quote_denom: msg.quote_denom,
            return_source_channel: msg.return_source_channel,
            transfer_channel: msg.transfer_channel,
            expected_connection: msg.expected_connection,
        },
    )?;

    IBC_LOCK.save(deps.storage, &Lock::new())?;

    LP_SHARES.save(deps.storage, &Uint128::zero())?;

    // this is a workaround so that the contract query does not fail for balance before deposits have been made successfully
    ICA_BALANCE.save(deps.storage, &Uint128::one())?;

    TIMED_OUT.save(deps.storage, &false)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    // Save the ibc message together with the sequence number, to be handled properly later at the ack, we can pass the ibc_kind one to one
    // TODO this needs and error check and error handling
    let pending = REPLIES.load(deps.storage, msg.id)?;
    let data = msg
        .result
        .into_result()
        .map_err(|msg| StdError::GenericErr {
            msg: format!("submsg error: {:?}", msg),
        })?
        .data
        .ok_or(ContractError::NoReplyData)
        .map_err(|_| StdError::NotFound {
            kind: "reply-data".to_string(),
        })?;

    let seq = parse_seq(data).map_err(|err| StdError::SerializeErr {
        source_type: "protobuf-decode".to_string(),
        msg: err.to_string(),
    })?;

    PENDING_ACK.save(deps.storage, seq, &pending)?;
    Ok(Response::default()
        .add_attribute("pending-msg", seq.to_string())
        .add_attribute("step", format!("{:?}", pending)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Bond { id } => execute_bond(deps, env, info, id),
        ExecuteMsg::StartUnbond { id, share_amount } => {
            execute_start_unbond(deps, env, info, id, share_amount)
        }
        ExecuteMsg::Unbond { id } => execute_unbond(deps, env, info, id),
        ExecuteMsg::AcceptReturningFunds { id } => {
            execute_accept_returning_funds(deps, &env, info, id)
        }
        ExecuteMsg::ReturnTransfer { amount } => execute_return_funds(deps, env, info, amount),
        ExecuteMsg::CloseChannel { channel_id } => execute_close_channel(deps, channel_id),
    }
}

pub fn execute_return_funds(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let msg = transfer_batch_unbond(
        deps.storage,
        &env,
        &PendingReturningUnbonds {
            unbonds: vec![ReturningUnbond {
                amount: RawAmount::LpShares(Uint128::new(100)),
                owner: info.sender,
                id: String::from("1"),
            }],
        },
        amount,
    )?;

    Ok(Response::new().add_submessage(msg))
}

pub fn execute_accept_returning_funds(
    deps: DepsMut,
    _env: &Env,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    let returning_amount = RETURNING
        .may_load(deps.storage, id)?
        .ok_or(ContractError::ReturningTransferNotFound)?;

    let amount = must_pay(&info, CONFIG.load(deps.storage)?.local_denom.as_str())?;
    if amount != returning_amount {
        return Err(ContractError::ReturningTransferIncorrectAmount);
    }

    Ok(Response::new()
        .add_attribute("returning-transfer", id.to_string())
        .add_attribute("success", "true"))
}

pub fn execute_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    let msg = do_bond(deps.storage, env, info.clone(), id)?;

    // if msg is some, we are dispatching an icq
    match msg {
        Some(submsg) => {
            IBC_LOCK.update(deps.storage, |lock| -> Result<Lock, ContractError> {
                Ok(lock.lock_bond())
            })?;
            Ok(Response::new()
                .add_submessage(submsg)
                .add_attribute("deposit", info.sender)
                .add_attribute("kind", "dispatch"))
        }
        None => Ok(Response::new()
            .add_attribute("deposit", info.sender)
            .add_attribute("kind", "queue")),
    }
}

pub fn execute_start_unbond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    share_amount: Uint128,
) -> Result<Response, ContractError> {
    do_start_unbond(
        deps.storage,
        StartUnbond {
            owner: info.sender.clone(),
            id,
            primitive_shares: share_amount,
        },
    )?;

    let msg = try_icq(deps.storage, env)?;

    match msg {
        Some(submsg) => {
            IBC_LOCK.update(deps.storage, |lock| -> Result<Lock, ContractError> {
                Ok(lock.lock_start_unbond())
            })?;
            Ok(Response::new()
                .add_submessage(submsg)
                .add_attribute("start-unbond", info.sender)
                .add_attribute("kind", "dispatch"))
        }
        None => Ok(Response::new()
            .add_attribute("start-unbond", info.sender)
            .add_attribute("kind", "queue")),
    }
}

pub fn execute_unbond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    do_unbond(deps.storage, &env, info.sender.clone(), id)?;

    let msg = try_icq(deps.storage, env)?;

    match msg {
        Some(submsg) => {
            IBC_LOCK.update(deps.storage, |lock| -> Result<Lock, ContractError> {
                Ok(lock.lock_unbond())
            })?;
            Ok(Response::new()
                .add_submessage(submsg)
                .add_attribute("unbond", info.sender)
                .add_attribute("kind", "dispatch"))
        }
        None => Ok(Response::new()
            .add_attribute("unbond", info.sender)
            .add_attribute("kind", "queue")),
    }
}


// transfer funds sent to the contract to an address on osmosis, this call ignores the lock system
pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    channel: String,
    to_address: String,
) -> Result<Response, ContractError> {
    let amount = must_pay(&info, &CONFIG.load(deps.storage)?.local_denom)?;

    let transfer = do_transfer(
        deps.storage,
        &env,
        amount,
        channel.clone(),
        to_address.clone(),
        // add a dummy ongoing deposit, actual ongoing deposit should calculate the claim using the total balance
        vec![OngoingDeposit {
            claim_amount: amount,
            owner: info.sender,
            raw_amount: RawAmount::LocalDenom(amount),
            bond_id: "id".to_string(),
        }],
    )?;

    Ok(Response::new()
        .add_submessage(transfer)
        .add_attribute("ibc-tranfer-channel", channel)
        .add_attribute("ibc-transfer-receiver", to_address))
}

pub fn execute_join_pool(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    pool_id: u64,
    denom: String,
    amount: Uint128,
    share_out_min_amount: Uint128,
) -> Result<Response, ContractError> {
    let join = do_ibc_join_pool_swap_extern_amount_in(
        deps.storage,
        env,
        pool_id,
        denom.clone(),
        amount,
        share_out_min_amount,
        // add a dummy ongoing deposit, actual ongoing deposit should calculate the claim using the total balance
        vec![OngoingDeposit {
            claim_amount: amount,
            owner: info.sender,
            raw_amount: RawAmount::LocalDenom(amount),
            bond_id: "id".to_string(),
        }],
    )?;

    Ok(Response::new()
        .add_submessage(join)
        .add_attribute("denom", denom))
}

pub fn execute_close_channel(deps: DepsMut, channel_id: String) -> Result<Response, ContractError> {
    if TIMED_OUT.load(deps.storage)? && channel_id == ICA_CHANNEL.load(deps.storage)? {
        Ok(Response::new().add_message(IbcMsg::CloseChannel { channel_id }))
    } else {
        Err(ContractError::IcaChannelAlreadySet)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    // update the config
    CONFIG.save(deps.storage, &msg.config)?;

    IBC_LOCK.save(deps.storage, &Lock::new())?;

    Ok(Response::new()
        .add_attribute("migrate", CONTRACT_NAME)
        .add_attribute("succes", "true"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Channels {} => to_binary(&handle_channels_query(deps)?),
        QueryMsg::Config {} => to_binary(&handle_config_query(deps)?),
        QueryMsg::IcaAddress {} => to_binary(&handle_ica_address_query(deps)?),
        QueryMsg::PrimitiveShares {} => to_binary(&handle_primitive_shares(deps)?),
        QueryMsg::IcaBalance {} => to_binary(&handle_ica_balance(deps)?),
        QueryMsg::IcaChannel {} => to_binary(&handle_ica_channel(deps)?),
        QueryMsg::Lock {} => to_binary(&handle_lock(deps)?),
        QueryMsg::LpShares {} => to_binary(&handle_lp_shares_query(deps)?),
        QueryMsg::TrappedErrors {} => to_binary(&handle_trapped_errors_query(deps)?),
        QueryMsg::ListUnbondingClaims {} => to_binary(&handle_list_unbonding_claims(deps)?),
        QueryMsg::UnbondingClaim { addr, id } => {
            to_binary(&handle_unbonding_claim_query(deps, addr, id)?)
        }
        QueryMsg::ListBondingClaims {} => to_binary(&handle_list_bonding_claims(deps)?),
        QueryMsg::ListPrimitiveShares {} => to_binary(&handle_list_primitive_shares(deps)?),
        QueryMsg::ListPendingAcks {} => to_binary(&handle_list_pending_acks(deps)?),
    }
}

#[cfg(test)]
mod tests {}
