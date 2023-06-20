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
            PENDING_BOND_QUEUE, TRAPS,
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
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::{gamm::v2::QuerySpotPriceResponse, lockup::LockedResponse},
    };
    use proptest::collection::vec;

    proptest! {

        #[test]
        fn test_handle_retry_join_pool_with_pending_deposits_works(
            (claim_amount, raw_amount, owner, bond_id) in (0usize..3).prop_flat_map(|size|
                (
                    // to avoid overflows, we limit the amounts to u32. also force amounts to be different than 0
                    vec(any::<u32>().prop_map(|x| (x as u128).max(1)), size..=size),
                    vec(any::<u32>().prop_map(|x| (x as u128).max(1)), size..=size),
                    vec("[a-z]+", size..=size),  // adjust the regex pattern based on your requirements
                    vec(any::<u32>().prop_map(|x| (x as u128).max(1)), size..=size),
                )
            ),
            raw_balalance_rq in any::<u32>(),
            quote_balance_rq in any::<u32>(),
            lp_balance_rq in any::<u32>(),
            join_pool_rq in any::<u32>(),
            exit_pool_base_rq in any::<u32>(),
            exit_pool_quote_rq in any::<u32>(),
            spot_price_rq in any::<u32>(),
            _lock_rq in any::<u32>(),
        ) {
            // println!("claim_amount: {:?}, raw_amount: {:?}, owner: {:?}, bond_id: {:?}", claim_amount, raw_amount, owner, bond_id);
            let mut deps = mock_dependencies();
            let env = mock_env();
            default_setup(deps.as_mut().storage).unwrap();

            IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();

            LOCK_ADMIN
                .save(deps.as_mut().storage, &Addr::unchecked("admin"), &Empty {})
                .unwrap();

            // mock the failed join pool trap with 3 bonds
            let failed = PendingBond {
                bonds: claim_amount.iter().zip(&raw_amount).zip(&owner).zip(&bond_id).map(|(((claim, raw), owner), id)| {
                    OngoingDeposit {
                        claim_amount: Uint128::new(*claim),
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
            assert_eq!(res.unwrap().messages.len(), 1);

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
                        denom: lp_balance_rq.to_string(),
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
                        amount: join_pool_rq.to_string(),
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
                    spot_price: spot_price_rq.to_string(),
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
}
