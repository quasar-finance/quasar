use crate::setup::{
    fixture_dex_router, init_test_contract, ADMIN_BALANCE_AMOUNT, DENOM_BASE, DENOM_QUOTE,
    MAX_SLIPPAGE_HIGH, PERFORMANCE_FEE_DEFAULT,
};

use cosmwasm_std::{coin, Coin, Decimal, Uint128};
use osmosis_std::types::{
    cosmos::base::v1beta1,
    osmosis::{
        concentratedliquidity::{
            poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool,
            v1beta1::{MsgCreatePosition, Pool, PoolsRequest, PositionByIdRequest},
        },
        poolmanager::v1beta1::SwapAmountInRoute,
    },
};
use osmosis_test_tube::{Account, ConcentratedLiquidity, Module, Wasm};
use prost::Message;
use std::str::FromStr;

use cl_vault::{
    msg::{
        ClQueryMsg, ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, ModifyRangeMsg, QueryMsg,
    },
    query::PositionResponse,
};

const DO_SWAP_DEPOSIT_TOKEN_IN_OFFSET: usize = 1;
const DO_SWAP_DEPOSIT_MIN_OUT_OFFSET: usize = 2;
const SWAP_SUCCESS_BASE_BALANCE_OFFSET: usize = 3;
const SWAP_SUCCESS_QUOTE_BALANCE_OFFSET: usize = 4;

#[test]
fn move_range_works() {
    let (app, contract_address, _dex_router, _cl_pool_id, _pools, admin, ..) =
        fixture_dex_router(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);

    let _before_position: PositionResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                ClQueryMsg::Position {},
            )),
        )
        .unwrap();

    let response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                lower_price: Decimal::from_str("0.65").unwrap(),
                upper_price: Decimal::from_str("1.3").unwrap(),
                max_slippage: Decimal::percent(89),
                ratio_of_swappable_funds_to_use: Decimal::one(),
                twap_window_seconds: 45,
                forced_swap_route: None,
                claim_after: None,
            })),
            &[],
            &admin,
        )
        .unwrap();

    for event in &response.events {
        let pos = event
            .attributes
            .iter()
            .position(|attr| attr.key == "action" && attr.value == "do_swap_deposit_merge");
        if let Some(pos) = pos {
            assert_eq!(
                event.attributes[pos + DO_SWAP_DEPOSIT_TOKEN_IN_OFFSET].value,
                "223645uatom"
            );
            assert_eq!(
                event.attributes[pos + DO_SWAP_DEPOSIT_MIN_OUT_OFFSET].value,
                "199044"
            );
        }

        let pos = event
            .attributes
            .iter()
            .position(|attr| attr.key == "action" && attr.value == "handle_swap_success");
        if let Some(pos) = pos {
            assert_eq!(
                event.attributes[pos + SWAP_SUCCESS_BASE_BALANCE_OFFSET].value,
                "776354uatom"
            );
            assert_eq!(
                event.attributes[pos + SWAP_SUCCESS_QUOTE_BALANCE_OFFSET].value,
                "1221399ubtc"
            );
        }
    }

    let response: PositionResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                ClQueryMsg::Position {},
            )),
        )
        .unwrap();
    assert_eq!(response.position_ids.len(), 1);
    let position_id = response.position_ids[0];
    assert_eq!(position_id, 3u64);

    let cl = ConcentratedLiquidity::new(&app);
    let pos = cl
        .query_position_by_id(&PositionByIdRequest { position_id })
        .unwrap()
        .position
        .unwrap();
    let pos_base: Coin = pos.asset0.unwrap().try_into().unwrap();
    let pos_quote: Coin = pos.asset1.unwrap().try_into().unwrap();
    assert_eq!(pos_base, coin(774929u128, DENOM_BASE));
    assert_eq!(pos_quote, coin(1221399u128, DENOM_QUOTE));
}

