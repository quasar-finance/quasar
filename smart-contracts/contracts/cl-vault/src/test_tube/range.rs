#[cfg(test)]
mod test {
    use std::str::FromStr;

    use cosmwasm_std::{Coin, Decimal, Uint128};
    use osmosis_std::types::{
        cosmos::base::v1beta1,
        osmosis::{
            concentratedliquidity::{
                poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool,
                v1beta1::{Pool, PoolsRequest, PositionByIdRequest},
            },
            poolmanager::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute},
        },
    };
    use osmosis_test_tube::{Account, ConcentratedLiquidity, Module, PoolManager, Wasm};

    use crate::{
        msg::{ExecuteMsg, ModifyRangeMsg, QueryMsg},
        test_tube::initialize::initialize::init_test_contract, query::PositionResponse,
    };

    use prost::Message;

    #[test]
    #[ignore]
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
        let pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        println!("{:?}", pool);

        let before_position: PositionResponse = wasm
            .query(
                contract.as_str(),
                &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();


        //  liquidity: "444754394944564241997324" }), asset0: Some(Coin { denom: "uatom", amount: "9766" }), asset1: Some(Coin { denom: "uosmo", amount: "100001" }
        println!("{:?}", cl.query_position_by_id(&PositionByIdRequest{position_id: before_position.position_ids[0]}).unwrap());


        let result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("400").unwrap().to_string(),
                        upper_price: Decimal::from_str("1466").unwrap().to_string(),
                        max_slippage: Decimal::permille(5),
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();

            let after_position: PositionResponse = wasm
            .query(
                contract.as_str(),
                &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();
            // liquidity: "136802306715768225461536" }), asset0: Some(Coin { denom: "uatom", amount: "3026" }), asset1: Some(Coin { denom: "uosmo", amount: "99994" }
            println!("{:?}", cl.query_position_by_id(&PositionByIdRequest{position_id: after_position.position_ids[0]}).unwrap());
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
        let pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        println!("{:?}", pool);

        let _result = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::from_str("20.71").unwrap().to_string(),
                        upper_price: Decimal::from_str("45").unwrap().to_string(),
                        max_slippage: Decimal::permille(5),
                    },
                )),
                &[],
                &admin,
            )
            .unwrap();
    }
}
