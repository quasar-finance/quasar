#[cfg(test)]
mod tests {
    use cosmwasm_std::{assert_approx_eq, Coin, Uint128};

    use osmosis_test_tube::{Account, Module, Wasm};

    use crate::{
        msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg},
        query::{AssetsBalanceResponse, TotalAssetsResponse, UserSharesBalanceResponse},
        test_tube::initialize::initialize::default_init,
    };

    const INITIAL_BALANCE_AMOUNT: u128 = 340282366920938463463374607431768211455u128;
    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";

    #[test]
    #[ignore]
    fn single_deposit_withdraw_works() {
        let (app, contract_address, _cl_pool_id, _admin) = default_init();
        let wasm = Wasm::new(&app);

        // Create Alice account
        let alice = app
            .init_account(&[
                Coin::new(INITIAL_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(INITIAL_BALANCE_AMOUNT, DENOM_QUOTE),
            ])
            .unwrap();

        let vault_assets_before: TotalAssetsResponse = wasm
            .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
            .unwrap();

        // TODO: Check this -> Certain deposit amounts do not work here due to an off by one error in Osmosis cl code. The value here is chosen to specifically work
        wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::ExactDeposit { recipient: None },
            &[
                Coin::new(1_000_000_000_000_000, DENOM_BASE),
                Coin::new(1_000_000_000_000_000, DENOM_QUOTE),
            ],
            &alice,
        )
        .unwrap();

        // Get shares for Alice from vault contract and assert
        let shares: UserSharesBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserSharesBalance {
                        user: alice.address(),
                    },
                )),
            )
            .unwrap();
        assert!(!shares.balance.is_zero());

        // Get user_assets for Alice from vault contract and assert
        let user_assets: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserAssetsBalance {
                        user: alice.address(),
                    },
                )),
            )
            .unwrap();

        // Assert Alice has been refunded, so we only expect around 500 to deposit here
        assert_approx_eq!(
            user_assets.balances[0].amount,
            Uint128::from(600_000_000_000_000u128), // TODO: remove hardcoded
            "0.1"
        );

        // Assert Alice as
        assert_approx_eq!(
            user_assets.balances[1].amount,
            Uint128::from(1_000_000_000_000_000u128), // TODO: remove hardcoded
            "0.001"
        );

        // Get vault assets and assert
        let vault_assets: TotalAssetsResponse = wasm
            .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
            .unwrap();
        assert_approx_eq!(
            vault_assets.token0.amount,
            vault_assets_before
                .token0
                .amount
                .checked_add(Uint128::from(600_000_000_000_000u128)) // TODO: remove hardcoded
                .unwrap(),
            "0.1"
        );

        // Assert vault assets taking in account the refunded amount to Alice, so we only expect around 500 to deposit here
        assert_approx_eq!(
            vault_assets.token1.amount,
            vault_assets_before
                .token1
                .amount
                .checked_add(Uint128::from(1_000_000_000_000_000u128)) // TODO: remove hardcoded
                .unwrap(),
            "0.001"
        );

        let _withdraw = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: shares.balance,
                },
                &[],
                &alice,
            )
            .unwrap();

        // TODO: verify the correct execution
    }

    #[test]
    #[ignore]
    fn multiple_deposit_withdraw_works() {
        let (app, contract_address, _cl_pool_id, _admin) = default_init();
        let wasm = Wasm::new(&app);

        // Create Alice account
        let alice = app
            .init_account(&[
                Coin::new(INITIAL_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(INITIAL_BALANCE_AMOUNT, DENOM_QUOTE),
            ])
            .unwrap();

        // Get vaults assets before doing anything for future assertions
        let vault_assets_before: TotalAssetsResponse = wasm
            .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
            .unwrap();

        // Loop 3 times to do multiple deposits as Alice
        for _ in 0..3 {
            wasm.execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(1_000_000_000_000_000_000, DENOM_BASE),
                    Coin::new(1_000_000_000_000_000_000, DENOM_QUOTE),
                ],
                &alice,
            )
            .unwrap();
        }

        // Get Alice shares from vault contract
        let shares: UserSharesBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserSharesBalance {
                        user: alice.address(),
                    },
                )),
            )
            .unwrap();
        assert!(!shares.balance.is_zero());

        // Get Alice assets from vault contract
        let user_assets: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserAssetsBalance {
                        user: alice.address(),
                    },
                )),
            )
            .unwrap();

        // deposit alice 3x 1_000_000_000_000_000_000. we should be close to 3*10^18 for the eth asset
        assert_approx_eq!(
            user_assets.balances[0].amount,
            Uint128::from(1_879_559_586_415_174_597u128), // TODO: remove hardcoded value
            "0.001"
        );
        // deposit alice 3x 1_000_000_000. we should be close to 3*10^9 for the osmo asset
        assert_approx_eq!(
            user_assets.balances[1].amount,
            Uint128::from(3_000_000_000_000_000_000u128),
            "0.001"
        );

        let user_assets_again: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::ConvertToAssets {
                    amount: shares.balance,
                },
            )
            .unwrap();
        assert_approx_eq!(
            user_assets_again.balances[0].amount,
            Uint128::from(1_879_559_586_415_174_597u128),
            "0.001"
        );
        assert_approx_eq!(
            user_assets_again.balances[1].amount,
            Uint128::from(3_000_000_000_000_000_000u128),
            "0.001"
        );

        let vault_assets: TotalAssetsResponse = wasm
            .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
            .unwrap();

        assert_approx_eq!(
            vault_assets.token0.amount,
            vault_assets_before
                .token0
                .amount
                .checked_add(Uint128::from(1_879_559_586_415_174_597u128))
                .unwrap(),
            "0.001"
        );
        // again we get refunded so we only expect around 500 to deposit here
        assert_approx_eq!(
            vault_assets.token1.amount,
            vault_assets_before
                .token1
                .amount
                .checked_add(Uint128::from(3_000_000_000_000_000_000u128))
                .unwrap(),
            "0.01"
        );

        let _withdraw = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: shares.balance,
                },
                &[],
                &alice,
            )
            .unwrap();
        // verify the correct execution
    }

    #[test]
    #[ignore]
    fn multiple_deposit_withdraw_unused_funds_works() {
        let (app, contract_address, _cl_pool_id, _admin) = default_init();
        //let bank = Bank::new(&app);

        let wasm = Wasm::new(&app);

        // Create 3 accounts
        let users = [
            app.init_account(&[
                Coin::new(
                    100_000_000_000_000_000_000_000_000_000_000_000_000,
                    DENOM_BASE,
                ),
                Coin::new(
                    100_000_000_000_000_000_000_000_000_000_000_000_000,
                    DENOM_QUOTE,
                ),
            ])
            .unwrap(),
            app.init_account(&[
                Coin::new(
                    100_000_000_000_000_000_000_000_000_000_000_000_000,
                    DENOM_BASE,
                ),
                Coin::new(
                    100_000_000_000_000_000_000_000_000_000_000_000_000,
                    DENOM_QUOTE,
                ),
            ])
            .unwrap(),
            app.init_account(&[
                Coin::new(
                    100_000_000_000_000_000_000_000_000_000_000_000_000,
                    DENOM_BASE,
                ),
                Coin::new(
                    100_000_000_000_000_000_000_000_000_000_000_000_000,
                    DENOM_QUOTE,
                ),
            ])
            .unwrap(),
        ];

        // this is the max deposit amount before overflow -> 100_000_000 ETH (100_000_000_000_000_000_000_000_000 Wei)
        let deposit_amount: u128 = 100_000_000_000_000_000_000_000;

        // you can scale this up to 1000 and still not failing, which would be like: 3 users x 100_000_000 ETH x 1000 = 300_000_000_000 (300 B) total deposited ETHs in the vault
        for _ in 0..10 {
            // depositing
            for user in &users {
                wasm.execute(
                    contract_address.as_str(),
                    &ExecuteMsg::ExactDeposit { recipient: None },
                    &[
                        Coin::new(deposit_amount, DENOM_BASE),
                        Coin::new(deposit_amount, DENOM_QUOTE),
                    ], // 1eth = 6k osmo
                    user,
                )
                .unwrap();
            }
        }

        // querying shares and withdrawing

        for user in users {
            let user_shares: UserSharesBalanceResponse = wasm
                .query(
                    contract_address.as_str(),
                    &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                        crate::msg::UserBalanceQueryMsg::UserSharesBalance {
                            user: user.address(),
                        },
                    )),
                )
                .unwrap();

            // let _balances = bank
            //     .query_all_balances(&QueryAllBalancesRequest {
            //         address: contract_address.to_string(),
            //         pagination: None,
            //     })
            //     .unwrap();
            // let pos_id: PositionResponse = wasm
            //     .query(
            //         contract_address.as_str(),
            //         &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
            //             crate::msg::ClQueryMsg::Position {},
            //         )),
            //     )
            //     .unwrap();
            // let _position = ConcentratedLiquidity::new(&app)
            //     .query_position_by_id(&PositionByIdRequest {
            //         position_id: pos_id.position_ids[0],
            //     })
            //     .unwrap();

            // withdrawing
            wasm.execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: user_shares.balance,
                },
                &[],
                &user,
            )
            .unwrap();
        }
    }
}
