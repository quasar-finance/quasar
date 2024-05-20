#[cfg(test)]
mod tests {
    use std::ops::Add;
    use std::ops::Div;
    use std::ops::Mul;
    use std::ops::Sub;
    use std::str::FromStr;

    use apollo_cw_asset::AssetInfoBase;
    use cosmwasm_std::assert_approx_eq;
    use cosmwasm_std::{Coin, Uint128};
    use cw_dex::osmosis::OsmosisPool;
    use cw_dex_router::operations::SwapOperationBase;
    use cw_dex_router::operations::SwapOperationsListUnchecked;
    use cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension;
    use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use osmosis_test_tube::{Account, Bank, Module, Wasm};

    use crate::msg::QueryMsg;
    use crate::msg::SwapAsset;
    use crate::msg::UserBalanceQueryMsg::UserSharesBalance;
    use crate::msg::{ExecuteMsg, ExtensionQueryMsg};
    use crate::query::AssetsBalanceResponse;
    use crate::query::TotalVaultTokenSupplyResponse;
    use crate::query::UserSharesBalanceResponse;
    use crate::test_tube::helpers::calculate_expected_refunds;
    use crate::test_tube::helpers::get_balance_amount;
    use crate::test_tube::helpers::get_event_attributes_by_ty_and_key;
    use crate::test_tube::initialize::initialize::INITIAL_POSITION_BURN;
    use crate::test_tube::initialize::initialize::{
        fixture_cw_dex_router, ACCOUNTS_INIT_BALANCE, ACCOUNTS_NUM, DENOM_BASE, DENOM_QUOTE,
        DENOM_REWARD, DEPOSIT_AMOUNT,
    };

    const DENOM_REWARD_AMOUNT: u128 = 100000000000;

    #[test]
    #[ignore]
    fn test_autocompound_with_rewards() {
        let (
            app,
            contract_address,
            _dex_router_addr,
            pools_ids,
            admin,
            deposit_ratio,
            deposit_ratio_approx,
        ) = fixture_cw_dex_router();
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

        // AFTER INITIAL SETUP

        // Assert total vault shares are just INITIAL_POSITION_BURN * 2 accordingly to the initial setup
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

        // Assert shares underlying assets
        // Here we expect the total existing shares from initial tokens provided to represent the INITIAL_POSITION_BURN amount for each asset
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

        // BEFORE DEPOSITS CHECKS

        // This will be useful after all deposits to assert the contract balance
        let balance_before_contract_base =
            get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());

        // Keep track of the total refunded amount on token0 from user deposits
        let mut refund0_amount_total = Uint128::zero();
        let mut refund1_amount_total = Uint128::zero();

        // Keep track of the total minted shares from deposits
        let mut total_minted_shares_from_deposits = Uint128::zero();
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
                get_event_attributes_by_ty_and_key(&exact_deposit, "wasm", vec!["refund0_amount"]);
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
                get_event_attributes_by_ty_and_key(&exact_deposit, "wasm", vec!["refund1_amount"]);
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

        // Assert total vault shares
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

