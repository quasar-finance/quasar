#[cfg(test)]
mod tests {
    use std::{marker::PhantomData, primitive};

    use cosmwasm_std::{
        from_binary,
        testing::{
            mock_dependencies, mock_dependencies_with_balances, mock_env, mock_info, MockApi,
            MockQuerier, MockStorage,
        },
        to_binary, Addr, Api, Binary, Coin, ContractResult, CustomQuery, Decimal, DepsMut, Empty,
        Env, MessageInfo, OwnedDeps, Querier, QuerierResult, QueryRequest, Response, Storage,
        Timestamp, Uint128,
    };
    use cw20::BalanceResponse;
    use cw_multi_test::next_block;
    use lp_strategy::{
        msg::{ConfigResponse, IcaBalanceResponse, PrimitiveSharesResponse},
        start_unbond::StartUnbond,
        state::Config,
    };
    use quasar_types::callback::{BondResponse, StartUnbondResponse};

    use crate::{
        contract::execute,
        contract::instantiate,
        contract::query,
        execute::may_pay_with_ratio,
        msg::{ExecuteMsg, InstantiateMsg, InvestmentResponse, PrimitiveConfig, PrimitiveInitMsg},
    };

    pub struct QuasarQuerier {
        pub primitive_states: Vec<(String, String, Uint128, Uint128)>,
    }

    impl QuasarQuerier {
        pub fn find_states_for_primitive(&self, address: String) -> (String, Uint128, Uint128) {
            let mut total_share = Uint128::zero();
            let mut total_balance = Uint128::zero();
            let mut this_denom = "".to_string();
            for (addr, denom, share, balance) in &self.primitive_states {
                if addr.eq(&address) {
                    this_denom = denom.to_string();
                    total_share = share.clone();
                    total_balance = balance.clone();
                }
            }
            (this_denom, total_share, total_balance)
        }
    }

