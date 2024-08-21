use crate::setup::{
    calculate_expected_refunds, fixture_dex_router, get_balance_amount,
    get_event_attributes_by_ty_and_key, ACCOUNTS_INIT_BALANCE, ACCOUNTS_NUM, DENOM_BASE,
    DENOM_QUOTE, DENOM_REWARD, DEPOSIT_AMOUNT, INITIAL_POSITION_BURN, PERFORMANCE_FEE_DEFAULT,
};

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::str::FromStr;

use cl_vault::msg::{
    ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, QueryMsg, SwapOperation,
    UserBalanceQueryMsg::UserSharesBalance,
};
use cl_vault::query::{
    AssetsBalanceResponse, TotalVaultTokenSupplyResponse, UserSharesBalanceResponse,
};
use cosmwasm_std::assert_approx_eq;
use cosmwasm_std::{Coin, Uint128};
use cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension;
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use osmosis_test_tube::{Account, Bank, Module, Wasm};

const DENOM_REWARD_AMOUNT: u128 = 100_000_000_000;
const APPROX_EQ_FACTOR: &str = "0.00005";

#[test]
fn test_autocompound_with_rewards_swap_non_vault_funds() {
    let (
        app,
        contract_address,
        _dex_router_addr,
        _vault_pool_id,
        swap_pools_ids,
        admin,
        deposit_ratio_base,
    ) = fixture_dex_router(PERFORMANCE_FEE_DEFAULT);
    let bm = Bank::new(&app);
    let wasm = Wasm::new(&app);

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

    let initial_total_vault_token_supply: TotalVaultTokenSupplyResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::TotalVaultTokenSupply {},
        )
        .unwrap();
    assert_eq!(
        INITIAL_POSITION_BURN.mul(2u128),
        initial_total_vault_token_supply.total.u128()
    );

    let shares_underlying_assets: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::ConvertToAssets {
                amount: initial_total_vault_token_supply.total,
            },
        )
        .unwrap();
    assert_eq!(
        Uint128::new(INITIAL_POSITION_BURN),
        shares_underlying_assets.balances[0].amount
    );
    assert_eq!(
        Uint128::new(INITIAL_POSITION_BURN),
        shares_underlying_assets.balances[1].amount
    );

    // BEFORE USERS DEPOSITS

    let mut refund0_amount_total = Uint128::zero();
    let mut refund1_amount_total = Uint128::zero();
    let mut total_minted_shares_from_deposits = Uint128::zero();

    let initial_base_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
    let initial_quote_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());

    // Execute exact_deposit for each account using the same amount of tokens for each asset and user
    for account in &accounts {
        let balance_user_shares_before: UserSharesBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &VaultExtension(ExtensionQueryMsg::Balances(UserSharesBalance {
                    user: account.address(),
                })),
            )
            .unwrap();
        // Assert user starts with 0 shares
        assert!(balance_user_shares_before.balance.is_zero());

        let exact_deposit = wasm
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

        let (expected_refund0, expected_refund1) =
            calculate_expected_refunds(DEPOSIT_AMOUNT, DEPOSIT_AMOUNT, deposit_ratio_base);

        // Assert balance refunded is either the expected value or not empty for token0
        let refund0_amount =
            get_event_attributes_by_ty_and_key(&exact_deposit, "wasm", vec!["refund0"]);
        let mut refund0_amount_parsed: u128 = 0;
        if expected_refund0 > 0 {
            assert_approx_eq!(
                Uint128::from_str(refund0_amount[0].value.as_str())
                    .unwrap()
                    .u128(),
                expected_refund0,
                APPROX_EQ_FACTOR
            );
            refund0_amount_parsed = refund0_amount[0].value.parse::<u128>().unwrap();
            refund0_amount_total = refund0_amount_total
                .checked_add(Uint128::new(refund0_amount_parsed))
                .unwrap();
        } else {
            assert!(refund0_amount.is_empty());
        }

        // Assert balance refunded is either the expected value or not empty for token1
        let refund1_amount =
            get_event_attributes_by_ty_and_key(&exact_deposit, "wasm", vec!["refund1"]);
        let mut refund1_amount_parsed: u128 = 0;
        if expected_refund1 > 0 {
            assert_eq!(
                Uint128::from_str(refund1_amount[0].value.as_str())
                    .unwrap()
                    .u128(),
                expected_refund1
            );
            refund1_amount_parsed = refund1_amount[0].value.parse::<u128>().unwrap();
            refund1_amount_total = refund1_amount_total
                .checked_add(Uint128::new(refund1_amount_parsed))
                .unwrap();
        } else {
            assert!(refund1_amount.is_empty());
        }

        // Assert after shares balance for the user accoridngly to the refunded amounts
        let balance_user_shares_after: UserSharesBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &VaultExtension(ExtensionQueryMsg::Balances(UserSharesBalance {
                    user: account.address(),
                })),
            )
            .unwrap();
        assert_eq!(
            balance_user_shares_after.balance,
            Uint128::new(
                DEPOSIT_AMOUNT
                    .mul(2u128)
                    .sub(refund0_amount_parsed)
                    .sub(refund1_amount_parsed)
            )
        );
        total_minted_shares_from_deposits = total_minted_shares_from_deposits
            .checked_add(balance_user_shares_after.balance)
            .unwrap();
    }

    // AFTER DEPOSITS CHECKS

    let after_deposit_vault_token_supply: TotalVaultTokenSupplyResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::TotalVaultTokenSupply {},
        )
        .unwrap();

    // Assert that the current vault token supply is equal to the initial plus each supply mint obtained from each deposit
    assert_eq!(
        total_minted_shares_from_deposits
            .checked_add(initial_total_vault_token_supply.total)
            .unwrap(),
        after_deposit_vault_token_supply.total
    );

    let after_deposit_total_assets: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::ConvertToAssets {
                amount: after_deposit_vault_token_supply.total,
            },
        )
        .unwrap();

    let users_total_deposit_per_asset = DEPOSIT_AMOUNT.checked_mul(ACCOUNTS_NUM as u128).unwrap();

    // Assert that the total vault shares are consistent with refunded amounts and initial burnt shares assets
    assert_eq!(
        users_total_deposit_per_asset
            .sub(refund0_amount_total.u128())
            .add(INITIAL_POSITION_BURN),
        after_deposit_total_assets.balances[0].amount.u128()
    );
    assert_eq!(
        users_total_deposit_per_asset
            .sub(refund1_amount_total.u128())
            .add(INITIAL_POSITION_BURN),
        after_deposit_total_assets.balances[1].amount.u128()
    );

    // Asssert that contract balances for base and quote denoms are consistent with the amount of funds deposited and refunded by users
    let expected_after_deposit_base_balance = users_total_deposit_per_asset
        .checked_add(initial_base_balance)
        .unwrap()
        .checked_sub(refund0_amount_total.u128())
        .unwrap();
    let after_deposit_base_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
    assert_eq!(
        expected_after_deposit_base_balance.to_string(),
        after_deposit_base_balance.to_string()
    );
    let expected_after_deposit_quote_balance = users_total_deposit_per_asset
        .checked_add(initial_quote_balance)
        .unwrap()
        .checked_sub(refund1_amount_total.u128())
        .unwrap();
    let after_deposit_quote_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
    assert_eq!(
        expected_after_deposit_quote_balance.to_string(),
        after_deposit_quote_balance.to_string()
    );

    // Airdrop some DENOM_REWARD funds to the contract that are not token0 nor token1 from vault position
    bm.send(
        MsgSend {
            from_address: admin.address(),
            to_address: contract_address.to_string(),
            amount: vec![OsmoCoin {
                denom: DENOM_REWARD.to_string(),
                amount: DENOM_REWARD_AMOUNT.to_string(),
            }],
        },
        &admin,
    )
    .unwrap();
    let initial_rewards_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_REWARD.to_string());
    assert_eq!(
        DENOM_REWARD_AMOUNT.to_string(),
        initial_rewards_balance.to_string(),
    );

    // SWAP NON VAULT ASSETS BEFORE AUTOCOMPOUND ASSETS

    let swap_token_in_amount = Uint128::new(DENOM_REWARD_AMOUNT)
        .checked_div(Uint128::new(2))
        .unwrap();

    // Calculate the total rewards swap amount using 0.01 as swap_fee as osmosis_test_tube hardcodes it this way: https://github.com/osmosis-labs/test-tube/blob/main/packages/osmosis-test-tube/src/module/gamm.rs#L59
    const BALANCER_FEE_RATE: f64 = 0.01;
    const INITIAL_AMOUNT: f64 = 1_000_000.0;
    let adjusted_numerator = ((1.0 - BALANCER_FEE_RATE) * INITIAL_AMOUNT) as u128;
    let denominator = 1_000_000u128;
    // we work with the assumption that the spot price is always 1.0 for both assets
    let swap_token_out_amount = swap_token_in_amount
        .checked_multiply_ratio(adjusted_numerator, denominator)
        .expect("Multiplication overflow");

    // We want to swap DENOM_REWARDS and pass the SwapOperation information to perform the swap using the dex-router
    wasm.execute(
        contract_address.as_str(),
        &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::SwapNonVaultFunds {
            swap_operations: vec![SwapOperation {
                token_in_denom: DENOM_REWARD.to_string(),
                pool_id_base: swap_pools_ids[1],
                pool_id_quote: swap_pools_ids[2],
                forced_swap_route_base: Some(vec![
                    SwapAmountInRoute {
                        pool_id: swap_pools_ids[2],
                        token_out_denom: DENOM_QUOTE.to_string(),
                    },
                    SwapAmountInRoute {
                        pool_id: swap_pools_ids[1],
                        token_out_denom: DENOM_BASE.to_string(),
                    },
                ]),
                forced_swap_route_quote: Some(vec![SwapAmountInRoute {
                    pool_id: swap_pools_ids[2],
                    token_out_denom: DENOM_QUOTE.to_string(),
                }]),
            }],
            twap_window_seconds: None,
        }),
        &[],
        &admin,
    )
    .unwrap();

    let after_swap_rewards_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_REWARD.to_string());
    assert_eq!(0u128, after_swap_rewards_balance);

    let after_swap_base_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
    // Assert vault position tokens balances increased accordingly to the swapped funds from DENOM_REWARD to DENOM_BASE and DENOM_QUOTE
    assert_eq!(
        after_deposit_base_balance
            .checked_add(swap_token_out_amount.into())
            .unwrap(),
        after_swap_base_balance
    );
    let after_swap_quote_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
    assert_eq!(
        after_deposit_quote_balance
            .checked_add(swap_token_out_amount.into())
            .unwrap(),
        after_swap_quote_balance
    );

    // Query contract to convert the same amount of LP token supply into assets after swapping non vault funds
    let after_swap_total_assets: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::ConvertToAssets {
                amount: after_deposit_vault_token_supply.total,
            },
        )
        .unwrap();

    // Check shares value of underlying assets increased after swapping non vault funds
    assert_eq!(
        users_total_deposit_per_asset
            .sub(refund0_amount_total.u128())
            .add(INITIAL_POSITION_BURN)
            .add(swap_token_out_amount.u128()),
        after_swap_total_assets.balances[0].amount.u128(),
    );
    assert_eq!(
        users_total_deposit_per_asset
            .sub(refund1_amount_total.u128())
            .add(INITIAL_POSITION_BURN)
            .add(swap_token_out_amount.u128()),
        after_swap_total_assets.balances[1].amount.u128(),
    );

    // AUTOCOMPOUND CONTRACT BALANCE ASSETS INTO POSITION

    wasm.execute(
        contract_address.as_str(),
        &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Autocompound {}),
        &[],
        &admin,
    )
    .unwrap();

    let after_autocompound_base_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
    let after_autocompound_quote_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
    assert!(
        after_autocompound_base_balance == 0u128 || after_autocompound_quote_balance == 0u128,
        "Either one of the balances must be 0, or both must be 0. Base balance: {}, Quote balance: {}",
        after_autocompound_base_balance,
        after_autocompound_quote_balance
    );

    let after_autocompound_vault_token_supply: TotalVaultTokenSupplyResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::TotalVaultTokenSupply {},
        )
        .unwrap();
    // Assert that total existing LP tokens didnt change after autocompound,
    // so we ensure that we just increase the vlaue of underlying assets for the same existing number of shares
    assert_eq!(
        after_deposit_vault_token_supply.total,
        after_autocompound_vault_token_supply.total
    );
    // Assert again, but with previously tracked values to ensure the autocompound worked as expected
    // We expect this to be the exact same amount of total shares of before autocompunding.
    assert_eq!(
        total_minted_shares_from_deposits
            .checked_add(initial_total_vault_token_supply.total)
            .unwrap(),
        after_autocompound_vault_token_supply.total
    );

    let after_autocompound_total_assets: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::ConvertToAssets {
                amount: after_autocompound_vault_token_supply.total,
            },
        )
        .unwrap();

    // Redeem all shares for each user and assert things accordingly
    for account in &accounts {
        let before_withdraw_base_balance =
            get_balance_amount(&app, account.address().to_string(), DENOM_BASE.to_string());
        let before_withdraw_quote_balance =
            get_balance_amount(&app, account.address().to_string(), DENOM_QUOTE.to_string());

        // Get shares balance for current account
        let shares_to_redeem: UserSharesBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &VaultExtension(ExtensionQueryMsg::Balances(UserSharesBalance {
                    user: account.address(),
                })),
            )
            .unwrap();

        if !shares_to_redeem.balance.is_zero() {
            wasm.execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: shares_to_redeem.balance,
                },
                &[],
                account,
            )
            .unwrap();

            // Assert after balances expecting the withdrawn amount
            // includes the compounded funds and idle funds from the vault position and balance
            let after_withdraw_base_balance =
                get_balance_amount(&app, account.address().to_string(), DENOM_BASE.to_string());
            assert_approx_eq!(
                after_withdraw_base_balance
                    .checked_sub(before_withdraw_base_balance)
                    .unwrap(),
                after_autocompound_total_assets.balances[0]
                    .amount
                    .u128()
                    .div(ACCOUNTS_NUM as u128),
                APPROX_EQ_FACTOR
            );
            let after_withdraw_quote_balance =
                get_balance_amount(&app, account.address().to_string(), DENOM_QUOTE.to_string());
            assert_approx_eq!(
                after_withdraw_quote_balance
                    .checked_sub(before_withdraw_quote_balance)
                    .unwrap(),
                after_autocompound_total_assets.balances[1]
                    .amount
                    .u128()
                    .div(ACCOUNTS_NUM as u128),
                APPROX_EQ_FACTOR
            );
        } else {
            panic!("User has no shares to redeem")
        }
    }

    // Assert total vault shares after autocompounding tokens.
    let after_withdraw_vault_token_supply: TotalVaultTokenSupplyResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::TotalVaultTokenSupply {},
        )
        .unwrap();
    assert_eq!(
        INITIAL_POSITION_BURN.mul(2u128),
        after_withdraw_vault_token_supply.total.u128()
    );
}
