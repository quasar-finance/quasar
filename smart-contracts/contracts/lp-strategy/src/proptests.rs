#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        attr,
        testing::{mock_dependencies, mock_env},
        to_binary, Addr, Binary, Coin, CosmosMsg, Empty, IbcMsg, MessageInfo, StdError, Uint128,
    };
    use proptest::prelude::*;
    use prost::Message;
    use quasar_types::icq::{CosmosResponse, InterchainQueryPacketAck};

    use crate::{
        bond::Bond,
        contract::execute_try_icq,
        error::Trap,
        execute::execute_retry,
        helpers::{IbcMsgKind, IcaMessages},
        ibc::handle_icq_ack,
        ibc_lock::Lock,
        state::{
            OngoingDeposit, PendingBond, RawAmount, FAILED_JOIN_QUEUE, IBC_LOCK, LOCK_ADMIN,
            PENDING_BOND_QUEUE, REJOIN_QUEUE, TRAPS,
        },
        test_helpers::{create_query_response, default_setup, pending_bond_to_bond},
    };
    use osmosis_std::types::{
        cosmos::bank::v1beta1::QueryBalanceResponse,
        osmosis::gamm::v1beta1::{
            QueryCalcExitPoolCoinsFromSharesResponse, QueryCalcJoinPoolSharesResponse,
        },
    };
    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin as OsmoCoin, osmosis::gamm::v2::QuerySpotPriceResponse,
    };
    use proptest::collection::vec;

    proptest! {

        #[test]
        fn test_handle_retry_join_pool_with_pending_deposits_works(
            // values to mock failed join pool
            (claim_amount, raw_amount, owner, bond_id) in (0usize..100).prop_flat_map(|size|
                (
                    // to avoid overflows, we limit the amounts to u64. also force amounts & bond_ids to be >= 1
                    vec(any::<u64>().prop_map(|x| (x as u128).max(1)), size..=size),
                    vec(any::<u64>().prop_map(|x| (x as u128).max(1)), size..=size),
                    vec("[a-z]+", size..=size),
                    vec(any::<u64>().prop_map(|x| (x as u128).max(1)), size..=size),
                )
            ),
            // values to mock pending deposits
            (amount_pd, owner_pd, bond_id_pd) in (0usize..100).prop_flat_map(|size|
                (
                    // to avoid overflows, we limit the amounts to u64. also force amounts & bond_ids to be >= 1
                    vec(any::<u64>().prop_map(|x| (x as u128).max(1)), size..=size),
                    vec("[a-z]+", size..=size),
                    vec(any::<u64>().prop_map(|x| (x as u128).max(1)), size..=size),
                )
            ),
            // values to mock ICQ ACK
            raw_balalance_rq in any::<u64>(),
            quote_balance_rq in any::<u64>(),
            lp_balance_rq in any::<u64>(),
            join_pool_rq in any::<u64>(),
            exit_pool_base_rq in any::<u64>(),
            exit_pool_quote_rq in any::<u64>(),
            spot_price_rq in any::<u64>(),
        ) {
            let mut deps = mock_dependencies();
            let env = mock_env();
            default_setup(deps.as_mut().storage).unwrap();

            IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();

            LOCK_ADMIN
                .save(deps.as_mut().storage, &Addr::unchecked("admin"), &Empty {})
                .unwrap();

            // mock the failed join pool trap with 3 bonds
            let failed = PendingBond {
                bonds: claim_amount.iter().zip(&raw_amount).zip(&owner).zip(&bond_id).map(|(((_claim, raw), owner), id)| {
                    OngoingDeposit {
                        claim_amount: Uint128::new(*raw),
                        raw_amount: RawAmount::LocalDenom(Uint128::new(*raw)),
                        owner: Addr::unchecked(owner),
                        bond_id: id.to_string(),
                    }
                }).collect(),
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
            let pending_bonds: Vec<Bond> = amount_pd.iter().zip(&owner_pd).zip(&bond_id_pd).map(|((amount, owner), id)| {
                Bond {
                    amount: Uint128::new(*amount),
                    owner: Addr::unchecked(owner),
                    bond_id: id.to_string(),
                }
            }).collect();

            for bond in pending_bonds.iter() {
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

        prop_assert!(!TRAPS.has(&deps.storage, (3539, "channel-35".to_string())));

        let mut attributes = vec![
            attr("action", "retry"),
            attr("kind", "join_pool"),
            ];

            for bond in &failed.bonds {
                if let RawAmount::LocalDenom(amount) = bond.raw_amount {
                    attributes.push(attr("bond_id", &bond.bond_id));
                    attributes.push(attr("amount", amount));
                }
            }

            prop_assert_eq!(
                res.attributes,
                attributes
            );

            // check that the failed join queue has the same mocked bonds
            let failed_join_queue: Result<Vec<Bond>, StdError> =
            FAILED_JOIN_QUEUE.iter(&deps.storage).unwrap().collect();
            prop_assert_eq!(failed_join_queue.unwrap(), pending_bond_to_bond(&failed));


            // manually trigger try_icq
            let res = execute_try_icq(deps.as_mut(), env.clone());
            prop_assert_eq!(res.unwrap().messages.len(), 1);

            // mocking the ICQ ACK
            let raw_balance = create_query_response(
                QueryBalanceResponse {
                    balance: Some(OsmoCoin {
                        denom: "uatom".to_string(),
                        amount: raw_balalance_rq.to_string(),
                    }),
                }
                .encode_to_vec(),
            );

            let quote_balance = create_query_response(
                QueryBalanceResponse {
                    balance: Some(OsmoCoin {
                        denom: "uosmo".to_string(),
                        amount: quote_balance_rq.to_string(),
                    }),
                }
                .encode_to_vec(),
            );

            let lp_balance = create_query_response(
                QueryBalanceResponse {
                    balance: Some(OsmoCoin {
                        denom: "uosmo".to_string(),
                        amount: lp_balance_rq.to_string(),
                    }),
                }
                .encode_to_vec(),
            );

            let exit_pool = create_query_response(
                QueryCalcExitPoolCoinsFromSharesResponse {
                    tokens_out: vec![
                        OsmoCoin {
                            // base denom
                            denom: "uosmo".to_string(),
                            amount: exit_pool_base_rq.to_string(),
                        },
                        OsmoCoin {
                            // quote denom
                            denom: "uqsr".to_string(),
                            amount: exit_pool_quote_rq.to_string(),
                        },
                    ],
                }
                .encode_to_vec(),
            );

            let spot_price = create_query_response(
                QuerySpotPriceResponse {
                    spot_price: (spot_price_rq+1).to_string(),
                }
                .encode_to_vec(),
            );

            let join_pool = create_query_response(
                QueryCalcJoinPoolSharesResponse {
                    share_out_amount: "123".to_string(),
                    tokens_out: vec![OsmoCoin {
                        denom: "uosmo".to_string(),
                        amount: join_pool_rq.to_string(),
                    }],
                }
                .encode_to_vec(),
            );

            // LockResponse is fixed to None in this test for simplicity
            // let lock = create_query_response(LockedResponse { lock: None }.encode_to_vec());

            let ibc_ack = InterchainQueryPacketAck {
                data: Binary::from(
                    &CosmosResponse {
                        responses: vec![
                            raw_balance,
                            quote_balance,
                            lp_balance,
                            exit_pool,
                            spot_price,
                            join_pool,
                            // lock,
                        ],
                    }
                    .encode_to_vec()[..],
                ),
            };

            // simulate that we received the ICQ ACK
            let res = handle_icq_ack(deps.as_mut().storage, env, to_binary(&ibc_ack).unwrap()).unwrap();

            // get the pending bonds total amount
            let pending_total_amount = pending_bonds.iter().fold(Uint128::zero(), |acc, bond| {
                acc + bond.amount
            });

            // check that the res amount matches the amount in the pending queue ONLY
            // only if there are messages
            if !res.messages.is_empty() {
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
                };
             }


            // if BOND_QUEUE & REJOIN_QUEUE are empty FAILED_JOIN_QUEUE items are not moved to REJOIN_QUEUE
            if !pending_bonds.is_empty() && !failed.bonds.is_empty() {
                prop_assert!(FAILED_JOIN_QUEUE.is_empty(&deps.storage).unwrap());
            }

            // PENDING_BOND_QUEUE should be empty
            prop_assert!(PENDING_BOND_QUEUE.is_empty(&deps.storage).unwrap());

            // failed bonds should be now in the REJOIN_QUEUE
            let rejoin_queue: Result<Vec<OngoingDeposit>, StdError> =
            REJOIN_QUEUE.iter(&deps.storage).unwrap().collect();

            // only check when there's pending bonds & failed bonds
            if !pending_bonds.is_empty() && !failed.bonds.is_empty() {
                assert_eq!(failed.bonds, rejoin_queue.unwrap());
            }
        }
    }
}
