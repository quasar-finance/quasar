use crate::setup::{
    fixture_default, fixture_dex_router, ACCOUNTS_INIT_BALANCE, DENOM_BASE,
    DENOM_QUOTE, MAX_SLIPPAGE_HIGH, PERFORMANCE_FEE_DEFAULT,
};
use cosmwasm_std::{Addr, Coin, Decimal, Uint128, Uint256};
use cl_vault::{
    msg::{
        AdminExtensionExecuteMsg, ClQueryMsg, ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg,
        QueryMsg,
    },
    query::{ActiveUsersResponse, VerifyTickCacheResponse},
};
use osmosis_test_tube::{Account, Module, Wasm};

#[test]
fn admin_build_tick_cache_works() {
    let (app, contract_address, _cl_pool_id, admin, _) = fixture_default(PERFORMANCE_FEE_DEFAULT);
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

#[test]
fn admin_execute_auto_claim_works() {
    let (app, contract_address, _, _cl_pool_id, _, admin, _) =
        fixture_dex_router(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);

    for _i in 1..10 {
        let accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, "uosmo"),
                ],
                2,
            )
            .unwrap();

        let amount_base = Uint128::new(10000);
        let amount_quote = Uint128::new(10000);
        let mut deposit_coins = vec![];
        if amount_base > Uint128::zero() {
            deposit_coins.push(Coin::new(amount_base.u128(), DENOM_BASE));
        }
        if amount_quote > Uint128::zero() {
            deposit_coins.push(Coin::new(amount_quote.u128(), DENOM_QUOTE));
        }
        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::AnyDeposit {
                    amount: amount_base,
                    asset: DENOM_BASE.to_string(),
                    recipient: Some(accounts[0].address()),
                    max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                },
                &deposit_coins,
                &accounts[0],
            )
            .unwrap();
    }

    let query_resp: ActiveUsersResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ActiveUsers {
                limit: 100,
                next_token: None,
            }),
        )
        .unwrap();

    assert!(!query_resp.users.is_empty(), "Expected users to be present");

    let users: Vec<(Addr, Uint256)> = query_resp
        .users
        .iter()
        .map(|user| {
            let addr = Addr::unchecked(user);
            (addr, Uint256::from(100u128))
        })
        .collect();

    let _ = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Admin(
                AdminExtensionExecuteMsg::AutoWithdraw { users: users.clone() },
            )),
            &[],
            &admin,
        )
        .unwrap();

    let updated_query_resp: ActiveUsersResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ActiveUsers {
                limit: 10,
                next_token: None,
            }),
        )
        .unwrap();

    for (addr, _) in users.clone() {
        assert!(
            updated_query_resp.users.contains(&addr.to_string()),
            "Expected user {} to be present in the active users list after auto claim",
            addr
        );
    }
}
