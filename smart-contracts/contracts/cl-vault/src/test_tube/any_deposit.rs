// redeposit

#[cfg(test)]
mod tests {
    use apollo_cw_asset::AssetInfoBase;
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::Coin;
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

    use crate::msg::ClQueryMsg::SharePrice;
    use crate::msg::UserBalanceQueryMsg::UserSharesBalance;
    use crate::msg::{AutoCompoundAsset, ExecuteMsg, ExtensionQueryMsg, ModifyRangeMsg, QueryMsg};
    use crate::query::{SharePriceResponse, UserSharesBalanceResponse};
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

        let mut shares_prices_ratio: Vec<Decimal> = vec![];
        let mut spot_prices: Vec<Decimal> = vec![];
        for account in &accounts {
            // // check for contract balance as it has not been redeposited yet
            // let balance = bm
            //     .query_all_balances(&QueryAllBalancesRequest {
            //         address: contract_address.to_string(),
            //         pagination: None,
            //     })
            //     .unwrap();
            // println!("contract balance before any deposit: {:?}", balance);

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

            // // check for contract balance as it has not been redeposited yet
            // let balance = bm
            //     .query_all_balances(&QueryAllBalancesRequest {
            //         address: contract_address.to_string(),
            //         pagination: None,
            //     })
            //     .unwrap();
            // println!("contract balance after any deposit: {:?}", balance);

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
            let share_price: SharePriceResponse = wasm
                .query(
                    contract_address.as_str(),
                    &VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(SharePrice {
                        shares: shares.balance,
                    })),
                )
                .unwrap();

            // Get and assert spot price is 1.0
            let pm = PoolManager::new(&app);
            let spot_price = pm
                .query_spot_price(&SpotPriceRequest {
                    base_asset_denom: DENOM_BASE.to_string(),
                    quote_asset_denom: DENOM_QUOTE.to_string(),
                    pool_id: cl_pool_id,
                })
                .unwrap();

            println!("share ratio : {:?}, spot price : {:?}, shares assigned to user : {:?}", Decimal::from_ratio(share_price.balances[0].amount, share_price.balances[1].amount), spot_price.spot_price, shares);
            shares_prices_ratio.push(Decimal::from_ratio(share_price.balances[0].amount, share_price.balances[1].amount));
            spot_prices.push(Decimal::from_str(spot_price.spot_price.as_str()).unwrap());
        }

        // todo fix percentage change issue
        println!("{:?}", spot_prices);
        for i in (0..shares_prices_ratio.len() - 1).rev() {
            let current_price = spot_prices[i] * Decimal::new(Uint128::new(10000000000000000000000000000000000000));
            let next_price = spot_prices[i + 1] * Decimal::new(Uint128::new(10000000000000000000000000000000000000));
            println!("{:?}", current_price);
            println!("{:?}", next_price);

            // Calculate the percentage change
            let percentage_change = (next_price - current_price) / current_price;

            println!("Percentage change between {} and {} is {}", current_price, next_price, percentage_change);
            // let current_price = shares_prices_ratio[i].clone();
            // let next_price = shares_prices_ratio[i + 1].clone();
            //
            // // Calculate the percentage change
            // let percentage_change = ((next_price - current_price) / current_price) * Decimal::new(Uint128::new(100));
            //
            // println!("Percentage change between {:?}", percentage_change);
            //
            // let current_price_1 = spot_prices[i].clone();
            // let next_price_1 = spot_prices[i + 1].clone();
            //
            // // Calculate the percentage change
            // let percentage_change = ((next_price_1 - current_price_1) / current_price_1.clone()) * Decimal::new(Uint128::new(100));
            //
            // println!("Percentage change between {:?}%", percentage_change);
        }

        // Declare swapper and claimer accounts
        let ops_accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                2,
            )
            .unwrap();
        let swapper = &ops_accounts[0];

        // Swaps to generate spread rewards on previously created user positions
        for _ in 0..SWAPS_NUM {
            PoolManager::new(&app)
                .swap_exact_amount_in(
                    MsgSwapExactAmountIn {
                        sender: swapper.address(),
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
                    &swapper,
                )
                .unwrap();
        }
    }
}

