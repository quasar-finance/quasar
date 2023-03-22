#[cfg(test)]
mod tests {
    use std::{marker::PhantomData, str::FromStr};

    use cosmwasm_std::{
        from_binary,
        testing::{mock_env, mock_info, MockApi, MockStorage},
        to_binary, Addr, BankMsg, Binary, Coin, ContractResult, CosmosMsg, Decimal, DepsMut, Empty,
        Env, Fraction, MessageInfo, OwnedDeps, Querier, QuerierResult, QueryRequest, Reply,
        Response, StdError, StdResult, SubMsgResponse, SubMsgResult, Timestamp, Uint128, WasmMsg,
    };
    use cw20::BalanceResponse;

    use cw_asset::AssetInfoBase;
    use lp_strategy::{
        msg::{
            ConfigResponse, IcaBalanceResponse, PrimitiveSharesResponse, UnbondingClaimResponse,
        },
        state::{Config, Unbond},
    };
    use prost::Message;
    use quasar_types::{
        callback::{BondResponse, StartUnbondResponse, UnbondResponse},
        types::{CoinRatio, CoinWeight},
    };

    use crate::{
        contract::execute,
        contract::instantiate,
        contract::query,
        contract::{reply, REPLY_INIT_VAULT_REWARDS},
        execute::{
            get_deposit_amount_weights, get_deposit_and_remainder_for_ratio, get_max_bond,
            get_token_amount_weights, may_pay_with_ratio,
        },
        msg::{ExecuteMsg, InstantiateMsg, InvestmentResponse, PrimitiveConfig, PrimitiveInitMsg},
    };

    #[derive(Clone, PartialEq, prost::Message)]
    struct MsgInstantiateContractResponse {
        #[prost(string, tag = "1")]
        pub contract_address: ::prost::alloc::string::String,
        #[prost(bytes, tag = "2")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }

    pub struct QuasarQuerier {
        // address, denom, share, balance
        pub primitive_states: Vec<(String, String, Uint128, Uint128)>,
        // address, unlock_time, attempted
        pub primitive_unlock_times: Vec<(String, Timestamp, bool)>,
    }

