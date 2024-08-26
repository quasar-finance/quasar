use crate::setup::{
    fixture_default, fixture_dex_router, get_amount_from_denom, DENOM_BASE, DENOM_QUOTE,
    PERFORMANCE_FEE_DEFAULT,
};

use cl_vault::{
    msg::{
        AuthzExtension, ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, QueryMsg,
        UserBalanceQueryMsg,
    },
    query::UserSharesBalanceResponse,
};
use cosmwasm_std::{assert_approx_eq, Coin, Decimal};
use osmosis_test_tube::{Account, Module, Wasm};

const INITIAL_BALANCE_AMOUNT: u128 = 1_000_000_000_000_000_000_000_000_000_000;

// check that the authz interface returns the exact same response as
// the regular interface. Thus the actual authz functionality is out of
// scope but contract functionality is in scope here

#[test]
fn exact_deposit_withdraw_equal() {
    let (app, contract_address, ..) = fixture_dex_router(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);

    let alice = app
        .init_account(&[
            Coin::new(INITIAL_BALANCE_AMOUNT, "uosmo"),
            Coin::new(INITIAL_BALANCE_AMOUNT, DENOM_BASE),
            Coin::new(INITIAL_BALANCE_AMOUNT, DENOM_QUOTE),
        ])
        .unwrap();

    let deposit0 = 1_000_000_000_000_000;
    let deposit1 = 1_000_000_000_000_000;

    let deposit_response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::ExactDeposit { recipient: None },
            &[
                Coin::new(deposit0, DENOM_BASE),
                Coin::new(deposit1, DENOM_QUOTE),
            ],
            &alice,
        )
        .unwrap();

    let authz_deposit_response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Authz(
                AuthzExtension::ExactDeposit {},
            )),
            &[
                Coin::new(deposit0, DENOM_BASE),
                Coin::new(deposit1, DENOM_QUOTE),
            ],
            &alice,
        )
        .unwrap();

    assert_eq!(deposit_response.data, authz_deposit_response.data);

    assert_eq!(
        deposit_response.events.iter().find(|e| e.ty == *"wasm"),
        authz_deposit_response
            .events
            .iter()
            .find(|e| e.ty == *"wasm")
    );

    let shares: UserSharesBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                UserBalanceQueryMsg::UserSharesBalance {
                    user: alice.address(),
                },
            )),
        )
        .unwrap();
    assert!(!shares.balance.is_zero());

    let to_withdraw = shares.balance.u128() / 2;

    let withdraw_response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::Redeem {
                recipient: None,
                amount: to_withdraw.into(),
            },
            &[],
            &alice,
        )
        .unwrap();

    let authz_withdraw_response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::Redeem {
                recipient: None,
                amount: to_withdraw.into(),
            },
            &[],
            &alice,
        )
        .unwrap();

    assert_eq!(withdraw_response.data, authz_withdraw_response.data);
    assert_eq!(
        withdraw_response.events.iter().find(|e| e.ty == *"wasm"),
        authz_withdraw_response
            .events
            .iter()
            .find(|e| e.ty == *"wasm")
    );
}

#[test]
fn any_deposit_withdraw_equal() {
    let (app, contract_address, ..) = fixture_default(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);

    // Create Alice account
    let alice = app
        .init_account(&[
            Coin::new(INITIAL_BALANCE_AMOUNT, "uosmo"),
            Coin::new(INITIAL_BALANCE_AMOUNT, DENOM_BASE),
        ])
        .unwrap();

    let deposit0 = 1_000_000_000_000_000;

    // Deposit
    let deposit_response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::AnyDeposit {
                amount: deposit0.into(),
                asset: DENOM_BASE.to_string(),
                recipient: None,
                max_slippage: Decimal::bps(900),
            },
            &[Coin::new(deposit0, DENOM_BASE)],
            &alice,
        )
        .unwrap();
    // Deposit via AuthZ
    let authz_deposit_response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Authz(AuthzExtension::AnyDeposit {
                max_slippage: Decimal::bps(900),
            })),
            &[Coin::new(deposit0, DENOM_BASE)],
            &alice,
        )
        .unwrap();

    // Assert deposits are equal
    assert_eq!(deposit_response.data, authz_deposit_response.data);
    let deposit_event = deposit_response
        .events
        .iter()
        .find(|e| e.ty == *"wasm")
        .unwrap();
    let authz_deposit_event = authz_deposit_response
        .events
        .iter()
        .find(|e| e.ty == *"wasm")
        .unwrap();
    // Assert events are equal
    assert_eq!(deposit_event.ty, authz_deposit_event.ty);
    assert_eq!(
        deposit_event.attributes.len(),
        authz_deposit_event.attributes.len()
    );
    // We need to assert approx here as in the any_deposit case,
    // depositing involve a swap that changes the pool's condition for the subsequent one.
    for (attr1, attr2) in deposit_event
        .attributes
        .iter()
        .zip(&authz_deposit_event.attributes)
    {
        if attr1.key == "token_in" || attr1.key == "token_out_min_amount" {
            assert_approx_eq!(
                get_amount_from_denom(&attr1.value),
                get_amount_from_denom(&attr2.value),
                "0.00001"
            );
        } else {
            assert_eq!(attr1, attr2);
        }
    }

    // Check the shares and compute the balance to withdraw
    let shares: UserSharesBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                UserBalanceQueryMsg::UserSharesBalance {
                    user: alice.address(),
                },
            )),
        )
        .unwrap();
    assert!(!shares.balance.is_zero());
    let to_withdraw = shares.balance.u128() / 2;

    // Withdraw
    let withdraw_response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::Redeem {
                recipient: None,
                amount: to_withdraw.into(),
            },
            &[],
            &alice,
        )
        .unwrap();
    // Withdraw via AuthZ
    let authz_withdraw_response = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::Redeem {
                recipient: None,
                amount: to_withdraw.into(),
            },
            &[],
            &alice,
        )
        .unwrap();

    // Assert withdraws are equal
    assert_eq!(withdraw_response.data, authz_withdraw_response.data);
    let withdraw_event = withdraw_response
        .events
        .iter()
        .find(|e| e.ty == *"wasm")
        .unwrap();
    let authz_withdraw_event = authz_withdraw_response
        .events
        .iter()
        .find(|e| e.ty == *"wasm")
        .unwrap();
    // Assert events are equal
    assert_eq!(withdraw_event.ty, authz_withdraw_event.ty);
    assert_eq!(
        withdraw_event.attributes.len(),
        authz_withdraw_event.attributes.len()
    );
    // We need to assert approx here as in the any_deposit case,
    // depositing involve a swap that changes the pool's condition for the subsequent one.
    for (attr1, attr2) in withdraw_event
        .attributes
        .iter()
        .zip(&authz_withdraw_event.attributes)
    {
        if attr1.key == "liquidity_amount" {
            assert_approx_eq!(
                get_amount_from_denom(&attr1.value),
                get_amount_from_denom(&attr2.value),
                "0.000000000000000001"
            );
        } else {
            assert_eq!(attr1, attr2);
        }
    }
}
