#[cfg(test)]
mod tests {
    use osmosis_test_tube::{Module, Wasm};

    use crate::{
        query::VerifyTickCacheResponse, test_tube::initialize::initialize::fixture_default,
    };

    #[test]
    #[ignore]
    fn range_admin_update_works() {
        let (_app, _contract_address, _cl_pool_id, _admin, _deposit_ratio) = fixture_default();
        // change the range admin and verify that it works
    }

    #[test]
    #[ignore]
    fn admin_build_tick_cache_works() {
        let (app, contract_address, _cl_pool_id, admin, _deposit_ratio) = fixture_default();
        let wasm = Wasm::new(&app);

        // When we will implement this entrypoint, if we do, purge it first

        // Execute build cache
        let build_resp = wasm
            .execute(
                contract_address.as_str(),
                &crate::msg::ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Admin(
                    crate::msg::AdminExtensionExecuteMsg::BuildTickCache {},
                )),
                &[],
                &admin,
            )
            .unwrap();
        // Check if the response contains the expected event
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
                &crate::msg::QueryMsg::VaultExtension(
                    crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                        crate::msg::ClQueryMsg::VerifyTickCache {},
                    ),
                ),
            )
            .unwrap();
        assert_eq!((), verify_resp.result.unwrap());
    }
}
