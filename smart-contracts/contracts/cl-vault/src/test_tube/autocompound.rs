#[cfg(test)]
mod tests {
    use std::ops::Mul;
    use std::ops::Sub;

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
    use crate::test_tube::helpers::get_balance_amount;
    use crate::test_tube::helpers::get_event_attributes_by_ty_and_key;
    use crate::test_tube::initialize::initialize::INITIAL_POSITION_BURN;
    use crate::test_tube::initialize::initialize::{
        fixture_cw_dex_router, ACCOUNTS_INIT_BALANCE, ACCOUNTS_NUM, DENOM_BASE, DENOM_QUOTE,
        DENOM_REWARD, DEPOSIT_AMOUNT,
    };

    const DENOM_REWARD_AMOUNT: &str = "100000000000";

    #[test]
    #[ignore]
    fn test_autocompound_with_rewards() {
        let (app, contract_address, _dex_router_addr, pools_ids, admin) = fixture_cw_dex_router();
        let bm = Bank::new(&app);
        let wasm = Wasm::new(&app);

        // Initialize accounts as users
        let accounts = app
            .init_accounts(
                &[
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
        // TODO: Deprecate this as soon as the deposit debalance is fixed
        let mut refund0_amount_total = Uint128::zero();
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

            // Make the deposit
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

            // Assert balance refunded is not empty for token0
            let refund0_amount =
                get_event_attributes_by_ty_and_key(&exact_deposit, "wasm", vec!["refund0_amount"]);
            println!("refund0_amount per-user: {:?}", refund0_amount);
            assert!(!refund0_amount.is_empty());
            let refund_amount_parsed = refund0_amount[0].value.parse::<u128>().unwrap();
            println!("refund_amount_parsed per-user: {:?}", refund_amount_parsed);
            refund0_amount_total = refund0_amount_total
                .checked_add(Uint128::new(refund_amount_parsed))
                .unwrap();
            // Assert we have no token1 refund
            let refund1_amount =
                get_event_attributes_by_ty_and_key(&exact_deposit, "wasm", vec!["refund1_amount"]);
            assert!(refund1_amount.is_empty());

            // Assert after shares balance for the user accoridngly to the refunded amounts
            // TODO: This is minting 8132_600000 because of the 1867_400000 refund0_amount
            let balance_user_shares_after: UserSharesBalanceResponse = wasm
                .query(
                    contract_address.as_str(),
                    &VaultExtension(ExtensionQueryMsg::Balances(UserSharesBalance {
                        user: account.address(),
                    })),
                )
                .unwrap();
            println!("{}", INITIAL_POSITION_BURN.mul(2u128).to_string());
            assert_eq!(
                balance_user_shares_after.balance,
                Uint128::new(DEPOSIT_AMOUNT.mul(2u128).sub(refund_amount_parsed))
            ); // TODO: There should be NO REFUND here

            // Increment the total minted shares counter
            total_minted_shares_from_deposits = total_minted_shares_from_deposits
                .checked_add(balance_user_shares_after.balance)
                .unwrap();
        }

        // AFTER DEPOSITS CHECKS

        // Assert total vault shares
        let total_vault_token_supply: TotalVaultTokenSupplyResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::TotalVaultTokenSupply {},
            )
            .unwrap();
        assert_eq!(
            total_minted_shares_from_deposits
                .checked_add(initial_total_vault_token_supply.total)
                .unwrap(),
            total_vault_token_supply.total
        );

        // Assert shares underlying assets
        let shares_assets: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::ConvertToAssets {
                    amount: Uint128::new(1000u128),
                },
            )
            .unwrap();
        assert_eq!(Uint128::new(500), shares_assets.balances[0].amount);
        assert_eq!(Uint128::new(500), shares_assets.balances[1].amount);

