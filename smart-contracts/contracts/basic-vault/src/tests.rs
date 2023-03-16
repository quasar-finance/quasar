#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use cosmwasm_std::{
        from_binary,
        testing::{mock_env, mock_info, MockApi, MockStorage},
        to_binary, Addr, BankMsg, Binary, Coin, ContractResult, CosmosMsg, Decimal, DepsMut, Empty,
        Env, MessageInfo, OwnedDeps, Querier, QuerierResult, QueryRequest, Response, StdError,
        StdResult, Timestamp, Uint128, WasmMsg,
    };
    use cw20::BalanceResponse;

    use lp_strategy::{
        msg::{
            ConfigResponse, IcaBalanceResponse, PrimitiveSharesResponse, UnbondingClaimResponse,
        },
        state::{Config, Unbond},
    };
    use quasar_types::callback::{BondResponse, StartUnbondResponse, UnbondResponse};

    use crate::{
        contract::execute,
        contract::instantiate,
        contract::query,
        execute::may_pay_with_ratio,
        msg::{ExecuteMsg, InstantiateMsg, InvestmentResponse, PrimitiveConfig, PrimitiveInitMsg},
        ContractError,
    };

    pub struct QuasarQuerier {
        // address, denom, share, balance
        pub primitive_states: Vec<(String, String, Uint128, Uint128)>,
        // address, unlock_time, attempted
        pub primitive_unlock_times: Vec<(String, Timestamp, bool)>,
    }

    impl QuasarQuerier {
        pub fn new(primitive_states: Vec<(String, String, Uint128, Uint128)>) -> QuasarQuerier {
            QuasarQuerier {
                primitive_states: primitive_states.clone(),
                primitive_unlock_times: vec![],
            }
        }

        pub fn find_states_for_primitive(&self, address: String) -> (String, Uint128, Uint128) {
            let mut total_share = Uint128::zero();
            let mut total_balance = Uint128::zero();
            let mut this_denom = "".to_string();
            for (addr, denom, share, balance) in &self.primitive_states {
                if addr.eq(&address) {
                    this_denom = denom.to_string();
                    total_share = *share;
                    total_balance = *balance;
                }
            }
            (this_denom, total_share, total_balance)
        }

        pub fn set_unbonding_claim_for_primitive(
            &mut self,
            address: String,
            time: Timestamp,
            attempted: bool,
        ) {
            let put = self
                .primitive_unlock_times
                .iter_mut()
                .find(|put| put.0 == address);
            match put {
                Some(p) => {
                    p.1 = time;
                    p.2 = attempted;
                    return;
                }
                None => self.primitive_unlock_times.push((address, time, attempted)),
            }
        }

        pub fn get_unbonding_claim_for_primitive(
            &self,
            address: String,
        ) -> StdResult<(Timestamp, bool)> {
            let prim = self.primitive_unlock_times.iter().find(|p| p.0 == address);

            match prim {
                Some(p) => Ok((p.1, p.2)),
                None => Err(StdError::GenericErr {
                    msg: "Unbonding claim not found".to_owned(),
                }),
            }
        }
    }

    impl Querier for QuasarQuerier {
        fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
            let request: QueryRequest<Empty> = from_binary(&Binary::from(bin_request)).unwrap();
            match request {
                QueryRequest::Wasm(wasm_query) => match wasm_query {
                    cosmwasm_std::WasmQuery::Smart { contract_addr, msg } => {
                        let primitive_query =
                            from_binary::<lp_strategy::msg::QueryMsg>(&msg).unwrap();

                        let (this_denom, total_share, total_balance) =
                            self.find_states_for_primitive(contract_addr.clone());
                        match primitive_query {
                            lp_strategy::msg::QueryMsg::PrimitiveShares {} => {
                                let response = PrimitiveSharesResponse { total: total_share };
                                QuerierResult::Ok(ContractResult::Ok(to_binary(&response).unwrap()))
                            }
                            lp_strategy::msg::QueryMsg::IcaBalance {} => {
                                let response = IcaBalanceResponse {
                                    amount: Coin {
                                        denom: this_denom,
                                        amount: total_balance,
                                    },
                                };
                                QuerierResult::Ok(ContractResult::Ok(to_binary(&response).unwrap()))
                            }
                            lp_strategy::msg::QueryMsg::Config {} => {
                                let config = Config {
                                    lock_period: 14,
                                    pool_id: 1,
                                    pool_denom: "gamm/pool/1".to_string(),
                                    local_denom: this_denom,
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
                            lp_strategy::msg::QueryMsg::UnbondingClaim { addr, id } => {
                                let query_result =
                                    self.get_unbonding_claim_for_primitive(contract_addr);
                                QuerierResult::Ok(match query_result {
                                    Ok((unlock_time, attempted)) => ContractResult::Ok(
                                        to_binary(&UnbondingClaimResponse {
                                            unbond: Unbond {
                                                lp_shares: Uint128::from(1u128),
                                                unlock_time,
                                                attempted: attempted,
                                                owner: Addr::unchecked(TEST_CREATOR),
                                                id,
                                            },
                                        })
                                        .unwrap(),
                                    ),
                                    Err(error) => ContractResult::Err(error.to_string()),
                                })
                            }
                            _ => {
                                QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                                    kind: format!(
                                        "Unmocked primitive query type: {primitive_query:?}"
                                    ),
                                })
                            }
                        }
                    }

                    _ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                        kind: format!("Unmocked wasm query type: {wasm_query:?}"),
                    }),
                },
                _ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                    kind: format!("Unmocked query type: {request:?}"),
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
            querier: QuasarQuerier::new(primitive_states),
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
        instantiate(deps, env.clone(), info.clone(), msg.clone()).unwrap()
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
        let query_res = query(deps.as_ref(), env, invest_query).unwrap();

        let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

        let (coins, remainder) =
            may_pay_with_ratio(&deps.as_ref(), &even_deposit(), investment_response.info).unwrap();

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
    //     let query_res = query(deps.as_ref(), env, invest_query).unwrap();

    //     let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

    //     let (coins, remainder) = may_pay_with_ratio(
    //         &deps.as_ref(),
    //         &[
    //             Coin {
    //                 denom: "ibc/uosmo".to_string(),
    //                 amount: Uint128::from(100u128),
    //             },
    //             Coin {
    //                 denom: "ibc/uatom".to_string(),
    //                 amount: Uint128::from(200u128),
    //             },
    //             Coin {
    //                 denom: "ibc/ustars".to_string(),
    //                 amount: Uint128::from(1000u128),
    //             },
    //         ],
    //         investment_response.info,
    //     )
    //     .unwrap();

    //     println!("coins: {coins:?}");
    //     println!("remainder: {remainder:?}");
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
    fn proper_bond() {
        let mut deps = mock_deps_with_primitives(even_primitives());
        let init_msg = init_msg_with_primitive_details(even_primitive_details());
        let info = mock_info(TEST_CREATOR, &even_deposit());
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(0, res.messages.len());

        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };
        let res = execute(deps.as_mut(), env, info, deposit_msg).unwrap();
        assert_eq!(res.messages.len(), 4);
        assert_eq!(res.attributes.first().unwrap().value, "1");

        if let CosmosMsg::Wasm(wasm_msg) = &res.messages[0].msg {
            if let WasmMsg::Execute {
                contract_addr,
                msg,
                funds,
            } = wasm_msg
            {
                assert_eq!(contract_addr, "quasar123");
                assert_eq!(funds.len(), 1);
                assert_eq!(funds[0].denom, "ibc/uosmo");
                assert_eq!(funds[0].amount, Uint128::from(99u128));
                if let lp_strategy::msg::ExecuteMsg::Bond { id } = from_binary(msg).unwrap() {
                    assert_eq!(id, "1")
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        if let CosmosMsg::Wasm(wasm_msg) = &res.messages[1].msg {
            if let WasmMsg::Execute {
                contract_addr,
                msg,
                funds,
            } = wasm_msg
            {
                assert_eq!(contract_addr, "quasar124");
                assert_eq!(funds.len(), 1);
                assert_eq!(funds[0].denom, "ibc/uatom");
                assert_eq!(funds[0].amount, Uint128::from(99u128));
                if let lp_strategy::msg::ExecuteMsg::Bond { id } = from_binary(msg).unwrap() {
                    assert_eq!(id, "1")
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        if let CosmosMsg::Wasm(wasm_msg) = &res.messages[2].msg {
            if let WasmMsg::Execute {
                contract_addr,
                msg,
                funds,
            } = wasm_msg
            {
                assert_eq!(contract_addr, "quasar125");
                assert_eq!(funds.len(), 1);
                assert_eq!(funds[0].denom, "ibc/ustars");
                assert_eq!(funds[0].amount, Uint128::from(99u128));
                if let lp_strategy::msg::ExecuteMsg::Bond { id } = from_binary(msg).unwrap() {
                    assert_eq!(id, "1")
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        if let CosmosMsg::Bank(msg) = &res.messages[3].msg {
            if let BankMsg::Send { to_address, amount } = msg {
                assert_eq!(to_address, TEST_CREATOR);
                assert_eq!(amount.len(), 3);
                assert_eq!(amount[0].amount, Uint128::from(1u128));
                assert_eq!(amount[1].amount, Uint128::from(1u128));
                assert_eq!(amount[2].amount, Uint128::from(1u128));
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn proper_bond_with_zero_primitive_balance() {
        let mut deps = mock_deps_with_primitives(vec![(
            "quasar123".to_string(),
            "ibc/uosmo".to_string(),
            Uint128::from(0u128),
            Uint128::from(0u128),
        )]);
        let init_msg = init_msg_with_primitive_details(vec![(
            "quasar123".to_string(),
            "ibc/uosmo".to_string(),
            Decimal::one(),
        )]);
        let info = mock_info(TEST_CREATOR, &even_deposit());
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(0, res.messages.len());

        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };
        let res = execute(deps.as_mut(), env, info, deposit_msg).unwrap();
        assert_eq!(res.messages.len(), 2);
        assert_eq!(res.attributes.first().unwrap().value, "1");
    }

    #[test]
    fn test_bond_with_bad_primitive_state() {
        let mut deps_1 = mock_deps_with_primitives(vec![(
            "quasar123".to_string(),
            "ibc/uosmo".to_string(),
            Uint128::from(0u128),
            Uint128::from(1u128),
        )]);
        let mut deps_2 = mock_deps_with_primitives(vec![(
            "quasar123".to_string(),
            "ibc/uosmo".to_string(),
            Uint128::from(1u128),
            Uint128::from(0u128),
        )]);
        let init_msg = init_msg_with_primitive_details(vec![(
            "quasar123".to_string(),
            "ibc/uosmo".to_string(),
            Decimal::one(),
        )]);
        let info = mock_info(TEST_CREATOR, &even_deposit());
        let env = mock_env();
        let _ = init(deps_1.as_mut(), &init_msg, &env, &info);
        let _ = init(deps_2.as_mut(), &init_msg, &env, &info);

        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };
        let res_1 = execute(
            deps_1.as_mut(),
            env.clone(),
            info.clone(),
            deposit_msg.clone(),
        )
        .unwrap_err();
        assert_eq!(res_1.to_string(), "Generic error: Unexpected primitive state, either both supply and balance should be zero, or neither.");
        let res_2 = execute(deps_2.as_mut(), env, info, deposit_msg).unwrap_err();
        assert_eq!(res_2.to_string(), "Generic error: Unexpected primitive state, either both supply and balance should be zero, or neither.");
    }

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
        assert_eq!(res.messages.len(), 4);
        assert_eq!(res.attributes.first().unwrap().value, "1");

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
        let balance_res = query(deps.as_ref(), env, balance_query).unwrap();
        let balance: BalanceResponse = from_binary(&balance_res).unwrap();

        assert_eq!(balance.balance, Uint128::from(99u128));
    }

    #[test]
    fn proper_unbond() {
        let mut deps = mock_deps_with_primitives(even_primitives());
        let init_msg = init_msg_with_primitive_details(even_primitive_details());
        let info = mock_info(TEST_CREATOR, &even_deposit());
        let mut env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(0, res.messages.len());

        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };
        let res = execute(deps.as_mut(), env.clone(), info, deposit_msg).unwrap();
        assert_eq!(res.messages.len(), 4);
        assert_eq!(res.attributes.first().unwrap().value, "1");

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
        assert_eq!(unbond_res.messages.len(), 3);
        assert_eq!(unbond_res.attributes[2].key, "burnt");
        assert_eq!(unbond_res.attributes[2].value, "99");
        assert_eq!(unbond_res.attributes[3].key, "bond_id");
        assert_eq!(unbond_res.attributes[3].value, "2");
        assert_eq!(unbond_res.attributes[6].key, "num_unbondable_ids");
        assert_eq!(unbond_res.attributes[6].value, "0");

        // todo replace with a macro
        if let CosmosMsg::Wasm(wasm_msg) = &unbond_res.messages[0].msg {
            if let WasmMsg::Execute {
                contract_addr,
                msg,
                funds,
            } = wasm_msg
            {
                assert_eq!(contract_addr, "quasar123");
                assert!(funds.is_empty());
                if let lp_strategy::msg::ExecuteMsg::StartUnbond { id, share_amount } =
                    from_binary(msg).unwrap()
                {
                    assert_eq!(id, "2");
                    assert_eq!(share_amount, Uint128::from(98u128));
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
        if let CosmosMsg::Wasm(wasm_msg) = &unbond_res.messages[1].msg {
            if let WasmMsg::Execute {
                contract_addr,
                msg,
                funds,
            } = wasm_msg
            {
                assert_eq!(contract_addr, "quasar124");
                assert!(funds.is_empty());
                if let lp_strategy::msg::ExecuteMsg::StartUnbond { id, share_amount } =
                    from_binary(msg).unwrap()
                {
                    assert_eq!(id, "2");
                    assert_eq!(share_amount, Uint128::from(98u128));
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
        if let CosmosMsg::Wasm(wasm_msg) = &unbond_res.messages[2].msg {
            if let WasmMsg::Execute {
                contract_addr,
                msg,
                funds,
            } = wasm_msg
            {
                assert_eq!(contract_addr, "quasar125");
                assert!(funds.is_empty());
                if let lp_strategy::msg::ExecuteMsg::StartUnbond { id, share_amount } =
                    from_binary(msg).unwrap()
                {
                    assert_eq!(id, "2");
                    assert_eq!(share_amount, Uint128::from(98u128));
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // get callbacks back
        let start_unbond_msg_p1 = ExecuteMsg::StartUnbondResponse(StartUnbondResponse {
            unbond_id: "2".to_string(),
            unlock_time: Timestamp::from_seconds(env.block.time.seconds() + 5),
        });
        let start_unbond_res = execute(
            deps.as_mut(),
            env.clone(),
            primitive_1_info,
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
            primitive_2_info,
            start_unbond_msg_p2,
        )
        .unwrap();
        assert_eq!(start_unbond_res.messages.len(), 0);

        let start_unbond_msg_p3 = ExecuteMsg::StartUnbondResponse(StartUnbondResponse {
            unbond_id: "2".to_string(),
            unlock_time: Timestamp::from_seconds(env.block.time.seconds() + 60),
        });
        let start_unbond_res = execute(
            deps.as_mut(),
            env.clone(),
            primitive_3_info,
            start_unbond_msg_p3,
        )
        .unwrap();
        assert_eq!(start_unbond_res.messages.len(), 0);

        // do unbond
        let do_unbond_info = mock_info(TEST_CREATOR, &[]);
        let do_unbond_msg = ExecuteMsg::Unbond { amount: None };
        let do_unbond_res = execute(
            deps.as_mut(),
            env.clone(),
            do_unbond_info.clone(),
            do_unbond_msg.clone(),
        )
        .unwrap();

        assert_eq!(do_unbond_res.messages.len(), 0);
        assert_eq!(do_unbond_res.attributes[2].key, "num_unbondable_ids");
        assert_eq!(do_unbond_res.attributes[2].value, "0");

        env.block.height += 4;
        env.block.time = env.block.time.plus_seconds(30);

        // set two of the primitives to be unbondable
        deps.querier.set_unbonding_claim_for_primitive(
            "quasar123".to_owned(),
            env.block.time.minus_seconds(5),
            false,
        );
        deps.querier.set_unbonding_claim_for_primitive(
            "quasar124".to_owned(),
            env.block.time.minus_seconds(5),
            false,
        );

        // unbond and see that 2 are unbondable
        let do_unbond_res = execute(
            deps.as_mut(),
            env.clone(),
            do_unbond_info.clone(),
            do_unbond_msg,
        )
        .unwrap();

        assert_eq!(do_unbond_res.messages.len(), 2);
        assert_eq!(do_unbond_res.attributes[2].key, "num_unbondable_ids");
        assert_eq!(do_unbond_res.attributes[2].value, "2");

        if let CosmosMsg::Wasm(wasm_msg) = &do_unbond_res.messages[0].msg {
            if let WasmMsg::Execute {
                contract_addr,
                msg,
                funds,
            } = wasm_msg
            {
                assert_eq!(contract_addr, "quasar123");
                assert!(funds.is_empty());
                if let lp_strategy::msg::ExecuteMsg::Unbond { id } = from_binary(msg).unwrap() {
                    assert_eq!(id, "2");
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
        if let CosmosMsg::Wasm(wasm_msg) = &do_unbond_res.messages[1].msg {
            if let WasmMsg::Execute {
                contract_addr,
                msg,
                funds,
            } = wasm_msg
            {
                assert_eq!(contract_addr, "quasar124");
                assert!(funds.is_empty());
                if let lp_strategy::msg::ExecuteMsg::Unbond { id } = from_binary(msg).unwrap() {
                    assert_eq!(id, "2");
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
        // set these two primitive unbonds to have been attempted already
        deps.querier.set_unbonding_claim_for_primitive(
            "quasar123".to_owned(),
            env.block.time.minus_seconds(5),
            true,
        );
        deps.querier.set_unbonding_claim_for_primitive(
            "quasar124".to_owned(),
            env.block.time.minus_seconds(5),
            true,
        );

        env.block.height += 5;
        env.block.time = env.block.time.plus_seconds(40);

// set last of the primitives to be unbondable
        deps.querier.set_unbonding_claim_for_primitive(
            "quasar125".to_owned(),
            env.block.time.minus_seconds(5),
            false,
        );
        

        // test that claim works the same way as unbond(amount:0)
        let claim_msg = ExecuteMsg::Claim {};
        let claim_res = execute(deps.as_mut(), env.clone(), do_unbond_info, claim_msg).unwrap();

        // todo: This assertion will change because we should ideally only expect one here, pending arch discussion
        assert_eq!(claim_res.messages.len(), 1);
        assert_eq!(claim_res.attributes[2].key, "num_unbondable_ids");
        assert_eq!(claim_res.attributes[2].value, "1");

        if let CosmosMsg::Wasm(wasm_msg) = &claim_res.messages[0].msg {
            if let WasmMsg::Execute {
                contract_addr,
                msg,
                funds,
            } = wasm_msg
            {
                // todo: This assertion will change because we should ideally only expect one here, pending arch discussions
                assert_eq!(contract_addr, "quasar125");
                assert!(funds.is_empty());
                if let lp_strategy::msg::ExecuteMsg::Unbond { id } = from_binary(msg).unwrap() {
                    assert_eq!(id, "2");
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // start callbacks from primitives for unbond
        let unbond_callback_msg = ExecuteMsg::UnbondResponse(UnbondResponse {
            unbond_id: "2".to_string(),
        });
        let p1_unbond_callback_info = mock_info(
            "quasar123",
            &[Coin {
                denom: "ibc/uosmo".to_string(),
                amount: Uint128::from(100u128),
            }],
        );
        let p1_unbond_callback_res = execute(
            deps.as_mut(),
            env.clone(),
            p1_unbond_callback_info,
            unbond_callback_msg.clone(),
        )
        .unwrap();
        assert_eq!(p1_unbond_callback_res.messages.len(), 0);

        let p2_unbond_callback_info = mock_info(
            "quasar124",
            &[Coin {
                denom: "ibc/uatom".to_string(),
                amount: Uint128::from(100u128),
            }],
        );
        let p2_unbond_callback_res = execute(
            deps.as_mut(),
            env.clone(),
            p2_unbond_callback_info,
            unbond_callback_msg.clone(),
        )
        .unwrap();
        assert_eq!(p2_unbond_callback_res.messages.len(), 0);

        let p3_unbond_callback_info = mock_info(
            "quasar125",
            &[Coin {
                denom: "ibc/ustars".to_string(),
                amount: Uint128::from(100u128),
            }],
        );
        let p3_unbond_callback_res = execute(
            deps.as_mut(),
            env,
            p3_unbond_callback_info,
            unbond_callback_msg,
        )
        .unwrap();
        assert_eq!(p3_unbond_callback_res.messages.len(), 3);

        if let CosmosMsg::Bank(bank_msg) = &p3_unbond_callback_res.messages[0].msg {
            if let BankMsg::Send { to_address, amount } = bank_msg {
                assert_eq!(to_address, TEST_CREATOR);
                assert_eq!(amount.len(), 1);
                assert_eq!(amount[0].denom, "ibc/uosmo");
                assert_eq!(amount[0].amount, Uint128::from(100u128));
            } else {
               panic!("unexpected bank message");
            }
        } else {
            panic!("unexpected message");
        }

        if let CosmosMsg::Bank(bank_msg) = &p3_unbond_callback_res.messages[1].msg {
            if let BankMsg::Send { to_address, amount } = bank_msg {
                assert_eq!(to_address, TEST_CREATOR);
                assert_eq!(amount.len(), 1);
                assert_eq!(amount[0].denom, "ibc/uatom");
                assert_eq!(amount[0].amount, Uint128::from(100u128));
            } else {
               panic!("unexpected bank message");
            }
        } else {
            panic!("unexpected message");
        }

        if let CosmosMsg::Bank(bank_msg) = &p3_unbond_callback_res.messages[2].msg {
            if let BankMsg::Send { to_address, amount } = bank_msg {
                assert_eq!(to_address, TEST_CREATOR);
                assert_eq!(amount.len(), 1);
                assert_eq!(amount[0].denom, "ibc/ustars");
                assert_eq!(amount[0].amount, Uint128::from(100u128));
            } else {
               panic!("unexpected bank message");
            }
        } else {
            panic!("unexpected message");
        }
    }

    #[test]
    fn test_multi_user_bond_unbond() {}

    #[test]
    fn test_recipient_not_sender() {}
}
