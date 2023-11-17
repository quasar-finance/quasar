#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::msg::ExecuteMsg;
    use crate::test_tube::initialize::initialize::init_test_contract;
    use cosmwasm_std::{Coin, Decimal, Uint128};
    use osmosis_std::types::cosmos::base::v1beta1::{self, Coin as OsmoCoin};
    use osmosis_std::types::osmosis::concentratedliquidity::poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SwapAmountInRoute,
    };
    use osmosis_test_tube::{Account, Module, PoolManager, Wasm};

    #[test]
    #[ignore]
    fn test_rewards_single_distribute_claim() {
        let (app, contract_address, cl_pool_id, _admin) = init_test_contract(
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

        let bob = app
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
                &[Coin::new(5_000_000, "uatom"), Coin::new(5_000_000, "uosmo")],
                &alice,
            )
            .unwrap();

        // do a bunch of swaps to get some swap fees
        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: "uatom".to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: "uosmo".to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: "uatom".to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: "uosmo".to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: "uatom".to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: "uosmo".to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: "uosmo".to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: "uatom".to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: "uosmo".to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: "uatom".to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: "uosmo".to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: "uatom".to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        let _res = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::DistributeRewards {}),
                &[],
                &alice,
            )
            .unwrap();
    }
}