    impl QuasarQuerier {
        pub fn new(primitive_states: Vec<(String, String, Uint128, Uint128)>) -> QuasarQuerier {
            QuasarQuerier {
                primitive_states,
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
                            lp_strategy::msg::QueryMsg::UnbondingClaim { addr: _, id } => {
                                let query_result =
                                    self.get_unbonding_claim_for_primitive(contract_addr);
                                QuerierResult::Ok(match query_result {
                                    Ok((unlock_time, attempted)) => ContractResult::Ok(
                                        to_binary(&UnbondingClaimResponse {
                                            unbond: Unbond {
                                                lp_shares: Uint128::from(1u128),
                                                unlock_time,
                                                attempted,
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
    const TEST_DEPOSITOR: &str = "depositor";

    fn init_msg() -> InstantiateMsg {
        InstantiateMsg {
            name: "Blazar Vault".to_string(),
            thesis: "to generate yield, I guess".to_string(),
            symbol: "BLZR".to_string(),
            decimals: 6,
            min_withdrawal: Uint128::one(),
            primitives: vec![PrimitiveConfig {
                weight: Decimal::from_str("1.0").unwrap(),
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
            vault_rewards_code_id: 123,
            reward_token: cw_asset::AssetInfoBase::Native("uqsr".to_string()),
            reward_distribution_schedules: vec![vault_rewards::state::DistributionSchedule {
                start: 1,
                end: 501,
                amount: Uint128::from(1000u128),
            }],
        }
    }

    fn reply_msg() -> Reply {
        let instantiate_reply = MsgInstantiateContractResponse {
            contract_address: "vault_rewards_addr".to_string(),
            data: vec![],
        };
        let mut encoded_instantiate_reply =
            Vec::<u8>::with_capacity(instantiate_reply.encoded_len());
        instantiate_reply
            .encode(&mut encoded_instantiate_reply)
            .unwrap();

        // reply to init our map for the vault rewards contract
        Reply {
            id: REPLY_INIT_VAULT_REWARDS,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: Some(encoded_instantiate_reply.into()),
            }),
        }
    }

    // convenience function to init primitives with a vec of tuples which are (local_denom, weight)

    fn init_msg_with_primitive_details(
        primitive_details: Vec<(String, String, Decimal)>,
    ) -> InstantiateMsg {
        InstantiateMsg {
            name: "Blazar Vault".to_string(),
            thesis: "to generate yield, I guess".to_string(),
            symbol: "BLZR".to_string(),
            decimals: 6,
            min_withdrawal: Uint128::one(),
            vault_rewards_code_id: 123,
            reward_token: cw_asset::AssetInfoBase::Native("uqsr".to_string()),
            reward_distribution_schedules: vec![vault_rewards::state::DistributionSchedule {
                start: 1,
                end: 501,
                amount: Uint128::from(1000u128),
            }],
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

        println!("res: {res:?}");
        assert_eq!(1, res.messages.len());

        if let CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id,
            msg,
            funds,
            admin,
            label,
        }) = &res.messages[0].msg
        {
            assert_eq!(123, *code_id);
            assert_eq!(0, funds.len());
            assert_eq!(admin.clone().unwrap(), TEST_CREATOR);
            assert_eq!(label, "vault-rewards");
            let msg: vault_rewards::msg::InstantiateMsg = from_binary(msg).unwrap();
            assert_eq!(
                Uint128::from(1000u128),
                msg.distribution_schedules[0].amount
            );
            if let AssetInfoBase::Native(native) = &msg.reward_token {
                assert_eq!("uqsr", native);
            } else {
                panic!("Unexpected reward token type");
            }
        } else {
            panic!("Unexpected message type");
        }
    }

    #[test]
    fn proper_bond_with_one_primitive() {
        let mut deps = mock_deps_with_primitives(vec![(
            "quasar123".to_string(),
            "ibc/uosmo".to_string(),
            Uint128::from(100u128),
            Uint128::from(100u128),
        )]);
        let msg = init_msg();
        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let _res = init(deps.as_mut(), &msg, &env, &info);

        let deposit_info = mock_info(
            TEST_DEPOSITOR,
            &[Coin {
                denom: "ibc/uosmo".to_string(),
                amount: Uint128::from(100u128),
            }],
        );
        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };

        let res = execute(deps.as_mut(), env, deposit_info, deposit_msg).unwrap();
        assert_eq!(res.messages.len(), 2);
        assert_eq!(res.attributes.first().unwrap().value, "1");

        if let CosmosMsg::Wasm(wasm_msg) = &res.messages.first().unwrap().msg {
            if let WasmMsg::Execute {
                contract_addr,
                msg: _,
                funds,
            } = wasm_msg
            {
                assert_eq!(contract_addr, "quasar123");
                assert_eq!(funds[0].amount, Uint128::from(100u128));
            } else {
                panic!("Wrong message type")
            }
        }
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

    fn _uneven_primitives() -> Vec<(String, String, Uint128, Uint128)> {
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
                Uint128::from(200u128),
                Uint128::from(400u128),
            ),
        ]
    }

    fn _uneven_primitive_details() -> Vec<(String, String, Decimal)> {
        vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Decimal::from_str("2.0").unwrap(),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Decimal::one(),
            ),
        ]
    }

    fn _uneven_deposit() -> Vec<Coin> {
        vec![
            Coin {
                denom: "ibc/uosmo".to_string(),
                amount: Uint128::from(1000u128),
            },
            Coin {
                denom: "ibc/uatom".to_string(),
                amount: Uint128::from(200u128),
            },
        ]
    }

    #[test]
    fn test_get_deposit_amount_weights() {
        let primitive_states = vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Uint128::from(100u128),
                Uint128::from(100u128),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Uint128::from(200u128),
                Uint128::from(400u128),
            ),
        ];
        let primitive_deets = vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Decimal::from_str("3.5").unwrap(),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Decimal::one(),
            ),
        ];

        let mut deps = mock_deps_with_primitives(primitive_states.clone());
        let init_msg = init_msg_with_primitive_details(primitive_deets.clone());

        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(1, res.messages.len());

        let invest_query = crate::msg::QueryMsg::Investment {};
        let query_res = query(deps.as_ref(), env, invest_query).unwrap();

        let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

        let weights =
            get_deposit_amount_weights(&deps.as_ref(), &investment_response.info.primitives)
                .unwrap();

        let first_weight = Decimal::from_ratio(
            primitive_deets[0].2 * primitive_states[0].3,
            primitive_states[0].2,
        );
        let second_weight = Decimal::from_ratio(
            primitive_deets[1].2 * primitive_states[1].3,
            primitive_states[1].2,
        );

        let total = first_weight + second_weight;

        let expected_first_weight = Decimal::from_ratio(
            first_weight.numerator() * total.denominator(),
            first_weight.denominator() * total.numerator(),
        );
        let expected_second_weight = Decimal::from_ratio(
            second_weight.numerator() * total.denominator(),
            second_weight.denominator() * total.numerator(),
        );

        assert_eq!(weights.ratio[0].weight, expected_first_weight);
        assert_eq!(weights.ratio[1].weight, expected_second_weight);
    }

