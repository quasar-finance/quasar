#![cfg(feature = "test-tube")]

use crate::setup::{
    fixture_default, get_event_attributes_by_ty_and_key, ACCOUNTS_INIT_BALANCE, DENOM_BASE,
    DENOM_QUOTE, PERFORMANCE_FEE_DEFAULT,
};

use cl_vault::{
    msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg, UserBalanceQueryMsg},
    query::{AssetsBalanceResponse, TotalAssetsResponse, UserSharesBalanceResponse},
};
use cosmwasm_std::{assert_approx_eq, Coin, Uint128};
use osmosis_test_tube::{Account, Module, Wasm};

#[test]
fn single_deposit_withdraw_works() {
    let (app, contract_address, _cl_pool_id, _admin, _deposit_ratio, _deposit_ratio_approx) =
        fixture_default(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);

    let alice = app
        .init_account(&[
            Coin::new(ACCOUNTS_INIT_BALANCE, "uosmo"),
            Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
            Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
        ])
        .unwrap();

    let vault_assets_before: TotalAssetsResponse = wasm
        .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
        .unwrap();

    // Get user_assets for Alice from vault contract and assert
    let _user_assets: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                UserBalanceQueryMsg::UserAssetsBalance {
                    user: alice.address(),
                },
            )),
        )
        .unwrap();

    /*
    user:assets: AssetsBalanceResponse { balances: [Coin { 281243579389884 "uatom" }, Coin { 448554353093648 "uosmo" }] }
    1_000_000_000_000_000
    0_448_554_353_093_648
    0_281_243_579_389_884
    so these tokens could 2x easily
     */

    let deposit0 = 1_000_000_000_000_000;
    let deposit1 = 1_000_000_000_000_000;

    let response = wasm
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

    let _vault_assets_after: TotalAssetsResponse = wasm
        .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
        .unwrap();

    // assert that the refund + used funds are equal to what we deposited
    let refund0: u128 = get_event_attributes_by_ty_and_key(&response, "wasm", vec!["refund0"])
        .first()
        .map(|attr| attr.value.parse().unwrap())
        .unwrap_or(0);
    let refund1: u128 = get_event_attributes_by_ty_and_key(&response, "wasm", vec!["refund1"])
        .first()
        .map(|attr| attr.value.parse().unwrap())
        .unwrap_or(0);

    let deposited0: u128 = get_event_attributes_by_ty_and_key(&response, "wasm", vec!["amount0"])
        .first()
        .map(|attr| attr.value.parse().unwrap())
        .unwrap_or(0);
    let deposited1: u128 = get_event_attributes_by_ty_and_key(&response, "wasm", vec!["amount1"])
        .first()
        .map(|attr| attr.value.parse().unwrap())
        .unwrap_or(0);

    assert_eq!(
        deposit0 + deposit1,
        refund0 + refund1 + deposited0 + deposited1
    );

    // Get shares for Alice from vault contract and assert
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

    // TODO should we calc from shares or userAssetsBalance
    let user_value: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::ConvertToAssets {
                amount: shares.balance,
            },
        )
        .unwrap();

    assert_approx_eq!(
        user_value.balances[0].amount,
        Uint128::from(deposited0),
        "0.000001"
    );
    assert_approx_eq!(
        user_value.balances[1].amount,
        Uint128::from(deposited1),
        "0.000001"
    );

    // Get user_assets for Alice from vault contract and assert
    let user_assets: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                UserBalanceQueryMsg::UserAssetsBalance {
                    user: alice.address(),
                },
            )),
        )
        .unwrap();

    // assert the token0 deposited by alice by checking the balance of alice
    // we expect sent - refunded here, or 627_000_000_000_000
    // TODO, The UserAssetsBalance query here returns too little, so either we mint too little or the query works incorrect
    assert_approx_eq!(
        user_assets.balances[0].amount,
        Uint128::from(deposited0),
        "0.000001"
    );

    // assert the token1 deposited by alice
    assert_approx_eq!(
        user_assets.balances[1].amount,
        Uint128::from(deposited1),
        "0.000001"
    );

    // Get vault assets and assert
    let vault_assets: TotalAssetsResponse = wasm
        .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
        .unwrap();
    assert_approx_eq!(
        vault_assets.token0.amount,
        vault_assets_before
            .token0
            .amount
            .checked_add(Uint128::from(deposited0))
            .unwrap(),
        "0.000001"
    );

    // Assert vault assets taking in account the refunded amount to Alice, so we only expect around 500 to deposit here
    assert_approx_eq!(
        vault_assets.token1.amount,
        vault_assets_before
            .token1
            .amount
            .checked_add(Uint128::from(deposited1))
            .unwrap(),
        "0.000001"
    );

    let _withdraw = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::Redeem {
                recipient: None,
                amount: shares.balance,
            },
            &[],
            &alice,
        )
        .unwrap();

    // TODO: verify the correct execution
}

