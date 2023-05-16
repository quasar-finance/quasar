#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, Attribute, DepsMut, Env, IbcMsg, IbcPacketAckMsg, MessageInfo, QuerierWrapper,
    Reply, Response, Storage, Uint128,
};
use cw2::set_contract_version;
use cw_utils::{must_pay, nonpayable};

use quasar_types::ibc::IcsAck;

use crate::admin::check_depositor;
use crate::bond::do_bond;
use crate::error::ContractError;
use crate::helpers::{create_callback_submsg, is_contract_admin, lock_try_icq, SubMsgKind};
use crate::ibc::{handle_failing_ack, handle_succesful_ack, on_packet_timeout};
use crate::ibc_lock::Lock;
use crate::ibc_util::{do_ibc_join_pool_swap_extern_amount_in, do_transfer};
use crate::icq::try_icq;
use crate::msg::{ExecuteMsg, InstantiateMsg, LockOnly, MigrateMsg, UnlockOnly};
use crate::reply::{handle_ack_reply, handle_callback_reply, handle_ibc_reply};
use crate::start_unbond::{do_start_unbond, StartUnbond};
use crate::state::{
    Config, LpCache, OngoingDeposit, RawAmount, ADMIN, CONFIG, DEPOSITOR, IBC_LOCK, ICA_CHANNEL,
    LP_SHARES, OSMO_LOCK, PENDING_ACK, REPLIES, RETURNING, TIMED_OUT, TOTAL_VAULT_BALANCE,
};
use crate::unbond::{do_unbond, finish_unbond, PendingReturningUnbonds};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:lp-strategy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;

    // ADMIN here is only used to decide who can deposit
    ADMIN.save(deps.storage, &info.sender)?;

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

    OSMO_LOCK.save(deps.storage, &u64::MAX)?;

    LP_SHARES.save(
        deps.storage,
        &LpCache {
            locked_shares: Uint128::zero(),
            w_unlocked_shares: Uint128::zero(),
            d_unlocked_shares: Uint128::zero(),
        },
    )?;

    TOTAL_VAULT_BALANCE.save(deps.storage, &Uint128::zero())?;

    TIMED_OUT.save(deps.storage, &false)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    // Save the ibc message together with the sequence number, to be handled properly later at the ack, we can pass the ibc_kind one to one
    // TODO this needs and error check and error handling
    let reply = REPLIES.load(deps.storage, msg.id)?;
    match reply {
        SubMsgKind::Ibc(pending, channel) => handle_ibc_reply(deps, msg, pending, channel),
        SubMsgKind::Ack(seq, channel) => handle_ack_reply(deps, msg, seq, channel),
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
        ExecuteMsg::AcceptReturningFunds { id, pending } => {
            execute_accept_returning_funds(deps.storage, deps.querier, info, id, pending)
        }
        ExecuteMsg::CloseChannel { channel_id } => execute_close_channel(deps, channel_id),
        ExecuteMsg::Ack { ack } => execute_ack(deps, env, info, ack),
        ExecuteMsg::TryIcq {} => execute_try_icq(deps, env),
        ExecuteMsg::SetDepositor { depositor } => execute_set_depositor(deps, info, depositor),
        ExecuteMsg::Unlock { unlock_only } => execute_unlock(deps, env, info, unlock_only),
        ExecuteMsg::Lock { lock_only } => execute_lock(deps, env, info, lock_only),
        ExecuteMsg::ManualTimeout {
            seq,
            channel,
            should_unlock,
        } => manual_timeout(deps, env, info, seq, channel, should_unlock),
    }
}

pub fn execute_lock(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    lock_only: LockOnly,
) -> Result<Response, ContractError> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;
    let mut lock = IBC_LOCK.load(deps.storage)?;

    match lock_only {
        LockOnly::Bond => lock = lock.lock_bond(),
        LockOnly::StartUnbond => lock = lock.lock_start_unbond(),
        LockOnly::Unbond => lock = lock.lock_unbond(),
        LockOnly::Migration => lock = lock.lock_migration(),
    };
    IBC_LOCK.save(deps.storage, &lock)?;

    Ok(Response::new().add_attribute("lock_only", lock_only.to_string()))
}

