#[cfg(test)]
mod test {
    use cosmwasm_std::{coin, Decimal, Uint128};
    use osmosis_std::types::{
        cosmos::base::v1beta1,
        osmosis::{
            concentratedliquidity::v1beta1::{MsgCreatePosition, Pool, PoolsRequest},
            poolmanager::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute},
        },
    };
    use osmosis_test_tube::{Account, ConcentratedLiquidity, Module, PoolManager, Wasm};
    use prost::Message;
    use std::str::FromStr;

    use crate::{
        msg::{ExecuteMsg, ModifyRangeMsg, QueryMsg},
        query::PositionResponse,
        test_tube::initialize::initialize::default_init,
    };

    const TOKENS_PROVIDED_AMOUNT: &str = "1000000000000";
    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";

    /// # Test: move_range_works_dym_usdc
    ///
    /// This test case initializes a Concentrated Liquidity (CL) pool with 18DEC and USDC tokens
    /// to simulate a real-world scenario on the blockchain with a specific spot price. The purpose
    /// of this test is to ensure that operations such as moving the range within the CL pool function
    /// correctly, especially when dealing with tokens of different decimal precisions.
    ///
    /// ## Initialization Parameters:
    /// - 18DEC Token (u18dec): 18 decimal places, represented as `1000000000000000000u18dec` (for 1 18DEC).
    /// - USDC Token (uusdc): 6 decimal places, represented as `7250000uusdc` (to establish a spot price of 7.25 USDC for 1 18DEC).
    ///
    /// The test initializes a CL pool with these tokens to establish a spot price of 7.25 18DEC/USD.
    /// This spot price accurately reflects the real-world ratio between 18DEC and USDC on the mainnet,
    /// adjusted for the blockchain's unit representation.
    ///
    /// ## Decimal Precision and Spot Price Consideration:
    /// The significant difference in decimal places between 18DEC (18 decimals) and USDC (6 decimals)
    /// necessitates precise calculation to ensure the spot price is accurately represented in the
    /// blockchain's terms. The chosen amounts of `1000000000000000000u18dec` for 18DEC and `7250000uusdc`
    /// for USDC effectively establish a starting spot price of 7.25 18DEC/USD in the CL pool, accurately
    /// representing the spot price in a manner that does not require adjustment for decimal places in
    /// the context of Osmosis' handling of token amounts.
    ///
    /// Spot price it would be: `spot_price = 7250000 / 1000000000000000000`,
    /// calculating the spot price in raw integer format without adjusting for decimal places, representing the USDC required to purchase one unit of 18DEC.
    #[test]
    #[ignore]
    fn move_range_works_dym_usdc() {
        let (app, contract, cl_pool_id, admin) = default_init(vec![
            v1beta1::Coin {
                denom: "u18dec".to_string(),
                amount: "1000000000000000000000000000000".to_string(),
            },
            v1beta1::Coin {
                denom: "uusdc".to_string(),
                amount: "7250000000000000000".to_string(),
            },
        ])
        .unwrap();
        let wasm = Wasm::new(&app);
        let cl = ConcentratedLiquidity::new(&app);

        // Two sided re-range (50% 50%)
        let _result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("0.000000000007").unwrap(),
                        upper_price: Decimal::from_str("0.0000000000075").unwrap(),
                        max_slippage: Decimal::bps(9500),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();

        // One-sided re-range (above current tick, 100% token0)
        let _result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("0.0000000000075").unwrap(),
                        upper_price: Decimal::from_str("0.000000000008").unwrap(),
                        max_slippage: Decimal::bps(9500),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();

        // One-sided re-range (below current tick, 100% token1)
        let _result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("0.0000000000065").unwrap(),
                        upper_price: Decimal::from_str("0.000000000007").unwrap(),
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

    #[test]
    #[ignore]
    fn move_range_works() {
        let (app, contract, cl_pool_id, admin) = default_init(vec![
            v1beta1::Coin {
                denom: DENOM_BASE.to_string(),
                amount: "1000000000000".to_string(),
            },
            v1beta1::Coin {
                denom: DENOM_QUOTE.to_string(),
                amount: "1000000000000".to_string(),
            },
        ])
        .unwrap();
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

        // do a swap to move the cur tick
        pm.swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: admin.address(),
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
            &admin,
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
        let (app, contract, cl_pool_id, admin) = default_init(vec![
            v1beta1::Coin {
                denom: DENOM_BASE.to_string(),
                amount: "1000000000000".to_string(),
            },
            v1beta1::Coin {
                denom: DENOM_QUOTE.to_string(),
                amount: "1000000000000".to_string(),
            },
        ])
        .unwrap();
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

        // do a swap to move the cur tick
        pm.swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: admin.address(),
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
            &admin,
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
        let (app, _contract, _cl_pool_id, admin) = default_init(vec![
            v1beta1::Coin {
                denom: DENOM_BASE.to_string(),
                amount: "1000000000000".to_string(),
            },
            v1beta1::Coin {
                denom: DENOM_QUOTE.to_string(),
                amount: "1000000000000".to_string(),
            },
        ])
        .unwrap();
        let cl = ConcentratedLiquidity::new(&app);

        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let pool: Pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        // from the spreadsheet
        // create a basic position on the pool
        let initial_position = MsgCreatePosition {
            pool_id: pool.id,
            sender: admin.address(),
            lower_tick: 30500000,
            upper_tick: 31500000,
            tokens_provided: vec![
                coin(3349580, DENOM_BASE).into(),
                coin(4280628569, DENOM_QUOTE).into(),
            ],
            token_min_amount0: "0".to_string(),
            token_min_amount1: "0".to_string(),
        };
        let _position = cl.create_position(initial_position, &admin).unwrap();
    }
}
