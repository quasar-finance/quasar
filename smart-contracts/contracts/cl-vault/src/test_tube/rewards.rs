#[cfg(test)]
mod tests {
    use crate::msg::ExecuteMsg;
    use crate::test_tube::helpers::{
        get_amount_from_denom, get_balance_amount, get_event_attributes_by_ty_and_key,
    };
    use crate::test_tube::initialize::initialize::{
        default_init, ACCOUNTS_INIT_BALANCE, ACCOUNTS_NUM, DENOM_BASE, DENOM_QUOTE, DEPOSIT_AMOUNT,
        PERFORMANCE_FEE,
    };
    use cosmwasm_std::Coin;
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SwapAmountInRoute,
    };
    use osmosis_test_tube::{Account, Module, PoolManager, Wasm};

    const SWAPS_NUM: usize = 10;
    const SWAPS_AMOUNT: &str = "100000000000000000";

    #[test]
    #[ignore]
    fn test_collect_rewards_with_rewards_works() {
        let (app, contract_address, cl_pool_id, admin) = default_init();

        // Initialize accounts
        let utility_account = app
            .init_account(&[
                Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
            ])
            .unwrap();

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

        // Swaps to generate spread rewards on previously created user positions
        for _ in 0..SWAPS_NUM {
            // TODO: This is not generating any spread_rewards
            PoolManager::new(&app)
                .swap_exact_amount_in(
                    MsgSwapExactAmountIn {
                        sender: admin.address(),
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
                    &admin,
                )
                .unwrap();
        }

        // Before balances
        let balance_admin_before =
            get_balance_amount(&app, admin.address().to_string(), DENOM_QUOTE.to_string());
        let balance_contract_before =
            get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());

        // Collect Rewards
        let result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                &utility_account,
            )
            .unwrap();

        // Extract 'tokens_out' attribute value for 'total_collect_spread_rewards'
        let tokens_out = get_event_attributes_by_ty_and_key(
            &result,
            "total_collect_spread_rewards",
            vec!["tokens_out"],
        );
        let tokens_out_u128 = get_amount_from_denom(&tokens_out[0].value);

        // Asserts distribution of claimed spread rewards
        assert_eq!(tokens_out_u128, 999u128); // Total

        // After balances
        let balance_admin_after =
            get_balance_amount(&app, admin.address().to_string(), DENOM_QUOTE.to_string());
        let balance_contract_after =
            get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());

        // Calculating the fee increment for the admin
        assert_eq!(
            balance_admin_after - balance_admin_before,
            (tokens_out_u128 * PERFORMANCE_FEE as u128) / 100,
            "Admin fee calculation mismatch"
        );

        // Calculating the increment for the contract (the rest of the amount)
        assert_eq!(
            balance_contract_after - balance_contract_before,
            ((tokens_out_u128 * (100 - PERFORMANCE_FEE) as u128) / 100) + 1, // This +1 is needed due to some loss in precision
            "Contract fee calculation mismatch"
        );
    }

    #[test]
    #[ignore]
    fn test_collect_rewards_no_rewards_works() {
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
    }
}
