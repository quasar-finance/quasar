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
        test_tube::initialize::initialize::init_test_contract, math::tick::purge_tick_exp_cache,
    };

    use prost::Message;

    const ADMIN_BALANCE_AMOUNT: u128 = 340282366920938463463374607431768211455u128;
    const TOKENS_PROVIDED_AMOUNT: &str = "1000000000000";
    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";

    #[test]
    #[ignore]
    fn move_range_works() {
        let (app, contract, cl_pool_id, admin) = init_test_contract(
            // TODO: Evaluate creating a default_init() variant i.e. out_of_range_init()
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: DENOM_BASE.to_string(),
                denom1: DENOM_QUOTE.to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            21205000,
            27448000,
            vec![
                v1beta1::Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                },
                v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        );
        let wasm = Wasm::new(&app);
        let cl = ConcentratedLiquidity::new(&app);

        // Create a second position (in range) in the pool with the admin user to allow for swapping during update range operation
        cl.create_position(
            MsgCreatePosition {
                pool_id: cl_pool_id,
                sender: admin.address(),
                lower_tick: -5000000,
                upper_tick: 500000,
                tokens_provided: vec![
                    v1beta1::Coin {
                        denom: DENOM_BASE.to_string(),
                        amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                    },
                    v1beta1::Coin {
                        denom: DENOM_QUOTE.to_string(),
                        amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                    },
                ],
                token_min_amount0: Uint128::zero().to_string(),
                token_min_amount1: Uint128::zero().to_string(),
            },
            &admin,
        )
        .unwrap();

        let alice = app
            .init_account(&[
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
            ])
            .unwrap();

        // do a swap to move the cur tick
        let pm = PoolManager::new(&app);
        pm.swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: alice.address(),
                routes: vec![SwapAmountInRoute {
                    pool_id: cl_pool_id,
                    token_out_denom: DENOM_BASE.to_string(),
                }],
                token_in: Some(v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: "1000".to_string(),
                }),
                token_out_min_amount: "1".to_string(),
            },
            &alice,
        )
        .unwrap();

        // let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        // let _pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

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
                        max_slippage: Decimal::bps(9500),
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
            // TODO: Evaluate creating a default_init() variant i.e. out_of_range_init()
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: DENOM_BASE.to_string(),
                denom1: DENOM_QUOTE.to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            21205000,
            27448000,
            vec![
                v1beta1::Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                },
                v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        );
        let wasm = Wasm::new(&app);
        let cl = ConcentratedLiquidity::new(&app);
        let pm = PoolManager::new(&app);

        // Create a second position (in range) in the pool with the admin user to allow for swapping during update range operation
        cl.create_position(
            MsgCreatePosition {
                pool_id: cl_pool_id,
                sender: admin.address(),
                lower_tick: -5000000,
                upper_tick: 500000,
                tokens_provided: vec![
                    v1beta1::Coin {
                        denom: DENOM_BASE.to_string(),
                        amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                    },
                    v1beta1::Coin {
                        denom: DENOM_QUOTE.to_string(),
                        amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                    },
                ],
                token_min_amount0: Uint128::zero().to_string(),
                token_min_amount1: Uint128::zero().to_string(),
            },
            &admin,
        )
        .unwrap();

        let alice = app
            .init_account(&[
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
            ])
            .unwrap();

        // do a swap to move the cur tick
        pm.swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: alice.address(),
                routes: vec![SwapAmountInRoute {
                    pool_id: cl_pool_id,
                    token_out_denom: DENOM_BASE.to_string(),
                }],
                token_in: Some(v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: "1000".to_string(),
                }),
                token_out_min_amount: "1".to_string(),
            },
            &alice,
        )
        .unwrap();

        //let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        //let _pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        let _result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("20.71").unwrap(),
                        upper_price: Decimal::from_str("45").unwrap(),
                        max_slippage: Decimal::bps(9500),
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
            // TODO: Evaluate using default_init()
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: DENOM_BASE.to_string(),  //token0 is uatom
                denom1: DENOM_QUOTE.to_string(), //token1 is uosmo
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            30500000, // 4500
            31500000, // 5500
            vec![
                v1beta1::Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: "1000000".to_string(),
                },
                v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: "1000000".to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        );
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, DENOM_BASE),
                Coin::new(1_000_000_000_000, DENOM_QUOTE),
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
                coin(3349580, DENOM_BASE).into(),
                coin(4280628569, DENOM_QUOTE).into(),
            ],
            token_min_amount0: "0".to_string(),
            token_min_amount1: "0".to_string(),
        };
        let _position = cl.create_position(initial_position, &alice).unwrap();
    }
}
