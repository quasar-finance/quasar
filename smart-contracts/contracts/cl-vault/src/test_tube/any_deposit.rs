#[cfg(test)]
mod tests {
    use crate::msg::UserBalanceQueryMsg::{UserAssetsBalance, UserSharesBalance};
    use crate::msg::{ExecuteMsg, ExtensionQueryMsg};
    use crate::query::{AssetsBalanceResponse, UserSharesBalanceResponse};
    use crate::test_tube::initialize::initialize::{
        fixture_default_less_slippage, ACCOUNTS_INIT_BALANCE, ACCOUNTS_NUM, DENOM_BASE,
        DENOM_QUOTE, DEPOSIT_AMOUNT,
    };
    use cosmwasm_std::{assert_approx_eq, Coin, Fraction};
    use cosmwasm_std::{Decimal, Uint128};
    use cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::SpotPriceRequest;
    use osmosis_test_tube::{Account, Module, PoolManager, Wasm};

    #[test]
    #[ignore]
    fn test_any_deposit() {
        let (app, contract_address, cl_pool_id, admin, _deposit_ratio) =
            fixture_default_less_slippage();

        let wasm = Wasm::new(&app);

        // Initialize accounts
        let accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                ACCOUNTS_NUM,
            )
            .unwrap();

        for account in &accounts {
            let pm = PoolManager::new(&app);
            let spot_price: Decimal = pm
                .query_spot_price(&SpotPriceRequest {
                    pool_id: cl_pool_id,
                    base_asset_denom: DENOM_BASE.to_string(),
                    quote_asset_denom: DENOM_QUOTE.to_string(),
                })
                .unwrap()
                .spot_price
                .parse()
                .unwrap();

            // TODO: Why random? This is not a proptest.
            // let total0 = Uint128::new(random_number)
            //     .checked_add(
            //         Uint128::new(random_number)
            //             .multiply_ratio(spot_price.denominator(), spot_price.numerator()),
            //     )
            //     .unwrap();

            // let _ = wasm
            //     .execute(
            //         contract_address.as_str(),
            //         &ExecuteMsg::AnyDeposit {
            //             amount: Default::default(),
            //             asset: "".to_string(),
            //             recipient: None,
            //         },
            //         &[
            //             Coin::new(random_number, DENOM_BASE),
            //             Coin::new(random_number, DENOM_QUOTE),
            //         ],
            //         account,
            //     )
            //     .unwrap();

            let _result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Autocompound {}),
                    &[],
                    &admin,
                )
                .unwrap();

            // Get shares for Alice from vault contract and assert
            let shares: UserSharesBalanceResponse = wasm
                .query(
                    contract_address.as_str(),
                    &VaultExtension(ExtensionQueryMsg::Balances(UserSharesBalance {
                        user: account.address(),
                    })),
                )
                .unwrap();
            assert!(!shares.balance.is_zero());

            // Get shares for Alice from vault contract and assert
            let asset_balance: AssetsBalanceResponse = wasm
                .query(
                    contract_address.as_str(),
                    &VaultExtension(ExtensionQueryMsg::Balances(UserAssetsBalance {
                        user: account.address(),
                    })),
                )
                .unwrap();

            let spot_price: Decimal = pm
                .query_spot_price(&SpotPriceRequest {
                    base_asset_denom: DENOM_BASE.to_string(),
                    quote_asset_denom: DENOM_QUOTE.to_string(),
                    pool_id: cl_pool_id,
                })
                .unwrap()
                .spot_price
                .parse()
                .unwrap();

            let total1 = asset_balance.balances[0]
                .amount
                .checked_add(
                    asset_balance.balances[1]
                        .amount
                        .multiply_ratio(spot_price.denominator(), spot_price.numerator()),
                )
                .unwrap();

            // assert deposited assets in asset0 and
            // share value of assets in asset0 has very less difference
            // ideally : 0.0011482
            // assert_approx_eq!(total0, total1, "0.0012482");
        }
    }
}
