#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use cosmwasm_std::{
        from_binary,
        testing::{
            mock_dependencies, mock_dependencies_with_balances, mock_env, mock_info, MockApi,
            MockQuerier, MockStorage,
        },
        to_binary, Api, Binary, Coin, ContractResult, CustomQuery, Decimal, DepsMut, Empty, Env,
        MessageInfo, OwnedDeps, Querier, QuerierResult, QueryRequest, Response, Storage, Uint128,
    };
    use lp_strategy::msg::{IcaBalanceResponse, PrimitiveSharesResponse};
    use quasar_types::callback::BondResponse;

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
                                _ => QuerierResult::Err(
                                    cosmwasm_std::SystemError::UnsupportedRequest {
                                        kind: "Unmocked primitive query type".to_owned(),
                                    },
                                ),
                            }
                        } else {
                            QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                                kind: "Unmocked primitive query type".to_owned(),
                            })
                        }
                    }
                    _ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                        kind: "Unmocked wasm query type".to_owned(),
                    }),
                },
                _ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                    kind: "Unmocked query type".to_owned(),
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
        let mut deps = mock_dependencies_with_balances(&[]);
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

    #[test]
    fn test_may_pay_with_ratio() {
        let mut deps = mock_deps_with_primitives(vec![
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
        ]);
        let init_msg = init_msg_with_primitive_details(vec![
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
        ]);
        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(0, res.messages.len());

        let invest_query = crate::msg::QueryMsg::Investment {};
        let query_res = query(deps.as_ref(), env.clone(), invest_query).unwrap();

        let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

        let (coins, remainder) = may_pay_with_ratio(
            &deps.as_ref(),
            &vec![
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
            ],
            investment_response.info,
        )
        .unwrap();

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
}
