#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Binary, Deps, DepsMut, Env, IbcMsg, IbcPacketAckMsg, MessageInfo,
    Reply, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_utils::must_pay;
use quasar_types::ibc::IcsAck;

use crate::bond::do_bond;
use crate::error::ContractError;
use crate::helpers::SubMsgKind;
use crate::ibc::{handle_failing_ack, handle_succesful_ack};
use crate::ibc_lock::{IbcLock, Lock};
use crate::ibc_util::{do_ibc_join_pool_swap_extern_amount_in, do_transfer};
use crate::icq::try_icq;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::queries::{
    handle_channels_query, handle_config_query, handle_ica_address_query, handle_ica_balance,
    handle_ica_channel, handle_list_bonding_claims, handle_list_pending_acks,
    handle_list_primitive_shares, handle_list_replies, handle_list_unbonding_claims, handle_lock,
    handle_lp_shares_query, handle_primitive_shares, handle_trapped_errors_query,
    handle_unbonding_claim_query,
};
use crate::reply::{handle_ack_reply, handle_callback_reply, handle_ibc_reply};
use crate::start_unbond::{do_start_unbond, StartUnbond};
use crate::state::{
    Config, LpCache, OngoingDeposit, RawAmount, BOND_QUEUE, CONFIG, IBC_LOCK, ICA_BALANCE,
    ICA_CHANNEL, LP_SHARES, REPLIES, RETURNING, START_UNBOND_QUEUE, TIMED_OUT, UNBOND_QUEUE,
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

    LP_SHARES.save(
        deps.storage,
        &LpCache {
            locked_shares: Uint128::zero(),
            w_unlocked_shares: Uint128::zero(),
            d_unlocked_shares: Uint128::zero(),
        },
    )?;

    // this is a workaround so that the contract query does not fail for balance before deposits have been made successfully
    ICA_BALANCE.save(deps.storage, &Uint128::one())?;

    TIMED_OUT.save(deps.storage, &false)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    // Save the ibc message together with the sequence number, to be handled properly later at the ack, we can pass the ibc_kind one to one
    // TODO this needs and error check and error handling
    let reply = REPLIES.load(deps.storage, msg.id)?;
    match reply {
        SubMsgKind::Ibc(pending) => handle_ibc_reply(deps, msg, pending),
        SubMsgKind::Ack(seq) => handle_ack_reply(deps, msg, seq),
        SubMsgKind::Callback(_callback) => handle_callback_reply(deps, msg, _callback),
    }
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
        ExecuteMsg::Ack { ack } => execute_ack(deps, env, info, ack),
        ExecuteMsg::TryIcq {} => execute_try_icq(deps, env),
    }
}

pub fn execute_try_icq(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    // if we're unlocked, we can empty the queues and send the submessages
    let mut lock = IBC_LOCK.load(deps.storage)?;
    let sub_msg = try_icq(deps.storage, deps.querier, env)?;
    let mut res = Response::new();

    if let Some(sub_msg) = sub_msg {
        if !BOND_QUEUE.is_empty(deps.storage)? {
            lock.bond = IbcLock::Locked;
            res = res.add_attribute("bond_queue", "locked");
        }
        if !START_UNBOND_QUEUE.is_empty(deps.storage)? {
            lock = lock.lock_start_unbond();
            res = res.add_attribute("start_unbond_queue", "locked");
        }
        if !UNBOND_QUEUE.is_empty(deps.storage)? {
            lock = lock.lock_unbond();
            res = res.add_attribute("unbond_queue", "locked");
        }
        if lock.is_unlocked() {
            res = res.add_attribute("IBC_LOCK", "unlocked");
        }
        IBC_LOCK.save(deps.storage, &lock)?;
        res = res.add_submessage(sub_msg);
    } else {
        res = res.add_attribute("IBC_LOCK", "locked");
    }
    Ok(res)
}

