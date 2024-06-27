#[cfg(test)]
mod tests {
    use crate::execute::admin::execute_auto_claim;
    use crate::execute::vault::execute_update_user_reward_index;
    use crate::state::{Config, DistributionSchedule, CONFIG};
    use cosmwasm_std::testing::{mock_env, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{
        attr, from_binary, to_binary, Addr, BankMsg, BankQuery, Coin, ContractResult, CosmosMsg,
        Empty, OwnedDeps, Querier, QuerierResult, QueryRequest, Storage, SubMsg, SystemError,
        SystemResult, Uint128, WasmQuery,
    };
    use cw20::{BalanceResponse, Cw20QueryMsg, TokenInfoResponse};
    use cw_asset::AssetInfo;
    use schemars::_serde_json::from_slice;
    use std::collections::HashMap;

    fn mock_config(storage: &mut dyn Storage) {
        let config = Config {
            vault_token: Addr::unchecked("vault_token"),
            reward_token: AssetInfo::Native("reward_token".to_string()),
            distribution_schedules: vec![DistributionSchedule {
                start: 1,
                end: 101,
                amount: Uint128::new(100000),
            }],
            total_claimed: Uint128::zero(),
        };
        CONFIG.save(storage, &config).unwrap();
    }

    struct CustomQuerier {
        base: MockQuerier,
        cw20_balances: HashMap<String, HashMap<String, Uint128>>, // Nested hashmap to handle different contract addresses and addresses within them
        native_balances: HashMap<String, Uint128>,                // To handle native token balances
    }

    impl Querier for CustomQuerier {
        fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
            let request: QueryRequest<Empty> = match from_slice(bin_request) {
                Ok(v) => v,
                Err(e) => {
                    return SystemResult::Err(SystemError::InvalidRequest {
                        error: format!("{:?}", e),
                        request: bin_request.into(),
                    })
                }
            };

            self.handle_query(&request)
        }
    }

    impl CustomQuerier {
        pub fn new(base: MockQuerier) -> Self {
            CustomQuerier {
                base,
                cw20_balances: HashMap::new(),
                native_balances: HashMap::new(),
            }
        }

        pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
            match request {
                QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                    match from_binary(msg) {
                        Ok(Cw20QueryMsg::Balance { address }) => {
                            if let Some(contract_balances) = self.cw20_balances.get(contract_addr) {
                                if let Some(balance) = contract_balances.get(&address.to_string()) {
                                    let response = BalanceResponse { balance: *balance };
                                    SystemResult::Ok(ContractResult::Ok(
                                        to_binary(&response).unwrap(),
                                    ))
                                } else {
                                    SystemResult::Err(SystemError::NoSuchContract {
                                        addr: address.clone(),
                                    })
                                }
                            } else {
                                SystemResult::Err(SystemError::NoSuchContract {
                                    addr: contract_addr.clone(),
                                })
                            }
                        }
                        Ok(Cw20QueryMsg::TokenInfo {}) => {
                            if contract_addr == "vault_token" {
                                let response = TokenInfoResponse {
                                    name: "Vault Token".to_string(),
                                    symbol: "VAULT".to_string(),
                                    decimals: 6,
                                    total_supply: Uint128::new(1000),
                                };
                                SystemResult::Ok(ContractResult::Ok(to_binary(&response).unwrap()))
                            } else {
                                SystemResult::Err(SystemError::NoSuchContract {
                                    addr: contract_addr.clone(),
                                })
                            }
                        }
                        _ => self.base.handle_query(request),
                    }
                }
                QueryRequest::Bank(BankQuery::Balance { address, denom }) => {
                    if denom == "reward_token" {
                        if let Some(balance) = self.native_balances.get(address) {
                            let response = cosmwasm_std::BalanceResponse {
                                amount: Coin {
                                    denom: "reward_token".to_string(),
                                    amount: *balance,
                                },
                            };
                            SystemResult::Ok(ContractResult::Ok(to_binary(&response).unwrap()))
                        } else {
                            SystemResult::Err(SystemError::NoSuchContract {
                                addr: address.clone(),
                            })
                        }
                    } else {
                        self.base.handle_query(request)
                    }
                }
                _ => self.base.handle_query(request),
            }
        }

        pub fn set_cw20_balance(&mut self, contract_addr: &str, address: &str, balance: Uint128) {
            self.cw20_balances
                .entry(contract_addr.to_string())
                .or_insert_with(HashMap::new)
                .insert(address.to_string(), balance);
        }

        pub fn set_native_balance(&mut self, address: &str, balance: Uint128) {
            self.native_balances.insert(address.to_string(), balance);
        }
    }

    fn mock_dependencies_with_custom_querier() -> OwnedDeps<MockStorage, MockApi, CustomQuerier> {
        let base_querier = MockQuerier::new(&[(MOCK_CONTRACT_ADDR, &[])]);
        let custom_querier = CustomQuerier::new(base_querier);
        OwnedDeps {
            storage: MockStorage::new(),
            api: MockApi::default(),
            querier: custom_querier,
            custom_query_type: Default::default(),
        }
    }

    #[test]
    fn test_execute_auto_claim() {
        let mut deps = mock_dependencies_with_custom_querier();
        let mut env = mock_env();

        // Initialize mock config
        mock_config(&mut deps.storage);
        deps.querier
            .set_cw20_balance("vault_token", "user1", Uint128::new(1000));
        deps.querier
            .set_native_balance(&env.contract.address.to_string(), Uint128::new(100000));

        // Define user addresses
        let user_addresses = vec![Addr::unchecked("user1")];

        // update height
        env.block.height = 1;

        execute_update_user_reward_index(deps.as_mut(), env.clone(), Addr::unchecked("user1"))
            .unwrap();

        // update height
        env.block.height = 101;

        let res = execute_auto_claim(deps.as_mut(), env.clone(), user_addresses).unwrap();

        // Verify the response
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "claim"),
                attr("user", "user1"),
                attr("amount", "100000"),
            ]
        );

        let expected_messages: Vec<SubMsg> = vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: "user1".to_string(),
            amount: vec![Coin {
                denom: "reward_token".to_string(),
                amount: Uint128::new(100000),
            }],
        }))];

        assert_eq!(res.messages, expected_messages);
    }

    #[test]
    fn test_execute_auto_claim_multiple_users() {
        let mut deps = mock_dependencies_with_custom_querier();
        let mut env = mock_env();

        // Initialize mock config
        mock_config(&mut deps.storage);
        deps.querier
            .set_cw20_balance("vault_token", "user1", Uint128::new(100));
        deps.querier
            .set_cw20_balance("vault_token", "user2", Uint128::new(900));
        deps.querier
            .set_native_balance(&env.contract.address.to_string(), Uint128::new(100000));

        // Define user addresses
        let user_addresses = vec![Addr::unchecked("user1"), Addr::unchecked("user2")];

        // Update height
        env.block.height = 1;

        execute_update_user_reward_index(deps.as_mut(), env.clone(), Addr::unchecked("user1"))
            .unwrap();
        execute_update_user_reward_index(deps.as_mut(), env.clone(), Addr::unchecked("user2"))
            .unwrap();

        // Update height
        env.block.height = 101;

        // Call the function to test
        let res = execute_auto_claim(deps.as_mut(), env.clone(), user_addresses).unwrap();

        // Verify the response
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "claim"),
                attr("user", "user1"),
                attr("amount", "10000"),
                attr("action", "claim"),
                attr("user", "user2"),
                attr("amount", "90000"),
            ]
        );

        let expected_messages: Vec<SubMsg> = vec![
            SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                to_address: "user1".to_string(),
                amount: vec![Coin {
                    denom: "reward_token".to_string(),
                    amount: Uint128::new(10000),
                }],
            })),
            SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                to_address: "user2".to_string(),
                amount: vec![Coin {
                    denom: "reward_token".to_string(),
                    amount: Uint128::new(90000),
                }],
            })),
        ];

        assert_eq!(res.messages, expected_messages);
    }
}
