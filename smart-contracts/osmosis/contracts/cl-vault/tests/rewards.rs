#![cfg(feature = "test-tube")]

mod setup;
use setup::{
    fixture_default, get_amount_from_denom, get_balance_amount, get_event_attributes_by_ty_and_key,
    ACCOUNTS_INIT_BALANCE, ACCOUNTS_NUM, DENOM_BASE, DENOM_QUOTE, DEPOSIT_AMOUNT,
    PERFORMANCE_FEE_DEFAULT,
};

use cl_vault::msg::{ExecuteMsg, ExtensionExecuteMsg};
use cosmwasm_std::Coin;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
use osmosis_std::types::osmosis::poolmanager::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute};
use osmosis_test_tube::{Account, Module, PoolManager, Wasm};

const SWAPS_NUM: usize = 10;
const SWAPS_AMOUNT: &str = "100000000000000000";

const PERFORMANCE_FEE_ZERO: u64 = 0;
const PERFORMANCE_FEE_FULL: u64 = 100;

#[test]
fn test_collect_rewards_with_rewards_default_works() {
    collect_rewards_with_rewards(PERFORMANCE_FEE_DEFAULT);
}

#[test]
fn test_collect_rewards_with_rewards_zero_works() {
    collect_rewards_with_rewards(PERFORMANCE_FEE_ZERO);
}

#[test]
fn test_collect_rewards_with_rewards_full_works() {
    collect_rewards_with_rewards(PERFORMANCE_FEE_FULL);
}

fn collect_rewards_with_rewards(performance_fee: u64) {
    let (app, contract_address, cl_pool_id, admin, _deposit_ratio, _deposit_ratio_approx) =
        fixture_default(performance_fee);

    // Initialize accounts
    let utility_account = app
        .init_account(&[
            Coin::new(ACCOUNTS_INIT_BALANCE, "uosmo"),
            Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
            Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
        ])
        .unwrap();

    // Initialize accounts
    let accounts = app
        .init_accounts(
            &[
                Coin::new(ACCOUNTS_INIT_BALANCE, "uosmo"),
                Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
            ],
            ACCOUNTS_NUM,
        )
        .unwrap();

    // Depositing with users
    let wasm = Wasm::new(&app);
    for account in &accounts {
        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                    Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                ],
                account,
            )
            .unwrap();
    }

    // Swaps to generate spread rewards on previously created user positions
    for _ in 0..SWAPS_NUM {
        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: admin.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: DENOM_BASE.to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: DENOM_QUOTE.to_string(),
                        amount: SWAPS_AMOUNT.to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &admin,
            )
            .unwrap();
    }

    // Before balances
    let balance_admin_before =
        get_balance_amount(&app, admin.address().to_string(), DENOM_QUOTE.to_string());
    let balance_contract_before =
        get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());

    // Collect Rewards
    let result = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::CollectRewards {}),
            &[],
            &utility_account,
        )
        .unwrap();

    // Extract 'tokens_out' attribute value for 'total_collect_spread_rewards'
    let tokens_out = get_event_attributes_by_ty_and_key(
        &result,
        "total_collect_spread_rewards",
        vec!["tokens_out"],
    );
    let tokens_out_u128 = get_amount_from_denom(&tokens_out[0].value);

    // Asserts distribution of claimed spread rewards
    assert_eq!(tokens_out_u128, 999u128); // Total

    // After balances
    let balance_admin_after =
        get_balance_amount(&app, admin.address().to_string(), DENOM_QUOTE.to_string());
    let balance_contract_after =
        get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());

    // Calculating the fee increment for the admin
    assert_eq!(
        balance_admin_after - balance_admin_before,
        (tokens_out_u128 * performance_fee as u128) / 100,
        "Admin fee calculation mismatch"
    );

    let final_balance = balance_contract_after - balance_contract_before;
    let expected_balance = ((tokens_out_u128 * (100 - performance_fee) as u128) / 100) + 1; // This +1 is needed due to some loss in precision

    // Calculate the difference between the actual and expected balance
    let balance_diff = final_balance as i128 - expected_balance as i128;

    // Assert that the balance difference is within the range of -1, 0, or 1
    assert!(
        (-1..=1).contains(&balance_diff),
        "Contract fee calculation mismatch: expected {}, got {}",
        expected_balance,
        final_balance
    );
}

#[test]
fn test_collect_rewards_no_rewards_works() {
    let (app, contract_address, _cl_pool_id, _admin, _deposit_ratio, _deposit_ratio_approx) =
        fixture_default(PERFORMANCE_FEE_DEFAULT);

    // Initialize accounts
    let accounts = app
        .init_accounts(
            &[
                Coin::new(ACCOUNTS_INIT_BALANCE, "uosmo"),
                Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
            ],
            ACCOUNTS_NUM,
        )
        .unwrap();

    // Depositing with users
    let wasm = Wasm::new(&app);
    for account in &accounts {
        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                    Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                ],
                account,
            )
            .unwrap();
    }

    // Declare claimer accounts
    let claimer = &accounts[0];

    // Collect and Distribute Rewards (there should be anything)
    let result = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::CollectRewards {}),
            &[],
            claimer,
        )
        .unwrap();
    // Extract 'tokens_out' attribute value for 'total_collect_incentives' and 'total_collect_spread_rewards'
    let tokens_out_incentives =
        get_event_attributes_by_ty_and_key(&result, "total_collect_incentives", vec!["tokens_out"]);
    let tokens_out_spread_rewards = get_event_attributes_by_ty_and_key(
        &result,
        "total_collect_spread_rewards",
        vec!["tokens_out"],
    );

    // Assert that 'tokens_out' values for both events are empty
    assert_eq!(tokens_out_incentives[0].value, "".to_string());
    assert_eq!(tokens_out_spread_rewards[0].value, "".to_string());
}