pub fn execute_ack(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: IbcPacketAckMsg,
) -> Result<Response, ContractError> {
    if env.contract.address != info.sender {
        return Err(ContractError::Unauthorized);
    }

    // TODO: trap error like in receive?
    // pro's acks happen anyway, cons?
    let ack: IcsAck = from_binary(&msg.acknowledgement.data)?;
    match ack {
        IcsAck::Result(val) => handle_succesful_ack(deps, env, msg, val),
        IcsAck::Error(err) => handle_failing_ack(deps, env, msg, err),
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
        PendingReturningUnbonds {
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
    let msg = do_bond(deps.storage, deps.querier, env, info.clone(), id)?;

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

    let msg = try_icq(deps.storage, deps.querier, env)?;

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

    let msg = try_icq(deps.storage, deps.querier, env)?;

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
        QueryMsg::ListReplies {} => to_binary(&handle_list_replies(deps)?),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        attr,
        testing::{mock_dependencies, mock_env},
        Addr, Timestamp,
    };

    use crate::{bond::Bond, state::Unbond, test_helpers::default_setup};

    use super::*;

    #[test]
    fn test_execute_try_icq_ibc_locked() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let mock_lock = Lock::new().lock_bond().lock_start_unbond().lock_unbond();
        IBC_LOCK.save(&mut deps.storage, &mock_lock).unwrap();
        let res = execute_try_icq(deps.as_mut(), env).unwrap();

        assert_eq!(res.attributes[0], attr("IBC_LOCK", "locked"));
        assert!(res.messages.is_empty());
        assert!(IBC_LOCK.load(&mut deps.storage).unwrap().is_locked());
    }

    #[test]
    fn test_execute_try_icq_ibc_unlocked_all_queues_empty() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let mock_lock = Lock::new();
        IBC_LOCK.save(&mut deps.storage, &mock_lock).unwrap();

        default_setup(deps.as_mut().storage).unwrap();
        let lp_cache = LpCache {
            locked_shares: Uint128::new(10),
            w_unlocked_shares: Uint128::new(10),
            d_unlocked_shares: Uint128::new(10),
        };
        LP_SHARES.save(deps.as_mut().storage, &lp_cache).unwrap();

        let res = execute_try_icq(deps.as_mut(), env).unwrap();

        assert_eq!(res.attributes[0], attr("IBC_LOCK", "unlocked"));
        assert!(IBC_LOCK.load(&mut deps.storage).unwrap().is_unlocked());
    }

    #[test]
    fn test_execute_try_icq_ibc_locked_all_queues_filled() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let mock_lock = Lock::new().lock_bond().lock_start_unbond().lock_unbond();
        default_setup(deps.as_mut().storage).unwrap();
        let lp_cache = LpCache {
            locked_shares: Uint128::new(10),
            w_unlocked_shares: Uint128::new(10),
            d_unlocked_shares: Uint128::new(10),
        };
        LP_SHARES.save(deps.as_mut().storage, &lp_cache).unwrap();
        IBC_LOCK.save(&mut deps.storage, &mock_lock).unwrap();

        BOND_QUEUE
            .push_back(
                &mut deps.storage,
                &Bond {
                    amount: Uint128::one(),
                    owner: Addr::unchecked("juan".to_string()),
                    bond_id: "bond_id_1".to_string(),
                },
            )
            .unwrap();
        START_UNBOND_QUEUE
            .push_back(
                &mut deps.storage,
                &StartUnbond {
                    owner: Addr::unchecked("pepe".to_string()),
                    id: "bond_id_10".to_string(),
                    primitive_shares: Uint128::new(10),
                },
            )
            .unwrap();
        UNBOND_QUEUE
            .push_back(
                &mut deps.storage,
                &Unbond {
                    owner: Addr::unchecked("pedro".to_string()),
                    id: "bond_id_100".to_string(),
                    lp_shares: Uint128::new(1000),
                    unlock_time: Timestamp::from_seconds(1000),
                    attempted: true,
                },
            )
            .unwrap();

        let res = execute_try_icq(deps.as_mut(), env).unwrap();

        assert_eq!(res.attributes[0], attr("IBC_LOCK", "locked"));
        assert!(IBC_LOCK.load(&mut deps.storage).unwrap().is_locked());
        assert!(res.messages.is_empty());
    }

    #[test]
    fn test_execute_try_icq_ibc_unlocked_bond_queue_full() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let mock_lock = Lock::new();
        default_setup(deps.as_mut().storage).unwrap();
        let lp_cache = LpCache {
            locked_shares: Uint128::new(10),
            w_unlocked_shares: Uint128::new(10),
            d_unlocked_shares: Uint128::new(10),
        };
        LP_SHARES.save(deps.as_mut().storage, &lp_cache).unwrap();
        IBC_LOCK.save(&mut deps.storage, &mock_lock).unwrap();

        BOND_QUEUE
            .push_back(
                &mut deps.storage,
                &Bond {
                    amount: Uint128::one(),
                    owner: Addr::unchecked("juan".to_string()),
                    bond_id: "bond_id_1".to_string(),
                },
            )
            .unwrap();

        let res = execute_try_icq(deps.as_mut(), env).unwrap();
        assert_eq!(res.attributes[0], attr("bond_queue", "locked"));
        let lock = IBC_LOCK.load(&mut deps.storage).unwrap();
        assert!(lock.bond.is_locked());
        assert!(lock.start_unbond.is_unlocked());
        assert!(lock.unbond.is_unlocked());
    }

    #[test]
    fn test_execute_try_icq_ibc_bond_queue_empty() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let mock_lock = Lock::new();
        default_setup(deps.as_mut().storage).unwrap();
        let lp_cache = LpCache {
            locked_shares: Uint128::new(10),
            w_unlocked_shares: Uint128::new(10),
            d_unlocked_shares: Uint128::new(10),
        };
        LP_SHARES.save(deps.as_mut().storage, &lp_cache).unwrap();
        IBC_LOCK.save(&mut deps.storage, &mock_lock).unwrap();

        START_UNBOND_QUEUE
            .push_back(
                &mut deps.storage,
                &StartUnbond {
                    owner: Addr::unchecked("pepe".to_string()),
                    id: "bond_id_10".to_string(),
                    primitive_shares: Uint128::new(10),
                },
            )
            .unwrap();
        UNBOND_QUEUE
            .push_back(
                &mut deps.storage,
                &Unbond {
                    owner: Addr::unchecked("pedro".to_string()),
                    id: "bond_id_100".to_string(),
                    lp_shares: Uint128::new(1000),
                    unlock_time: Timestamp::from_seconds(1000),
                    attempted: true,
                },
            )
            .unwrap();

        let res = execute_try_icq(deps.as_mut(), env).unwrap();
        assert_eq!(res.attributes[0], attr("start_unbond_queue", "locked"));
        assert_eq!(res.attributes[1], attr("unbond_queue", "locked"));
        let lock = IBC_LOCK.load(&mut deps.storage).unwrap();
        assert!(lock.bond.is_unlocked());
        assert!(lock.start_unbond.is_locked());
        assert!(lock.unbond.is_locked());
    }
}
