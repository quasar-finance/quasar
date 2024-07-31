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
    ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, QueryMsg,
    UserBalanceQueryMsg::UserSharesBalance,
};
use cl_vault::query::{
    AssetsBalanceResponse, TotalVaultTokenSupplyResponse, UserSharesBalanceResponse,
};
use cl_vault::vault::swap::SwapOperation;
use cosmwasm_std::assert_approx_eq;
use cosmwasm_std::{Coin, Uint128};
use cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension;
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use osmosis_test_tube::{Account, Bank, Module, Wasm};

const DENOM_REWARD_AMOUNT: u128 = 100000000000;

#[test]
fn test_autocompound_with_rewards_swap_non_vault_funds() {
    let (
        app,
        contract_address,
        _dex_router_addr,
        _vault_pool_id,
        swap_pools_ids,
        admin,
        deposit_ratio,
        deposit_ratio_approx,
    ) = fixture_dex_router(PERFORMANCE_FEE_DEFAULT);
    let bm = Bank::new(&app);
    let wasm = Wasm::new(&app);

    // Initialize accounts as users
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

    // Assert that this is equal to the burnt amount of the initial position.
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

    // Get the initial worth of tokens of the initial vault total token supply
    // Assert that the underlying assets are the same amount we burnt during the first position creation in instantiation
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

    // Keep track of the total refunded amount on token0 from user deposits
    let mut refund0_amount_total = Uint128::zero();
    let mut refund1_amount_total = Uint128::zero();
    // Keep track of the total minted shares from deposits
    let mut total_minted_shares_from_deposits = Uint128::zero();

    // Get the balance of the contract before any user deposit
    // This will be useful after all deposits to assert the contract balance
    let initial_base_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
    let initial_quote_balance =
        get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());

    // Execute exact_deposit for each account using the same amount of tokens for each asset and user
    for account in &accounts {
        // Assert user starts with 0 shares
        let balance_user_shares_before: UserSharesBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &VaultExtension(ExtensionQueryMsg::Balances(UserSharesBalance {
                    user: account.address(),
                })),
            )
            .unwrap();
        assert!(balance_user_shares_before.balance.is_zero());

        // Make the deposit asserting the correct refund based on deposit ratio
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

        // Expected refund amounts based on the deposit ratio
        let (expected_refund0, expected_refund1) =
            calculate_expected_refunds(DEPOSIT_AMOUNT, DEPOSIT_AMOUNT, deposit_ratio);

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
                &deposit_ratio_approx
            );
            // Increment the refund0 amount total count for future math assertions
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
            assert_approx_eq!(
                Uint128::from_str(refund1_amount[0].value.as_str())
                    .unwrap()
                    .u128(),
                expected_refund1,
                &deposit_ratio_approx
            );
            // Increment the refund1 amount total count for future math assertions
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

        // Increment the total minted shares counter
        total_minted_shares_from_deposits = total_minted_shares_from_deposits
            .checked_add(balance_user_shares_after.balance)
            .unwrap();
    }

    // AFTER DEPOSITS CHECKS

    // Get total amount of vault token supply after the deposits of users
    // Assert that the current vault token supply is equal to the initial plus each supply mint obtained from each deposit
    let total_vault_token_supply_after_deposit: TotalVaultTokenSupplyResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::TotalVaultTokenSupply {},
        )
        .unwrap();
    assert_eq!(
        total_minted_shares_from_deposits
            .checked_add(initial_total_vault_token_supply.total)
            .unwrap(),
        total_vault_token_supply_after_deposit.total
    );

    // Get the worth of assets of the total current supply of vault tokens
    // Assert that the total vault shares are consistent with refunded amounts and initial burnt shares assets
    let shares_assets: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::ConvertToAssets {
                amount: total_vault_token_supply_after_deposit.total,
            },
        )
        .unwrap();
    let users_total_deposit_per_asset = DEPOSIT_AMOUNT.checked_mul(ACCOUNTS_NUM as u128).unwrap();
    assert_eq!(
        users_total_deposit_per_asset
            .sub(refund0_amount_total.u128())
            .add(INITIAL_POSITION_BURN),
        shares_assets.balances[0].amount.u128()
    );
    assert_eq!(
        users_total_deposit_per_asset
            .sub(refund1_amount_total.u128())
            .add(INITIAL_POSITION_BURN),
        shares_assets.balances[1].amount.u128()
    );

    // Get the current contract balance again after deposits
    // Asssert that contract balances for base and quote denoms are consistent with the amount of funds deposited and refunded by users
    // for base denom
    let expected_balance_base_after_deposit = users_total_deposit_per_asset
        .checked_add(initial_base_balance)
        .unwrap()
        .checked_sub(refund0_amount_total.u128())
        .unwrap();

    let contract_balance_base =
        get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
    assert_eq!(
        expected_balance_base_after_deposit.to_string(),
        contract_balance_base.to_string()
    );
    // for quote denom
    let expected_balance_quote_after_deposit = users_total_deposit_per_asset
        .checked_add(initial_quote_balance)
        .unwrap()
        .checked_sub(refund1_amount_total.u128())
        .unwrap();
    let contract_balance_quote =
        get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
    assert_eq!(
        expected_balance_quote_after_deposit.to_string(),
        contract_balance_quote.to_string()
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

    // Assert contract balance about the just airdropped non vault token funds
    let contract_balance_rewards =
        get_balance_amount(&app, contract_address.to_string(), DENOM_REWARD.to_string());
    assert_eq!(
        DENOM_REWARD_AMOUNT.to_string(),
        contract_balance_rewards.to_string(),
    );

    // SWAP NON VAULT ASSETS BEFORE AUTOCOMPOUND ASSETS

    // Calculate the numerator and denominator for the deposit_ratio obtained by the fixture
    // This allows to know the current position balance so we can predict how many token should be swapped into base, and how much into quote
    // Consider that here we are hardcoding a 0.01 fee which is the one Balancer pool appplies over the swap, no slippage taken in account by that tho.
    let numerator = ((1.0 - 0.01) * 1_000_000.0) as u128;
    let denominator = 1_000_000u128;

    let expected_amount_rewards_to_both_assets = Uint128::new(DENOM_REWARD_AMOUNT)
        .checked_multiply_ratio(numerator, denominator)
        .expect("Multiplication overflow");

    // Split based on current position balance
    let deposit_ratio_quote = 1.0 - deposit_ratio;
    let expected_amount_rewards_to_base = Uint128::from(
        (expected_amount_rewards_to_both_assets.u128() as f64 * deposit_ratio) as u128,
    );
    let expected_amount_rewards_to_quote = Uint128::from(
        (expected_amount_rewards_to_both_assets.u128() as f64 * deposit_ratio_quote) as u128,
    );

    // Execute the swap non vault funds
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

    // Assert there is no balance for DENOM_REWARD anymore after we swapped
    let balances_after_swap_rewards =
        get_balance_amount(&app, contract_address.to_string(), DENOM_REWARD.to_string());
    assert_eq!(
        expected_amount_rewards_to_both_assets
            .checked_sub(expected_amount_rewards_to_base)
            .unwrap()
            .checked_sub(expected_amount_rewards_to_quote)
            .unwrap()
            .u128(),
        balances_after_swap_rewards
    );

    // Get the contract balances for base and quote denoms after the swap of non vault funds
    // Assert vault position tokens balances increased accordingly to the swapped funds from DENOM_REWARD to DENOM_BASE and DENOM_QUOTE
    let balances_after_swap_base =
        get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
    // assert_approx_eq!(
    //     expected_balance_base_after_deposit
    //         .checked_add(expected_amount_rewards_to_base.into())
    //         .unwrap(),
    //     balances_after_swap_base,
    //     &deposit_ratio_approx
    // );
    let balances_after_swap_quote =
        get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
    // assert_approx_eq!(
    //     expected_balance_quote_after_deposit
    //         .checked_add(expected_amount_rewards_to_quote.into())
    //         .unwrap(),
    //     balances_after_swap_quote,
    //     &deposit_ratio_approx
    // );

    // Query contract to convert the same amount of LP token supply into assets after swapping non vault funds
    let shares_assets: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::ConvertToAssets {
                amount: total_vault_token_supply_after_deposit.total,
            },
        )
        .unwrap();

    // Check shares value of underlying assets increased after swapping non vault funds
    // assert_approx_eq!(
    //     users_total_deposit_per_asset
    //         .sub(refund0_amount_total.u128())
    //         .add(INITIAL_POSITION_BURN)
    //         .add(expected_amount_rewards_to_base.u128()),
    //     shares_assets.balances[0].amount.u128(),
    //     &deposit_ratio_approx
    // );
    // assert_approx_eq!(
    //     users_total_deposit_per_asset
    //         .sub(refund1_amount_total.u128())
    //         .add(INITIAL_POSITION_BURN)
    //         .add(expected_amount_rewards_to_quote.u128()),
    //     shares_assets.balances[1].amount.u128(),
    //     &deposit_ratio_approx
    // );

    // AUTOCOMPOUND CONTRACT BALANCE ASSETS INTO POSITION

    wasm.execute(
        contract_address.as_str(),
        &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Autocompound {}),
        &[],
        &admin,
    )
    .unwrap();

    // Get contract balances after the autocompound
    // Assert there are no funds left in the contract after autocompound
    let contract_balance_after_autocompound_base =
        get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
    let contract_balance_after_autocompound_quote =
        get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
    // assert_approx_eq!(
    //     contract_balance_after_autocompound_base,
    //     0u128,
    //     &deposit_ratio_approx
    // );
    // assert_approx_eq!(
    //     contract_balance_after_autocompound_quote,
    //     0u128,
    //     &deposit_ratio_approx
    // );

    // Get again the total vault token supply after autocompound
    // Assert that total existing LP tokens didnt change after autocompound,
    // so we ensure that we just increase the vlaue of underlying assets for the same existing number of shares
    let total_vault_token_supply_after_autocompound: TotalVaultTokenSupplyResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::TotalVaultTokenSupply {},
        )
        .unwrap();
    assert_eq!(
        total_vault_token_supply_after_deposit.total,
        total_vault_token_supply_after_autocompound.total
    );
    // Assert again, but with previously tracked values to ensure the autocompound worked as expected
    // We expect this to be the exact same amount of total shares of before autocompunding.
    assert_eq!(
        total_minted_shares_from_deposits
            .checked_add(initial_total_vault_token_supply.total)
            .unwrap(),
        total_vault_token_supply_after_autocompound.total
    );

    // Query contract to convert all LP token supply into assets after autocompound
    let shares_assets: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::ConvertToAssets {
                amount: total_vault_token_supply_after_autocompound.total,
            },
        )
        .unwrap();

    // Redeem all shares for each user and assert things accordingly
    for account in &accounts {
        // Get balances before for current account
        let balances_before_withdraw_base_denom =
            get_balance_amount(&app, account.address().to_string(), DENOM_BASE.to_string());
        let balances_before_withdraw_quote_denom =
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

        // If the current account have some share to redeem
        if !shares_to_redeem.balance.is_zero() {
            // Redeem all shares_to_redeem.balance
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
            // Base
            let balances_after_withdraw_base_denom =
                get_balance_amount(&app, account.address().to_string(), DENOM_BASE.to_string());
            assert_approx_eq!(
                balances_after_withdraw_base_denom
                    .checked_sub(balances_before_withdraw_base_denom)
                    .unwrap(),
                shares_assets.balances[0]
                    .amount
                    .u128()
                    .div(ACCOUNTS_NUM as u128),
                &deposit_ratio_approx
            );
            // Quote
            let balances_after_withdraw_quote_denom =
                get_balance_amount(&app, account.address().to_string(), DENOM_QUOTE.to_string());
            assert_approx_eq!(
                balances_after_withdraw_quote_denom
                    .checked_sub(balances_before_withdraw_quote_denom)
                    .unwrap(),
                shares_assets.balances[1]
                    .amount
                    .u128()
                    .div(ACCOUNTS_NUM as u128),
                &deposit_ratio_approx
            );
        } else {
            panic!("User has no shares to redeem")
        }
    }

    // Assert total vault shares after autocompounding tokens.
    let total_vault_token_supply_after_users_redeem: TotalVaultTokenSupplyResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::TotalVaultTokenSupply {},
        )
        .unwrap();
    assert_eq!(
        INITIAL_POSITION_BURN.mul(2u128),
        total_vault_token_supply_after_users_redeem.total.u128()
    );
}
