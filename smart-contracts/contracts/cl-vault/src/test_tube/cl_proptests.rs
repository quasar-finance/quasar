#[cfg(test)]
mod tests {
    use cosmwasm_std::{Coin, Decimal, Uint128};
    use cw_vault_multi_standard::VaultInfoResponse;
    use osmosis_std::types::osmosis::{
        concentratedliquidity::v1beta1::{Pool, PoolsRequest},
        tokenfactory::v1beta1::QueryDenomsFromCreatorRequest,
    };
    use osmosis_test_tube::{
        cosmrs::proto::traits::Message, Account, ConcentratedLiquidity, Module, TokenFactory, Wasm,
    };

    use crate::{
        msg::{ClQueryMsg, ExecuteMsg, ExtensionQueryMsg, ModifyRangeMsg, QueryMsg},
        query::{PoolResponse, UserBalanceResponse},
        test_tube::default_init,
    };

    #[test]
    #[ignore]
    fn multiple_deposit_withdraw_works() {
        let (app, contract_address, _cl_pool_id, _admin) = init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(340282366920938463463374607431768211455u128, "uatom"),
                Coin::new(340282366920938463463374607431768211455u128, "uosmo"),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: "uatom".to_string(),
                denom1: "uosmo".to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            -5000000, // 0.5 spot price
            500000,   // 1.5 spot price
            vec![
                v1beta1::Coin {
                    denom: "uatom".to_string(),
                    amount: "1000000000000".to_string(),
                },
                v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "1000000000000".to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        );
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let wasm = Wasm::new(&app);

        let _ = wasm
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

        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[Coin::new(5_000, "uatom"), Coin::new(5_000, "uosmo")],
                &alice,
            )
            .unwrap();

        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[Coin::new(5_000, "uatom"), Coin::new(5_000, "uosmo")],
                &alice,
            )
            .unwrap();

        let shares: UserBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserLockedBalance {
                        user: alice.address(),
                    },
                )),
            )
            .unwrap();
        assert!(!shares.balance.is_zero());

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
        let (app, contract_address, _cl_pool_id, _admin) = init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(340282366920938463463374607431768211455u128, "uatom"),
                Coin::new(340282366920938463463374607431768211455u128, "uosmo"),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: "uatom".to_string(),
                denom1: "uosmo".to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            -5000000, // 0.5 spot price
            500000,   // 1.5 spot price
            vec![
                v1beta1::Coin {
                    denom: "uatom".to_string(),
                    amount: "1000000000000".to_string(),
                },
                v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "1000000000000".to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        );
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let wasm = Wasm::new(&app);

        let deposit = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[Coin::new(5_000, "uatom"), Coin::new(5_000, "uosmo")],
                &alice,
            )
            .unwrap();

        let _mint = deposit.events.iter().find(|e| e.ty == "tf_mint").unwrap();

        let shares: UserBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserLockedBalance {
                        user: alice.address(),
                    },
                )),
            )
            .unwrap();
        assert!(!shares.balance.is_zero());

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

    // #[test]
    // #[ignore]
    fn move_range_works() {
        let (app, contract, _cl_pool_id, admin) = init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(340282366920938463463374607431768211455u128, "uatom"),
                Coin::new(340282366920938463463374607431768211455u128, "uosmo"),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: "uatom".to_string(),
                denom1: "uosmo".to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            -5000000, // 0.5 spot price
            500000,   // 1.5 spot price
            vec![
                v1beta1::Coin {
                    denom: "uatom".to_string(),
                    amount: "1000000000000".to_string(),
                },
                v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "1000000000000".to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        );
        let _alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let wasm = Wasm::new(&app);
        let _result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Uint128::new(2),
                        upper_price: Uint128::new(200),
                        max_slippage: Decimal::permille(5),
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();
    }

    #[test]
    #[ignore]
    fn default_init_works() {
        let (app, contract_address, _cl_pool_id, _admin) = init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(340282366920938463463374607431768211455u128, "uatom"),
                Coin::new(340282366920938463463374607431768211455u128, "uosmo"),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: "uatom".to_string(),
                denom1: "uosmo".to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            -5000000, // 0.5 spot price
            500000,   // 1.5 spot price
            vec![
                v1beta1::Coin {
                    denom: "uatom".to_string(),
                    amount: "1000000000000".to_string(),
                },
                v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "1000000000000".to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        );
        let wasm = Wasm::new(&app);
        let cl = ConcentratedLiquidity::new(&app);
        let tf = TokenFactory::new(&app);

        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        let resp = wasm
            .query::<QueryMsg, PoolResponse>(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                    ClQueryMsg::Pool {},
                )),
            )
            .unwrap();

        assert_eq!(resp.pool_config.pool_id, pool.id);
        assert_eq!(resp.pool_config.token0, pool.token0);
        assert_eq!(resp.pool_config.token1, pool.token1);

        let resp = wasm
            .query::<QueryMsg, VaultInfoResponse>(contract_address.as_str(), &QueryMsg::Info {})
            .unwrap();

        assert_eq!(resp.tokens, vec![pool.token0, pool.token1]);
        assert_eq!(
            resp.vault_token,
            tf.query_denoms_from_creator(&QueryDenomsFromCreatorRequest {
                creator: contract_address.to_string()
            })
            .unwrap()
            .denoms[0]
        );
    }
}
