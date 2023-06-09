use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::nonpayable;

use crate::{
    admin::is_lock_admin,
    error::ContractError,
    helpers::{IbcMsgKind, IcaMessages},
    state::TRAPS,
    unbond::{do_unbond, PendingReturningUnbonds},
};

/// The retry entry point will be used to retry any failed ICA message given the sequence number and the channel.
/// Depending on the type of ICA message, the contract will handle the retry differently.
/// Funds cannot be sent and, for now, only the lock admin can call retry.
pub fn execute_retry(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    seq: u64,
    channel: String,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    // for now, only the lock admin can retry
    is_lock_admin(deps.storage, &deps.querier, &env, &info.sender)?;

    let traps = TRAPS.load(deps.storage, (seq, channel.clone()))?;
    match traps.step {
        IbcMsgKind::Ica(ica_kind) => match ica_kind {
            IcaMessages::ExitPool(pending) => {
                handle_retry_exit_pool(deps, env, pending, seq, channel)
            }
            _ => todo!(),
        },
        _ => todo!(),
    }
}

/// The handle retry exit pool checks that pending unbonds is not empty and then iterates over the pending unbonds vector.
/// For each unbond, it will check that unbond time has expired and push it to the front of the pending unbond queue.
/// A manual TryIcq will be needed to dispatch the IBC message.
pub fn handle_retry_exit_pool(
    deps: DepsMut,
    env: Env,
    pending: PendingReturningUnbonds,
    seq: u64,
    channel: String,
) -> Result<Response, ContractError> {
    if pending.unbonds.is_empty() {
        return Err(ContractError::NoPendingUnbonds);
    }

    let mut resp = Response::new();

    for pu in pending.unbonds {
        do_unbond(deps.storage, &env, pu.owner.clone(), pu.id.clone())?;
        resp = resp
            .add_attribute("unbond", pu.owner.clone())
            .add_attribute("unbond_id", pu.id);
    }

    resp = resp.add_attribute("action", "retry");

    // remove the entry from traps so retrying a single failed tx cannot be triggered twice
    TRAPS.remove(deps.storage, (seq, channel));

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        attr,
        testing::{mock_dependencies, mock_env},
        Addr, Empty, Timestamp, Uint128,
    };

    use crate::{
        error::Trap,
        state::{RawAmount, Unbond, LOCK_ADMIN, PENDING_UNBOND_QUEUE, UNBONDING_CLAIMS},
        unbond::ReturningUnbond,
    };

    use super::*;

    #[test]
    fn test_handle_retry_exit_pool_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let pending = PendingReturningUnbonds {
            unbonds: vec![
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(101)),
                    owner: Addr::unchecked("owner1"),
                    id: "1".to_string(),
                },
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(102)),
                    owner: Addr::unchecked("owner2"),
                    id: "2".to_string(),
                },
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(103)),
                    owner: Addr::unchecked("owner3"),
                    id: "3".to_string(),
                },
            ],
        };

        TRAPS
            .save(
                deps.as_mut().storage,
                (3539, "channel-35".to_string()),
                &Trap {
                    error: "exit pool failed on osmosis".to_string(),
                    step: IbcMsgKind::Ica(IcaMessages::ExitPool(pending)),
                    last_succesful: true,
                },
            )
            .unwrap();

        LOCK_ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked("admin"), &Empty {})
            .unwrap();

        UNBONDING_CLAIMS
            .save(
                deps.as_mut().storage,
                (Addr::unchecked("owner1"), "1".to_string()),
                &Unbond {
                    lp_shares: Uint128::new(101),
                    unlock_time: Timestamp::from_seconds(1000),
                    attempted: true,
                    owner: Addr::unchecked("owner1"),
                    id: "1".to_string(),
                },
            )
            .unwrap();

        UNBONDING_CLAIMS
            .save(
                deps.as_mut().storage,
                (Addr::unchecked("owner2"), "2".to_string()),
                &Unbond {
                    lp_shares: Uint128::new(101),
                    unlock_time: Timestamp::from_seconds(1000),
                    attempted: true,
                    owner: Addr::unchecked("owner2"),
                    id: "2".to_string(),
                },
            )
            .unwrap();

        UNBONDING_CLAIMS
            .save(
                deps.as_mut().storage,
                (Addr::unchecked("owner3"), "3".to_string()),
                &Unbond {
                    lp_shares: Uint128::new(103),
                    unlock_time: Timestamp::from_seconds(1000),
                    attempted: true,
                    owner: Addr::unchecked("owner3"),
                    id: "3".to_string(),
                },
            )
            .unwrap();

        let res = execute_retry(
            deps.as_mut(),
            env,
            MessageInfo {
                sender: Addr::unchecked("admin"),
                funds: vec![],
            },
            3539,
            "channel-35".to_string(),
        )
        .unwrap();

        assert_eq!(
            res.attributes,
            vec![
                attr("unbond", "owner1"),
                attr("unbond_id", "1"),
                attr("unbond", "owner2"),
                attr("unbond_id", "2"),
                attr("unbond", "owner3"),
                attr("unbond_id", "3"),
                attr("action", "retry"),
            ]
        );

        assert_eq!(PENDING_UNBOND_QUEUE.len(&deps.storage).unwrap(), 3);
        assert_eq!(
            PENDING_UNBOND_QUEUE.back(&deps.storage).unwrap().unwrap(),
            Unbond {
                lp_shares: Uint128::new(103),
                unlock_time: Timestamp::from_seconds(1000),
                attempted: true,
                owner: Addr::unchecked("owner3"),
                id: "3".to_string(),
            }
        );
    }

    #[test]
    fn test_handle_retry_exit_pool_empty_pending_unbonds_fails() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let pending = PendingReturningUnbonds { unbonds: vec![] };

        TRAPS
            .save(
                deps.as_mut().storage,
                (3539, "channel-35".to_string()),
                &Trap {
                    error: "exit pool failed on osmosis".to_string(),
                    step: IbcMsgKind::Ica(IcaMessages::ExitPool(pending)),
                    last_succesful: true,
                },
            )
            .unwrap();

        LOCK_ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked("admin"), &Empty {})
            .unwrap();

        UNBONDING_CLAIMS
            .save(
                deps.as_mut().storage,
                (Addr::unchecked("owner1"), "1".to_string()),
                &Unbond {
                    lp_shares: Uint128::new(101),
                    unlock_time: Timestamp::from_seconds(1000),
                    attempted: true,
                    owner: Addr::unchecked("owner1"),
                    id: "1".to_string(),
                },
            )
            .unwrap();

        let res = execute_retry(
            deps.as_mut(),
            env,
            MessageInfo {
                sender: Addr::unchecked("admin"),
                funds: vec![],
            },
            3539,
            "channel-35".to_string(),
        )
        .unwrap_err();

        assert_eq!(res, ContractError::NoPendingUnbonds,);
        assert!(PENDING_UNBOND_QUEUE.is_empty(&deps.storage).unwrap());
    }

    #[test]
    fn test_handle_retry_exit_pool_without_all_unbonding_claims_fails() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let pending = PendingReturningUnbonds {
            unbonds: vec![
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(101)),
                    owner: Addr::unchecked("owner1"),
                    id: "1".to_string(),
                },
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(102)),
                    owner: Addr::unchecked("owner2"),
                    id: "2".to_string(),
                },
            ],
        };

        TRAPS
            .save(
                deps.as_mut().storage,
                (3539, "channel-35".to_string()),
                &Trap {
                    error: "exit pool failed on osmosis".to_string(),
                    step: IbcMsgKind::Ica(IcaMessages::ExitPool(pending)),
                    last_succesful: true,
                },
            )
            .unwrap();

        LOCK_ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked("admin"), &Empty {})
            .unwrap();

        UNBONDING_CLAIMS
            .save(
                deps.as_mut().storage,
                (Addr::unchecked("owner1"), "1".to_string()),
                &Unbond {
                    lp_shares: Uint128::new(101),
                    unlock_time: Timestamp::from_seconds(1000),
                    attempted: true,
                    owner: Addr::unchecked("owner1"),
                    id: "1".to_string(),
                },
            )
            .unwrap();

        let res = execute_retry(
            deps.as_mut(),
            env,
            MessageInfo {
                sender: Addr::unchecked("admin"),
                funds: vec![],
            },
            3539,
            "channel-35".to_string(),
        );

        assert!(res.is_err());

        // even though the unbonding claim for owner2 is missing, the retry will still add the unbonding claim for owner1
        assert_eq!(PENDING_UNBOND_QUEUE.len(&deps.storage).unwrap(), 1);
        assert_eq!(
            PENDING_UNBOND_QUEUE.back(&deps.storage).unwrap().unwrap(),
            Unbond {
                lp_shares: Uint128::new(101),
                unlock_time: Timestamp::from_seconds(1000),
                attempted: true,
                owner: Addr::unchecked("owner1"),
                id: "1".to_string(),
            }
        );
    }

    #[test]
    fn test_handle_retry_exit_pool_as_not_admin_fails() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        LOCK_ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked("admin"), &Empty {})
            .unwrap();

        let res = execute_retry(
            deps.as_mut(),
            env,
            MessageInfo {
                sender: Addr::unchecked("not_admin"),
                funds: vec![],
            },
            3539,
            "channel-35".to_string(),
        );

        assert!(res.is_err());
    }

    #[test]
    fn test_handle_retry_exit_pool_with_wrong_seq_channel_fails() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let pending = PendingReturningUnbonds {
            unbonds: vec![
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(101)),
                    owner: Addr::unchecked("owner1"),
                    id: "1".to_string(),
                },
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(102)),
                    owner: Addr::unchecked("owner2"),
                    id: "2".to_string(),
                },
            ],
        };

        TRAPS
            .save(
                deps.as_mut().storage,
                (3539, "channel-35".to_string()),
                &Trap {
                    error: "exit pool failed on osmosis".to_string(),
                    step: IbcMsgKind::Ica(IcaMessages::ExitPool(pending)),
                    last_succesful: true,
                },
            )
            .unwrap();

        LOCK_ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked("admin"), &Empty {})
            .unwrap();

        let res = execute_retry(
            deps.as_mut(),
            env,
            MessageInfo {
                sender: Addr::unchecked("admin"),
                funds: vec![],
            },
            0,
            "random_channel".to_string(),
        );

        assert!(res.is_err());
    }

    #[test]
    fn test_handle_retry_exit_pool_twice_fails() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let pending = PendingReturningUnbonds {
            unbonds: vec![
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(101)),
                    owner: Addr::unchecked("owner1"),
                    id: "1".to_string(),
                },
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(102)),
                    owner: Addr::unchecked("owner2"),
                    id: "2".to_string(),
                },
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(103)),
                    owner: Addr::unchecked("owner3"),
                    id: "3".to_string(),
                },
            ],
        };

        TRAPS
            .save(
                deps.as_mut().storage,
                (3539, "channel-35".to_string()),
                &Trap {
                    error: "exit pool failed on osmosis".to_string(),
                    step: IbcMsgKind::Ica(IcaMessages::ExitPool(pending)),
                    last_succesful: true,
                },
            )
            .unwrap();

        LOCK_ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked("admin"), &Empty {})
            .unwrap();

        UNBONDING_CLAIMS
            .save(
                deps.as_mut().storage,
                (Addr::unchecked("owner1"), "1".to_string()),
                &Unbond {
                    lp_shares: Uint128::new(101),
                    unlock_time: Timestamp::from_seconds(1000),
                    attempted: true,
                    owner: Addr::unchecked("owner1"),
                    id: "1".to_string(),
                },
            )
            .unwrap();

        UNBONDING_CLAIMS
            .save(
                deps.as_mut().storage,
                (Addr::unchecked("owner2"), "2".to_string()),
                &Unbond {
                    lp_shares: Uint128::new(101),
                    unlock_time: Timestamp::from_seconds(1000),
                    attempted: true,
                    owner: Addr::unchecked("owner2"),
                    id: "2".to_string(),
                },
            )
            .unwrap();

        UNBONDING_CLAIMS
            .save(
                deps.as_mut().storage,
                (Addr::unchecked("owner3"), "3".to_string()),
                &Unbond {
                    lp_shares: Uint128::new(103),
                    unlock_time: Timestamp::from_seconds(1000),
                    attempted: true,
                    owner: Addr::unchecked("owner3"),
                    id: "3".to_string(),
                },
            )
            .unwrap();

        let res = execute_retry(
            deps.as_mut(),
            env.clone(),
            MessageInfo {
                sender: Addr::unchecked("admin"),
                funds: vec![],
            },
            3539,
            "channel-35".to_string(),
        )
        .unwrap();

        assert_eq!(
            res.attributes,
            vec![
                attr("unbond", "owner1"),
                attr("unbond_id", "1"),
                attr("unbond", "owner2"),
                attr("unbond_id", "2"),
                attr("unbond", "owner3"),
                attr("unbond_id", "3"),
                attr("action", "retry"),
            ]
        );

        assert_eq!(PENDING_UNBOND_QUEUE.len(&deps.storage).unwrap(), 3);
        assert_eq!(
            PENDING_UNBOND_QUEUE.back(&deps.storage).unwrap().unwrap(),
            Unbond {
                lp_shares: Uint128::new(103),
                unlock_time: Timestamp::from_seconds(1000),
                attempted: true,
                owner: Addr::unchecked("owner3"),
                id: "3".to_string(),
            }
        );

        // execute retry for same seq & channel should fail
        let res = execute_retry(
            deps.as_mut(),
            env,
            MessageInfo {
                sender: Addr::unchecked("admin"),
                funds: vec![],
            },
            3539,
            "channel-35".to_string(),
        );

        assert!(res.is_err());
    }
}
