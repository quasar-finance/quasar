#[cfg(test)]
mod tests {
    use crate::msg::ExecuteMsg;
    use crate::test_tube::helpers::{
        get_event_attributes_by_ty_and_key, get_event_value_amount_numeric,
    };
    use crate::test_tube::initialize::initialize::default_init;
    use cosmwasm_std::{assert_approx_eq, Coin, Uint128};
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SwapAmountInRoute,
    };
    use osmosis_test_tube::RunnerError::ExecuteError;
    use osmosis_test_tube::{Account, Module, PoolManager, Wasm};

    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";
    const ACCOUNTS_NUM: u64 = 10;
    const ACCOUNTS_INIT_BALANCE: u128 = 1_000_000_000_000_000;
    const DEPOSIT_AMOUNT: u128 = 5_000_000;
    const SWAPS_NUM: usize = 10;
    const SWAPS_AMOUNT: &str = "1000000000";
    const DISTRIBUTION_CYCLES: usize = 25;

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

        let mut expected_rewards_per_user: u64 = 0;
        // Collect init
        for i in 0..(ACCOUNTS_NUM - 1) {
            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {
                        amount_of_users: Uint128::one(),
                    }),
                    &[],
                    claimer,
                )
                .unwrap();
            if i == 0 {
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
                expected_rewards_per_user =
                    (tokens_out_spread_rewards_u128 as f64 * 0.8) as u64 / ACCOUNTS_NUM;
            } else {
                // Extract the 'is_last_collection' attribute from the 'wasm' event
                let is_last_collection =
                    get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_collection"]);
                assert_eq!(is_last_collection[0].value, "false".to_string());
            }
        }

        // Collect one more time to finish, even if we extra deposited with one more user we expect the distribution to finish
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {
                    amount_of_users: Uint128::one(),
                }),
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
                        crate::msg::ExtensionExecuteMsg::DistributeRewards {
                            amount_of_users: Uint128::one(), // hardcoding 1
                        },
                    ),
                    &[],
                    claimer,
                )
                .unwrap();

            // Extract the 'is_last_distribution' attribute from the 'wasm' event
            let is_last_distribution =
                get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_distribution"]);
            assert_eq!(is_last_distribution[0].value, "false".to_string());
        }

        // Deposit with old accounts to try exploiting amount of rewards in the current calculation
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
        // Initialize extra accounts
        let extra_accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                ACCOUNTS_NUM,
            )
            .unwrap();
        // Deposit with those new accounts in order to try exploit eligibility
        for account in &extra_accounts {
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

        // Distribute one more time to finish, even if we extra deposited with one more user we expect the distribution to finish
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::DistributeRewards {
                    amount_of_users: Uint128::one(),
                }),
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
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ClaimRewards {}),
                    &[],
                    account,
                )
                .unwrap();

            let coin_received =
                get_event_attributes_by_ty_and_key(&result, "coin_received", vec!["amount"]);
            let coin_received_u128 = get_event_value_amount_numeric(&coin_received[1].value); // taking index 1 in this case as there are more then 1 coin_received tys
            assert_eq!(coin_received_u128, expected_rewards_per_user as u128);
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

            let mut expected_rewards_per_user: u64 = 0;
            // Collect init
            for i in 0..(ACCOUNTS_NUM - 1) {
                let result = wasm
                    .execute(
                        contract_address.as_str(),
                        &ExecuteMsg::VaultExtension(
                            crate::msg::ExtensionExecuteMsg::CollectRewards {
                                amount_of_users: Uint128::new(1),
                            },
                        ),
                        &[],
                        claimer,
                    )
                    .unwrap();
                if i == 0 {
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
                    expected_rewards_per_user =
                        (tokens_out_spread_rewards_u128 as f64 * 0.8) as u64 / ACCOUNTS_NUM;
                } else {
                    // Extract the 'is_last_collection' attribute from the 'wasm' event
                    let is_last_collection = get_event_attributes_by_ty_and_key(
                        &result,
                        "wasm",
                        vec!["is_last_collection"],
                    );
                    assert_eq!(is_last_collection[0].value, "false".to_string());
                }
            }

            // Collect one more time to finish, even if we extra deposited with one more user we expect the distribution to finish
            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {
                        amount_of_users: Uint128::one(),
                    }),
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
                            crate::msg::ExtensionExecuteMsg::DistributeRewards {
                                amount_of_users: Uint128::one(), // hardcoding 1
                            },
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
                    &ExecuteMsg::VaultExtension(
                        crate::msg::ExtensionExecuteMsg::DistributeRewards {
                            amount_of_users: Uint128::one(),
                        },
                    ),
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
                            crate::msg::ExtensionExecuteMsg::ClaimRewards {},
                        ),
                        &[],
                        account,
                    )
                    .unwrap();

                let coin_received =
                    get_event_attributes_by_ty_and_key(&result, "coin_received", vec!["amount"]);
                let coin_received_u128 = get_event_value_amount_numeric(&coin_received[1].value); // taking index 1 in this case as there are more then 1 coin_received tys
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

        // TODO: adapt this with a loop

        // Collect and Distribute Rewards (there should be anything)
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {
                    amount_of_users: Uint128::one(),
                }),
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

        // Try to collect one more time, this should be failing
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {
                    amount_of_users: Uint128::one(),
                }),
                &[],
                claimer,
            )
            .unwrap_err();
        // Assert that the response is an error
        assert!(
            matches!(result, ExecuteError { msg } if msg.contains("failed to execute message; message index: 0: Vault is already distributing"))
        );

        // Distribute just one time, as there are no rewards we expect this to clear the state even if 1 user < 10 users
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::DistributeRewards {
                    amount_of_users: Uint128::one(),
                }),
                &[],
                claimer,
            )
            .unwrap();

        // Extract the 'is_last_distribution' attribute from the 'wasm' event
        let is_last_distribution =
            get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_distribution"]);
        assert_eq!(is_last_distribution[0].value, "true".to_string());

        // Distribute one more time, we expect to receive an Error here as IS_DISTRIBUTING is false
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::DistributeRewards {
                    amount_of_users: Uint128::one(),
                }),
                &[],
                claimer,
            )
            .unwrap_err();
        // Assert that the response is an error
        assert!(
            matches!(result, ExecuteError { msg } if msg.contains("failed to execute message; message index: 0: Vault is not distributing rewards, claiming is needed first: execute wasm contract failed"))
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