#[test]
fn move_range_cw_dex_works() {
    let (app, contract_address, _dex_router_addr, _vault_pool_id, _swap_pools_ids, admin, ..) =
        fixture_dex_router(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);

    let _before_position: PositionResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                ClQueryMsg::Position {},
            )),
        )
        .unwrap();

    let response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                lower_price: Decimal::from_str("400").unwrap(),
                upper_price: Decimal::from_str("1466").unwrap(),
                max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                ratio_of_swappable_funds_to_use: Decimal::one(),
                twap_window_seconds: 45,
                forced_swap_route: None,
                claim_after: None,
            })),
            &[],
            &admin,
        )
        .unwrap();
    for event in &response.events {
        let pos = event
            .attributes
            .iter()
            .position(|attr| attr.key == "action" && attr.value == "do_swap_deposit_merge");
        if let Some(pos) = pos {
            assert_eq!(
                event.attributes[pos + DO_SWAP_DEPOSIT_TOKEN_IN_OFFSET].value,
                "999999ubtc"
            );
            assert_eq!(
                event.attributes[pos + DO_SWAP_DEPOSIT_MIN_OUT_OFFSET].value,
                "899999"
            );
        }

        let pos = event
            .attributes
            .iter()
            .position(|attr| attr.key == "action" && attr.value == "handle_swap_success");
        if let Some(pos) = pos {
            assert_eq!(
                event.attributes[pos + SWAP_SUCCESS_BASE_BALANCE_OFFSET].value,
                "1989999uatom"
            );
            assert_eq!(
                event.attributes[pos + SWAP_SUCCESS_QUOTE_BALANCE_OFFSET].value,
                "0ubtc"
            );
        }
    }

    let response: PositionResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                ClQueryMsg::Position {},
            )),
        )
        .unwrap();
    assert_eq!(response.position_ids.len(), 1);
    let position_id = response.position_ids[0];
    assert_eq!(position_id, 3u64);

    let cl = ConcentratedLiquidity::new(&app);
    let pos = cl
        .query_position_by_id(&PositionByIdRequest { position_id })
        .unwrap()
        .position
        .unwrap();
    let pos_base: Coin = pos.asset0.unwrap().try_into().unwrap();
    let pos_quote: Coin = pos.asset1.unwrap().try_into().unwrap();
    assert_eq!(pos_base, coin(1989999u128, DENOM_BASE));
    assert_eq!(pos_quote, coin(0u128, DENOM_QUOTE));
}

#[test]
fn move_range_cw_dex_works_forced_swap_route() {
    let (app, contract_address, _dex_router_addr, vault_pool_id, _swap_pools_ids, admin, ..) =
        fixture_dex_router(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);

    let _before_position: PositionResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                ClQueryMsg::Position {},
            )),
        )
        .unwrap();

    // Define CW Dex Router swap route to force
    // In this case we are going from in range, to out of range to the upper side, so we swap all the quote token to base token
    let path1 = SwapAmountInRoute {
        pool_id: vault_pool_id,
        token_out_denom: DENOM_BASE.to_string(),
    };

    let response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                lower_price: Decimal::from_str("400").unwrap(),
                upper_price: Decimal::from_str("1466").unwrap(),
                max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                ratio_of_swappable_funds_to_use: Decimal::one(),
                twap_window_seconds: 45,
                forced_swap_route: Some(vec![path1]),
                claim_after: None,
            })),
            &[],
            &admin,
        )
        .unwrap();

    for event in &response.events {
        let pos = event
            .attributes
            .iter()
            .position(|attr| attr.key == "action" && attr.value == "do_swap_deposit_merge");
        if let Some(pos) = pos {
            assert_eq!(
                event.attributes[pos + DO_SWAP_DEPOSIT_TOKEN_IN_OFFSET].value,
                "999999ubtc"
            );
            assert_eq!(
                event.attributes[pos + DO_SWAP_DEPOSIT_MIN_OUT_OFFSET].value,
                "899999"
            );
        }

        let pos = event
            .attributes
            .iter()
            .position(|attr| attr.key == "action" && attr.value == "handle_swap_success");
        if let Some(pos) = pos {
            assert_eq!(
                event.attributes[pos + SWAP_SUCCESS_BASE_BALANCE_OFFSET].value,
                "1989999uatom"
            );
            assert_eq!(
                event.attributes[pos + SWAP_SUCCESS_QUOTE_BALANCE_OFFSET].value,
                "0ubtc"
            );
        }
    }
    let response: PositionResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                ClQueryMsg::Position {},
            )),
        )
        .unwrap();
    assert_eq!(response.position_ids.len(), 1);
    let position_id = response.position_ids[0];
    assert_eq!(position_id, 3u64);

    let cl = ConcentratedLiquidity::new(&app);
    let pos = cl
        .query_position_by_id(&PositionByIdRequest { position_id })
        .unwrap()
        .position
        .unwrap();
    let pos_base: Coin = pos.asset0.unwrap().try_into().unwrap();
    let pos_quote: Coin = pos.asset1.unwrap().try_into().unwrap();
    assert_eq!(pos_base, coin(1989999u128, DENOM_BASE));
    assert_eq!(pos_quote, coin(0u128, DENOM_QUOTE));
}