        // Airdrop some DENOM_REWARD funds to the contract, this will be like idle claimed spread rewards
        let _send = bm
            .send(
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

        // declare expected contract balance after 10x user deposits
        let users_total_deposit_per_asset =
            DEPOSIT_AMOUNT.checked_mul(ACCOUNTS_NUM as u128).unwrap();
        let expected_balance_base_after_deposit = users_total_deposit_per_asset
            .checked_add(balance_before_contract_base)
            .unwrap()
            .checked_sub(refund0_amount_total.u128())
            .unwrap();

        // <assert balances
        let balances_rewards =
            get_balance_amount(&app, contract_address.to_string(), DENOM_REWARD.to_string());
        assert_eq!(
            DENOM_REWARD_AMOUNT.to_string(),
            balances_rewards.to_string(),
        );

        // We expect (10_000_000 uatom * 10users), so a total of 50.00 $ATOM - (balance_base_before  - refund0_amount_total) from expected_balance_base_after_deposit
        let balances_base =
            get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
        assert_eq!(
            expected_balance_base_after_deposit.to_string(),
            balances_base.to_string()
        );

        // We expect (10_000_000 uosmo * 10users) so a total of 50 $OSMO
        let balances_quote =
            get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
        assert_eq!(
            users_total_deposit_per_asset.to_string(),
            balances_quote.to_string()
        );

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
        let _swap_non_vault_funds = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::SwapNonVaultFunds {
                    force_swap_route: false,
                    swap_routes: vec![SwapAsset {
                        token_in_denom: DENOM_REWARD.to_string(),
                        recommended_swap_route_token_0: Option::from(
                            SwapOperationsListUnchecked::new(path1),
                        ),
                        recommended_swap_route_token_1: Option::from(
                            SwapOperationsListUnchecked::new(path2),
                        ),
                    }],
                }),
                &[],
                &admin,
            )
            .unwrap();
        // 50000000000ustride to 49500000000uatom
        // 50000000000ustride to 49500000000uosmo
        // TODO: assert on swap_non_vault_funds rsponse how much token_out against the previous balance
        // TODO: Log how many swap fees / price impact we incurred into so we can assert at the end of test the total vaults assets by shares among users

        // Assert there is no balance for DENOM_REWARD (ustrd) and there is more DENOM_BASE
        let balances_after_swap_rewards =
            get_balance_amount(&app, contract_address.to_string(), DENOM_REWARD.to_string());
        assert_eq!(0u128, balances_after_swap_rewards);
        let balances_after_swap_base =
            get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
        assert_eq!(
            expected_balance_base_after_deposit
                .checked_add(49500000000u128)
                .unwrap(),
            balances_after_swap_base
        );
        let balances_after_swap_quote =
            get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
        assert_eq!(
            50000000000u128.checked_add(49500000000u128).unwrap(),
            balances_after_swap_quote
        );

        // Assert shares underlying assets
        let shares_assets: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::ConvertToAssets {
                    amount: Uint128::new(1000u128),
                },
            )
            .unwrap();
        // TODO: Check this asserts COMPUTE THEM RATHER THAN MAGIC VALUES
        assert_eq!(Uint128::new(993), shares_assets.balances[0].amount);
        assert_eq!(Uint128::new(1223), shares_assets.balances[1].amount);

        let _autocompound_resp = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Autocompound {}),
                &[],
                &admin,
            )
            .unwrap();

        // Assert balances after AUTOCOMPOUND
        let balances_after_autocompound_base =
            get_balance_amount(&app, contract_address.to_string(), DENOM_BASE.to_string());
        assert_eq!(
            18511274090u128, // TODO: De hardcode this
            balances_after_autocompound_base
        );
        let balances_after_autocompound_quote =
            get_balance_amount(&app, contract_address.to_string(), DENOM_QUOTE.to_string());
        assert_eq!(0u128, balances_after_autocompound_quote);

        // Assert total vault shares after autocompounding tokens.
        // We expect this to be the exact same amount of total shares of before autocompunding.
        // Assert total vault shares
        let total_vault_token_supply: TotalVaultTokenSupplyResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::TotalVaultTokenSupply {},
            )
            .unwrap();
        assert_eq!(
            total_minted_shares_from_deposits
                .checked_add(initial_total_vault_token_supply.total)
                .unwrap(),
            total_vault_token_supply.total
        );

        // Assert shares underlying assets
        let shares_assets: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::ConvertToAssets {
                    amount: Uint128::new(1000u128),
                },
            )
            .unwrap();
        // TODO: Check this asserts COMPUTE THEM RATHER THAN MAGIC VALUES
        assert_eq!(Uint128::new(993), shares_assets.balances[0].amount);
        assert_eq!(Uint128::new(1223), shares_assets.balances[1].amount);

        // TODO: Check these More asserts
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

                // Assert after balances
                let balances_after_withdraw_base_denom =
                    get_balance_amount(&app, account.address().to_string(), DENOM_BASE.to_string());
                assert_eq!(
                    true,
                    balances_after_withdraw_base_denom
                        .checked_sub(balances_before_withdraw_base_denom)
                        .unwrap()
                        > DEPOSIT_AMOUNT
                );
                let balances_after_withdraw_quote_denom = get_balance_amount(
                    &app,
                    account.address().to_string(),
                    DENOM_QUOTE.to_string(),
                );
                assert_eq!(
                    true,
                    balances_after_withdraw_quote_denom
                        .checked_sub(balances_before_withdraw_quote_denom)
                        .unwrap()
                        > DEPOSIT_AMOUNT
                );
            }
        }

        // Assert total vault shares after autocompounding tokens.
        let total_vault_token_supply: TotalVaultTokenSupplyResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::TotalVaultTokenSupply {},
            )
            .unwrap();
        assert_eq!(Uint128::new(2000), total_vault_token_supply.total);

        // Assert shares underlying assets
        let shares_assets: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::ConvertToAssets {
                    amount: Uint128::new(1000u128),
                },
            )
            .unwrap();
        // TODO: Check this asserts COMPUTE THEM RATHER THAN MAGIC VALUES
        assert_eq!(Uint128::new(994), shares_assets.balances[0].amount);
        assert_eq!(Uint128::new(1223), shares_assets.balances[1].amount);
    }
}
