#[cfg(test)]
mod tests {
    use crate::msg::ExecuteMsg;
    use crate::test_tube::initialize::initialize::default_init;
    use cosmwasm_std::{Coin, Uint128};
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SwapAmountInRoute,
    };
    use osmosis_test_tube::{Account, Module, PoolManager, Wasm};

    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";
    const ACCOUNTS_NUM: usize = 10;
    const ACCOUNTS_INIT_BALANCE: u128 = 1_000_000_000_000;
    const DEPOSIT_AMOUNT: u128 = 5_000_000;
    const SWAPS_NUM: usize = 50;

    #[test]
    #[ignore]
    fn test_rewards_single_distribute_claim() {
        let (app, contract_address, cl_pool_id, _admin) = default_init();

        // Initialize accounts
        let mut accounts = Vec::new();
        for _ in 0..ACCOUNTS_NUM {
            let account = app
                .init_account(&[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ])
                .unwrap();
            accounts.push(account);
        }

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
                            amount: "1000000".to_string(),
                        }),
                        token_out_min_amount: "1".to_string(),
                    },
                    &swapper,
                )
                .unwrap();
        }

        // Collect and Distribute Rewards
        let _res = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                claimer,
            )
            .unwrap();

        for _ in 0..(ACCOUNTS_NUM - 1) {
            // Adjust the number of distribute actions as needed
            let result = wasm.execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::DistributeRewards {
                    amount_of_users: Uint128::new(1), // hardcoding 1
                }),
                &[],
                claimer,
            );
            // TODO: Assert is_last_distribution is false as we are iterating ACCOUNTS_NUM - 1 and we expect the process to do not finish in this loop
        }

        // TODO Distribute one more time
        let result = wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::DistributeRewards {
                amount_of_users: Uint128::new(1),
            }),
            &[],
            claimer,
        );

        // TODO Assert is_last_distribution is true and state is cleared such as IS_DISTRIBUTING and USER_REWARDS

        // TODO: Assert users balances increased accordingly to distribution amounts
    }

    #[test]
    #[ignore]
    fn test_rewards_single_distribute_claim_no_rewards_works() {
        let (app, contract_address, _cl_pool_id, _admin) = default_init();

        // Initialize accounts
            let accounts = app
                .init_accounts(&[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ], ACCOUNTS_NUM.try_into().unwrap())
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

        // Collect and Distribute Rewards
        let _res = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CollectRewards {}),
                &[],
                claimer,
            )
            .unwrap();

            // since there are no rewards, the first call to distribute rewards will flip IS_DISTRIBUTING to false
            let result = wasm.execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::DistributeRewards {
                    amount_of_users: Uint128::new(1), // hardcoding 1
                }),
                &[],
                claimer,
            ).unwrap();
            // TODO: Assert is_last_distribution is true
        

        // TODO Distribute one more time
        let result = wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::DistributeRewards {
                amount_of_users: Uint128::new(1),
            }),
            &[],
            claimer,
        ).unwrap_err();

        // TODO Assert is_last_distribution is true and state is cleared such as IS_DISTRIBUTING and USER_REWARDS

        // TODO: Assert users balances has not increased, accordingly to 0 distribution
    }
}
