#[cfg(test)]
mod tests {
    use crate::msg::UserBalanceQueryMsg::{UserAssetsBalance, UserSharesBalance};
    use crate::msg::{ExecuteMsg, ExtensionQueryMsg};
    use crate::query::{AssetsBalanceResponse, UserSharesBalanceResponse};
    use crate::test_tube::initialize::initialize::default_init_for_less_slippage;
    use cosmwasm_std::{assert_approx_eq, Coin, Fraction};
    use cosmwasm_std::{Decimal, Uint128};
    use cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::SpotPriceRequest;
    use osmosis_test_tube::{Account, Module, PoolManager, Wasm};
    use rand::{thread_rng, Rng};

    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";
    const ACCOUNTS_NUM: u64 = 10;
    const ACCOUNTS_INIT_BALANCE: u128 = 1_000_000_000_000_000;
    const DEPOSIT_AMOUNT_CAP: u128 = 5_000_000_000;

    #[test]
    #[ignore]
    fn test_any_deposit() {
        let (app, contract_address, cl_pool_id, admin) = default_init_for_less_slippage();

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

            let mut rng = thread_rng();
            let random_number: u128 = rng.gen_range(10_000..=DEPOSIT_AMOUNT_CAP);

            let total0 = Uint128::new(random_number)
                .checked_add(
                    Uint128::new(random_number)
                        .multiply_ratio(spot_price.denominator(), spot_price.numerator()),
                )
                .unwrap();

            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::AnyDeposit {
                        amount: Default::default(),
                        asset: "".to_string(),
                        recipient: None,
                    },
                    &[
                        Coin::new(random_number, DENOM_BASE),
                        Coin::new(random_number, DENOM_QUOTE),
                    ],
                    account,
                )
                .unwrap();

            let _result = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Redeposit {}),
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
            assert_approx_eq!(total0, total1, "0.0012482");
        }
    }
}