pub fn execute_unlock(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    unlock_only: UnlockOnly,
) -> Result<Response, ContractError> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;
    let mut lock = IBC_LOCK.load(deps.storage)?;

    match unlock_only {
        UnlockOnly::Bond => lock = lock.unlock_bond(),
        UnlockOnly::StartUnbond => lock = lock.unlock_start_unbond(),
        UnlockOnly::Unbond => lock = lock.unlock_unbond(),
        UnlockOnly::Migration => lock = lock.unlock_migration(),
    };
    IBC_LOCK.save(deps.storage, &lock)?;

    Ok(Response::new().add_attribute("unlock_only", unlock_only.to_string()))
}

pub fn manual_timeout(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sequence: u64,
    channel: String,
    should_unlock: bool,
) -> Result<Response, ContractError> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;

    let response = on_packet_timeout(
        deps,
        sequence,
        channel,
        "timeout".to_string(),
        should_unlock,
    )?;

    Ok(Response::new()
        .add_attributes(response.attributes)
        .add_events(response.events)
        .add_submessages(response.messages))
}

pub fn execute_set_depositor(
    deps: DepsMut,
    info: MessageInfo,
    depositor: String,
) -> Result<Response, ContractError> {
    if info.sender == ADMIN.load(deps.storage)? {
        let depositor_addr = deps.api.addr_validate(depositor.as_str())?;
        DEPOSITOR.save(deps.storage, &depositor_addr)?;
        Ok(Response::new().add_attribute("set-depositor", depositor))
    } else {
        Err(ContractError::Unauthorized)
    }
}

pub fn execute_try_icq(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    // if we're unlocked, we can empty the queues and send the submessages
    let msg = try_icq(deps.storage, deps.querier, env)?;
    let res = lock_try_icq(deps, msg)?;
    Ok(res.add_attribute("action", "try_icq"))
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

pub fn execute_accept_returning_funds(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    info: MessageInfo,
    id: u64,
    pending: PendingReturningUnbonds,
) -> Result<Response, ContractError> {
    let returning_amount = RETURNING
        .may_load(storage, id)?
        .ok_or(ContractError::ReturningTransferNotFound)?;

    let amount = must_pay(&info, CONFIG.load(storage)?.local_denom.as_str())?;
    if amount != returning_amount {
        return Err(ContractError::ReturningTransferIncorrectAmount);
    }

    let mut callback_submsgs = vec![];
    for unbond in pending.unbonds.iter() {
        let cosmos_msg = finish_unbond(storage, querier, unbond)?;
        callback_submsgs.push(create_callback_submsg(
            storage,
            cosmos_msg,
            unbond.owner.clone(),
            unbond.id.clone(),
        )?)
    }

    Ok(Response::new()
        .add_attribute("callback-submsgs", callback_submsgs.len().to_string())
        .add_attribute("returning-transfer", id.to_string())
        .add_attribute("success", "true")
        .add_submessages(callback_submsgs))
}

pub fn execute_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    if !check_depositor(deps.storage, &info.sender)? {
        return Err(ContractError::Unauthorized);
    }

    let amount = must_pay(&info, &CONFIG.load(deps.storage)?.local_denom)?;

    let msg = do_bond(
        deps.storage,
        deps.querier,
        env,
        amount,
        info.sender.clone(),
        id,
    )?;

    let attributes = vec![
        Attribute::new("action", "bond"),
        Attribute::new("sender", info.sender),
        Attribute::new("token_amount", amount),
    ];

    let res = lock_try_icq(deps, msg)?;
    Ok(res.add_attributes(attributes))
}

pub fn execute_start_unbond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    share_amount: Uint128,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    if !check_depositor(deps.storage, &info.sender)? {
        return Err(ContractError::Unauthorized);
    }

    do_start_unbond(
        deps.storage,
        StartUnbond {
            owner: info.sender.clone(),
            id: id.clone(),
            primitive_shares: share_amount,
        },
    )?;

    let msg = try_icq(deps.storage, deps.querier, env)?;
    let attributes = vec![
        Attribute::new("action", "start-unbond"),
        Attribute::new("sender", info.sender),
        Attribute::new("prim_share_amount", share_amount),
        Attribute::new("unbond_id", id),
    ];

    let res = lock_try_icq(deps, msg)?;
    Ok(res.add_attributes(attributes))
}

