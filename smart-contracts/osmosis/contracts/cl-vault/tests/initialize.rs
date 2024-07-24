#![cfg(feature = "test-tube")]

mod setup;
use setup::{fixture_default, DENOM_BASE, DENOM_QUOTE, MAX_SLIPPAGE_HIGH, PERFORMANCE_FEE_DEFAULT};

use cl_vault::{
    msg::{
        ClQueryMsg, ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, ModifyRangeMsg, QueryMsg,
    },
    query::PoolResponse,
};
use cosmwasm_std::{Coin, Decimal};
use cw_vault_multi_standard::VaultInfoResponse;
use osmosis_std::types::{
    cosmos::base::v1beta1,
    osmosis::{
        concentratedliquidity::v1beta1::{Pool, PoolsRequest},
        poolmanager::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute},
        tokenfactory::v1beta1::QueryDenomsFromCreatorRequest,
    },
};
use osmosis_test_tube::{
    cosmrs::proto::traits::Message, Account, ConcentratedLiquidity, Module, PoolManager,
    TokenFactory, Wasm,
};
use std::str::FromStr;

#[test]
fn fixture_default_works() {
    let (app, contract_address, cl_pool_id, admin, _deposit_ratio, _deposit_ratio_approx) =
        fixture_default(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);
    let cl = ConcentratedLiquidity::new(&app);
    let tf = TokenFactory::new(&app);
    let pm = PoolManager::new(&app);

    let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
    let vault_pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

    let resp = wasm
        .query::<QueryMsg, PoolResponse>(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                ClQueryMsg::Pool {},
            )),
        )
        .unwrap();

    assert_eq!(resp.pool_config.pool_id, vault_pool.id);
    assert_eq!(resp.pool_config.token0, vault_pool.token0);
    assert_eq!(resp.pool_config.token1, vault_pool.token1);

    let resp = wasm
        .query::<QueryMsg, VaultInfoResponse>(contract_address.as_str(), &QueryMsg::Info {})
        .unwrap();

    assert_eq!(resp.tokens, vec![vault_pool.token0, vault_pool.token1]);
    assert_eq!(
        resp.vault_token,
        tf.query_denoms_from_creator(&QueryDenomsFromCreatorRequest {
            creator: contract_address.to_string()
        })
        .unwrap()
        .denoms[0]
    );

    // Create Alice account
    let alice = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uosmo"),
            Coin::new(1_000_000_000_000, DENOM_BASE),
            Coin::new(1_000_000_000_000, DENOM_QUOTE),
        ])
        .unwrap();

    // Swap some funds as Alice to move the pool's curent tick
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

    // Increment the app time for twaps to function
    app.increase_time(1000000);

    // Update range of vault as Admin
    wasm.execute(
        contract_address.as_str(),
        &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
            lower_price: Decimal::from_str("0.993").unwrap(),
            upper_price: Decimal::from_str("1.002").unwrap(),
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
}
