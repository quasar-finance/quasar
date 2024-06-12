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
            poolmanager::v1beta1::SwapAmountInRoute,
        },
    };
    use osmosis_test_tube::{Account, ConcentratedLiquidity, Module, Wasm};
    use prost::Message;

    use crate::{
        msg::{ExecuteMsg, ModifyRange, MovePosition, QueryMsg},
        query::{MainPositionResponse, PositionsResponse},
        test_tube::initialize::initialize::{
            fixture_default, fixture_dex_router, init_test_contract, ADMIN_BALANCE_AMOUNT,
            DENOM_BASE, DENOM_QUOTE, MAX_SLIPPAGE_HIGH, PERFORMANCE_FEE_DEFAULT,
        },
    };

    #[test]
    #[ignore]
    fn move_range_works() {
        let (app, contract_address, _cl_pool_id, admin, _deposit_ratio, _deposit_ratio_approx) =
            fixture_default(PERFORMANCE_FEE_DEFAULT);
        let wasm = Wasm::new(&app);

        let before_position: MainPositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::MainPosition {},
                )),
            )
            .unwrap();

        let _result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRange::MovePosition(MovePosition {
                        position_id: before_position.position_id,
                        lower_price: Decimal::from_str("400").unwrap(),
                        upper_price: Decimal::from_str("1466").unwrap(),
                        max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                        forced_swap_route: None,
                        claim_after: None,
                    }),
                )),
                &[],
                &admin,
            )
            .unwrap();

        let after_position: MainPositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::MainPosition {},
                )),
            )
            .unwrap();

        // move_position should assign a new position id
        assert_ne!(before_position, after_position)
    }

    #[test]
    #[ignore]
    fn move_range_cw_dex_works() {
        let (
            app,
            contract_address,
            _dex_router_addr,
            _vault_pool_id,
            _swap_pools_ids,
            admin,
            _deposit_ratio,
            _deposit_ratio_approx,
        ) = fixture_dex_router(PERFORMANCE_FEE_DEFAULT);
        let wasm = Wasm::new(&app);

        let _before_position: PositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();

        let _result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("400").unwrap(),
                        upper_price: Decimal::from_str("1466").unwrap(),
                        max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                        // forced_swap_route: Some(vec![path1]),
                        forced_swap_route: None,
                        claim_after: None,
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();

        let _after_position: PositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();
    }

    // TODO: further enhance this forced swap test logic
    #[test]
    #[ignore]
    fn move_range_cw_dex_works_forced_swap_route() {
        let (
            app,
            contract_address,
            _dex_router_addr,
            vault_pool_id,
            _swap_pools_ids,
            admin,
            _deposit_ratio,
            _deposit_ratio_approx,
        ) = fixture_dex_router(PERFORMANCE_FEE_DEFAULT);
        let wasm = Wasm::new(&app);

        let _before_position: PositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();

        // Define CW Dex Router swap route to force
        // In this case we are going from in range, to out of range to the upper side, so we swap all the quote token to base token
        let path1 = SwapAmountInRoute {
            pool_id: vault_pool_id,
            token_out_denom: DENOM_BASE.to_string(),
        };

        let _result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("400").unwrap(),
                        upper_price: Decimal::from_str("1466").unwrap(),
                        max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                        forced_swap_route: Some(vec![path1]),
                        claim_after: None,
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();

        let _after_position: PositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();
    }

    #[test]
    #[ignore]
    fn move_range_single_side_works() {
        let (app, contract_address, _cl_pool_id, admin, _deposit_ratio, _deposit_ratio_approx) =
            fixture_default(PERFORMANCE_FEE_DEFAULT);
        let wasm = Wasm::new(&app);

        let _result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRange::MovePosition(MovePosition {
                        position_id: main_position.position_id,
                        lower_price: Decimal::from_str("20.71").unwrap(),
                        upper_price: Decimal::from_str("45").unwrap(),
                        max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                        forced_swap_route: None,
                        claim_after: None,
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();

        let _result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("0.1").unwrap(),
                        upper_price: Decimal::from_str("0.2").unwrap(),
                        max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                        forced_swap_route: None,
                        claim_after: None,
                    }),
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
        let (app, _contract, _cl_pool_id, _admin, _deposit_ratio, _deposit_ratio_approx) =
            init_test_contract(
                // TODO: Evaluate using fixture_default()
                "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
                &[
                    Coin::new(ADMIN_BALANCE_AMOUNT, "uosmo"),
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
                PERFORMANCE_FEE_DEFAULT,
            );
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uosmo"),
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