#[test]
fn multiple_deposit_withdraw_works() {
    let (app, contract_address, _cl_pool_id, _admin, _deposit_ratio, _deposit_ratio_approx) =
        fixture_default(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);

    // Create Alice account
    let alice = app
        .init_account(&[
            Coin::new(ACCOUNTS_INIT_BALANCE, "uosmo"),
            Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
            Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
        ])
        .unwrap();

    // Get vaults assets before doing anything for future assertions
    let vault_assets_before: TotalAssetsResponse = wasm
        .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
        .unwrap();

    // Loop 3 times to do multiple deposits as Alice
    for _ in 0..3 {
        wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::ExactDeposit { recipient: None },
            &[
                Coin::new(ACCOUNTS_INIT_BALANCE / 10, DENOM_BASE),
                Coin::new(ACCOUNTS_INIT_BALANCE / 10, DENOM_QUOTE),
            ],
            &alice,
        )
        .unwrap();
    }

    // Get Alice shares from vault contract
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

    // Get Alice assets from vault contract
    let user_assets: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                UserBalanceQueryMsg::UserAssetsBalance {
                    user: alice.address(),
                },
            )),
        )
        .unwrap();

    // deposit alice 3x 1_000_000_000_000_000_000. we should be close to 3*10^18 for the eth asset
    assert_approx_eq!(
        user_assets.balances[0].amount,
        Uint128::from(187_955_958_641_517u128), // TODO: remove hardcoded value
        "0.0000005"
    );
    // deposit alice 3x 1_000_000_000. we should be close to 3*10^9 for the osmo asset
    assert_approx_eq!(
        user_assets.balances[1].amount,
        Uint128::from(300_000_000_000_000u128), // TODO: remove hardcoded value
        "0.0000005"
    );

    let user_assets_again: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::ConvertToAssets {
                amount: shares.balance,
            },
        )
        .unwrap();
    assert_approx_eq!(
        user_assets_again.balances[0].amount,
        Uint128::from(187_955_958_641_517u128),
        "0.0000005"
    );
    assert_approx_eq!(
        user_assets_again.balances[1].amount,
        Uint128::from(300_000_000_000_000u128),
        "0.0000005"
    );

    let vault_assets: TotalAssetsResponse = wasm
        .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
        .unwrap();

    assert_approx_eq!(
        vault_assets.token0.amount,
        vault_assets_before
            .token0
            .amount
            .checked_add(Uint128::from(187_955_958_641_517u128))
            .unwrap(),
        "0.0000005"
    );
    // again we get refunded so we only expect around 500 to deposit here
    assert_approx_eq!(
        vault_assets.token1.amount,
        vault_assets_before
            .token1
            .amount
            .checked_add(Uint128::from(300_000_000_000_000u128))
            .unwrap(),
        "0.0000005"
    );

    let _withdraw = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::Redeem {
                recipient: None,
                amount: shares.balance,
            },
            &[],
            &alice,
        )
        .unwrap();
    // verify the correct execution
}

#[test]
fn multiple_deposit_withdraw_unused_funds_works() {
    let (app, contract_address, _cl_pool_id, _admin, _deposit_ratio, _deposit_ratio_approx) =
        fixture_default(PERFORMANCE_FEE_DEFAULT);
    let wasm = Wasm::new(&app);

    // Create 3 accounts
    let users = [
        app.init_account(&[
            Coin::new(100_000_000_000_000_000_000_000_000_000_000_000_000, "uosmo"),
            Coin::new(
                100_000_000_000_000_000_000_000_000_000_000_000_000,
                DENOM_BASE,
            ),
            Coin::new(
                100_000_000_000_000_000_000_000_000_000_000_000_000,
                DENOM_QUOTE,
            ),
        ])
        .unwrap(),
        app.init_account(&[
            Coin::new(100_000_000_000_000_000_000_000_000_000_000_000_000, "uosmo"),
            Coin::new(
                100_000_000_000_000_000_000_000_000_000_000_000_000,
                DENOM_BASE,
            ),
            Coin::new(
                100_000_000_000_000_000_000_000_000_000_000_000_000,
                DENOM_QUOTE,
            ),
        ])
        .unwrap(),
        app.init_account(&[
            Coin::new(100_000_000_000_000_000_000_000_000_000_000_000_000, "uosmo"),
            Coin::new(
                100_000_000_000_000_000_000_000_000_000_000_000_000,
                DENOM_BASE,
            ),
            Coin::new(
                100_000_000_000_000_000_000_000_000_000_000_000_000,
                DENOM_QUOTE,
            ),
        ])
        .unwrap(),
    ];

    // this is the max deposit amount before overflow -> 100_000_000 ETH (100_000_000_000_000_000_000_000_000 Wei)
    let deposit_amount: u128 = 100_000_000_000_000_000_000_000;

    // you can scale this up to 1000 and still not failing, which would be like: 3 users x 100_000_000 ETH x 1000 = 300_000_000_000 (300 B) total deposited ETHs in the vault
    for _ in 0..10 {
        // depositing
        for user in &users {
            wasm.execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(deposit_amount, DENOM_BASE),
                    Coin::new(deposit_amount, DENOM_QUOTE),
                ], // 1eth = 6k osmo
                user,
            )
            .unwrap();
        }
    }

    // querying shares and withdrawing

    for user in users {
        let user_shares: UserSharesBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    UserBalanceQueryMsg::UserSharesBalance {
                        user: user.address(),
                    },
                )),
            )
            .unwrap();

        wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::Redeem {
                recipient: None,
                amount: user_shares.balance,
            },
            &[],
            &user,
        )
        .unwrap();
    }
}
