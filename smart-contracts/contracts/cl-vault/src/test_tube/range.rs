#[cfg(test)]
mod test {
    use std::str::FromStr;

    use cosmwasm_std::{coin, Coin, Decimal, Uint128};
    use osmosis_std::types::{
        cosmos::base::v1beta1,
        osmosis::{
            concentratedliquidity::{
                poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool,
                v1beta1::{MsgCreatePosition, Pool, PoolsRequest},
            },
            poolmanager::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute},
        },
    };
    use osmosis_test_tube::{Account, ConcentratedLiquidity, Module, PoolManager, Wasm};

    use crate::{
        msg::{ExecuteMsg, ModifyRangeMsg, QueryMsg},
        query::PositionResponse,
        test_tube::initialize::initialize::init_test_contract,
    };

    use prost::Message;

    #[test]
    #[ignore]
    fn move_range_partial_swap_works() {
        let (app, contract, cl_pool_id, admin) = init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(1_000_000_000_000_000, "uosmo"),
                Coin::new(
                    1_000_000_000_000_000,
                    "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
                ),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: "uosmo".to_string(),
                denom1: "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2"
                    .to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            -1000,
            0,
            vec![
                v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "1000000000000".to_string(),
                },
                v1beta1::Coin {
                    denom: "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2"
                        .to_string(),
                    amount: "140000000000".to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        );
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000_000, "uosmo"),
                Coin::new(
                    1_000_000_000_000_000,
                    "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
                ),
            ])
            .unwrap();

        let wasm = Wasm::new(&app);

        // do a swap to move the cur tick
        let pm = PoolManager::new(&app);
        pm.swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: alice.address(),
                routes: vec![SwapAmountInRoute {
                    pool_id: cl_pool_id,
                    token_out_denom: "uosmo".to_string(),
                }],
                token_in: Some(v1beta1::Coin {
                    denom: "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2"
                        .to_string(),
                    amount: "100000000000".to_string(),
                }),
                token_out_min_amount: "1".to_string(),
            },
            &alice,
        )
        .unwrap();

        // move the position completely out of range so we 100% in one asset
        let _result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("1.0001").unwrap(),
                        upper_price: Decimal::from_str("1.0002").unwrap(),
                        max_slippage: Decimal::permille(5),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();

        // deposit 2 million uosmo from alice
        let _result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[coin(2_000_000_000_000, "uosmo")],
                &admin,
            )
            .unwrap();

        // now do a partial swap during the range movement
        let result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("0.9992").unwrap(),
                        upper_price: Decimal::from_str("1.0043").unwrap(),
                        max_slippage: Decimal::permille(5),
                        ratio_of_swappable_funds_to_use: Decimal::percent(5),
                        twap_window_seconds: 45,
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();
        // for asserting the result, to this new position we expect to swap 13961732009 uosmo to 13960467092 uatom
        // which is about equal to get_single_sided_deposit_0_to_1_swap_amount(balance0 * 0.005)
        assert_eq!(
            result
                .events
                .iter()
                .find(|e| e.ty == "token_swapped")
                .unwrap()
                .attributes
                .iter()
                .find(|a| a.key == "tokens_in")
                .unwrap()
                .value,
            "13961732009uosmo"
        )
    }

    // #[test]
    // #[ignore]
    fn move_range_works() {
        let (app, contract, cl_pool_id, admin) = init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: "uatom".to_string(),
                denom1: "uosmo".to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            21205000,
            27448000,
            vec![
                v1beta1::Coin {
                    denom: "uatom".to_string(),
                    amount: "10000000000".to_string(),
                },
                v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "10000000000".to_string(),
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
        let cl = ConcentratedLiquidity::new(&app);

        // do a swap to move the cur tick
        let pm = PoolManager::new(&app);
        pm.swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: alice.address(),
                routes: vec![SwapAmountInRoute {
                    pool_id: cl_pool_id,
                    token_out_denom: "uatom".to_string(),
                }],
                token_in: Some(v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "1000".to_string(),
                }),
                token_out_min_amount: "1".to_string(),
            },
            &alice,
        )
        .unwrap();

        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let _pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        let _before_position: PositionResponse = wasm
            .query(
                contract.as_str(),
                &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();

        let _result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("400").unwrap(),
                        upper_price: Decimal::from_str("1466").unwrap(),
                        max_slippage: Decimal::permille(5),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();

        let _after_position: PositionResponse = wasm
            .query(
                contract.as_str(),
                &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();
    }

    #[test]
    #[ignore]
    fn move_range_same_single_side_works() {
        let (app, contract, cl_pool_id, admin) = init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: "uatom".to_string(),
                denom1: "uosmo".to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            21205000,
            27448000,
            vec![
                v1beta1::Coin {
                    denom: "uatom".to_string(),
                    amount: "10000000000".to_string(),
                },
                v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "10000000000".to_string(),
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
        let cl = ConcentratedLiquidity::new(&app);

        // do a swap to move the cur tick
        let pm = PoolManager::new(&app);
        pm.swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: alice.address(),
                routes: vec![SwapAmountInRoute {
                    pool_id: cl_pool_id,
                    token_out_denom: "uatom".to_string(),
                }],
                token_in: Some(v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "1000".to_string(),
                }),
                token_out_min_amount: "1".to_string(),
            },
            &alice,
        )
        .unwrap();

        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let _pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        let _result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("20.71").unwrap(),
                        upper_price: Decimal::from_str("45").unwrap(),
                        max_slippage: Decimal::permille(5),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();
    }

    /*
    we try the following position from https://docs.google.com/spreadsheets/d/1xPsKsQkM0apTZQPBBwVlEyB5Sk31sw6eE8U0FgnTWUQ/edit?usp=sharing
    lower_price:   4500
    current_price: 4692.937
    upper_price:   5500

    the spreadsheet says we need to leave 42806.28569 in token x and swap over 157193.7143
    157193.7143 / 4692.937 = 33.49580749
    both token amounts are used in 5 decimals, since the leftover amount is in 5 decimals
    so we want to deposit 4280628569 and 3349580
    */
    #[test]
    #[ignore]
    fn test_swap_math_poc() {
        let (app, _contract, _cl_pool_id, _admin) = init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: "uatom".to_string(), //token0 is uatom
                denom1: "uosmo".to_string(), //token1 is uosmo
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            30500000, // 4500
            31500000, // 5500
            vec![
                v1beta1::Coin {
                    denom: "uatom".to_string(),
                    amount: "1000000".to_string(),
                },
                v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "1000000".to_string(),
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

        let cl = ConcentratedLiquidity::new(&app);

        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let pool: Pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        // from the spreadsheet
        // create a basic position on the pool
        let initial_position = MsgCreatePosition {
            pool_id: pool.id,
            sender: alice.address(),
            lower_tick: 30500000,
            upper_tick: 31500000,
            tokens_provided: vec![
                coin(3349580, "uatom").into(),
                coin(4280628569, "uosmo").into(),
            ],
            token_min_amount0: "0".to_string(),
            token_min_amount1: "0".to_string(),
        };
        let _position = cl.create_position(initial_position, &alice).unwrap();
    }
}
