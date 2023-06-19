use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::nonpayable;

use crate::{
    admin::is_lock_admin,
    bond::Bond,
    error::ContractError,
    helpers::{IbcMsgKind, IcaMessages},
    state::{PendingBond, RawAmount, FAILED_JOIN_QUEUE, TRAPS},
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
            IcaMessages::JoinSwapExternAmountIn(pending) => {
                handle_retry_join_pool(deps, env, pending, seq, channel)
            }
            _ => todo!(),
        },
        _ => todo!(),
    }
}

pub fn handle_retry_join_pool(
    deps: DepsMut,
    _env: Env,
    pending: PendingBond,
    seq: u64,
    channel: String,
) -> Result<Response, ContractError> {
    let mut resp = Response::new()
        .add_attribute("action", "retry")
        .add_attribute("kind", "join_pool");

    for ongoing_deposit in pending.bonds {
        match ongoing_deposit.raw_amount {
            RawAmount::LocalDenom(amount) => {
                FAILED_JOIN_QUEUE.push_back(
                    deps.storage,
                    &Bond {
                        amount,
                        owner: ongoing_deposit.owner,
                        bond_id: ongoing_deposit.bond_id.clone(),
                    },
                )?;
                resp = resp
                    .add_attribute("bond_id", ongoing_deposit.bond_id)
                    .add_attribute("amount", amount);
            }
            // We should never have LP shares here
            RawAmount::LpShares(_) => return Err(ContractError::IncorrectRawAmount),
        }
    }

    TRAPS.remove(deps.storage, (seq, channel));

    Ok(resp)
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

    resp = resp
        .add_attribute("action", "retry")
        .add_attribute("kind", "exit_pool");

    // remove the entry from traps so retrying a single failed tx cannot be triggered twice
    TRAPS.remove(deps.storage, (seq, channel));

    Ok(resp)
}

