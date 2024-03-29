// rewards

#[cfg(test)]
mod tests {
    use apollo_cw_asset::AssetInfoBase;
    use cosmwasm_std::Coin;
    use cosmwasm_std::{Decimal, Uint128};
    use cw_dex::osmosis::OsmosisPool;
    use cw_dex_router::operations::{SwapOperationBase, SwapOperationsListUnchecked};
    use cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension;
    use osmosis_std::types::cosmos::bank::v1beta1::{MsgSend, QueryBalanceRequest};
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SwapAmountInRoute,
    };
    use osmosis_test_tube::RunnerError::ExecuteError;
    use osmosis_test_tube::{Account, Bank, Module, PoolManager, Wasm};
    use std::str::FromStr;

    use crate::msg::ClQueryMsg::SharePrice;
    use crate::msg::UserBalanceQueryMsg::UserSharesBalance;
    use crate::msg::{AutoCompoundAsset, ExecuteMsg, ExtensionQueryMsg, ModifyRangeMsg};
    use crate::query::{SharePriceResponse, UserSharesBalanceResponse};
    use crate::test_tube::helpers::{get_amount_from_denom, get_event_attributes_by_ty_and_key};
    use crate::test_tube::initialize::initialize::{
        default_init, dex_cl_init_cl_pools, dex_cl_init_lp_pools,
    };

    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";
    const DENOM_REWARD: &str = "ustride";
    const ACCOUNTS_NUM: u64 = 10;
    const ACCOUNTS_INIT_BALANCE: u128 = 1_000_000_000_000_000;
    const DEPOSIT_AMOUNT: u128 = 5_000_000;
    const SWAPS_NUM: usize = 10;
    const SWAPS_AMOUNT: &str = "1000000000";

    #[test]
    #[ignore]
    fn test_auto_compound_rewards_lp_pools() {
        let (app, contract_address, _dex_router_addr, cl_pool_id, lp_pools_ids, admin) =
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

        let _result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                &admin,
            )
            .unwrap();

        let _send = bm.send(
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

        let shares_price: SharePriceResponse = wasm
            .query(
                contract_address.as_str(),
                &VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(SharePrice {
                    shares: Uint128::new(10),
                })),
            )
            .unwrap();
        println!("{:?}", shares_price.balances);
        assert_eq!(Uint128::new(2), shares_price.balances[0].amount);
        assert_eq!(Uint128::new(3), shares_price.balances[1].amount);

        let path1 = vec![
            SwapOperationBase::new(
                cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pools_ids[1])),
                AssetInfoBase::Native(DENOM_REWARD.to_string()),
                AssetInfoBase::Native(DENOM_QUOTE.to_string()),
            ),
            SwapOperationBase::new(
                cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pools_ids[0])),
                AssetInfoBase::Native(DENOM_QUOTE.to_string()),
                AssetInfoBase::Native(DENOM_BASE.to_string()),
            ),
        ];
        let path2 = vec![SwapOperationBase::new(
            cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pools_ids[1])),
            AssetInfoBase::Native(DENOM_REWARD.to_string()),
            AssetInfoBase::Native(DENOM_QUOTE.to_string()),
        )];

        let _auto_compound = wasm
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

        let _update_range = wasm
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

        let shares_price: SharePriceResponse = wasm
            .query(
                contract_address.as_str(),
                &VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(SharePrice {
                    shares: Uint128::new(10),
                })),
            )
            .unwrap();
        assert_eq!(Uint128::new(2763), shares_price.balances[0].amount);
        assert_eq!(Uint128::new(4368), shares_price.balances[1].amount);

        for account in &accounts {
            // Get balances before for current account
            let balances_before_withdraw_quote_denom = get_amount_from_denom(
                &bm.query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_QUOTE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount,
            );
            let balances_before_withdraw_base_denom = get_amount_from_denom(
                &bm.query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_BASE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount,
            );

            // Get shares balance for current account
            let shares_to_redeem: UserSharesBalanceResponse = wasm
                .query(
                    contract_address.as_str(),
                    &VaultExtension(ExtensionQueryMsg::Balances(UserSharesBalance {
                        user: account.address(),
                    })),
                )
                .unwrap();

            // If the current account have some share to redeem
            if !shares_to_redeem.balance.is_zero() {
                // Redeem all shares_to_redeem.balance
                wasm.execute(
                    contract_address.as_str(),
                    &ExecuteMsg::Redeem {
                        recipient: None,
                        amount: shares_to_redeem.balance,
                    },
                    &[],
                    account,
                )
                .unwrap();

                let balances_after_withdraw_quote_denom = get_amount_from_denom(
                    &bm.query_balance(&QueryBalanceRequest {
                        address: account.address(),
                        denom: DENOM_QUOTE.to_string(),
                    })
                    .unwrap()
                    .balance
                    .unwrap()
                    .amount,
                );
                let balances_after_withdraw_base_denom = get_amount_from_denom(
                    &bm.query_balance(&QueryBalanceRequest {
                        address: account.address(),
                        denom: DENOM_BASE.to_string(),
                    })
                    .unwrap()
                    .balance
                    .unwrap()
                    .amount,
                );

                assert_eq!(
                    true,
                    balances_after_withdraw_quote_denom
                        .checked_sub(balances_before_withdraw_quote_denom)
                        .unwrap()
                        > DEPOSIT_AMOUNT
                );
                assert_eq!(
                    true,
                    balances_after_withdraw_base_denom
                        .checked_sub(balances_before_withdraw_base_denom)
                        .unwrap()
                        > DEPOSIT_AMOUNT
                );
            }
        }

        // as there are no more deposists the share price should not change
        let shares_price: SharePriceResponse = wasm
            .query(
                contract_address.as_str(),
                &VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(SharePrice {
                    shares: Uint128::new(10),
                })),
            )
            .unwrap();
        assert_eq!(Uint128::new(2763), shares_price.balances[0].amount);
        assert_eq!(Uint128::new(4368), shares_price.balances[1].amount);
    }

    #[test]
    #[ignore]
    fn test_auto_compound_rewards_cl_pools() {
        let (
            app,
            contract_address,
            _dex_router_addr,
            cl_pool_id,
            lp_pool1,
            lp_pool2,
            _lp_pool3,
            admin,
        ) = dex_cl_init_cl_pools();
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

        let _result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                &admin,
            )
            .unwrap();

        let _send = bm.send(
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

        let _auto_compound = wasm
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

        let _update_range = wasm
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
            // Get balances before for current account
            let balances_before_withdraw_quote_denom = get_amount_from_denom(
                &bm.query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_QUOTE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount,
            );
            let balances_before_withdraw_base_denom = get_amount_from_denom(
                &bm.query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_BASE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount,
            );
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

            let balances_after_withdraw_quote_denom = get_amount_from_denom(
                &bm.query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_QUOTE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount,
            );
            let balances_after_withdraw_base_denom = get_amount_from_denom(
                &bm.query_balance(&QueryBalanceRequest {
                    address: account.address(),
                    denom: DENOM_BASE.to_string(),
                })
                .unwrap()
                .balance
                .unwrap()
                .amount,
            );

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
    fn test_migration_step_with_rewards_works() {
        let (app, contract_address, cl_pool_id, admin) = default_init();

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
        let ops_accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, "rewards"),
                ],
                2,
            )
            .unwrap();
        let swapper = &ops_accounts[0];

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

        // TODO: With autocompounding we are not collecting rewards anymore. We should populate the USER_REWARDS somehow here.
        // add this code to the exact deposit function for this todo
        // MIGRATION_STATUS.save(deps.storage, &MigrationStatus::Open)?;
        // USER_REWARDS.save(deps.storage, info.clone().sender, &CoinList::from_coins(vec![
        //     Coin{
        //         denom: "rewards".to_string(),
        //         amount: Uint128::new(10000000000000),
        //     }
        // ]))?;

        let bm = Bank::new(&app);

        for account in &ops_accounts {
            let _send = bm.send(
                MsgSend {
                    from_address: account.address(),
                    to_address: contract_address.to_string(),
                    amount: vec![OsmoCoin {
                        denom: "rewards".to_string(),
                        amount: ACCOUNTS_INIT_BALANCE.to_string(),
                    }],
                },
                &account,
            );
        }

        // todo : un-comment this whenever testing for migration in auto compounding
        // let balance = bm.query_balance(&QueryBalanceRequest{
        //     address: contract_address.to_string(),
        //     denom: "rewards".to_string(),
        // }).unwrap();
        // assert_eq!("2000000000000000".to_string(), balance.balance.unwrap().amount);
        //
        // for account in &accounts {
        //     let balance = bm.query_balance(&QueryBalanceRequest{
        //         address: account.address(),
        //         denom: "rewards".to_string(),
        //     }).unwrap();
        //     assert_eq!("0".to_string(), balance.balance.unwrap().amount);
        // }

        // Collect init
        for _i in 0..(ACCOUNTS_NUM - 1) {
            let result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::MigrationStep {
                        amount_of_users: Uint128::one(), // this is ignored the first time but lets pass it anyway for now
                    }),
                    &[],
                    &admin,
                )
                .unwrap();
            // Extract the 'is_last_collection' attribute from the 'wasm' event
            let is_last_collection = get_event_attributes_by_ty_and_key(
                &result,
                "wasm",
                vec!["is_last_execution", "migration_status"],
            );
            assert_eq!(is_last_collection[0].value, "Open".to_string());
            assert_eq!(is_last_collection[1].value, "false".to_string());
        }

        // Try to collect one more time, this should be closing the process and set to Ready as there are not rewards
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::MigrationStep {
                    amount_of_users: Uint128::one(),
                }),
                &[],
                &admin,
            )
            .unwrap();

        let rewards_status =
            get_event_attributes_by_ty_and_key(&result, "wasm", vec!["migration_status"]);
        assert_eq!(rewards_status[0].value, "Closed".to_string());
        // Extract the 'is_last_collection' attribute from the 'wasm' event
        let is_last_collection =
            get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_execution"]);
        assert_eq!(is_last_collection[0].value, "true".to_string());

        // todo : un-comment whenever testing for auto compound migration testing
        // let balance = bm.query_balance(&QueryBalanceRequest{
        //     address: contract_address.to_string(),
        //     denom: "rewards".to_string(),
        // }).unwrap();
        // assert_eq!("1900000000000000".to_string(), balance.balance.unwrap().amount);
        //
        // for account in &accounts {
        //     let balance = bm.query_balance(&QueryBalanceRequest{
        //         address: account.address(),
        //         denom: "rewards".to_string(),
        //     }).unwrap();
        //     assert_eq!("10000000000000".to_string(), balance.balance.unwrap().amount);
        // }

        // Distribute just one time, as there are no rewards we expect this to clear the state even if 1 user < 10 users
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::MigrationStep {
                    amount_of_users: Uint128::one(),
                }),
                &[],
                &admin,
            )
            .unwrap_err();

        // Assert that the response is an error
        assert!(
            matches!(result, ExecuteError { msg } if msg.contains("failed to execute message; message index: 0: Migration status is closed: execute wasm contract failed"))
        );
    }

    #[test]
    #[ignore]
    fn test_migration_step_no_rewards_works() {
        let (app, contract_address, _cl_pool_id, admin) = default_init();

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

        // Try to collect one more time, this should be closing the process and set to Ready as there are not rewards
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::MigrationStep {
                    amount_of_users: Uint128::one(),
                }),
                &[],
                &admin,
            )
            .unwrap();

        let rewards_status =
            get_event_attributes_by_ty_and_key(&result, "wasm", vec!["migration_status"]);
        assert_eq!(rewards_status[0].value, "Closed".to_string());
        // Extract the 'is_last_collection' attribute from the 'wasm' event
        let is_last_collection =
            get_event_attributes_by_ty_and_key(&result, "wasm", vec!["is_last_execution"]);
        assert_eq!(is_last_collection[0].value, "true".to_string());

        // Distribute just one time, as there are no rewards we expect this to clear the state even if 1 user < 10 users
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::MigrationStep {
                    amount_of_users: Uint128::one(),
                }),
                &[],
                &admin,
            )
            .unwrap_err();

        println!("{:?}", result);

        // Assert that the response is an error
        assert!(
            matches!(result, ExecuteError { msg } if msg.contains("failed to execute message; message index: 0: Migration status is closed: execute wasm contract failed"))
        );
    }
}