    impl Querier for QuasarQuerier {
        fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
            let request: QueryRequest<Empty> = from_binary(&Binary::from(bin_request)).unwrap();
            match request {
                QueryRequest::Wasm(wasm_query) => match wasm_query {
                    cosmwasm_std::WasmQuery::Smart { contract_addr, msg } => {
                        if let primitive_query =
                            from_binary::<lp_strategy::msg::QueryMsg>(&msg).unwrap()
                        {
                            let (this_denom, total_share, total_balance) =
                                self.find_states_for_primitive(contract_addr);
                            match primitive_query {
                                lp_strategy::msg::QueryMsg::PrimitiveShares {} => {
                                    let response = PrimitiveSharesResponse { total: total_share };
                                    QuerierResult::Ok(ContractResult::Ok(
                                        to_binary(&response).unwrap(),
                                    ))
                                }
                                lp_strategy::msg::QueryMsg::IcaBalance {} => {
                                    let response = IcaBalanceResponse {
                                        amount: Coin {
                                            denom: this_denom.clone(),
                                            amount: total_balance,
                                        },
                                    };
                                    QuerierResult::Ok(ContractResult::Ok(
                                        to_binary(&response).unwrap(),
                                    ))
                                }
                                lp_strategy::msg::QueryMsg::Config {} => {
                                    let config = Config {
                                        lock_period: 14,
                                        pool_id: 1,
                                        pool_denom: "gamm/pool/1".to_string(),
                                        local_denom: this_denom.to_string(),
                                        base_denom: "uosmo".to_string(),
                                        quote_denom: "uatom".to_string(),
                                        transfer_channel: "channel-0".to_string(),
                                        return_source_channel: "channel-0".to_string(),
                                        expected_connection: "connection-0".to_string(),
                                    };
                                    QuerierResult::Ok(ContractResult::Ok(
                                        to_binary(&ConfigResponse { config }).unwrap(),
                                    ))
                                }
                                _ => QuerierResult::Err(
                                    cosmwasm_std::SystemError::UnsupportedRequest {
                                        kind: format!(
                                            "Unmocked primitive query type: {:?}",
                                            primitive_query
                                        ),
                                    },
                                ),
                            }
                        } else {
                            QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                                kind: format!("Unmocked primitive query type: {:?}", msg),
                            })
                        }
                    }
                    _ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                        kind: format!("Unmocked wasm query type: {:?}", wasm_query),
                    }),
                },
                _ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                    kind: format!("Unmocked query type: {:?}", request),
                }),
            }
            // QuerierResult::Ok(ContractResult::Ok(to_binary(&"hello").unwrap()))
        }
    }

    fn mock_deps_with_primitives(
        primitive_states: Vec<(String, String, Uint128, Uint128)>,
    ) -> OwnedDeps<MockStorage, MockApi, QuasarQuerier, Empty> {
        OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: QuasarQuerier { primitive_states },
            custom_query_type: PhantomData,
        }
    }

    const TEST_CREATOR: &str = "creator";

    fn init_msg() -> InstantiateMsg {
        InstantiateMsg {
            name: "Blazar Vault".to_string(),
            symbol: "BLZR".to_string(),
            decimals: 6,
            min_withdrawal: Uint128::one(),
            primitives: vec![PrimitiveConfig {
                weight: Decimal::one(),
                address: "quasar123".to_string(),
                init: PrimitiveInitMsg::LP(lp_strategy::msg::InstantiateMsg {
                    lock_period: 14, // this is supposed to be nanos i think
                    pool_id: 1,
                    pool_denom: "gamm/pool/1".to_string(),
                    local_denom: "ibc/uosmo".to_string(),
                    base_denom: "uosmo".to_string(),
                    quote_denom: "uatom".to_string(),
                    transfer_channel: "channel-0".to_string(),
                    return_source_channel: "channel-0".to_string(),
                    expected_connection: "connection-0".to_string(),
                }),
            }],
        }
    }

    // convenience function to init primitives with a vec of tuples which are (local_denom, weight)

    fn init_msg_with_primitive_details(
        primitive_details: Vec<(String, String, Decimal)>,
    ) -> InstantiateMsg {
        InstantiateMsg {
            name: "Blazar Vault".to_string(),
            symbol: "BLZR".to_string(),
            decimals: 6,
            min_withdrawal: Uint128::one(),
            primitives: primitive_details
                .iter()
                .map(|pd| {
                    PrimitiveConfig {
                        weight: pd.2,
                        address: pd.0.clone(),
                        init: PrimitiveInitMsg::LP(lp_strategy::msg::InstantiateMsg {
                            lock_period: 14, // this is supposed to be nanos i think
                            pool_id: 1,
                            pool_denom: "gamm/pool/1".to_string(),
                            local_denom: pd.1.clone(),
                            base_denom: "uosmo".to_string(),
                            quote_denom: "uatom".to_string(),
                            transfer_channel: "channel-0".to_string(),
                            return_source_channel: "channel-0".to_string(),
                            expected_connection: "connection-0".to_string(),
                        }),
                    }
                })
                .collect(),
        }
    }

    fn init<'a>(deps: DepsMut, msg: &InstantiateMsg, env: &Env, info: &MessageInfo) -> Response {
        let res = instantiate(deps, env.clone(), info.clone(), msg.clone()).unwrap();

        res
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_deps_with_primitives(even_primitives());
        let msg = init_msg();
        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &msg, &env, &info);

        assert_eq!(0, res.messages.len());
    }

    // #[test]
    // fn proper_on_bond_callback() {
    //     let mut deps = mock_dependencies_with_balances(&[]);
    //     let msg = init_msg();
    //     let info = mock_info(TEST_CREATOR, &[]);
    //     let env = mock_env();
    //     _ = init(deps.as_mut(), &msg, &env, &info);

    //     let execute_msg = ExecuteMsg::BondResponse(BondResponse {
    //         share_amount: Uint128::from(100u128),
    //         bond_id: Uint128::from(1u128).to_string(),
    //     });

    //     let res = execute(deps.as_mut(), env, info, execute_msg).unwrap();
    //     assert_eq!(0, res.messages.len());
    // }

    fn even_primitives() -> Vec<(String, String, Uint128, Uint128)> {
        vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Uint128::from(100u128),
                Uint128::from(100u128),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Uint128::from(100u128),
                Uint128::from(100u128),
            ),
            (
                "quasar125".to_string(),
                "ibc/ustars".to_string(),
                Uint128::from(100u128),
                Uint128::from(100u128),
            ),
        ]
    }

    fn even_primitive_details() -> Vec<(String, String, Decimal)> {
        vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Decimal::one(),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Decimal::one(),
            ),
            (
                "quasar125".to_string(),
                "ibc/ustars".to_string(),
                Decimal::one(),
            ),
        ]
    }

    fn even_deposit() -> Vec<Coin> {
        vec![
            Coin {
                denom: "ibc/uosmo".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "ibc/uatom".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "ibc/ustars".to_string(),
                amount: Uint128::from(100u128),
            },
        ]
    }

    #[test]
    fn test_may_pay_with_even_ratio() {
        let mut deps = mock_deps_with_primitives(even_primitives());
        let init_msg = init_msg_with_primitive_details(even_primitive_details());
        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(0, res.messages.len());

        let invest_query = crate::msg::QueryMsg::Investment {};
        let query_res = query(deps.as_ref(), env.clone(), invest_query).unwrap();

        let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

        let (coins, remainder) =
            may_pay_with_ratio(&deps.as_ref(), &even_deposit(), investment_response.info).unwrap();

        println!("coins: {:?}", coins);
        println!("remainder: {:?}", remainder);
        assert_eq!(coins.len(), 3);
        assert_eq!(coins[0].amount, Uint128::from(99u128)); // 99 because 0.33333 results in coins getting floored
        assert_eq!(coins[1].amount, Uint128::from(99u128));
        assert_eq!(coins[2].amount, Uint128::from(99u128));

        assert_eq!(remainder.len(), 3);
        assert_eq!(remainder[0].amount, Uint128::from(1u128));
        assert_eq!(remainder[1].amount, Uint128::from(1u128));
        assert_eq!(remainder[2].amount, Uint128::from(1u128));
    }

    // #[test]
    // fn test_may_pay_with_uneven_ratio() {
    //     let mut deps = mock_deps_with_primitives(vec![
    //         (
    //             "quasar123".to_string(),
    //             "ibc/uosmo".to_string(),
    //             Uint128::from(1000u128),
    //             Uint128::from(1000u128),
    //         ),
    //         (
    //             "quasar124".to_string(),
    //             "ibc/uatom".to_string(),
    //             Uint128::from(500u128),
    //             Uint128::from(1000u128),
    //         ),
    //         (
    //             "quasar125".to_string(),
    //             "ibc/ustars".to_string(),
    //             Uint128::from(250u128),
    //             Uint128::from(100u128),
    //         ),
    //     ]);
    //     let init_msg = init_msg_with_primitive_details(vec![
    //         (
    //             "quasar123".to_string(),
    //             "ibc/uosmo".to_string(),
    //             Decimal::one(),
    //         ),
    //         (
    //             "quasar124".to_string(),
    //             "ibc/uatom".to_string(),
    //             Decimal::one(),
    //         ),
    //         (
    //             "quasar125".to_string(),
    //             "ibc/ustars".to_string(),
    //             Decimal::from_ratio(3u128, 10u128),
    //         ),
    //     ]);
    //     let info = mock_info(TEST_CREATOR, &[]);
    //     let env = mock_env();
    //     let res = init(deps.as_mut(), &init_msg, &env, &info);
    //     assert_eq!(0, res.messages.len());

    //     let invest_query = crate::msg::QueryMsg::Investment {};
    //     let query_res = query(deps.as_ref(), env.clone(), invest_query).unwrap();

    //     let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

    //     let (coins, remainder) = may_pay_with_ratio(
    //         &deps.as_ref(),
    //         &vec![
    //             Coin {
    //                 denom: "ibc/uosmo".to_string(),
    //                 amount: Uint128::from(100u128),
    //             },
    //             Coin {
    //                 denom: "ibc/uatom".to_string(),
    //                 amount: Uint128::from(100u128),
    //             },
    //             Coin {
    //                 denom: "ibc/ustars".to_string(),
    //                 amount: Uint128::from(100u128),
    //             },
    //         ],
    //         investment_response.info,
    //     )
    //     .unwrap();

    //     println!("coins: {:?}", coins);
    //     println!("remainder: {:?}", remainder);
    //     assert_eq!(coins.len(), 3);
    //     assert_eq!(coins[0].amount, Uint128::from(36u128));
    //     assert_eq!(coins[1].amount, Uint128::from(73u128));
    //     assert_eq!(coins[2].amount, Uint128::from(4u128));

    //     assert_eq!(remainder.len(), 3);
    //     assert_eq!(remainder[0].amount, Uint128::from(1u128));
    //     assert_eq!(remainder[1].amount, Uint128::from(1u128));
    //     assert_eq!(remainder[2].amount, Uint128::from(1u128));
    // }

    #[test]
    fn proper_bond_response_callback() {
        let mut deps = mock_deps_with_primitives(even_primitives());
        let init_msg = init_msg_with_primitive_details(even_primitive_details());
        let info = mock_info(TEST_CREATOR, &even_deposit());
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(0, res.messages.len());

        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };
        let res = execute(deps.as_mut(), env.clone(), info, deposit_msg).unwrap();
        assert_eq!(res.messages.len(), 6);
        assert_eq!(res.attributes.first().unwrap().value, "1");
        // todo: verify message passed back here

        // in this scenario we expect 1000/1000 * 100 = 100 shares back from each primitive
        let primitive_1_info = mock_info("quasar123", &[]);
        let primitive_1_msg = ExecuteMsg::BondResponse(BondResponse {
            share_amount: 100u128.into(),
            bond_id: "1".to_string(),
        });
        let p1_res = execute(
            deps.as_mut(),
            env.clone(),
            primitive_1_info,
            primitive_1_msg,
        )
        .unwrap();
        assert_eq!(p1_res.messages.len(), 0);

        let primitive_2_info = mock_info("quasar124", &[]);
        let primitive_2_msg = ExecuteMsg::BondResponse(BondResponse {
            share_amount: 100u128.into(),
            bond_id: "1".to_string(),
        });
        let p2_res = execute(
            deps.as_mut(),
            env.clone(),
            primitive_2_info,
            primitive_2_msg,
        )
        .unwrap();
        assert_eq!(p2_res.messages.len(), 0);

        let primitive_3_info = mock_info("quasar125", &[]);
        let primitive_3_msg = ExecuteMsg::BondResponse(BondResponse {
            share_amount: 100u128.into(),
            bond_id: "1".to_string(),
        });
        let p3_res = execute(
            deps.as_mut(),
            env.clone(),
            primitive_3_info,
            primitive_3_msg,
        )
        .unwrap();
        assert_eq!(p3_res.messages.len(), 0);

        let balance_query = crate::msg::QueryMsg::Balance {
            address: TEST_CREATOR.to_string(),
        };
        let balance_res = query(deps.as_ref(), env.clone(), balance_query).unwrap();
        let balance: BalanceResponse = from_binary(&balance_res).unwrap();

        assert_eq!(balance.balance, Uint128::from(99u128));
    }

    #[test]
    fn proper_unbond() {
        let mut deps = mock_deps_with_primitives(even_primitives());
        let init_msg = init_msg_with_primitive_details(even_primitive_details());
        let info = mock_info(TEST_CREATOR, &even_deposit());
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(0, res.messages.len());

        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };
        let res = execute(deps.as_mut(), env.clone(), info, deposit_msg).unwrap();
        assert_eq!(res.messages.len(), 6);
        assert_eq!(res.attributes.first().unwrap().value, "1");
        // todo: verify message passed back here

        // in this scenario we expect 1000/1000 * 100 = 100 shares back from each primitive
        let primitive_1_info = mock_info("quasar123", &[]);
        let primitive_1_msg = ExecuteMsg::BondResponse(BondResponse {
            share_amount: 100u128.into(),
            bond_id: "1".to_string(),
        });
        let p1_res = execute(
            deps.as_mut(),
            env.clone(),
            primitive_1_info.clone(),
            primitive_1_msg,
        )
        .unwrap();
        assert_eq!(p1_res.messages.len(), 0);

        let primitive_2_info = mock_info("quasar124", &[]);
        let primitive_2_msg = ExecuteMsg::BondResponse(BondResponse {
            share_amount: 100u128.into(),
            bond_id: "1".to_string(),
        });
        let p2_res = execute(
            deps.as_mut(),
            env.clone(),
            primitive_2_info.clone(),
            primitive_2_msg,
        )
        .unwrap();
        assert_eq!(p2_res.messages.len(), 0);

        let primitive_3_info = mock_info("quasar125", &[]);
        let primitive_3_msg = ExecuteMsg::BondResponse(BondResponse {
            share_amount: 100u128.into(),
            bond_id: "1".to_string(),
        });
        let p3_res = execute(
            deps.as_mut(),
            env.clone(),
            primitive_3_info.clone(),
            primitive_3_msg,
        )
        .unwrap();
        assert_eq!(p3_res.messages.len(), 0);

        let balance_query = crate::msg::QueryMsg::Balance {
            address: TEST_CREATOR.to_string(),
        };
        let balance_res = query(deps.as_ref(), env.clone(), balance_query).unwrap();
        let balance: BalanceResponse = from_binary(&balance_res).unwrap();

        assert_eq!(balance.balance, Uint128::from(99u128));

        // start unbond
        let unbond_info = mock_info(TEST_CREATOR, &[]);
        let unbond_msg = ExecuteMsg::Unbond {
            amount: Option::Some(balance.balance),
        };
        let unbond_res = execute(deps.as_mut(), env.clone(), unbond_info, unbond_msg).unwrap();
        assert_eq!(unbond_res.attributes[2].value, "99"); // burnt
        assert_eq!(unbond_res.attributes[3].value, "2"); // bond_id
        assert_eq!(unbond_res.attributes[6].value, "0"); // num_unbondable_ids

        // get callbacks back
        let start_unbond_msg_p1 = ExecuteMsg::StartUnbondResponse(StartUnbondResponse {
            unbond_id: "2".to_string(),
            unlock_time: Timestamp::from_seconds(env.block.time.seconds() + 5),
        });
        let start_unbond_res = execute(
            deps.as_mut(),
            env.clone(),
            primitive_1_info.clone(),
            start_unbond_msg_p1,
        )
        .unwrap();
        assert_eq!(start_unbond_res.messages.len(), 0);

        let start_unbond_msg_p2 = ExecuteMsg::StartUnbondResponse(StartUnbondResponse {
            unbond_id: "2".to_string(),
            unlock_time: Timestamp::from_seconds(env.block.time.seconds() + 5),
        });
        let start_unbond_res = execute(
            deps.as_mut(),
            env.clone(),
            primitive_2_info.clone(),
            start_unbond_msg_p2,
        )
        .unwrap();
        assert_eq!(start_unbond_res.messages.len(), 0);

        let start_unbond_msg_p3 = ExecuteMsg::StartUnbondResponse(StartUnbondResponse {
            unbond_id: "2".to_string(),
            unlock_time: Timestamp::from_seconds(env.block.time.seconds() - 5),
        });
        let start_unbond_res = execute(
            deps.as_mut(),
            env.clone(),
            primitive_3_info.clone(),
            start_unbond_msg_p3,
        )
        .unwrap();
        assert_eq!(start_unbond_res.messages.len(), 0);

        // do unbond
        let do_unbond_info = mock_info(TEST_CREATOR, &[]);
        let do_unbond_msg = ExecuteMsg::Unbond { amount: None };
        let do_unbond_res =
            execute(deps.as_mut(), env.clone(), do_unbond_info, do_unbond_msg).unwrap();
        println!("{:?}", do_unbond_res);
        assert_eq!(unbond_res.attributes[2].value, "9"); // burnt
        assert_eq!(unbond_res.attributes[3].value, "2"); // bond_id
        assert_eq!(unbond_res.attributes[6].value, "0"); // num_unbondable_ids
    }

    #[test]
    fn test_recipient_not_sender() {}
}