    #[test]
    fn test_get_token_amount_weights() {
        let primitive_states = vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Uint128::from(100u128),
                Uint128::from(100u128),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Uint128::from(200u128),
                Uint128::from(400u128),
            ),
        ];
        let primitive_deets = vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Decimal::from_str("3.5").unwrap(),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Decimal::one(),
            ),
        ];

        let mut deps = mock_deps_with_primitives(primitive_states.clone());
        let init_msg = init_msg_with_primitive_details(primitive_deets.clone());

        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(1, res.messages.len());

        let invest_query = crate::msg::QueryMsg::Investment {};
        let query_res = query(deps.as_ref(), env, invest_query).unwrap();

        let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

        let _weights =
            get_deposit_amount_weights(&deps.as_ref(), &investment_response.info.primitives)
                .unwrap();

        let first_weight = Decimal::from_ratio(
            primitive_deets[0].2 * primitive_states[0].3,
            primitive_states[0].2,
        );
        let second_weight = Decimal::from_ratio(
            primitive_deets[1].2 * primitive_states[1].3,
            primitive_states[1].2,
        );

        let total = first_weight + second_weight;

        let expected_first_weight = Decimal::from_ratio(
            first_weight.numerator() * total.denominator(),
            first_weight.denominator() * total.numerator(),
        );
        let expected_second_weight = Decimal::from_ratio(
            second_weight.numerator() * total.denominator(),
            second_weight.denominator() * total.numerator(),
        );

        let token_weights = get_token_amount_weights(&[
            CoinWeight {
                denom: "ibc/uosmo".to_string(),
                weight: expected_first_weight,
            },
            CoinWeight {
                denom: "ibc/uatom".to_string(),
                weight: expected_second_weight,
            },
        ])
        .unwrap();

        assert_eq!(token_weights[0].weight, expected_first_weight);
        assert_eq!(token_weights[1].weight, expected_second_weight);
    }

    #[test]
    fn test_get_token_amount_weights_duplicate_tokens() {}

    #[test]
    fn test_get_max_bond() {
        let primitive_states = vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Uint128::from(100u128),
                Uint128::from(100u128),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Uint128::from(200u128),
                Uint128::from(400u128),
            ),
        ];
        let primitive_deets = vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Decimal::from_str("3.5").unwrap(),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Decimal::one(),
            ),
        ];

        let funds = vec![
            Coin {
                denom: "ibc/uosmo".to_string(),
                amount: Uint128::from(1000u128),
            },
            Coin {
                denom: "ibc/uatom".to_string(),
                amount: Uint128::from(200u128),
            },
        ];

        let mut deps = mock_deps_with_primitives(primitive_states.clone());
        let init_msg = init_msg_with_primitive_details(primitive_deets.clone());

        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(1, res.messages.len());

        let invest_query = crate::msg::QueryMsg::Investment {};
        let query_res = query(deps.as_ref(), env, invest_query).unwrap();

        let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

        let weights =
            get_deposit_amount_weights(&deps.as_ref(), &investment_response.info.primitives)
                .unwrap();

        let first_weight = Decimal::from_ratio(
            primitive_deets[0].2 * primitive_states[0].3,
            primitive_states[0].2,
        );
        let second_weight = Decimal::from_ratio(
            primitive_deets[1].2 * primitive_states[1].3,
            primitive_states[1].2,
        );

        let total = first_weight + second_weight;

        let expected_first_weight = Decimal::from_ratio(
            first_weight.numerator() * total.denominator(),
            first_weight.denominator() * total.numerator(),
        );
        let expected_second_weight = Decimal::from_ratio(
            second_weight.numerator() * total.denominator(),
            second_weight.denominator() * total.numerator(),
        );

        assert_eq!(weights.ratio[0].weight, expected_first_weight);
        assert_eq!(weights.ratio[1].weight, expected_second_weight);

        let token_weights = get_token_amount_weights(&[
            CoinWeight {
                denom: "ibc/uosmo".to_string(),
                weight: expected_first_weight,
            },
            CoinWeight {
                denom: "ibc/uatom".to_string(),
                weight: expected_second_weight,
            },
        ])
        .unwrap();

        assert_eq!(token_weights[0].weight, expected_first_weight);
        assert_eq!(token_weights[1].weight, expected_second_weight);

        let expected_max_bond = std::cmp::min(
            Decimal::from_ratio(
                funds[0].amount * token_weights[0].weight.denominator(),
                token_weights[0].weight.numerator(),
            )
            .to_uint_floor(),
            Decimal::from_ratio(
                funds[1].amount * token_weights[1].weight.denominator(),
                token_weights[1].weight.numerator(),
            )
            .to_uint_floor(),
        );

        let max_bond = get_max_bond(&funds, &token_weights).unwrap();

        assert_eq!(max_bond.to_uint_floor(), expected_max_bond);
    }

    #[test]
    fn test_get_deposit_and_remainder_for_ratio() {
        let primitive_states = vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Uint128::from(100u128),
                Uint128::from(100u128),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Uint128::from(200u128),
                Uint128::from(400u128),
            ),
        ];
        let primitive_deets = vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Decimal::from_str("3.5").unwrap(),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Decimal::one(),
            ),
        ];

        let funds = vec![
            Coin {
                denom: "ibc/uosmo".to_string(),
                amount: Uint128::from(1000u128),
            },
            Coin {
                denom: "ibc/uatom".to_string(),
                amount: Uint128::from(200u128),
            },
        ];

        let mut deps = mock_deps_with_primitives(primitive_states.clone());
        let init_msg = init_msg_with_primitive_details(primitive_deets.clone());

        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(1, res.messages.len());

        let invest_query = crate::msg::QueryMsg::Investment {};
        let query_res = query(deps.as_ref(), env, invest_query).unwrap();

        let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

        let _weights =
            get_deposit_amount_weights(&deps.as_ref(), &investment_response.info.primitives)
                .unwrap();

        let first_weight = Decimal::from_ratio(
            primitive_deets[0].2 * primitive_states[0].3,
            primitive_states[0].2,
        );
        let second_weight = Decimal::from_ratio(
            primitive_deets[1].2 * primitive_states[1].3,
            primitive_states[1].2,
        );

        let total = first_weight + second_weight;

        let expected_first_weight = Decimal::from_ratio(
            first_weight.numerator() * total.denominator(),
            first_weight.denominator() * total.numerator(),
        );
        let expected_second_weight = Decimal::from_ratio(
            second_weight.numerator() * total.denominator(),
            second_weight.denominator() * total.numerator(),
        );

        let token_weights = get_token_amount_weights(&[
            CoinWeight {
                denom: "ibc/uosmo".to_string(),
                weight: expected_first_weight,
            },
            CoinWeight {
                denom: "ibc/uatom".to_string(),
                weight: expected_second_weight,
            },
        ])
        .unwrap();

        assert_eq!(token_weights[0].weight, expected_first_weight);
        assert_eq!(token_weights[1].weight, expected_second_weight);

        let expected_max_bond = std::cmp::min(
            Decimal::from_ratio(
                funds[0].amount * token_weights[0].weight.denominator(),
                token_weights[0].weight.numerator(),
            )
            .to_uint_floor(),
            Decimal::from_ratio(
                funds[1].amount * token_weights[1].weight.denominator(),
                token_weights[1].weight.numerator(),
            )
            .to_uint_floor(),
        );

        let max_bond = get_max_bond(&funds, &token_weights).unwrap();

        assert_eq!(max_bond.to_uint_floor(), expected_max_bond);

        let (deposit, remainder) = get_deposit_and_remainder_for_ratio(
            &funds,
            max_bond,
            &CoinRatio {
                ratio: token_weights.clone(),
            },
        )
        .unwrap();

        let expected_first_deposit = Decimal::from_ratio(
            token_weights[0].weight.numerator() * max_bond,
            token_weights[0].weight.denominator(),
        );
        let expected_second_deposit = Decimal::from_ratio(
            token_weights[1].weight.numerator() * max_bond,
            token_weights[1].weight.denominator(),
        );

        assert_eq!(deposit[0].amount, expected_first_deposit.to_uint_floor());
        assert_eq!(deposit[1].amount, expected_second_deposit.to_uint_floor());

        assert_eq!(
            remainder[0].amount,
            funds[0].amount - expected_first_deposit.to_uint_floor()
        );
        assert_eq!(
            remainder[1].amount,
            funds[1].amount - expected_second_deposit.to_uint_floor()
        );
    }

    #[test]
    fn test_get_deposit_and_remainder_for_ratio_three_primitives() {
        let primitive_states = vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Uint128::from(100u128),
                Uint128::from(100u128),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Uint128::from(200u128),
                Uint128::from(400u128),
            ),
            (
                "quasar125".to_string(),
                "ibc/ustars".to_string(),
                Uint128::from(600u128),
                Uint128::from(450u128),
            ),
        ];
        let primitive_deets = vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Decimal::from_str("3.5").unwrap(),
            ),
            (
                "quasar124".to_string(),
                "ibc/uatom".to_string(),
                Decimal::one(),
            ),
            (
                "quasar125".to_string(),
                "ibc/ustars".to_string(),
                Decimal::from_str("0.5").unwrap(),
            ),
        ];

        let funds = vec![
            Coin {
                denom: "ibc/uosmo".to_string(),
                amount: Uint128::from(1000u128),
            },
            Coin {
                denom: "ibc/uatom".to_string(),
                amount: Uint128::from(200u128),
            },
            Coin {
                denom: "ibc/ustars".to_string(),
                amount: Uint128::from(200u128),
            },
        ];

        let mut deps = mock_deps_with_primitives(primitive_states.clone());
        let init_msg = init_msg_with_primitive_details(primitive_deets.clone());

        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(1, res.messages.len());

        let invest_query = crate::msg::QueryMsg::Investment {};
        let query_res = query(deps.as_ref(), env, invest_query).unwrap();

        let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

        let weights =
            get_deposit_amount_weights(&deps.as_ref(), &investment_response.info.primitives)
                .unwrap();

        let first_weight = Decimal::from_ratio(
            primitive_deets[0].2 * primitive_states[0].3,
            primitive_states[0].2,
        );
        let second_weight = Decimal::from_ratio(
            primitive_deets[1].2 * primitive_states[1].3,
            primitive_states[1].2,
        );
        let third_weight = Decimal::from_ratio(
            primitive_deets[2].2 * primitive_states[2].3,
            primitive_states[2].2,
        );

        let total = first_weight + second_weight + third_weight;

        let expected_first_weight = Decimal::from_ratio(
            first_weight.numerator() * total.denominator(),
            first_weight.denominator() * total.numerator(),
        );
        let expected_second_weight = Decimal::from_ratio(
            second_weight.numerator() * total.denominator(),
            second_weight.denominator() * total.numerator(),
        );
        let expected_third_weight = Decimal::from_ratio(
            third_weight.numerator() * total.denominator(),
            third_weight.denominator() * total.numerator(),
        );

        assert_eq!(weights.ratio[0].weight, expected_first_weight);
        assert_eq!(weights.ratio[1].weight, expected_second_weight);
        assert_eq!(weights.ratio[2].weight, expected_third_weight);

        let token_weights = get_token_amount_weights(&[
            CoinWeight {
                denom: "ibc/uosmo".to_string(),
                weight: expected_first_weight,
            },
            CoinWeight {
                denom: "ibc/uatom".to_string(),
                weight: expected_second_weight,
            },
            CoinWeight {
                denom: "ibc/ustars".to_string(),
                weight: expected_third_weight,
            },
        ])
        .unwrap();

        assert_eq!(token_weights[0].weight, expected_first_weight);
        assert_eq!(token_weights[1].weight, expected_second_weight);
        assert_eq!(token_weights[2].weight, expected_third_weight);

        let expected_max_bond = std::cmp::min(
            Decimal::from_ratio(
                funds[0].amount * token_weights[0].weight.denominator(),
                token_weights[0].weight.numerator(),
            )
            .to_uint_floor(),
            Decimal::from_ratio(
                funds[1].amount * token_weights[1].weight.denominator(),
                token_weights[1].weight.numerator(),
            )
            .to_uint_floor(),
        );

        let max_bond = get_max_bond(&funds, &token_weights).unwrap();

        assert_eq!(max_bond.to_uint_floor(), expected_max_bond);

        let (deposit, remainder) = get_deposit_and_remainder_for_ratio(
            &funds,
            max_bond,
            &CoinRatio {
                ratio: token_weights.clone(),
            },
        )
        .unwrap();

        let expected_first_deposit = Decimal::from_ratio(
            token_weights[0].weight.numerator() * max_bond,
            token_weights[0].weight.denominator(),
        );
        let expected_second_deposit = Decimal::from_ratio(
            token_weights[1].weight.numerator() * max_bond,
            token_weights[1].weight.denominator(),
        );
        let expected_third_deposit = Decimal::from_ratio(
            token_weights[2].weight.numerator() * max_bond,
            token_weights[2].weight.denominator(),
        );

        assert_eq!(deposit[0].amount, expected_first_deposit.to_uint_floor());
        assert_eq!(deposit[1].amount, expected_second_deposit.to_uint_floor());
        assert_eq!(deposit[2].amount, expected_third_deposit.to_uint_floor());

        println!("remainder: {remainder:?}");
        println!("deposit: {deposit:?}");
        assert_eq!(
            remainder[0].amount,
            funds[0].amount - expected_first_deposit.to_uint_floor()
        );
        assert_eq!(
            remainder[1].amount,
            funds[1].amount - expected_second_deposit.to_uint_floor()
        );
        assert_eq!(
            remainder[2].amount,
            funds[2].amount - expected_third_deposit.to_uint_floor()
        );
    }

    #[test]
    fn test_may_pay_with_one_primitive() {
        let mut deps = mock_deps_with_primitives(vec![(
            "quasar123".to_string(),
            "ibc/uosmo".to_string(),
            Uint128::from(100u128),
            Uint128::from(100u128),
        )]);
        let init_msg = init_msg_with_primitive_details(vec![(
            "quasar123".to_string(),
            "ibc/uosmo".to_string(),
            Decimal::from_str("2.0").unwrap(),
        )]);

        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(1, res.messages.len());

        let invest_query = crate::msg::QueryMsg::Investment {};
        let query_res = query(deps.as_ref(), env, invest_query).unwrap();

        let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

        let funds = &[Coin {
            denom: "ibc/uosmo".to_string(),
            amount: Uint128::from(200u128),
        }];

        // load cached balance of primitive contracts
        let deposit_amount_ratio =
            get_deposit_amount_weights(&deps.as_ref(), &investment_response.info.primitives)
                .unwrap();

        let token_weights: Vec<CoinWeight> =
            get_token_amount_weights(&deposit_amount_ratio.ratio).unwrap();

        let max_bond = get_max_bond(funds, &token_weights).unwrap();

        let (coins, remainder) =
            get_deposit_and_remainder_for_ratio(funds, max_bond, &deposit_amount_ratio).unwrap();

        assert_eq!(coins.len(), 1);
        assert_eq!(coins[0].amount, Uint128::from(200u128));

        assert_eq!(remainder.len(), 1);
        assert_eq!(remainder[0].amount, Uint128::from(0u128));
    }

    #[test]
    fn test_may_pay_with_even_ratio() {
        let mut deps = mock_deps_with_primitives(even_primitives());
        let init_msg = init_msg_with_primitive_details(even_primitive_details());
        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(1, res.messages.len());

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

    #[test]
    fn test_may_pay_with_uneven_ratio() {
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
                Uint128::from(150u128),
                Uint128::from(200u128),
            ),
            (
                "quasar125".to_string(),
                "ibc/ustars".to_string(),
                Uint128::from(250u128),
                Uint128::from(300u128),
            ),
        ]);
        let init_msg = init_msg_with_primitive_details(vec![
            (
                "quasar123".to_string(),
                "ibc/uosmo".to_string(),
                Decimal::from_str("2.0").unwrap(),
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
        assert_eq!(1, res.messages.len());

        let invest_query = crate::msg::QueryMsg::Investment {};
        let query_res = query(deps.as_ref(), env, invest_query).unwrap();

        let investment_response: InvestmentResponse = from_binary(&query_res).unwrap();

        let funds = &[
            Coin {
                denom: "ibc/uosmo".to_string(),
                amount: Uint128::from(200u128),
            },
            Coin {
                denom: "ibc/uatom".to_string(),
                amount: Uint128::from(150u128),
            },
            Coin {
                denom: "ibc/ustars".to_string(),
                amount: Uint128::from(140u128),
            },
        ];

        // load cached balance of primitive contracts
        let deposit_amount_ratio =
            get_deposit_amount_weights(&deps.as_ref(), &investment_response.info.primitives)
                .unwrap();

        let token_weights: Vec<CoinWeight> =
            get_token_amount_weights(&deposit_amount_ratio.ratio).unwrap();

        let max_bond = get_max_bond(funds, &token_weights).unwrap();

        let (coins, remainder) =
            get_deposit_and_remainder_for_ratio(funds, max_bond, &deposit_amount_ratio).unwrap();

        assert_eq!(coins.len(), 3);
        assert_eq!(coins[0].amount, Uint128::from(199u128)); // these numbers have been verified
        assert_eq!(coins[1].amount, Uint128::from(133u128));
        assert_eq!(coins[2].amount, Uint128::from(119u128));

        assert_eq!(remainder.len(), 3);
        assert_eq!(remainder[0].amount, Uint128::from(1u128));
        assert_eq!(remainder[1].amount, Uint128::from(17u128));
        assert_eq!(remainder[2].amount, Uint128::from(21u128));
    }

    #[test]
    fn proper_bond() {
        let mut deps = mock_deps_with_primitives(even_primitives());
        let init_msg = init_msg_with_primitive_details(even_primitive_details());
        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(1, res.messages.len());

        let deposit_info = mock_info(TEST_DEPOSITOR, &even_deposit());
        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };
        let res = execute(deps.as_mut(), env, deposit_info, deposit_msg).unwrap();
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
                assert_eq!(to_address, TEST_DEPOSITOR);
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
        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(1, res.messages.len());

        let deposit_info = mock_info(TEST_DEPOSITOR, &even_deposit());
        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };
        let res = execute(deps.as_mut(), env, deposit_info, deposit_msg).unwrap();
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
        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let _ = init(deps_1.as_mut(), &init_msg, &env, &info);
        let _ = init(deps_2.as_mut(), &init_msg, &env, &info);

        let deposit_info = mock_info(TEST_DEPOSITOR, &even_deposit());
        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };
        let res_1 = execute(
            deps_1.as_mut(),
            env.clone(),
            deposit_info.clone(),
            deposit_msg.clone(),
        )
        .unwrap_err();
        assert_eq!(res_1.to_string(), "Generic error: Unexpected primitive state, either both supply and balance should be zero, or neither.");
        let res_2 = execute(deps_2.as_mut(), env, deposit_info, deposit_msg).unwrap_err();
        assert_eq!(res_2.to_string(), "Generic error: Unexpected primitive state, either both supply and balance should be zero, or neither.");
    }

    #[test]
    fn proper_bond_response_callback() {
        let mut deps = mock_deps_with_primitives(even_primitives());
        let init_msg = init_msg_with_primitive_details(even_primitive_details());
        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(1, res.messages.len());

        let reply_msg = reply_msg();
        let res = reply(deps.as_mut(), env.clone(), reply_msg).unwrap();
        assert_eq!(res.messages.len(), 0);

        let deposit_info = mock_info(TEST_DEPOSITOR, &even_deposit());
        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };
        let res = execute(deps.as_mut(), env.clone(), deposit_info, deposit_msg).unwrap();
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
        assert_eq!(p3_res.messages.len(), 1);

        let balance_query = crate::msg::QueryMsg::Balance {
            address: TEST_DEPOSITOR.to_string(),
        };
        let balance_res = query(deps.as_ref(), env, balance_query).unwrap();
        let balance: BalanceResponse = from_binary(&balance_res).unwrap();

        assert_eq!(balance.balance, Uint128::from(99u128));
    }

    #[test]
    fn proper_unbond() {
        let mut deps = mock_deps_with_primitives(even_primitives());
        let init_msg = init_msg_with_primitive_details(even_primitive_details());
        let info = mock_info(TEST_CREATOR, &[]);
        let mut env = mock_env();
        let res = init(deps.as_mut(), &init_msg, &env, &info);
        assert_eq!(1, res.messages.len());

        let reply_msg = reply_msg();
        let _res = reply(deps.as_mut(), env.clone(), reply_msg).unwrap();

        let deposit_info = mock_info(TEST_DEPOSITOR, &even_deposit());
        let deposit_msg = ExecuteMsg::Bond {
            recipient: Option::None,
        };
        let res = execute(deps.as_mut(), env.clone(), deposit_info, deposit_msg).unwrap();
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
        println!("p3_res: {p3_res:?}");
        assert_eq!(p3_res.messages.len(), 1);
        if let CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr,
            msg,
            funds: _,
        }) = &p3_res.messages[0].msg
        {
            assert_eq!(contract_addr, "vault_rewards_addr");
            if let vault_rewards::msg::ExecuteMsg::Vault(
                vault_rewards::msg::VaultExecuteMsg::UpdateUserRewardIndex(user_reward_index),
            ) = from_binary(msg).unwrap()
            {
                assert_eq!(user_reward_index, TEST_DEPOSITOR);
            } else {
                panic!("wrong message");
            }
        } else {
            panic!("wrong message");
        }

        let balance_query = crate::msg::QueryMsg::Balance {
            address: TEST_DEPOSITOR.to_string(),
        };
        let balance_res = query(deps.as_ref(), env.clone(), balance_query).unwrap();
        let balance: BalanceResponse = from_binary(&balance_res).unwrap();

        assert_eq!(balance.balance, Uint128::from(99u128));

        // start unbond
        let unbond_info = mock_info(TEST_DEPOSITOR, &[]);
        let unbond_msg = ExecuteMsg::Unbond {
            amount: Option::Some(balance.balance),
        };
        let unbond_res = execute(deps.as_mut(), env.clone(), unbond_info, unbond_msg).unwrap();
        assert_eq!(unbond_res.messages.len(), 4);
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
        if let CosmosMsg::Wasm(wasm_msg) = &unbond_res.messages[3].msg {
            if let WasmMsg::Execute {
                contract_addr,
                msg,
                funds,
            } = wasm_msg
            {
                assert_eq!(contract_addr, "vault_rewards_addr");
                assert!(funds.is_empty());
                if let vault_rewards::msg::ExecuteMsg::Vault(
                    vault_rewards::msg::VaultExecuteMsg::UpdateUserRewardIndex(user_addr),
                ) = from_binary(msg).unwrap()
                {
                    assert_eq!(user_addr, TEST_DEPOSITOR);
                } else {
                    assert!(false);
                }
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
        let do_unbond_info = mock_info(TEST_DEPOSITOR, &[]);
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
                assert_eq!(to_address, TEST_DEPOSITOR);
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
                assert_eq!(to_address, TEST_DEPOSITOR);
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
                assert_eq!(to_address, TEST_DEPOSITOR);
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

    #[test]
    fn test_dup_token_deposits() {
        let env = mock_env();
        const ADDRESS: &str = "quasar";
        const DENOM_LOCAL_CHAIN: &str = "ibc/uosmo";
        const SHARES: Uint128 = Uint128::new(100);
        const BALANCE: Uint128 = Uint128::new(100);

        let deposit_amounts = vec![
            Uint128::new(10),
            Uint128::new(1_000),
            Uint128::new(1_000_000_000),
        ];

        for deposit_amount in deposit_amounts {
            // test params
            let weights = vec![
                Decimal::from_str("0.2").unwrap(),
                Decimal::from_str("0.3").unwrap(),
                Decimal::from_str("0.5").unwrap(),
            ];

            let mut primitive_states: Vec<(String, String, Uint128, Uint128)> = Vec::new();
            let mut primitive_details: Vec<(String, String, Decimal)> = Vec::new();
            for i in 1..=3 {
                primitive_states.push((
                    format!("{ADDRESS}{i}"),
                    DENOM_LOCAL_CHAIN.to_string(),
                    SHARES,
                    BALANCE,
                ));
                primitive_details.push((
                    format!("{ADDRESS}{i}"),
                    DENOM_LOCAL_CHAIN.to_string(),
                    weights[i - 1],
                ));
            }
            let mut deps = mock_deps_with_primitives(primitive_states);
            let init_info = mock_info(TEST_CREATOR, &[]);

            let init_msg = init_msg_with_primitive_details(primitive_details);
            let init_res = init(deps.as_mut(), &init_msg, &env, &init_info);
            assert_eq!(1, init_res.messages.len());

            // deposit 3 times to the same vault
            let deposit_info = mock_info(
                TEST_CREATOR,
                &[Coin {
                    denom: DENOM_LOCAL_CHAIN.to_string(),
                    amount: deposit_amount,
                }],
            );
            let deposit_msg = ExecuteMsg::Bond {
                recipient: Option::None,
            };
            let deposit_res = execute(
                deps.as_mut(),
                env.clone(),
                deposit_info.clone(),
                deposit_msg,
            )
            .unwrap();

            assert_eq!(deposit_res.messages.len(), init_msg.primitives.len() + 1);

            // total money sent to the vault
            let total_money = deposit_info.funds[0].amount;

            // total weight from init msg
            let total_weight: Decimal = init_msg.primitives.iter().map(|p| p.weight).sum();
            assert_eq!(total_weight, Decimal::one());

            for (i, msg) in deposit_res.messages.iter().enumerate() {
                if let CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: _,
                    funds,
                    msg: _,
                }) = &msg.msg
                {
                    // weight[i] / total_weight * total_money = money_output[i]
                    let expected = init_msg.primitives[i].weight / total_weight * total_money;
                    assert_eq!(expected, funds[0].amount);
                }
            }
        }
    }
}
