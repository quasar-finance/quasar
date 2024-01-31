// rewards

#[cfg(test)]
mod tests {
    use std::ops::Mul;
    use std::str::FromStr;

    use apollo_cw_asset::AssetInfoBase;
    use cosmwasm_std::Decimal;
    use cosmwasm_std::{assert_approx_eq, Coin};
    use cw_dex::osmosis::OsmosisPool;
    use cw_dex_router::operations::{SwapOperationBase, SwapOperationsListUnchecked};
    use cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension;
    use osmosis_std::types::cosmos::bank::v1beta1::{MsgSend, QueryBalanceRequest};
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SwapAmountInRoute,
    };
    use osmosis_test_tube::RunnerError::ExecuteError;
    use osmosis_test_tube::{Account, Bank, Module, PoolManager, Runner, Wasm};

    use crate::msg::UserBalanceQueryMsg::UserSharesBalance;
    use crate::msg::{AutoCompoundAsset, ExecuteMsg, ExtensionQueryMsg, ModifyRangeMsg};
    use crate::query::UserSharesBalanceResponse;
    use crate::test_tube::helpers::{
        get_event_attributes_by_ty_and_key, get_event_value_amount_numeric,
    };
    use crate::test_tube::initialize::initialize::{default_init, dex_cl_init_lp_pools, dex_cl_init_cl_pools};

    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";
    const DENOM_REWARD: &str = "ustride";
    const ACCOUNTS_NUM: u64 = 10;
    const ACCOUNTS_INIT_BALANCE: u128 = 1_000_000_000_000_000;
    const DEPOSIT_AMOUNT: u128 = 5_000_000;
    const SWAPS_NUM: usize = 10;
    const SWAPS_AMOUNT: &str = "1000000000";
    const DISTRIBUTION_CYCLES: usize = 25;

    #[test]
    #[ignore]
    fn test_auto_compound_rewards_lp_pools() {
        let (app, contract_address, dex_router_addr, cl_pool_id, lp_pool1, lp_pool2, admin) =
            dex_cl_init_lp_pools();
        let bm = Bank::new(&app);

        // Initialize accounts
        let accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                ACCOUNTS_NUM,
            )
            .unwrap();

        let wasm = Wasm::new(&app);

        for account in &accounts {
            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::ExactDeposit { recipient: None },
                    &[
                        Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                        Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                    ],
                    account,
                )
                .unwrap();
        }

        // Declare swapper accounts
        let swapper = &accounts[0];

        // Swaps to generate spread rewards on previously created user positions
        for _ in 0..SWAPS_NUM {
            PoolManager::new(&app)
                .swap_exact_amount_in(
                    MsgSwapExactAmountIn {
                        sender: swapper.address(),
                        routes: vec![SwapAmountInRoute {
                            pool_id: cl_pool_id,
                            token_out_denom: DENOM_BASE.to_string(),
                        }],
                        token_in: Some(OsmoCoin {
                            denom: DENOM_QUOTE.to_string(),
                            amount: SWAPS_AMOUNT.to_string(),
                        }),
                        token_out_min_amount: "1".to_string(),
                    },
                    &swapper,
                )
                .unwrap();
        }

        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                &admin,
            )
            .unwrap();

        let send = bm.send(
            MsgSend {
                from_address: admin.address(),
                to_address: contract_address.to_string(),
                amount: vec![OsmoCoin {
                    denom: DENOM_REWARD.to_string(),
                    amount: "100000000000".to_string(),
                }],
            },
            &admin,
        );

        let balances_quote = bm
            .query_balance(&QueryBalanceRequest {
                address: contract_address.to_string(),
                denom: DENOM_QUOTE.to_string(),
            })
            .unwrap();
        assert_eq!("4999".to_string(), balances_quote.balance.unwrap().amount);
        let balances_rewards = bm
            .query_balance(&QueryBalanceRequest {
                address: contract_address.to_string(),
                denom: DENOM_REWARD.to_string(),
            })
            .unwrap();
        assert_eq!(
            "100000000000".to_string(),
            balances_rewards.balance.unwrap().amount,
        );

        let path1 = vec![
            SwapOperationBase::new(
                cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool2)),
                AssetInfoBase::Native(DENOM_REWARD.to_string()),
                AssetInfoBase::Native(DENOM_QUOTE.to_string()),
            ),
            SwapOperationBase::new(
                cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool1)),
                AssetInfoBase::Native(DENOM_QUOTE.to_string()),
                AssetInfoBase::Native(DENOM_BASE.to_string()),
            ),
        ];
        let path2 = vec![SwapOperationBase::new(
            cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool2)),
            AssetInfoBase::Native(DENOM_REWARD.to_string()),
            AssetInfoBase::Native(DENOM_QUOTE.to_string()),
        )];

        let auto_compound = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::AutoCompoundRewards {
                    force_swap_route: false,
                    swap_routes: vec![AutoCompoundAsset {
                        token_in_denom: DENOM_REWARD.to_string(),
                        recommended_swap_route_token_0: Option::from(
                            SwapOperationsListUnchecked::new(path1),
                        ),
                        recommended_swap_route_token_1: Option::from(
                            SwapOperationsListUnchecked::new(path2),
                        ),
                    }],
                }),
                &[],
                &admin,
            )
            .unwrap();

        let balances_after = bm
            .query_balance(&QueryBalanceRequest {
                address: contract_address.to_string(),
                denom: DENOM_REWARD.to_string(),
            })
            .unwrap();
        assert_eq!("0".to_string(), balances_after.balance.unwrap().amount);
        // let balances_after = bm.query_balance(&QueryBalanceRequest { address: contract_address.to_string(), denom: DENOM_BASE.to_string() }).unwrap();
        // assert_eq!("49005000373".to_string(), balances_after.balance.unwrap().amount);
        // let balances_after = bm.query_balance(&QueryBalanceRequest { address: contract_address.to_string(), denom: DENOM_QUOTE.to_string() }).unwrap();
        // assert_eq!("49500004998".to_string(), balances_after.balance.unwrap().amount);

        let update_range = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("0.51").unwrap(),
                        upper_price: Decimal::from_str("1.49").unwrap(),
                        max_slippage: Decimal::bps(9500),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();

        for account in &accounts {
            let balances_before_withdraw_quote_denom = bm
                .query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_QUOTE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount
                .parse::<u128>()
                .unwrap_or_default();
            let balances_before_withdraw_base_denom = bm
                .query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_BASE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount
                .parse::<u128>()
                .unwrap_or_default();

            let shares_to_redeem: UserSharesBalanceResponse = wasm
                .query(
                    contract_address.as_str(),
                    &VaultExtension(ExtensionQueryMsg::Balances(UserSharesBalance {
                        user: account.address(),
                    })),
                )
                .unwrap();

            if shares_to_redeem.balance.is_zero() {
                continue;
            }

            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::Redeem {
                        recipient: None,
                        amount: shares_to_redeem.balance,
                    },
                    &[],
                    account,
                )
                .unwrap();

            let balances_after_withdraw_quote_denom = bm
                .query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_QUOTE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount
                .parse::<u128>()
                .unwrap_or_default();
            let balances_after_withdraw_base_denom = bm
                .query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_BASE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount
                .parse::<u128>()
                .unwrap_or_default();

            assert_eq!(
                true,
                balances_after_withdraw_quote_denom - balances_before_withdraw_quote_denom
                    > DEPOSIT_AMOUNT
            );
            assert_eq!(
                true,
                balances_after_withdraw_base_denom - balances_before_withdraw_base_denom
                    > DEPOSIT_AMOUNT
            );
        }
    }

    #[test]
    #[ignore]
    fn test_auto_compound_rewards_cl_pools() {
        let (app, contract_address, dex_router_addr, cl_pool_id, lp_pool1, lp_pool2, lp_pool3, admin) =
            dex_cl_init_cl_pools();
        let bm = Bank::new(&app);

        // Initialize accounts
        let accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                ACCOUNTS_NUM,
            )
            .unwrap();

        let wasm = Wasm::new(&app);

        for account in &accounts {
            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::ExactDeposit { recipient: None },
                    &[
                        Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                        Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                    ],
                    account,
                )
                .unwrap();
        }

        // Declare swapper accounts
        let swapper = &accounts[0];

        // Swaps to generate spread rewards on previously created user positions
        for _ in 0..SWAPS_NUM {
            PoolManager::new(&app)
                .swap_exact_amount_in(
                    MsgSwapExactAmountIn {
                        sender: swapper.address(),
                        routes: vec![SwapAmountInRoute {
                            pool_id: cl_pool_id,
                            token_out_denom: DENOM_BASE.to_string(),
                        }],
                        token_in: Some(OsmoCoin {
                            denom: DENOM_QUOTE.to_string(),
                            amount: SWAPS_AMOUNT.to_string(),
                        }),
                        token_out_min_amount: "1".to_string(),
                    },
                    &swapper,
                )
                .unwrap();
        }

        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                &admin,
            )
            .unwrap();

        let send = bm.send(
            MsgSend {
                from_address: admin.address(),
                to_address: contract_address.to_string(),
                amount: vec![OsmoCoin {
                    denom: DENOM_REWARD.to_string(),
                    amount: "100000000000".to_string(),
                }],
            },
            &admin,
        );

        let balances_quote = bm
            .query_balance(&QueryBalanceRequest {
                address: contract_address.to_string(),
                denom: DENOM_QUOTE.to_string(),
            })
            .unwrap();
        assert_eq!("4999".to_string(), balances_quote.balance.unwrap().amount);
        let balances_rewards = bm
            .query_balance(&QueryBalanceRequest {
                address: contract_address.to_string(),
                denom: DENOM_REWARD.to_string(),
            })
            .unwrap();
        assert_eq!(
            "100000000000".to_string(),
            balances_rewards.balance.unwrap().amount,
        );

        let path1 = vec![
            SwapOperationBase::new(
                cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool2)),
                AssetInfoBase::Native(DENOM_REWARD.to_string()),
                AssetInfoBase::Native(DENOM_QUOTE.to_string()),
            ),
            SwapOperationBase::new(
                cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool1)),
                AssetInfoBase::Native(DENOM_QUOTE.to_string()),
                AssetInfoBase::Native(DENOM_BASE.to_string()),
            ),
        ];
        let path2 = vec![SwapOperationBase::new(
            cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool2)),
            AssetInfoBase::Native(DENOM_REWARD.to_string()),
            AssetInfoBase::Native(DENOM_QUOTE.to_string()),
        )];

        let auto_compound = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::AutoCompoundRewards {
                    force_swap_route: false,
                    swap_routes: vec![AutoCompoundAsset {
                        token_in_denom: DENOM_REWARD.to_string(),
                        recommended_swap_route_token_0: Option::from(
                            SwapOperationsListUnchecked::new(path1),
                        ),
                        recommended_swap_route_token_1: Option::from(
                            SwapOperationsListUnchecked::new(path2),
                        ),
                    }],
                }),
                &[],
                &admin,
            )
            .unwrap();

        let balances_after = bm
            .query_balance(&QueryBalanceRequest {
                address: contract_address.to_string(),
                denom: DENOM_REWARD.to_string(),
            })
            .unwrap();
        assert_eq!("0".to_string(), balances_after.balance.unwrap().amount);
        // let balances_after = bm.query_balance(&QueryBalanceRequest { address: contract_address.to_string(), denom: DENOM_BASE.to_string() }).unwrap();
        // assert_eq!("49005000373".to_string(), balances_after.balance.unwrap().amount);
        // let balances_after = bm.query_balance(&QueryBalanceRequest { address: contract_address.to_string(), denom: DENOM_QUOTE.to_string() }).unwrap();
        // assert_eq!("49500004998".to_string(), balances_after.balance.unwrap().amount);

        let update_range = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("0.51").unwrap(),
                        upper_price: Decimal::from_str("1.49").unwrap(),
                        max_slippage: Decimal::bps(9500),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();

        for account in &accounts {
            let balances_before_withdraw_quote_denom = bm
                .query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_QUOTE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount
                .parse::<u128>()
                .unwrap_or_default();
            let balances_before_withdraw_base_denom = bm
                .query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_BASE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount
                .parse::<u128>()
                .unwrap_or_default();

            let shares_to_redeem: UserSharesBalanceResponse = wasm
                .query(
                    contract_address.as_str(),
                    &VaultExtension(ExtensionQueryMsg::Balances(UserSharesBalance {
                        user: account.address(),
                    })),
                )
                .unwrap();

            if shares_to_redeem.balance.is_zero() {
                continue;
            }

            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::Redeem {
                        recipient: None,
                        amount: shares_to_redeem.balance,
                    },
                    &[],
                    account,
                )
                .unwrap();

            let balances_after_withdraw_quote_denom = bm
                .query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_QUOTE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount
                .parse::<u128>()
                .unwrap_or_default();
            let balances_after_withdraw_base_denom = bm
                .query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_BASE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount
                .parse::<u128>()
                .unwrap_or_default();

            assert_eq!(
                true,
                balances_after_withdraw_quote_denom - balances_before_withdraw_quote_denom
                    > DEPOSIT_AMOUNT
            );
            assert_eq!(
                true,
                balances_after_withdraw_base_denom - balances_before_withdraw_base_denom
                    > DEPOSIT_AMOUNT
            );
        }
    }

    #[test]
    #[ignore]
    fn test_rewards_single_distribute_claim() {
        let (app, contract_address, cl_pool_id, _admin) = default_init();

        // Initialize accounts
        let accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                ACCOUNTS_NUM,
            )
            .unwrap();

        // Depositing with users
        let wasm = Wasm::new(&app);
        for account in &accounts {
            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::ExactDeposit { recipient: None },
                    &[
                        Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                        Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                    ],
                    account,
                )
                .unwrap();
        }

        // Declare swapper and claimer accounts
        let swapper = &accounts[0];
        let claimer = &accounts[1];

        // Swaps to generate spread rewards on previously created user positions
        for _ in 0..SWAPS_NUM {
            PoolManager::new(&app)
                .swap_exact_amount_in(
                    MsgSwapExactAmountIn {
                        sender: swapper.address(),
                        routes: vec![SwapAmountInRoute {
                            pool_id: cl_pool_id,
                            token_out_denom: DENOM_BASE.to_string(),
                        }],
                        token_in: Some(OsmoCoin {
                            denom: DENOM_QUOTE.to_string(),
                            amount: SWAPS_AMOUNT.to_string(),
                        }),
                        token_out_min_amount: "1".to_string(),
                    },
                    &swapper,
                )
                .unwrap();
        }

        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                claimer,
            )
            .unwrap();

        // Extract 'tokens_out' attribute value for 'total_collect_spread_rewards'
        let tokens_out_spread_rewards = get_event_attributes_by_ty_and_key(
            &result,
            "total_collect_spread_rewards",
            vec!["tokens_out"],
        );

        // Assert that 'tokens_out' values for events are empty
        assert_ne!(tokens_out_spread_rewards[0].value, "".to_string());
        let tokens_out_spread_rewards_u128: u128 =
            get_event_value_amount_numeric(&tokens_out_spread_rewards[0].value);
        let expected_rewards_per_user =
            (tokens_out_spread_rewards_u128 as f64 * 0.8) as u64 / ACCOUNTS_NUM;

        // Collect init
        for _ in 0..(ACCOUNTS_NUM - 1) {
            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                    &[],
                    claimer,
                )
                .unwrap();
            // Extract the 'is_last_collection' attribute from the 'wasm' event
            let is_last_collection = get_event_attributes_by_ty_and_key(
                &result,
                "wasm",
                vec!["is_last_collection", "rewards_status"],
            );
            assert_eq!(is_last_collection[0].value, "Collecting".to_string());
            assert_eq!(is_last_collection[1].value, "false".to_string());
        }

        // Collect one more time to finish, even if we extra deposited with one more user we expect the distribution to finish
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                claimer,
            )
            .unwrap();

        // Extract the 'is_last_collection' attribute from the 'wasm' event
        let is_last_collection = get_event_attributes_by_ty_and_key(
            &result,
            "wasm",
            vec!["is_last_collection", "rewards_status"],
        );
        assert_eq!(is_last_collection[0].value, "Distributing".to_string());
        assert_eq!(is_last_collection[1].value, "true".to_string());

        for _ in 0..(ACCOUNTS_NUM - 1) {
            // Adjust the number of distribute actions as needed
            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                    &[],
                    claimer,
                )
                .unwrap();

            // Extract the 'is_last_distribution' attribute from the 'wasm' event
            let is_last_distribution = get_event_attributes_by_ty_and_key(
                &result,
                "wasm",
                vec!["is_last_distribution", "rewards_status"],
            );
            assert_eq!(is_last_distribution[0].value, "Distributing".to_string());
            assert_eq!(is_last_distribution[1].value, "false".to_string());
        }

        // Distribute one more time to finish, even if we extra deposited with one more user we expect the distribution to finish
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                claimer,
            )
            .unwrap();

        // Extract the 'is_last_distribution' attribute from the 'wasm' event
        let is_last_distribution = get_event_attributes_by_ty_and_key(
            &result,
            "wasm",
            vec!["is_last_distribution", "rewards_status"],
        );
        assert_eq!(is_last_distribution[0].value, "Ready".to_string());
        assert_eq!(is_last_distribution[1].value, "true".to_string());

        // Loop users and claim for each one of them
        for account in &accounts {
            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                    &[],
                    account,
                )
                .unwrap();

            let coin_received =
                get_event_attributes_by_ty_and_key(&result, "coin_received", vec!["amount"]);
            let coin_received_u128 = get_event_value_amount_numeric(&coin_received[1].value);
            // taking index 1 in this case as there are more then 1 coin_received tys
            assert_approx_eq!(
                coin_received_u128,
                expected_rewards_per_user as u128,
                "0.005"
            );
        }
    }

    #[test]
    #[ignore]
    fn test_rewards_single_distribute_claim_cycles() {
        let (app, contract_address, cl_pool_id, _admin) = default_init();

        // Initialize accounts
        let accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                ACCOUNTS_NUM,
            )
            .unwrap();

        // Declare swapper and claimer accounts
        let swapper = &accounts[0];
        let claimer = &accounts[1];

        for _ in 0..DISTRIBUTION_CYCLES {
            // Depositing with users
            let wasm = Wasm::new(&app);
            for account in &accounts {
                let _ = wasm
                    .execute(
                        contract_address.as_str(),
                        &ExecuteMsg::ExactDeposit { recipient: None },
                        &[
                            Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                            Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                        ],
                        account,
                    )
                    .unwrap();
            }

            // Swaps to generate spread rewards on previously created user positions
            for _ in 0..SWAPS_NUM {
                PoolManager::new(&app)
                    .swap_exact_amount_in(
                        MsgSwapExactAmountIn {
                            sender: swapper.address(),
                            routes: vec![SwapAmountInRoute {
                                pool_id: cl_pool_id,
                                token_out_denom: DENOM_BASE.to_string(),
                            }],
                            token_in: Some(OsmoCoin {
                                denom: DENOM_QUOTE.to_string(),
                                amount: SWAPS_AMOUNT.to_string(),
                            }),
                            token_out_min_amount: "1".to_string(),
                        },
                        &swapper,
                    )
                    .unwrap();
            }

            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                    &[],
                    claimer,
                )
                .unwrap();
            // Extract 'tokens_out' attribute value for 'total_collect_spread_rewards'
            let tokens_out_spread_rewards = get_event_attributes_by_ty_and_key(
                &result,
                "total_collect_spread_rewards",
                vec!["tokens_out"],
            );

            // Assert that 'tokens_out' values for events are empty
            assert_ne!(tokens_out_spread_rewards[0].value, "".to_string());
            let tokens_out_spread_rewards_u128: u128 =
                get_event_value_amount_numeric(&tokens_out_spread_rewards[0].value);
            let expected_rewards_per_user =
                (tokens_out_spread_rewards_u128 as f64 * 0.8) as u64 / ACCOUNTS_NUM;
            // Collect init
            for _ in 0..(ACCOUNTS_NUM - 1) {
                let result = wasm
                    .execute(
                        contract_address.as_str(),
                        &ExecuteMsg::VaultExtension(
                            crate::msg::ExtensionExecuteMsg::CollectRewards {},
                        ),
                        &[],
                        claimer,
                    )
                    .unwrap();
                // Extract the 'is_last_collection' attribute from the 'wasm' event
                let is_last_collection =
                    get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_collection"]);
                assert_eq!(is_last_collection[0].value, "false".to_string());
            }

            // Collect one more time to finish, even if we extra deposited with one more user we expect the distribution to finish
            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                    &[],
                    claimer,
                )
                .unwrap();

            // Extract the 'is_last_collection' attribute from the 'wasm' event
            let is_last_collection =
                get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_collection"]);
            assert_eq!(is_last_collection[0].value, "true".to_string());

            for _ in 0..(ACCOUNTS_NUM - 1) {
                // Adjust the number of distribute actions as needed
                let result = wasm
                    .execute(
                        contract_address.as_str(),
                        &ExecuteMsg::VaultExtension(
                            crate::msg::ExtensionExecuteMsg::CollectRewards {},
                        ),
                        &[],
                        claimer,
                    )
                    .unwrap();

                // Extract the 'is_last_distribution' attribute from the 'wasm' event
                let is_last_distribution = get_event_attributes_by_ty_and_key(
                    &result,
                    "wasm",
                    vec!["is_last_distribution"],
                );
                assert_eq!(is_last_distribution[0].value, "false".to_string());
            }

            // Distribute one more time to finish, even if we extra deposited with one more user we expect the distribution to finish
            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                    &[],
                    claimer,
                )
                .unwrap();

            // Extract the 'is_last_distribution' attribute from the 'wasm' event
            let is_last_distribution =
                get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_distribution"]);
            assert_eq!(is_last_distribution[0].value, "true".to_string());

            // Loop users and claim for each one of them
            for account in &accounts {
                let result = wasm
                    .execute(
                        contract_address.as_str(),
                        &ExecuteMsg::VaultExtension(
                            crate::msg::ExtensionExecuteMsg::CollectRewards {},
                        ),
                        &[],
                        account,
                    )
                    .unwrap();

                let coin_received =
                    get_event_attributes_by_ty_and_key(&result, "coin_received", vec!["amount"]);
                let coin_received_u128 = get_event_value_amount_numeric(&coin_received[1].value);
                // taking index 1 in this case as there are more then 1 coin_received tys
                assert_approx_eq!(
                    coin_received_u128,
                    expected_rewards_per_user as u128,
                    "0.005"
                );
            }
        }
    }

    #[test]
    #[ignore]
    fn test_rewards_single_distribute_claim_no_rewards_works() {
        let (app, contract_address, _cl_pool_id, _admin) = default_init();

        // Initialize accounts
        let accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                ACCOUNTS_NUM,
            )
            .unwrap();

        // Depositing with users
        let wasm = Wasm::new(&app);
        for account in &accounts {
            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::ExactDeposit { recipient: None },
                    &[
                        Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                        Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                    ],
                    account,
                )
                .unwrap();
        }

        // Declare claimer accounts
        let claimer = &accounts[0];

        // Collect and Distribute Rewards (there should be anything)
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                claimer,
            )
            .unwrap();
        // Extract 'tokens_out' attribute value for 'total_collect_incentives' and 'total_collect_spread_rewards'
        let tokens_out_incentives = get_event_attributes_by_ty_and_key(
            &result,
            "total_collect_incentives",
            vec!["tokens_out"],
        );
        let tokens_out_spread_rewards = get_event_attributes_by_ty_and_key(
            &result,
            "total_collect_spread_rewards",
            vec!["tokens_out"],
        );

        // Assert that 'tokens_out' values for both events are empty
        assert_eq!(tokens_out_incentives[0].value, "".to_string());
        assert_eq!(tokens_out_spread_rewards[0].value, "".to_string());

        // Try to collect one more time, this should be closing the process and set to Ready as there are not rewards
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                claimer,
            )
            .unwrap();

        let rewards_status =
            get_event_attributes_by_ty_and_key(&result, "wasm", vec!["rewards_status"]);
        assert_eq!(rewards_status[0].value, "Ready".to_string());
        // Extract the 'is_last_collection' attribute from the 'wasm' event
        let is_last_collection =
            get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_collection"]);
        assert_eq!(is_last_collection[0].value, "true".to_string());

        // Distribute just one time, as there are no rewards we expect this to clear the state even if 1 user < 10 users
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                claimer,
            )
            .unwrap_err();

        // Assert that the response is an error
        assert!(
            matches!(result, ExecuteError { msg } if msg.contains("failed to execute message; message index: 0: Vault is not distributing rewards, claiming is needed first: execute wasm contract failed"))
        );
    }

    #[test]
    #[ignore]
    fn test_rewards_single_distribute_claim_deposit_between() {
        let (app, contract_address, cl_pool_id, _admin) = default_init();

        // Initialize accounts
        let accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                ACCOUNTS_NUM,
            )
            .unwrap();

        // Depositing with users
        let wasm = Wasm::new(&app);
        for account in &accounts {
            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::ExactDeposit { recipient: None },
                    &[
                        Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                        Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                    ],
                    account,
                )
                .unwrap();
        }

        // Declare swapper and claimer accounts
        let swapper = &accounts[0];
        let claimer = &accounts[1];

        // Swaps to generate spread rewards on previously created user positions
        for _ in 0..SWAPS_NUM {
            PoolManager::new(&app)
                .swap_exact_amount_in(
                    MsgSwapExactAmountIn {
                        sender: swapper.address(),
                        routes: vec![SwapAmountInRoute {
                            pool_id: cl_pool_id,
                            token_out_denom: DENOM_BASE.to_string(),
                        }],
                        token_in: Some(OsmoCoin {
                            denom: DENOM_QUOTE.to_string(),
                            amount: SWAPS_AMOUNT.to_string(),
                        }),
                        token_out_min_amount: "1".to_string(),
                    },
                    &swapper,
                )
                .unwrap();
        }

        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                claimer,
            )
            .unwrap();

        // Extract 'tokens_out' attribute value for 'total_collect_spread_rewards'
        let tokens_out_spread_rewards = get_event_attributes_by_ty_and_key(
            &result,
            "total_collect_spread_rewards",
            vec!["tokens_out"],
        );

        // Collect init
        for _ in 0..(ACCOUNTS_NUM - 1) {
            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                    &[],
                    claimer,
                )
                .unwrap();
            // Extract the 'is_last_collection' attribute from the 'wasm' event
            let is_last_collection =
                get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_collection"]);
            assert_eq!(is_last_collection[0].value, "false".to_string());
        }

        // Deposit with old accounts to change amount of rewards in the current calculation
        for account in &accounts {
            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::ExactDeposit { recipient: None },
                    &[
                        Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                        Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                    ],
                    &account,
                )
                .unwrap();
        }

        // Collect one more time to finish, even if we extra deposited with one more user we expect the distribution to finish
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                claimer,
            )
            .unwrap();

        // Extract the 'is_last_collection' attribute from the 'wasm' event
        let is_last_collection =
            get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_collection"]);
        assert_eq!(is_last_collection[0].value, "true".to_string());

        for _ in 0..(ACCOUNTS_NUM - 1) {
            // Adjust the number of distribute actions as needed
            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                    &[],
                    claimer,
                )
                .unwrap();

            // Extract the 'is_last_distribution' attribute from the 'wasm' event
            let is_last_distribution =
                get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_distribution"]);
            assert_eq!(is_last_distribution[0].value, "false".to_string());
        }

        // Distribute one more time to finish, even if we extra deposited with one more user we expect the distribution to finish
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                claimer,
            )
            .unwrap();

        // Extract the 'is_last_distribution' attribute from the 'wasm' event
        let is_last_distribution =
            get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_distribution"]);
        assert_eq!(is_last_distribution[0].value, "true".to_string());

        // Loop users and claim for each one of them
        let mut rewards_received = vec![];
        for account in &accounts {
            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                    &[],
                    account,
                )
                .unwrap();

            let coin_received =
                get_event_attributes_by_ty_and_key(&result, "coin_received", vec!["amount"]);
            let coin_received_u128 = get_event_value_amount_numeric(&coin_received[1].value); // taking index 1 in this case as there are more then 1 coin_received tys
            rewards_received.push(coin_received_u128);
        }

        // Assert that 'tokens_out' values for events are empty
        assert_ne!(tokens_out_spread_rewards[0].value, "".to_string());
        let tokens_out_spread_rewards_u128: u128 =
            get_event_value_amount_numeric(&tokens_out_spread_rewards[0].value);
        let rewards_less_performance_fee = (tokens_out_spread_rewards_u128 as f64 * 0.8) as u64;
        let expected_rewards_per_user = rewards_less_performance_fee / (ACCOUNTS_NUM + 1); // hardcoding +1 due to test logic, we will deposit once more with a single account doubling its shares amount
        let expected_rewards_per_user_double = expected_rewards_per_user.mul(2);

        let double_rewards_value: Vec<u128> = rewards_received
            .iter()
            .filter(|&&x| x > expected_rewards_per_user as u128)
            .cloned()
            .collect();
        let single_rewards_count = rewards_received
            .iter()
            .filter(|&&x| x == expected_rewards_per_user as u128)
            .count();

        assert_approx_eq!(
            double_rewards_value[0],
            expected_rewards_per_user_double as u128,
            "0.005"
        );
        assert_eq!(
            single_rewards_count, 9,
            "There should be exactly one account with double rewards."
        );
    }

    // fn get_cases() -> u32 {
    //     std::env::var("PROPTEST_CASES")
    //         .unwrap_or("100".to_string())
    //         .parse()
    //         .unwrap()
    // }

    // proptest! {
    //     #![proptest_config(ProptestConfig::with_cases(get_cases()))]
    //     #[test]
    //     #[ignore]
    //     fn test_rewards_single_distribute_claim_max_users(users in 10..u64::MAX) {
    //     let (app, contract_address, cl_pool_id, _admin) = default_init();

    //     // Initialize accounts
    //     let accounts = app
    //         .init_accounts(
    //             &[
    //                 Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
    //                 Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
    //             ],
    //             users,
    //         )
    //         .unwrap();

    //     // Depositing with users
    //     let wasm = Wasm::new(&app);
    //     for account in &accounts {
    //         let _ = wasm
    //             .execute(
    //                 contract_address.as_str(),
    //                 &ExecuteMsg::ExactDeposit { recipient: None },
    //                 &[
    //                     Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
    //                     Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
    //                 ],
    //                 account,
    //             )
    //             .unwrap();
    //     }

    //     // Declare swapper and claimer accounts
    //     let swapper = &accounts[0];
    //     let claimer = &accounts[1];

    //     // Swaps to generate spread rewards on previously created user positions
    //     for _ in 0..SWAPS_NUM {
    //         PoolManager::new(&app)
    //             .swap_exact_amount_in(
    //                 MsgSwapExactAmountIn {
    //                     sender: swapper.address(),
    //                     routes: vec![SwapAmountInRoute {
    //                         pool_id: cl_pool_id,
    //                         token_out_denom: DENOM_BASE.to_string(),
    //                     }],
    //                     token_in: Some(OsmoCoin {
    //                         denom: DENOM_QUOTE.to_string(),
    //                         amount: SWAPS_AMOUNT.to_string(),
    //                     }),
    //                     token_out_min_amount: "1".to_string(),
    //                 },
    //                 &swapper,
    //             )
    //             .unwrap();
    //     }

    //     // Collect and Distribute Rewards
    //     let result = wasm
    //         .execute(
    //             contract_address.as_str(),
    //             &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
    //             &[],
    //             claimer,
    //         )
    //         .unwrap();
    //     println!("collect result {:?}", result);
    //     // Extract 'tokens_out' attribute value for 'total_collect_spread_rewards'
    //     let tokens_out_spread_rewards = get_event_attributes_by_ty_and_key(
    //         &result,
    //         "total_collect_spread_rewards",
    //         vec!["tokens_out"],
    //     );

    //     // Assert that 'tokens_out' values for events are empty
    //     assert_ne!(tokens_out_spread_rewards[0].value, "".to_string());
    //     let tokens_out_spread_rewards_u128: u128 =
    //         get_event_value_amount_numeric(&tokens_out_spread_rewards[0].value);
    //     println!(
    //         "tokens_out_spread_rewards_u128 {}",
    //         tokens_out_spread_rewards_u128
    //     );
    //     let expected_rewards_per_user = tokens_out_spread_rewards_u128 as u64 / users;
    //     println!("expected_rewards_per_user {}", expected_rewards_per_user);

    //     for _ in 0..(users - 1) {
    //         // Adjust the number of distribute actions as needed
    //         let result = wasm
    //             .execute(
    //                 contract_address.as_str(),
    //                 &ExecuteMsg::VaultExtension(
    //                     crate::msg::ExtensionExecuteMsg::DistributeRewards {
    //                         amount_of_users: Uint128::new(1), // hardcoding 1
    //                     },
    //                 ),
    //                 &[],
    //                 claimer,
    //             )
    //             .unwrap();
    //         println!("distribute result {:?}", result);

    //         // Extract the 'is_last_distribution' attribute from the 'wasm' event
    //         let is_last_distribution =
    //             get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_distribution"]);
    //         assert_eq!(is_last_distribution[0].value, "false".to_string());
    //     }

    //     // Initialize accounts
    //     let extra_accounts = app
    //         .init_accounts(
    //             &[
    //                 Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
    //                 Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
    //             ],
    //             ACCOUNTS_NUM,
    //         )
    //         .unwrap();
    //     for account in &extra_accounts {
    //         let _ = wasm
    //             .execute(
    //                 contract_address.as_str(),
    //                 &ExecuteMsg::ExactDeposit { recipient: None },
    //                 &[
    //                     Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
    //                     Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
    //                 ],
    //                 account,
    //             )
    //             .unwrap();
    //     }

    //     // Distribute one more time to finish, even if we extra deposited with one more user we expect the distribution to finish
    //     let result = wasm
    //         .execute(
    //             contract_address.as_str(),
    //             &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::DistributeRewards {
    //                 amount_of_users: Uint128::new(1),
    //             }),
    //             &[],
    //             claimer,
    //         )
    //         .unwrap();
    //     println!("distribute result {:?}", result);

    //     // Extract the 'is_last_distribution' attribute from the 'wasm' event
    //     let is_last_distribution =
    //         get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_distribution"]);
    //     assert_eq!(is_last_distribution[0].value, "true".to_string());

    //     // TODO: Assert USER_REWARDS increased accordingly to distribution amounts

    //     // Loop users and claim for each one of them
    //     for account in &accounts {
    //         let result = wasm
    //             .execute(
    //                 contract_address.as_str(),
    //                 &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ClaimRewards {}),
    //                 &[],
    //                 account,
    //             )
    //             .unwrap();

    //         println!("claim result {:?}", result);
    //         // TODO: Assert Attribute { key: "amount", value: "2499uosmo" }
    //     }
    // }
    // }
}