        // Assert shares underlying assets
        let shares_assets: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::ConvertToAssets {
                    amount: total_vault_token_supply_after_deposit.total,
                },
            )
            .unwrap();

        // declare expected contract balance after 10x user deposits
        let users_total_deposit_per_asset =
            DEPOSIT_AMOUNT.checked_mul(ACCOUNTS_NUM as u128).unwrap();

        // Assert that the total vault shares are consistent with refunded amounts and initial burnt shares assets
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

        // Asssert contract balances for base and quote denoms
        let expected_balance_base_after_deposit = users_total_deposit_per_asset
            .checked_add(balance_before_contract_base)
            .unwrap()
            .checked_sub(refund0_amount_total.u128())
            .unwrap();
        let balances_base =
            get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
        assert_eq!(
            expected_balance_base_after_deposit.to_string(),
            balances_base.to_string()
        );
        let balances_quote =
            get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
        assert_eq!(
            users_total_deposit_per_asset.to_string(),
            balances_quote.to_string()
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
        let balances_rewards =
            get_balance_amount(&app, contract_address.to_string(), DENOM_REWARD.to_string());
        assert_eq!(
            DENOM_REWARD_AMOUNT.to_string(),
            balances_rewards.to_string(),
        );

        // SWAP NON VAULT ASSETS BEFORE AUTOCOMPOUND ASSETS

        // Define CW Dex Router swap routes
        let path1 = vec![
            SwapOperationBase::new(
                cw_dex::Pool::Osmosis(OsmosisPool::unchecked(pools_ids[1])),
                AssetInfoBase::Native(DENOM_REWARD.to_string()),
                AssetInfoBase::Native(DENOM_QUOTE.to_string()),
            ),
            SwapOperationBase::new(
                cw_dex::Pool::Osmosis(OsmosisPool::unchecked(pools_ids[0])),
                AssetInfoBase::Native(DENOM_QUOTE.to_string()),
                AssetInfoBase::Native(DENOM_BASE.to_string()),
            ),
        ];
        let path2 = vec![SwapOperationBase::new(
            cw_dex::Pool::Osmosis(OsmosisPool::unchecked(pools_ids[1])),
            AssetInfoBase::Native(DENOM_REWARD.to_string()),
            AssetInfoBase::Native(DENOM_QUOTE.to_string()),
        )];

        // Swap non vault funds to vault funds
        // 50000000000ustride to 49500000000uatom as spot price 1.0 less swap_fees
        // 50000000000ustride to 49500000000uosmo as spot price 1.0 less swap_fees
        let expected_amount_rewards = 49500000000u128;

        wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::SwapNonVaultFunds {
                force_swap_route: false,
                swap_routes: vec![SwapAsset {
                    token_in_denom: DENOM_REWARD.to_string(),
                    recommended_swap_route_token_0: Option::from(SwapOperationsListUnchecked::new(
                        path1,
                    )),
                    recommended_swap_route_token_1: Option::from(SwapOperationsListUnchecked::new(
                        path2,
                    )),
                }],
            }),
            &[],
            &admin,
        )
        .unwrap();
        // Assert there is no balance for DENOM_REWARD (ustrd) and there is more DENOM_BASE
        let balances_after_swap_rewards =
            get_balance_amount(&app, contract_address.to_string(), DENOM_REWARD.to_string());
        assert_eq!(0u128, balances_after_swap_rewards);
        // Assert vault position tokens balances
        let balances_after_swap_base =
            get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
        assert_eq!(
            expected_balance_base_after_deposit
                .checked_add(expected_amount_rewards)
                .unwrap(),
            balances_after_swap_base
        );
        let balances_after_swap_quote =
            get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
        assert_eq!(
            DENOM_REWARD_AMOUNT
                .div(2)
                .checked_add(expected_amount_rewards)
                .unwrap(),
            balances_after_swap_quote
        );

        // Query contract to convert all LP token supply into assets after swapping non vault funds
        let shares_assets: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::ConvertToAssets {
                    amount: total_vault_token_supply_after_deposit.total,
                },
            )
            .unwrap();
        // Check shares value of underlying assets after swapping non vault funds
        assert_eq!(
            users_total_deposit_per_asset
                .sub(refund0_amount_total.u128())
                .add(INITIAL_POSITION_BURN)
                .add(expected_amount_rewards),
            shares_assets.balances[0].amount.u128()
        );
        assert_eq!(
            users_total_deposit_per_asset
                .sub(refund1_amount_total.u128())
                .add(INITIAL_POSITION_BURN)
                .add(expected_amount_rewards),
            shares_assets.balances[1].amount.u128()
        );

        // AUTOCOMPOUND CONTRCT BALANCE ASSETS INTO POSITION
        wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Autocompound {}),
            &[],
            &admin,
        )
        .unwrap();

        // Assert balances after autocompound of funds
        let (expected_refund_base, expected_refund_quote) =
            calculate_expected_refunds(49500000000, 49500000000, deposit_ratio);
        // Base about rewards swapped to token base
        let balances_after_autocompound_base =
            get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
        assert_approx_eq!(
            expected_refund_base,
            balances_after_autocompound_base,
            &deposit_ratio_approx
        );
        // Quote about rewards swapped to token quote
        let balances_after_autocompound_quote =
            get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
        assert_eq!(expected_refund_quote, balances_after_autocompound_quote);

        // Assert total vault shares
        let total_vault_token_supply_after_autocompound: TotalVaultTokenSupplyResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::TotalVaultTokenSupply {},
            )
            .unwrap();
        // Assert that total existing LP tokens didnt change after autocompound,
        // so we ensure that we just increase the vlaue of underlying assets for the same existing number of shares
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
                let balances_after_withdraw_quote_denom = get_balance_amount(
                    &app,
                    account.address().to_string(),
                    DENOM_QUOTE.to_string(),
                );
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
}
