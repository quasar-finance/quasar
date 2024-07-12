use std::str::FromStr;

use cl_vault::{
    msg::{CreatePosition, ExtensionExecuteMsgFns},
    state::VaultConfig,
};
use cw_vault_multi_standard::msg::VaultStandardExecuteMsgFns;
use interface::cl_vault::ClVaultContract;
use osmosis_helpers::concentratedliquidity::create_cl_pool;

use cosmwasm_std::{coin, Decimal};
use cw_orch::prelude::*;
use cw_orch_osmosis_test_tube::{osmosis_test_tube::Account, OsmosisTestTube};
use interface::range_middleware::RangeMiddlewareContract;
use range_middleware::{
    msg::{ExecuteMsgFns, QueryMsgFns},
    range::{execute::RangeExecuteMsgFns, query::RangeQueryMsgFns},
    state::{RangeUpdates, UpdateActions},
};

#[test]
fn submit_range_works() {
    let denom0 = "sat";
    let denom1 = "uatom";
    let mut chain = OsmosisTestTube::new(vec![
        coin(1_000_000_000_000, "uosmo"),
        coin(1_000_000_000_000, denom0),
        coin(1_000_000_000_000, denom1),
    ]);
    let tick_spacing = 100;
    let spread_factor = Decimal::from_str("0.1").unwrap();

    let pool = create_cl_pool(
        &chain.app.borrow(),
        denom0.to_string(),
        denom1.to_string(),
        tick_spacing,
        spread_factor,
        chain.sender.as_ref(),
    );

    let alice = chain
        .init_account(vec![
            coin(100_000_000_000_000_u128, denom0),
            coin(100_000_000_000_000_u128, denom1),
        ])
        .unwrap();

    let range_middleware = RangeMiddlewareContract::new(chain.clone());

    range_middleware.upload().unwrap();
    range_middleware
        .instantiate(
            &range_middleware::msg::InstantiateMsg {
                range_submitter_admin: chain.sender().to_string(),
                range_executor_admin: alice.as_ref().address(),
            },
            None,
            None,
        )
        .unwrap();

    let cl_vault = ClVaultContract::new(chain.clone());
    cl_vault.upload().unwrap();

    cl_vault
        .instantiate(
            &cl_vault::msg::InstantiateMsg {
                thesis: "test things".to_string(),
                name: "test-vault".to_string(),
                admin: alice.address(),
                range_admin: range_middleware.addr_str().unwrap(),
                pool_id: pool.id,
                // TODO change the dex router
                config: VaultConfig {
                    performance_fee: Decimal::percent(20),
                    treasury: Addr::unchecked(alice.as_ref().address()),
                    swap_max_slippage: Decimal::permille(20),
                    dex_router: Addr::unchecked(alice.as_ref().address()),
                },
                vault_token_subdenom: "test".to_string(),
                initial_lower_tick: -50000,
                initial_upper_tick: 5000,
            },
            None,
            Some(&[coin(10_000_u128, denom0), coin(10_000_u128, denom1)]),
        )
        .unwrap();

    cl_vault
        .exact_deposit(
            None,
            &[
                coin(100_000_000_u128, denom0),
                coin(100_000_000_u128, denom1),
            ],
        )
        .unwrap();

    let update = RangeUpdates {
        cl_vault_address: cl_vault.addr_str().unwrap(),
        updates: vec![
            UpdateActions::CreatePosition(CreatePosition {
                lower_price: Decimal::from_str("1.1").unwrap(),
                upper_price: Decimal::from_str("1.5").unwrap(),
                claim_after: None,
                max_token0: None,
                max_token1: None,
            }),
            UpdateActions::CreatePosition(CreatePosition {
                lower_price: Decimal::from_str("0.5").unwrap(),
                upper_price: Decimal::from_str("0.9").unwrap(),
                claim_after: None,
                max_token0: None,
                max_token1: None,
            }),
        ]
        .into(),
    };

    let res = range_middleware.submit_new_range(update.clone()).unwrap();

    assert_eq!(
        update,
        range_middleware
            .get_queued_range_updates_for_contract(cl_vault.addr_str().unwrap())
            .unwrap()
    )
}
