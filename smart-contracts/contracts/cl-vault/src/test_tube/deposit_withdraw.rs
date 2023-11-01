#[cfg(test)]
mod tests {
    use cosmwasm_std::{assert_approx_eq, coin, Coin, Uint128};

    use osmosis_std::types::{
        cosmos::bank::v1beta1::{MsgSend, QueryAllBalancesRequest},
        osmosis::concentratedliquidity::v1beta1::PositionByIdRequest,
    };
    use osmosis_test_tube::{Account, Bank, ConcentratedLiquidity, Module, Wasm};

    use crate::{
        msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg},
        query::{
            AssetsBalanceResponse, PositionResponse, TotalAssetsResponse, UserSharesBalanceResponse,
        },
        test_tube::{default_init, initialize::initialize::init_18dec},
    };

    #[test]
    #[ignore]
    fn multiple_deposit_withdraw_unused_funds_works() {
        let (app, contract_address, _cl_pool_id, _admin) = init_18dec();
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000_000_000_000, "gwei"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();
        let bob = app
            .init_account(&[
                Coin::new(1_000_000_000_000_000_000_000, "gwei"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let bank = Bank::new(&app);
        let wasm = Wasm::new(&app);

        // depositing
        let _res = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(1_000_000_000_000_000_000, "gwei"),
                    Coin::new(6_000_000_000, "uosmo"),
                ], // 1eth = 6k osmo
                &alice,
            )
            .unwrap();

        // The contract right now has 89874 free uosmo, if we send another 89874 free uosmo, we double the amount of free
        // liquidity, but we want to double the amount of total liquidity, so we first query to contract to get how many
        // assets we have in the position
        let pos_id: PositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();
        let _position = ConcentratedLiquidity::new(&app)
            .query_position_by_id(&PositionByIdRequest {
                position_id: pos_id.position_ids[0],
            })
            .unwrap();
        // This amount should decrease the amount of shares we get back
        // "uatom", amount: "100000" }), asset1: Some(Coin { denom: "uosmo", amount: "10126"
        // to dilute 50%, we need to send uatom100000, 10631uosmo + 89874+uosmo = 100000uosmo
        // aka double the liquidty

        bank.send(
            MsgSend {
                from_address: alice.address(),
                to_address: contract_address.to_string(),
                amount: vec![
                    coin(9995_000_000_000, "gwei").into(),
                    coin(1012, "uosmo").into(),
                ],
            },
            &alice,
        )
        .unwrap();

        let _res = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(1_000_000_000_000_000_000, "gwei"),
                    Coin::new(6_000_000_000, "uosmo"),
                ], // 1eth = 6k osmo
                &bob,
            )
            .unwrap();

        // 2766182566501133149875859 before banksend,
        // 1926137978194597565946694 after banksend
        // does this make sense?
        // when we withdraw 2766182566501133149875859 shares, we should get our original amount back +
        // 2766182566501133149875859 / total_shares * 89874 back, remember we had original free osmo
        // and sent free osmo
        // the second share amount should only get it's original amount back

        // let _ = wasm
        //     .execute(
        //         contract_address.as_str(),
        //         &ExecuteMsg::ExactDeposit { recipient: None },
        //         &[   Coin::new(1_000_000_000_000_000_000, "gwei"),
        // Coin::new(6_000_000_000, "uosmo")],
        //         &alice,
        //     )
        //     .unwrap();

        let alice_shares: UserSharesBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserSharesBalance {
                        user: alice.address(),
                    },
                )),
            )
            .unwrap();
        let bob_shares: UserSharesBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserSharesBalance {
                        user: bob.address(),
                    },
                )),
            )
            .unwrap();

        let _balances = bank
            .query_all_balances(&QueryAllBalancesRequest {
                address: contract_address.to_string(),
                pagination: None,
            })
            .unwrap();
        let pos_id: PositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();
        let _position = ConcentratedLiquidity::new(&app)
            .query_position_by_id(&PositionByIdRequest {
                position_id: pos_id.position_ids[0],
            })
            .unwrap();
        // This amount should decrease the amount of shares we get back

        let _withdraw = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: bob_shares.balance,
                },
                &[],
                &bob,
            )
            .unwrap();

        let _withdraw = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: alice_shares.balance,
                },
                &[],
                &alice,
            )
            .unwrap();
        // we receive "token0_amount", value: "2018" }, Attribute { key: "token1_amount", value: "3503
        // we used 5000uatom to deposit and 507 uosmo, thus we are down 3000 uatom and up 2996 uosmo
    }

    #[test]
    #[ignore]
    fn multiple_deposit_withdraw_works() {
        let (app, contract_address, _cl_pool_id, _admin) = init_18dec();
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000_000_000_000_000, "gwei"),
                Coin::new(1_000_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let wasm = Wasm::new(&app);

        let vault_assets_before: TotalAssetsResponse = wasm
            .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
            .unwrap();

        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(1_000_000_000_000_000_000, "gwei"),
                    Coin::new(6_000_000_000, "uosmo"),
                ],
                &alice,
            )
            .unwrap();

        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(1_000_000_000_000_000_000, "gwei"),
                    Coin::new(6_000_000_000, "uosmo"),
                ],
                &alice,
            )
            .unwrap();

        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(1_000_000_000_000_000_000, "gwei"),
                    Coin::new(6_000_000_000, "uosmo"),
                ],
                &alice,
            )
            .unwrap();

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
        println!("{:?}", user_assets);
        assert_approx_eq!(
            user_assets.balances[0].amount,
            Uint128::from(15000u128),
            "0.001"
        );
        assert_approx_eq!(
            user_assets.balances[1].amount,
            Uint128::from(1516u128),
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
            Uint128::from(15000u128),
            "0.001"
        );
        assert_approx_eq!(
            user_assets_again.balances[1].amount,
            Uint128::from(1516u128),
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
                .checked_add(Uint128::from(15000u128))
                .unwrap(),
            "0.001"
        );
        // again we get refunded so we only expect around 500 to deposit here
        assert_approx_eq!(
            vault_assets.token1.amount,
            vault_assets_before
                .token1
                .amount
                .checked_add(Uint128::from(1516u128))
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
    fn single_deposit_withdraw_works() {
        let (app, contract_address, _cl_pool_id, _admin) = default_init();
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let wasm = Wasm::new(&app);

        let vault_assets_before: TotalAssetsResponse = wasm
            .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
            .unwrap();

        let _deposit = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(1_000_000_000_000_000_000, "gwei"),
                    Coin::new(6_000_000_000, "uosmo"),
                ],
                &alice,
            )
            .unwrap();

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
        assert_approx_eq!(
            user_assets.balances[0].amount,
            Uint128::from(5000u128),
            "0.001"
        );
        // we get refunded so we only expect around 500 to deposit here
        assert_approx_eq!(
            user_assets.balances[1].amount,
            Uint128::from(500u128),
            "0.01"
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
            Uint128::from(5000u128),
            "0.001"
        );
        // again we get refunded so we only expect around 500 to deposit here
        assert_approx_eq!(
            user_assets_again.balances[1].amount,
            Uint128::from(500u128),
            "0.01"
        );

        let vault_assets: TotalAssetsResponse = wasm
            .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
            .unwrap();
        assert_approx_eq!(
            vault_assets.token0.amount,
            vault_assets_before
                .token0
                .amount
                .checked_add(Uint128::from(5000u128))
                .unwrap(),
            "0.001"
        );
        // again we get refunded so we only expect around 500 to deposit here
        assert_approx_eq!(
            vault_assets.token1.amount,
            vault_assets_before
                .token1
                .amount
                .checked_add(Uint128::from(500u128))
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
}
