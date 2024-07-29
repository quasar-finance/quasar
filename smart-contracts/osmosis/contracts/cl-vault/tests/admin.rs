#![cfg(feature = "test-tube")]

mod setup;
use setup::{fixture_default, PERFORMANCE_FEE_DEFAULT};

use cl_vault::{
    msg::{
        AdminExtensionExecuteMsg, ClQueryMsg, ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg,
        QueryMsg,
    },
    query::VerifyTickCacheResponse,
};
use osmosis_test_tube::{Module, Wasm};

#[test]
fn admin_build_tick_cache_works() {
    let (app, contract_address, _cl_pool_id, admin, _deposit_ratio, _deposit_ratio_approx) =
        fixture_default(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);

    let build_resp = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Admin(
                AdminExtensionExecuteMsg::BuildTickCache {},
            )),
            &[],
            &admin,
        )
        .unwrap();
    let has_expected_event = build_resp.events.iter().any(|event| {
        event.ty == "wasm"
            && event
                .attributes
                .iter()
                .any(|attr| attr.key == "action" && attr.value == "build_tick_exp_cache")
    });
    assert!(has_expected_event, "Expected event not found in build_resp");

    // Verify query and assert
    let verify_resp: VerifyTickCacheResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                ClQueryMsg::VerifyTickCache {},
            )),
        )
        .unwrap();
    assert!(verify_resp.result.is_ok());
}
