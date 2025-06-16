use crate::setup::{
    fixture_default, fixture_dex_router, ACCOUNTS_INIT_BALANCE, DENOM_BASE, DENOM_QUOTE,
    MAX_SLIPPAGE_HIGH, PERFORMANCE_FEE_DEFAULT,
};
use cl_vault::{
    msg::{
        AdminExtensionExecuteMsg, ClQueryMsg, ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg,
        QueryMsg,
    },
    query::{ActiveUsersResponse, VerifyTickCacheResponse},
};
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use osmosis_std::types::cosmos::bank::v1beta1::QueryBalanceRequest;
use osmosis_test_tube::{Account, Bank, Module, Wasm};

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

    let accounts = app
        .init_accounts(
            &[
                Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                Coin::new(ACCOUNTS_INIT_BALANCE, "uosmo"),
            ],
            10,
        )
        .unwrap();

    for account in accounts.iter().take(10) {
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
                    recipient: Some(account.address()),
                    max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                },
                &deposit_coins,
                account,
            )
            .unwrap();
    }

    // Query active users
    let query_resp: ActiveUsersResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Users {
                limit: 100,
                start_bound_exclusive: None,
            }),
        )
        .unwrap();

    assert!(!query_resp.users.is_empty(), "Expected users to be present");

    // Prepare users for auto claim
    let users: Vec<(Addr, Uint128)> = query_resp
        .users
        .iter()
        .map(|(addr, balance)| (addr.clone(), *balance)) // Keep as (String, Uint256)
        .collect();

    // Record user token balances before auto withdraw
    let bank = Bank::new(&app);
    let user_balances_before: Vec<(Addr, Coin, Coin)> = users
        .iter()
        .map(|(addr, _)| {
            let balance_base_resp = bank
                .query_balance(&QueryBalanceRequest {
                    address: addr.to_string(),
                    denom: DENOM_BASE.to_string(),
                })
                .unwrap();
            let balance_quote_resp = bank
                .query_balance(&QueryBalanceRequest {
                    address: addr.to_string(),
                    denom: DENOM_QUOTE.to_string(),
                })
                .unwrap();
            
            let balance_base = Coin::new(
                balance_base_resp.balance.as_ref().map_or(0, |b| b.amount.parse().unwrap_or(0)),
                DENOM_BASE,
            );
            let balance_quote = Coin::new(
                balance_quote_resp.balance.as_ref().map_or(0, |b| b.amount.parse().unwrap_or(0)),
                DENOM_QUOTE,
            );
            
            (addr.clone(), balance_base, balance_quote)
        })
        .collect();

    // Execute auto claim
    let _ = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Admin(
                AdminExtensionExecuteMsg::AutoWithdraw {
                    users: users
                        .clone()
                        .into_iter()
                        .map(|(u, a)| (u.to_string(), a))
                        .collect(),
                },
            )),
            &[],
            &admin,
        )
        .unwrap();

    // Query active users again to verify shares are zero
    let updated_query_resp: ActiveUsersResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Users {
                limit: 10,
                start_bound_exclusive: None,
            }),
        )
        .unwrap();

    // Verify shares are zeroed and users received their funds
    for (addr, balance_before_base, balance_before_quote) in &user_balances_before {
        // Check that shares are zero
        let user_shares = updated_query_resp
            .users
            .iter()
            .find(|(user_addr, _)| user_addr == addr);
        assert!(
            user_shares.is_none() || user_shares.unwrap().1.is_zero(),
            "Expected user {} to have zero shares after auto withdraw, but found {}",
            addr,
            user_shares
                .map(|(_, balance)| balance)
                .unwrap_or(&Uint128::zero())
        );

        // Check that users received their underlying tokens
        let balance_after_base_resp = bank
            .query_balance(&QueryBalanceRequest {
                address: addr.to_string(),
                denom: DENOM_BASE.to_string(),
            })
            .unwrap();
        let balance_after_quote_resp = bank
            .query_balance(&QueryBalanceRequest {
                address: addr.to_string(),
                denom: DENOM_QUOTE.to_string(),
            })
            .unwrap();
        
        let balance_after_base = Coin::new(
            balance_after_base_resp.balance.as_ref().map_or(0, |b| b.amount.parse().unwrap_or(0)),
            DENOM_BASE,
        );
        let balance_after_quote = Coin::new(
            balance_after_quote_resp.balance.as_ref().map_or(0, |b| b.amount.parse().unwrap_or(0)),
            DENOM_QUOTE,
        );

        assert!(
            balance_after_base.amount > balance_before_base.amount,
            "User {} should have received base tokens. Before: {}, After: {}",
            addr,
            balance_before_base.amount,
            balance_after_base.amount
        );
        assert!(
            balance_after_quote.amount > balance_before_quote.amount,
            "User {} should have received quote tokens. Before: {}, After: {}",
            addr,
            balance_before_quote.amount,
            balance_after_quote.amount
        );

        println!(
            "User {}: Base tokens {} -> {} (+{}), Quote tokens {} -> {} (+{})",
            addr,
            balance_before_base.amount,
            balance_after_base.amount,
            balance_after_base.amount.saturating_sub(balance_before_base.amount),
            balance_before_quote.amount,
            balance_after_quote.amount,
            balance_after_quote.amount.saturating_sub(balance_before_quote.amount)
        );
    }
}