pub fn execute_unbond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    if !check_depositor(deps.storage, &info.sender)? {
        return Err(ContractError::Unauthorized);
    }

    do_unbond(deps.storage, &env, info.sender.clone(), id.clone())?;

    let msg = try_icq(deps.storage, deps.querier, env)?;

    let attributes = vec![
        Attribute::new("action", "unbond"),
        Attribute::new("sender", info.sender),
        Attribute::new("unbond_id", id),
    ];

    let res = lock_try_icq(deps, msg)?;
    Ok(res.add_attributes(attributes))
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
    // remove old pending acks
    for key in msg.delete_pending.clone() {
        PENDING_ACK.remove(deps.storage, key)
    }

    Ok(Response::new()
        .add_attribute("migrate", CONTRACT_NAME)
        .add_attribute("success", "true")
        .add_attribute("removed", msg.delete_pending.len().to_string()))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        attr, coins,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Timestamp,
    };
    use cw_utils::PaymentError;

    use crate::{
        bond::Bond,
        state::{
            Unbond, BOND_QUEUE, PENDING_BOND_QUEUE, PENDING_UNBOND_QUEUE, SHARES,
            START_UNBOND_QUEUE, UNBONDING_CLAIMS, UNBOND_QUEUE,
        },
        test_helpers::default_setup,
    };

    use super::*;

    #[test]
    fn migrate_msg_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let entries = vec![
            (
                (1, "channel-1".to_string()),
                crate::helpers::IbcMsgKind::Ica(crate::helpers::IcaMessages::ExitPool(
                    PendingReturningUnbonds { unbonds: vec![] },
                )),
            ),
            (
                (2, "channel-1".to_string()),
                crate::helpers::IbcMsgKind::Ica(crate::helpers::IcaMessages::ExitPool(
                    PendingReturningUnbonds { unbonds: vec![] },
                )),
            ),
            (
                (1, "channel-3".to_string()),
                crate::helpers::IbcMsgKind::Ica(crate::helpers::IcaMessages::ExitPool(
                    PendingReturningUnbonds { unbonds: vec![] },
                )),
            ),
            (
                (1, "channel-1".to_string()),
                crate::helpers::IbcMsgKind::Icq,
            ),
            (
                (1, "channel-2".to_string()),
                crate::helpers::IbcMsgKind::Icq,
            ),
            (
                (1, "channel-4".to_string()),
                crate::helpers::IbcMsgKind::Icq,
            ),
        ];

        for (key, value) in entries.clone() {
            PENDING_ACK
                .save(deps.as_mut().storage, key, &value)
                .unwrap();
        }

        let msg = MigrateMsg {
            delete_pending: entries.iter().map(|(key, _)| key.clone()).collect(),
        };

        migrate(deps.as_mut(), env, msg).unwrap();
        assert!(PENDING_ACK.is_empty(deps.as_ref().storage))
    }

    #[test]
    fn test_execute_try_icq_ibc_locked() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let mock_lock = Lock::new().lock_bond().lock_start_unbond().lock_unbond();
        IBC_LOCK.save(&mut deps.storage, &mock_lock).unwrap();
        let res = execute_try_icq(deps.as_mut(), env).unwrap();

        assert_eq!(res.attributes[0], attr("IBC_LOCK", "locked"));
        assert!(res.messages.is_empty());
        assert!(IBC_LOCK.load(&deps.storage).unwrap().is_locked());
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
        assert!(IBC_LOCK.load(&deps.storage).unwrap().is_unlocked());
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
        assert!(IBC_LOCK.load(&deps.storage).unwrap().is_locked());
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
        let lock = IBC_LOCK.load(&deps.storage).unwrap();
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
        let lock = IBC_LOCK.load(&deps.storage).unwrap();
        assert!(lock.bond.is_unlocked());
        assert!(lock.start_unbond.is_locked());
        assert!(lock.unbond.is_unlocked());
    }

    #[test]
    fn test_start_unbond_with_funds() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("pepe", &coins(420, "uqsr"));
        let msg = ExecuteMsg::StartUnbond {
            id: "bond_id_1".to_string(),
            share_amount: Uint128::new(69),
        };

        let res = execute(deps.as_mut(), env, info, msg);
        assert_eq!(res.unwrap_err(), PaymentError::NonPayable {}.into());
    }

    #[test]
    fn test_unbond_with_funds() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr0000", &coins(420, "uqsr"));
        let msg = ExecuteMsg::Unbond {
            id: "unbond_id".to_string(),
        };

        let res = execute(deps.as_mut(), env, info, msg);
        assert_eq!(res.unwrap_err(), PaymentError::NonPayable {}.into());
    }

    #[test]
    fn test_execute_try_icq_locked() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up locked IBC state
        IBC_LOCK
            .save(deps.as_mut().storage, &Lock::new().lock_bond())
            .unwrap();

        let res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                ("IBC_LOCK", "locked"),
                ("kind", "queue"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when IBC is locked"
        );
    }

    #[test]
    fn test_execute_try_icq_unlocked_empty_queues() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();

        // Empty bond, start_unbond, and unbond queues, so we insert nothing
        let res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                ("IBC_LOCK", "unlocked"),
                ("kind", "dispatch"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when IBC is unlocked and queues are empty"
        );
    }

    #[test]
    fn test_execute__start_unbond_with_bond_queue() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();
        DEPOSITOR
            .save(deps.as_mut().storage, &Addr::unchecked("vault-bob"))
            .unwrap();

        let info = MessageInfo {
            sender: Addr::unchecked("vault-bob"),
            funds: vec![],
        };
        let id = "4".to_string();

        SHARES
            .save(
                deps.as_mut().storage,
                Addr::unchecked("vault-bob"),
                &Uint128::new(10000),
            )
            .unwrap();

        BOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Bond {
                    amount: Uint128::new(1000),
                    owner: Addr::unchecked("alice"),
                    bond_id: "2".to_string(),
                },
            )
            .unwrap();

        let _ = execute_start_unbond(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            id.clone(),
            Uint128::new(1000),
        )
        .unwrap();
        assert_eq!(
            IBC_LOCK.load(deps.as_ref().storage).unwrap(),
            Lock::new().lock_bond()
        )
    }

    #[test]
    fn test_execute_bond_and_execute_try_icq_filled_queues() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();
        DEPOSITOR
            .save(deps.as_mut().storage, &Addr::unchecked("vault-bob"))
            .unwrap();

        let info = MessageInfo {
            sender: Addr::unchecked("vault-bob"),
            funds: coins(1000, "ibc/local_osmo"),
        };
        let id = "4".to_string();

        BOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Bond {
                    amount: Uint128::new(1000),
                    owner: Addr::unchecked("vault-bob"),
                    bond_id: "1".to_string(),
                },
            )
            .unwrap();
        START_UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &StartUnbond {
                    owner: Addr::unchecked("vault-bob"),
                    id: "2".to_string(),
                    primitive_shares: Uint128::new(1000),
                },
            )
            .unwrap();
        UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Unbond {
                    lp_shares: Uint128::new(10000),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                    owner: Addr::unchecked("vault-bob"),
                    id: "3".to_string(),
                },
            )
            .unwrap();
        PENDING_UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Unbond {
                    lp_shares: Uint128::new(10000),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                    owner: Addr::unchecked("vault-bob"),
                    id: "5".to_string(),
                },
            )
            .unwrap();

        // Call execute_bond
        let bond_res = execute_bond(deps.as_mut(), env.clone(), info.clone(), id.clone()).unwrap();

        // Verify that the bond operation was queued
        assert_eq!(
            bond_res.attributes,
            vec![
                ("bond_queue", "locked"),
                ("kind", "dispatch"),
                ("action", "bond"),
                ("sender", "vault-bob"),
                ("token_amount", "1000")
            ],
            "Unexpected attributes when executing bond"
        );
        assert_eq!(
            IBC_LOCK.load(deps.as_mut().storage).unwrap(),
            Lock::new().lock_bond()
        );

        // Call execute_try_icq
        let try_icq_res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();

        // Verify that the bond queue is locked
        assert_eq!(
            try_icq_res.attributes,
            vec![
                ("IBC_LOCK", "locked"),
                ("kind", "queue"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when executing try_icq"
        );
        assert_eq!(try_icq_res.messages.len(), 0)
    }

    #[test]
    fn test_execute_try_icq_filled_queues() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();
        DEPOSITOR
            .save(deps.as_mut().storage, &Addr::unchecked("vault-bob"))
            .unwrap();

        let info = MessageInfo {
            sender: Addr::unchecked("vault-bob"),
            funds: coins(1000, "ibc/local_osmo"),
        };
        let id = "4".to_string();

        BOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Bond {
                    amount: Uint128::new(1000),
                    owner: Addr::unchecked("vault-bob"),
                    bond_id: "1".to_string(),
                },
            )
            .unwrap();
        START_UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &StartUnbond {
                    owner: Addr::unchecked("vault-bob"),
                    id: "2".to_string(),
                    primitive_shares: Uint128::new(1000),
                },
            )
            .unwrap();
        UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Unbond {
                    lp_shares: Uint128::new(10000),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                    owner: Addr::unchecked("vault-bob"),
                    id: "3".to_string(),
                },
            )
            .unwrap();
        PENDING_UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Unbond {
                    lp_shares: Uint128::new(10000),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                    owner: Addr::unchecked("vault-bob"),
                    id: "5".to_string(),
                },
            )
            .unwrap();

        // Call execute_try_icq
        let try_icq_res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();

        // Verify that the bond queue was locked
        assert_eq!(
            try_icq_res.attributes,
            vec![
                ("bond_queue", "locked"),
                ("kind", "dispatch"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when executing try_icq"
        );
        assert_eq!(
            IBC_LOCK.load(deps.as_mut().storage).unwrap(),
            Lock::new().lock_bond()
        );

        assert_eq!(try_icq_res.messages.len(), 1);

        // Call execute_bond
        let bond_res = execute_bond(deps.as_mut(), env.clone(), info.clone(), id.clone()).unwrap();

        // Verify that the bond operation was queued
        assert_eq!(
            bond_res.attributes,
            vec![
                ("IBC_LOCK", "locked"),
                ("kind", "queue"),
                ("action", "bond"),
                ("sender", "vault-bob"),
                ("token_amount", "1000")
            ],
            "Unexpected attributes when executing bond"
        );
    }

    #[test]
    fn test_execute_unbond_and_execute_try_icq_filled_queues() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();
        DEPOSITOR
            .save(deps.as_mut().storage, &Addr::unchecked("vault-bob"))
            .unwrap();

        let info = MessageInfo {
            sender: Addr::unchecked("vault-bob"),
            funds: vec![],
        };
        let id = "4".to_string();

        BOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Bond {
                    amount: Uint128::new(1000),
                    owner: Addr::unchecked("vault-bob"),
                    bond_id: "1".to_string(),
                },
            )
            .unwrap();
        START_UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &StartUnbond {
                    owner: Addr::unchecked("vault-bob"),
                    id: "2".to_string(),
                    primitive_shares: Uint128::new(1000),
                },
            )
            .unwrap();
        PENDING_UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Unbond {
                    lp_shares: Uint128::new(10000),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                    owner: Addr::unchecked("vault-bob"),
                    id: "5".to_string(),
                },
            )
            .unwrap();
        UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Unbond {
                    lp_shares: Uint128::new(10000),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                    owner: Addr::unchecked("vault-bob"),
                    id: "3".to_string(),
                },
            )
            .unwrap();
        PENDING_UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Unbond {
                    lp_shares: Uint128::new(10000),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                    owner: Addr::unchecked("vault-bob"),
                    id: "5".to_string(),
                },
            )
            .unwrap();

        UNBONDING_CLAIMS
            .save(
                deps.as_mut().storage,
                (Addr::unchecked("vault-bob"), "4".to_string()),
                &Unbond {
                    lp_shares: Uint128::new(10000),
                    unlock_time: Timestamp::from_seconds(1000),
                    attempted: false,
                    owner: Addr::unchecked("vault-bob"),
                    id: "4".to_string(),
                },
            )
            .unwrap();

        // Call execute_try_icq
        let try_icq_res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();

        assert_eq!(
            IBC_LOCK.load(deps.as_mut().storage).unwrap(),
            Lock::new().lock_bond()
        );

        // Verify that the bond queue was locked
        assert_eq!(
            try_icq_res.attributes,
            vec![
                ("bond_queue", "locked"),
                ("kind", "dispatch"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when executing try_icq"
        );
        assert_eq!(try_icq_res.messages.len(), 1);

        // Call execute_unbond
        let unbond_res =
            execute_unbond(deps.as_mut(), env.clone(), info.clone(), id.clone()).unwrap();

        // Verify that the unbond operation was queued
        assert_eq!(
            unbond_res.attributes,
            vec![
                ("IBC_LOCK", "locked"),
                ("kind", "queue"),
                ("action", "unbond"),
                ("sender", "vault-bob"),
                ("unbond_id", "4")
            ],
            "Unexpected attributes when executing unbond"
        )
    }

    #[test]
    fn test_execute_try_icq_unlocked_pending_bond_queue_not_empty() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();

        // Add an item to the pending bond queue
        PENDING_BOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Bond {
                    amount: Uint128::new(1000),
                    owner: Addr::unchecked("bob"),
                    bond_id: "1".to_string(),
                },
            )
            .unwrap();

        let res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                ("bond_queue", "locked"),
                ("kind", "dispatch"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when IBC is unlocked and bond queue is not empty"
        );
        assert_eq!(res.messages.len(), 1)
    }

    #[test]
    fn test_execute_try_icq_unlocked_start_unbond_queue_not_empty() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();

        // Add an item to the bond queue
        START_UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &StartUnbond {
                    owner: Addr::unchecked("alice"),
                    id: "2".to_string(),
                    primitive_shares: Uint128::new(1000),
                },
            )
            .unwrap();

        let res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                ("start_unbond_queue", "locked"),
                ("kind", "dispatch"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when IBC is unlocked and start_unbond queue is not empty"
        );
        assert_eq!(res.messages.len(), 1)
    }

    #[test]
    fn test_execute_try_icq_unlocked_pending_unbond_queue_not_empty() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();

        // Add an item to the pending bond queue
        PENDING_UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Unbond {
                    owner: Addr::unchecked("alice"),
                    id: "2".to_string(),
                    lp_shares: Uint128::new(1000),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                },
            )
            .unwrap();

        let res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                ("unbond_queue", "locked"),
                ("kind", "dispatch"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when IBC is unlocked and start_unbond queue is not empty"
        );
        assert_eq!(res.messages.len(), 1)
    }

    #[test]
    fn test_execute_try_icq_unlocked_unbond_queue_not_empty() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();

        // Add an item to the unbond queue
        UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Unbond {
                    owner: Addr::unchecked("alice"),
                    id: "2".to_string(),
                    lp_shares: Uint128::new(1000),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                },
            )
            .unwrap();

        let res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                ("unbond_queue", "locked"),
                ("kind", "dispatch"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when IBC is unlocked and start_unbond queue is not empty"
        );
        assert_eq!(res.messages.len(), 1)
    }

    #[test]
    fn test_execute_try_icq_unlocked_bond_start_unbond_queue_not_empty() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();

        // Add an item to the bond queue
        BOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Bond {
                    amount: Uint128::new(1000),
                    owner: Addr::unchecked("bob"),
                    bond_id: "1".to_string(),
                },
            )
            .unwrap();
        START_UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &StartUnbond {
                    owner: Addr::unchecked("alice"),
                    id: "2".to_string(),
                    primitive_shares: Uint128::new(1000),
                },
            )
            .unwrap();

        let res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                ("bond_queue", "locked"),
                ("kind", "dispatch"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when IBC is unlocked and bond queue is not empty"
        );
        assert_eq!(
            IBC_LOCK.load(deps.as_mut().storage).unwrap(),
            Lock::new().lock_bond()
        )
    }

    #[test]
    fn test_execute_try_icq_unlocked_bond_unbond_queue_not_empty() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();

        // Add an item to the bond queue and unbond queue
        BOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Bond {
                    amount: Uint128::new(1000),
                    owner: Addr::unchecked("bob"),
                    bond_id: "1".to_string(),
                },
            )
            .unwrap();
        UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Unbond {
                    owner: Addr::unchecked("alice"),
                    id: "2".to_string(),
                    lp_shares: Uint128::new(1000),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                },
            )
            .unwrap();

        let res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                ("bond_queue", "locked"),
                ("kind", "dispatch"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when IBC is unlocked and bond queue is not empty"
        );
        assert_eq!(
            IBC_LOCK.load(deps.as_mut().storage).unwrap(),
            Lock::new().lock_bond()
        )
    }

    #[test]
    fn test_execute_try_icq_unlocked_start_unbond_unbond_queue_not_empty() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Set up unlocked IBC state
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();
        default_setup(deps.as_mut().storage).unwrap();

        // Add an item to the start unbond queue and unbond queue
        START_UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &StartUnbond {
                    owner: Addr::unchecked("alice"),
                    id: "2".to_string(),
                    primitive_shares: Uint128::new(1000),
                },
            )
            .unwrap();
        UNBOND_QUEUE
            .push_back(
                deps.as_mut().storage,
                &Unbond {
                    owner: Addr::unchecked("alice"),
                    id: "2".to_string(),
                    lp_shares: Uint128::new(1000),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                },
            )
            .unwrap();

        let res = execute_try_icq(deps.as_mut(), env.clone()).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                ("start_unbond_queue", "locked"),
                ("kind", "dispatch"),
                ("action", "try_icq")
            ],
            "Unexpected attributes when IBC is unlocked and bond queue is not empty"
        );
        assert_eq!(
            IBC_LOCK.load(deps.as_mut().storage).unwrap(),
            Lock::new().lock_start_unbond()
        )
    }
}
