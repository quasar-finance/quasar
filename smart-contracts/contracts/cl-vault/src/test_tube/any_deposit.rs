// redeposit

#[cfg(test)]
mod tests {
    use apollo_cw_asset::AssetInfoBase;
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::{assert_approx_eq, Coin, Fraction};
    use cosmwasm_std::{Decimal, Uint128};
    use cw_dex::osmosis::OsmosisPool;
    use cw_dex_router::operations::{SwapOperationBase, SwapOperationsListUnchecked};
    use cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension;
    use osmosis_std::types::cosmos::bank::v1beta1::{
        MsgSend, QueryAllBalancesRequest, QueryBalanceRequest,
    };
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SwapAmountInRoute, SpotPriceRequest,
    };
    use osmosis_test_tube::RunnerError::ExecuteError;
    use osmosis_test_tube::{Account, Bank, Module, PoolManager, Wasm};
    use std::str::FromStr;
    use osmosis_std::types::cosmos::orm::query::v1alpha1::index_value::Value::Uint;

    use crate::msg::ClQueryMsg::SharePrice;
    use crate::msg::UserBalanceQueryMsg::{UserAssetsBalance, UserSharesBalance};
    use crate::msg::{AutoCompoundAsset, ExecuteMsg, ExtensionQueryMsg, ModifyRangeMsg, QueryMsg};
    use crate::query::{SharePriceResponse, UserSharesBalanceResponse, AssetsBalanceResponse};
    use crate::state::USER_REWARDS;
    use crate::test_tube::helpers::{get_amount_from_denom, get_event_attributes_by_ty_and_key};
    use crate::test_tube::initialize::initialize::{default_init, default_init_for_less_slippage, dex_cl_init_cl_pools, dex_cl_init_lp_pools};

    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";
    const DENOM_REWARD: &str = "ustride";
    const ACCOUNTS_NUM: u64 = 30;
    const ACCOUNTS_INIT_BALANCE: u128 = 1_000_000_000_000_000;
    const DEPOSIT_AMOUNT: u128 = 5_000_000;
    const SWAPS_NUM: usize = 10;
    const SWAPS_AMOUNT: &str = "1000000000";

    #[test]
    #[ignore]
    fn test_any_deposit() {
        let (app, contract_address, cl_pool_id, admin) = default_init_for_less_slippage();

        let wasm = Wasm::new(&app);
        let bm = Bank::new(&app);

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
            let spot_price : Decimal = pm
                .query_spot_price(&SpotPriceRequest {
                    base_asset_denom: DENOM_BASE.to_string(),
                    quote_asset_denom: DENOM_QUOTE.to_string(),
                    pool_id: cl_pool_id,
                })
                .unwrap()
                .spot_price
                .parse()
                .unwrap();

            let total0 = Uint128::new(DEPOSIT_AMOUNT)
                .checked_add(Uint128::new(DEPOSIT_AMOUNT).multiply_ratio(spot_price.denominator(), spot_price.numerator())).unwrap();

            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::AnyDeposit {
                        amount: Default::default(),
                        asset: "".to_string(),
                        recipient: None,
                    },
                    &[
                        Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                        Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                    ],
                    account,
                )
                .unwrap();


            // todo fix shares after redeposit
            // app.increase_time(10);
            // let _result = wasm
            //     .execute(
            //         contract_address.as_str(),
            //         &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Redeposit {}),
            //         &[],
            //         &admin,
            //     )
            //     .unwrap();
            // app.increase_time(10);


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


            let total1 = asset_balance.balances[0].amount
                .checked_add(asset_balance.balances[1].amount.multiply_ratio(spot_price.denominator(), spot_price.numerator())).unwrap();

            // assert the token1 deposited by alice
            assert_approx_eq!(
                total0,
                total1,
                "0.0011482"
            );
        }
    }
}