#[test]
fn move_range_single_side_works() {
    let (app, contract_address, _dex_router_addr, _vault_pool_id, _swap_pools_ids, admin, ..) =
        fixture_dex_router(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);

    let response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                lower_price: Decimal::from_str("20.71").unwrap(),
                upper_price: Decimal::from_str("45").unwrap(),
                max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                ratio_of_swappable_funds_to_use: Decimal::one(),
                twap_window_seconds: 45,
                forced_swap_route: None,
                claim_after: None,
            })),
            &[],
            &admin,
        )
        .unwrap();

    for event in &response.events {
        let pos = event
            .attributes
            .iter()
            .position(|attr| attr.key == "action" && attr.value == "do_swap_deposit_merge");
        if let Some(pos) = pos {
            assert_eq!(
                event.attributes[pos + DO_SWAP_DEPOSIT_TOKEN_IN_OFFSET].value,
                "999999ubtc"
            );
            assert_eq!(
                event.attributes[pos + DO_SWAP_DEPOSIT_MIN_OUT_OFFSET].value,
                "899999"
            );
        }

        let pos = event
            .attributes
            .iter()
            .position(|attr| attr.key == "action" && attr.value == "handle_swap_success");
        if let Some(pos) = pos {
            assert_eq!(
                event.attributes[pos + SWAP_SUCCESS_BASE_BALANCE_OFFSET].value,
                "1989999uatom"
            );
            assert_eq!(
                event.attributes[pos + SWAP_SUCCESS_QUOTE_BALANCE_OFFSET].value,
                "0ubtc"
            );
        }
    }

    let response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                lower_price: Decimal::from_str("0.1").unwrap(),
                upper_price: Decimal::from_str("0.2").unwrap(),
                max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                ratio_of_swappable_funds_to_use: Decimal::one(),
                twap_window_seconds: 45,
                forced_swap_route: None,
                claim_after: None,
            })),
            &[],
            &admin,
        )
        .unwrap();

    for event in &response.events {
        let pos = event
            .attributes
            .iter()
            .position(|attr| attr.key == "action" && attr.value == "do_swap_deposit_merge");
        if let Some(pos) = pos {
            assert_eq!(
                event.attributes[pos + DO_SWAP_DEPOSIT_TOKEN_IN_OFFSET].value,
                "1989998uatom"
            );
            assert_eq!(
                event.attributes[pos + DO_SWAP_DEPOSIT_MIN_OUT_OFFSET].value,
                "1790998"
            );
        }

        let pos = event
            .attributes
            .iter()
            .position(|attr| attr.key == "action" && attr.value == "handle_swap_success");
        if let Some(pos) = pos {
            assert_eq!(
                event.attributes[pos + SWAP_SUCCESS_BASE_BALANCE_OFFSET].value,
                "0uatom"
            );
            assert_eq!(
                event.attributes[pos + SWAP_SUCCESS_QUOTE_BALANCE_OFFSET].value,
                "1970100ubtc"
            );
        }
    }

    let response: PositionResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                ClQueryMsg::Position {},
            )),
        )
        .unwrap();
    assert_eq!(response.position_ids.len(), 1);
    let position_id = response.position_ids[0];
    assert_eq!(position_id, 4u64);

    let cl = ConcentratedLiquidity::new(&app);
    let pos = cl
        .query_position_by_id(&PositionByIdRequest { position_id })
        .unwrap()
        .position
        .unwrap();
    let pos_base: Coin = pos.asset0.unwrap().try_into().unwrap();
    let pos_quote: Coin = pos.asset1.unwrap().try_into().unwrap();
    assert_eq!(pos_base, coin(0u128, DENOM_BASE));
    assert_eq!(pos_quote, coin(1970100u128, DENOM_QUOTE));
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
fn test_swap_math_poc() {
    let (app, _contract, _cl_pool_id, _admin, _) = init_test_contract(
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