#[cfg(test)]
mod tests {
    // use cosmos_sdk_proto::tendermint::abci::ResponseQuery;
    use cosmwasm_std::{Binary, Coin, CosmosMsg, IbcMsg};
    use osmosis_std::types::cosmos::bank::v1beta1::QueryBalanceResponse;
    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::{gamm::v2::QuerySpotPriceResponse, lockup::LockedResponse},
    };

    use cosmwasm_std::{
        attr,
        testing::{mock_dependencies, mock_env},
        to_binary, Addr, Empty, StdError, Timestamp, Uint128,
    };
    use osmosis_std::types::osmosis::gamm::v1beta1::{
        QueryCalcExitPoolCoinsFromSharesResponse, QueryCalcJoinPoolSharesResponse,
    };
    use prost::Message;
    use quasar_types::icq::{CosmosResponse, InterchainQueryPacketAck};

    use crate::ibc::handle_icq_ack;
    use crate::state::PENDING_BOND_QUEUE;
    use crate::test_helpers::{create_query_response, pending_bond_to_bond};
    use crate::{
        contract::execute_try_icq,
        error::Trap,
        ibc_lock::Lock,
        state::{
            OngoingDeposit, RawAmount, Unbond, IBC_LOCK, LOCK_ADMIN, PENDING_UNBOND_QUEUE,
            UNBONDING_CLAIMS,
        },
        test_helpers::default_setup,
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

        assert!(!TRAPS.has(&deps.storage, (3539, "channel-35".to_string())));

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
                attr("kind", "exit_pool"),
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

        assert!(!TRAPS.has(&deps.storage, (3539, "channel-35".to_string())));

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
                attr("kind", "exit_pool"),
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

    #[test]
    fn test_handle_retry_join_pool_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        default_setup(deps.as_mut().storage).unwrap();

        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();

        LOCK_ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked("admin"), &Empty {})
            .unwrap();

        // mock the failed join pool trap with 3 bonds
        let failed = PendingBond {
            bonds: vec![
                OngoingDeposit {
                    claim_amount: Uint128::new(100),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(1000)),
                    owner: Addr::unchecked("address"),
                    bond_id: "1".to_string(),
                },
                OngoingDeposit {
                    claim_amount: Uint128::new(99),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(999)),
                    owner: Addr::unchecked("address"),
                    bond_id: "2".to_string(),
                },
                OngoingDeposit {
                    claim_amount: Uint128::new(101),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(1000)),
                    owner: Addr::unchecked("address"),
                    bond_id: "3".to_string(),
                },
            ],
        };

        TRAPS
            .save(
                deps.as_mut().storage,
                (3539, "channel-35".to_string()),
                &Trap {
                    error: "join pool failed on osmosis".to_string(),
                    step: IbcMsgKind::Ica(IcaMessages::JoinSwapExternAmountIn(failed.clone())),
                    last_succesful: true,
                },
            )
            .unwrap();

        // manually trigger retry join pool
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

        assert!(!TRAPS.has(&deps.storage, (3539, "channel-35".to_string())));

        assert_eq!(
            res.attributes,
            vec![
                attr("action", "retry"),
                attr("kind", "join_pool"),
                attr("bond_id", "1"),
                attr("amount", Uint128::new(1000)),
                attr("bond_id", "2"),
                attr("amount", Uint128::new(999)),
                attr("bond_id", "3"),
                attr("amount", Uint128::new(1000)),
            ]
        );

        // check that the failed join queue has the same mocked bonds
        let failed_join_queue: Result<Vec<Bond>, StdError> =
            FAILED_JOIN_QUEUE.iter(&deps.storage).unwrap().collect();

        let failed_bonds = vec![
            Bond {
                amount: Uint128::new(1000),
                owner: Addr::unchecked("address"),
                bond_id: "1".to_string(),
            },
            Bond {
                amount: Uint128::new(999),
                owner: Addr::unchecked("address"),
                bond_id: "2".to_string(),
            },
            Bond {
                amount: Uint128::new(1000),
                owner: Addr::unchecked("address"),
                bond_id: "3".to_string(),
            },
        ];

        assert_eq!(failed_join_queue.as_ref().unwrap(), &failed_bonds);

        // manually trigger try_icq
        let res = execute_try_icq(deps.as_mut(), env.clone());
        assert_eq!(res.unwrap().messages.len(), 1);

        // mocking the ICQ ACK
        let raw_balance = create_query_response(
            QueryBalanceResponse {
                balance: Some(OsmoCoin {
                    denom: "uatom".to_string(),
                    amount: "1000".to_string(),
                }),
            }
            .encode_to_vec(),
        );

        let quote_balance = create_query_response(
            QueryBalanceResponse {
                balance: Some(OsmoCoin {
                    denom: "uosmo".to_string(),
                    amount: "1000".to_string(),
                }),
            }
            .encode_to_vec(),
        );

        let lp_balance = create_query_response(
            QueryBalanceResponse {
                balance: Some(OsmoCoin {
                    denom: "uosmo".to_string(),
                    amount: "1000".to_string(),
                }),
            }
            .encode_to_vec(),
        );

        let join_pool = create_query_response(
            QueryCalcJoinPoolSharesResponse {
                share_out_amount: "123".to_string(),
                tokens_out: vec![OsmoCoin {
                    denom: "uosmo".to_string(),
                    amount: "123".to_string(),
                }],
            }
            .encode_to_vec(),
        );

        let exit_pool = create_query_response(
            QueryCalcExitPoolCoinsFromSharesResponse {
                tokens_out: vec![
                    OsmoCoin {
                        // base denom
                        denom: "uosmo".to_string(),
                        amount: "123".to_string(),
                    },
                    OsmoCoin {
                        // quote denom
                        denom: "uqsr".to_string(),
                        amount: "123".to_string(),
                    },
                ],
            }
            .encode_to_vec(),
        );

        let spot_price = create_query_response(
            QuerySpotPriceResponse {
                spot_price: "123".to_string(),
            }
            .encode_to_vec(),
        );

        let lock = create_query_response(LockedResponse { lock: None }.encode_to_vec());

        let ibc_ack = InterchainQueryPacketAck {
            data: Binary::from(
                &CosmosResponse {
                    responses: vec![
                        raw_balance,
                        quote_balance,
                        lp_balance,
                        join_pool,
                        exit_pool,
                        spot_price,
                        lock,
                    ],
                }
                .encode_to_vec()[..],
            ),
        };

        let res = handle_icq_ack(deps.as_mut().storage, env, to_binary(&ibc_ack).unwrap()).unwrap();

        // get the failed pending bonds total amount
        let pending_total_amount = failed.bonds.iter().fold(Uint128::zero(), |acc, bond| {
            let amount = match bond.raw_amount {
                RawAmount::LocalDenom(amount) => amount,
                RawAmount::LpShares(_) => panic!("unexpected lp shares"),
            };
            acc + amount
        });

        // check that the res amount matches the amount in the failed join queue
        match &res.messages[0].msg {
            CosmosMsg::Ibc(IbcMsg::Transfer { amount, .. }) => {
                assert_eq!(
                    amount,
                    &Coin {
                        denom: "ibc/local_osmo".to_string(),
                        amount: pending_total_amount,
                    }
                );
            }
            _ => panic!("unexpected message type"),
        }

        // check that the failed join queue is emptied
        assert!(FAILED_JOIN_QUEUE.is_empty(&deps.storage).unwrap());
    }

    #[test]
    fn test_handle_retry_join_pool_with_pending_deposits_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        default_setup(deps.as_mut().storage).unwrap();

        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();

        LOCK_ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked("admin"), &Empty {})
            .unwrap();

        // mock the failed join pool trap with 3 bonds
        let failed = PendingBond {
            bonds: vec![
                OngoingDeposit {
                    claim_amount: Uint128::new(100),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(1000)),
                    owner: Addr::unchecked("address"),
                    bond_id: "1".to_string(),
                },
                OngoingDeposit {
                    claim_amount: Uint128::new(99),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(999)),
                    owner: Addr::unchecked("address"),
                    bond_id: "2".to_string(),
                },
                OngoingDeposit {
                    claim_amount: Uint128::new(101),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(1000)),
                    owner: Addr::unchecked("address"),
                    bond_id: "3".to_string(),
                },
            ],
        };

        TRAPS
            .save(
                deps.as_mut().storage,
                (3539, "channel-35".to_string()),
                &Trap {
                    error: "join pool failed on osmosis".to_string(),
                    step: IbcMsgKind::Ica(IcaMessages::JoinSwapExternAmountIn(failed.clone())),
                    last_succesful: true,
                },
            )
            .unwrap();

        // mock pending deposits and add them to the pending queue
        let pedning_bonds = vec![
            Bond {
                amount: Uint128::new(5_000),
                owner: Addr::unchecked("address"),
                bond_id: "1".to_string(),
            },
            Bond {
                amount: Uint128::new(10_000),
                owner: Addr::unchecked("address"),
                bond_id: "2".to_string(),
            },
        ];

        for bond in pedning_bonds.iter() {
            PENDING_BOND_QUEUE
                .push_back(deps.as_mut().storage, bond)
                .unwrap();
        }

        // manually trigger retry join pool
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

        assert!(!TRAPS.has(&deps.storage, (3539, "channel-35".to_string())));

        // failed bonds amount should not be included in the response as funds are already 
        // in Osmosis
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "retry"),
                attr("kind", "join_pool"),
                attr("bond_id", "1"),
                attr("amount", Uint128::new(1000)),
                attr("bond_id", "2"),
                attr("amount", Uint128::new(999)),
                attr("bond_id", "3"),
                attr("amount", Uint128::new(1000)),
            ]
        );

        // check that the failed join queue has the same mocked bonds
        let failed_join_queue: Result<Vec<Bond>, StdError> =
            FAILED_JOIN_QUEUE.iter(&deps.storage).unwrap().collect();        
        assert_eq!(failed_join_queue.unwrap(), pending_bond_to_bond(&failed));

        // manually trigger try_icq
        let res = execute_try_icq(deps.as_mut(), env.clone());
        assert_eq!(res.unwrap().messages.len(), 1);

        // mocking the ICQ ACK
        let raw_balance = create_query_response(
            QueryBalanceResponse {
                balance: Some(OsmoCoin {
                    denom: "uatom".to_string(),
                    amount: "1000".to_string(),
                }),
            }
            .encode_to_vec(),
        );

        let quote_balance = create_query_response(
            QueryBalanceResponse {
                balance: Some(OsmoCoin {
                    denom: "uosmo".to_string(),
                    amount: "1000".to_string(),
                }),
            }
            .encode_to_vec(),
        );

        let lp_balance = create_query_response(
            QueryBalanceResponse {
                balance: Some(OsmoCoin {
                    denom: "uosmo".to_string(),
                    amount: "1000".to_string(),
                }),
            }
            .encode_to_vec(),
        );

        let join_pool = create_query_response(
            QueryCalcJoinPoolSharesResponse {
                share_out_amount: "123".to_string(),
                tokens_out: vec![OsmoCoin {
                    denom: "uosmo".to_string(),
                    amount: "123".to_string(),
                }],
            }
            .encode_to_vec(),
        );

        let exit_pool = create_query_response(
            QueryCalcExitPoolCoinsFromSharesResponse {
                tokens_out: vec![
                    OsmoCoin {
                        // base denom
                        denom: "uosmo".to_string(),
                        amount: "123".to_string(),
                    },
                    OsmoCoin {
                        // quote denom
                        denom: "uqsr".to_string(),
                        amount: "123".to_string(),
                    },
                ],
            }
            .encode_to_vec(),
        );

        let spot_price = create_query_response(
            QuerySpotPriceResponse {
                spot_price: "123".to_string(),
            }
            .encode_to_vec(),
        );

        let lock = create_query_response(LockedResponse { lock: None }.encode_to_vec());

        let ibc_ack = InterchainQueryPacketAck {
            data: Binary::from(
                &CosmosResponse {
                    responses: vec![
                        raw_balance,
                        quote_balance,
                        lp_balance,
                        join_pool,
                        exit_pool,
                        spot_price,
                        lock,
                    ],
                }
                .encode_to_vec()[..],
            ),
        };

        let res = handle_icq_ack(deps.as_mut().storage, env, to_binary(&ibc_ack).unwrap()).unwrap();

        // get the failed pending bonds total amount
        let failed_total_amount = failed.bonds.iter().fold(Uint128::zero(), |acc, bond| {
            let amount = match bond.raw_amount {
                RawAmount::LocalDenom(amount) => amount,
                RawAmount::LpShares(_) => panic!("unexpected lp shares"),
            };
            acc + amount
        });

        // get the pending bonds total amount
        let pending_total_amount = pedning_bonds.iter().fold(Uint128::zero(), |acc, bond| {
            acc + bond.amount
        });

        // check that the res amount matches the amount in both queues
        match &res.messages[0].msg {
            CosmosMsg::Ibc(IbcMsg::Transfer { amount, .. }) => {
                assert_eq!(
                    amount,
                    &Coin {
                        denom: "ibc/local_osmo".to_string(),
                        amount: failed_total_amount + pending_total_amount,
                    }
                );
            }
            _ => panic!("unexpected message type"),
        }

        // check that the failed join & pending queues are emptied
        assert!(FAILED_JOIN_QUEUE.is_empty(&deps.storage).unwrap());
        assert!(PENDING_BOND_QUEUE.is_empty(&deps.storage).unwrap());
    }
}
