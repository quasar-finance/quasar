#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR, MockApi};
    use cosmwasm_std::{from_binary, to_binary, Addr, Api, BankMsg, Coin, ContractResult, CosmosMsg, Empty, Env, OwnedDeps, Querier, QuerierResult, QueryRequest, Response, StdError, SystemError, SystemResult, Uint128, Storage, WasmQuery, DepsMut, attr, SubMsg, QuerierWrapper, Deps, BlockInfo};
    use cw20::{BalanceResponse, Cw20Contract, Cw20QueryMsg, TokenInfoResponse};
    use cw_asset::{Asset, AssetInfo};
    use schemars::_serde_json::from_slice;
    use crate::execute::admin::execute_auto_claim;
    use crate::execute::user::{execute_claim, get_claim_amount};
    use crate::execute::vault::execute_update_user_reward_index;
    use crate::state::{Config, CONFIG, DistributionSchedule, REWARD_INDEX, RewardIndex, USER_REWARD_INDEX, UserBalance, UserRewardIndex};
    use crate::VaultRewardsError;

    fn mock_config(storage: &mut dyn Storage) {
        let config = Config {
            vault_token: Addr::unchecked("vault_token"),
            reward_token: AssetInfo::Native("reward_token".to_string()),
            distribution_schedules: vec![
                DistributionSchedule{
                    start: 1,
                    end: 100,
                    amount: Uint128::new(100000),
                },
            ],
            total_claimed: Uint128::zero(),
        };
        CONFIG.save(storage, &config).unwrap();
    }


    struct CustomQuerier {
        base: MockQuerier,
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
            CustomQuerier { base }
        }

        pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
            match request {
                QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                    match from_binary(msg) {
                        Ok(Cw20QueryMsg::Balance { address: _ }) => {
                            if contract_addr == "vault_token" {
                                let balance = Uint128::new(100);
                                let response = BalanceResponse { balance };
                                SystemResult::Ok(ContractResult::Ok(to_binary(&response).unwrap()))
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
                _ => self.base.handle_query(request),
            }
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
        let info = mock_info("caller", &[]);

        // Initialize mock config
        mock_config(&mut deps.storage);

        // Define user addresses
        let user_addresses = vec![Addr::unchecked("user1")];

        // update height
        env.block.height = 20;

        execute_update_user_reward_index(deps.as_mut(), env.clone(), Addr::unchecked("user1")).unwrap();

        // update height
        env.block.height = 80;

        // todo : fails here as contract does not have funds to send.
        let res = execute_auto_claim(deps.as_mut(), env.clone(), user_addresses).unwrap();


        // Verify the response
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "claim"),
                attr("user", "user1"),
                attr("amount", "50"),
            ]
        );

        let expected_messages: Vec<SubMsg> = vec![
            SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                to_address: "user1".to_string(),
                amount: vec![],
            })),
            SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                to_address: "user2".to_string(),
                amount: vec![],
            })),
        ];

        assert_eq!(res.messages, expected_messages);
    }
}