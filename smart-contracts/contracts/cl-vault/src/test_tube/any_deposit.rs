#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::msg::ExecuteMsg;
    use crate::test_tube::initialize::initialize::{
        fixture_cw_dex_router, ACCOUNTS_INIT_BALANCE, ACCOUNTS_NUM, DENOM_BASE, DENOM_QUOTE,
    };
    use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
    use osmosis_std::types::cosmos::bank::v1beta1::{
        QueryAllBalancesRequest, QueryAllBalancesResponse,
    };
    use osmosis_std::types::osmosis::poolmanager::v1beta1::SpotPriceRequest;
    use osmosis_test_tube::{
        Account, Bank, Module, OsmosisTestApp, PoolManager, SigningAccount, Wasm,
    };

    const MAX_SLIPPAGE: f64 = 0.1;

    #[test]
    fn test_any_deposit() {
        let test_cases = vec![
            (DENOM_BASE, Uint128::new(10000000), Uint128::zero()),
            (DENOM_QUOTE, Uint128::new(5000000), Uint128::zero()),
            (DENOM_BASE, Uint128::new(20000000), Uint128::zero()),
            (DENOM_QUOTE, Uint128::new(15000000), Uint128::zero()),
            (DENOM_BASE, Uint128::new(10000000), Uint128::new(5000000)),
        ];

        for (_asset, amount_base, amount_quote) in test_cases {
            let (app, contract_address, _dex_router_addr, pools_ids, admin, _, _) =
                fixture_cw_dex_router();
            any_deposit(
                app,
                contract_address.clone(),
                pools_ids.clone(),
                admin,
                amount_base,
                amount_quote,
            );
        }
    }

    fn any_deposit(
        app: OsmosisTestApp,
        contract_address: Addr,
        _pool_ids: Vec<u64>,
        _admin: SigningAccount,
        amount_base: Uint128,
        amount_quote: Uint128,
    ) {
        let bm = Bank::new(&app);
        let wasm = Wasm::new(&app);
        let pm = PoolManager::new(&app);

        // Initialize accounts
        let accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, "uosmo"),
                ],
                ACCOUNTS_NUM,
            )
            .unwrap();

        let initial_balance: QueryAllBalancesResponse = bm
            .query_all_balances(&QueryAllBalancesRequest {
                address: contract_address.clone().into_string(),
                pagination: None,
            })
            .unwrap();

        // Simulate a deposit
        let mut deposit_coins = vec![];
        if amount_base > Uint128::zero() {
            deposit_coins.push(Coin::new(amount_base.u128(), DENOM_BASE));
        }
        if amount_quote > Uint128::zero() {
            deposit_coins.push(Coin::new(amount_quote.u128(), DENOM_QUOTE));
        }

        if !deposit_coins.is_empty() {
            let _ = wasm
                .execute(
                    contract_address.clone().as_str(),
                    &ExecuteMsg::AnyDeposit {
                        amount: amount_base,
                        asset: DENOM_BASE.to_string(),
                        recipient: Some(accounts[0].address()),
                        max_slippage: Decimal::bps(9000),
                    },
                    &deposit_coins,
                    &accounts[0],
                )
                .unwrap();

            let final_balance: QueryAllBalancesResponse = bm
                .query_all_balances(&QueryAllBalancesRequest {
                    address: contract_address.into_string(),
                    pagination: None,
                })
                .unwrap();

            //Get Spot Price
            let spot_price = pm
                .query_spot_price(&SpotPriceRequest {
                    pool_id: 1,
                    base_asset_denom: DENOM_BASE.to_string(),
                    quote_asset_denom: DENOM_QUOTE.to_string(),
                })
                .unwrap();
            let spot_price_base_to_quote = Decimal::from_str(&spot_price.spot_price).unwrap();
            let spot_price_quote_to_base = Decimal::one() / spot_price_base_to_quote;

            let initial_base = initial_balance
                .balances
                .iter()
                .find(|coin| coin.denom == DENOM_BASE)
                .map(|coin| Uint128::from(coin.amount.parse::<u128>().unwrap()))
                .unwrap_or_else(Uint128::zero);

            let initial_quote_in_base = initial_balance
                .balances
                .iter()
                .find(|coin| coin.denom == DENOM_QUOTE)
                .map(|coin| {
                    let quote_amount = Uint128::from(coin.amount.parse::<u128>().unwrap());
                    let quote_as_base = quote_amount * spot_price_quote_to_base;
                    quote_as_base
                })
                .unwrap_or_else(Uint128::zero);

            let initial_total_in_base = initial_base + initial_quote_in_base;

            let final_base = final_balance
                .balances
                .iter()
                .find(|coin| coin.denom == DENOM_BASE)
                .map(|coin| Uint128::from(coin.amount.parse::<u128>().unwrap()))
                .unwrap_or_else(Uint128::zero);

            let final_quote_in_base = final_balance
                .balances
                .iter()
                .find(|coin| coin.denom == DENOM_QUOTE)
                .map(|coin| {
                    let quote_amount = Uint128::from(coin.amount.parse::<u128>().unwrap());
                    let quote_as_base = quote_amount * spot_price_quote_to_base;
                    quote_as_base
                })
                .unwrap_or_else(Uint128::zero);

            let final_total_in_base = final_base + final_quote_in_base;

            let deposit_base_in_base = amount_base;
            let deposit_quote_in_base = amount_quote * spot_price_quote_to_base;

            let deposit_in_base = deposit_base_in_base + deposit_quote_in_base;

            let allowable_diff =
                (initial_total_in_base + deposit_in_base).u128() as f64 * MAX_SLIPPAGE;

            let total_diff = ((initial_total_in_base + deposit_in_base).u128() as f64
                - final_total_in_base.u128() as f64)
                .abs();

            assert!(
                total_diff <= allowable_diff,
                "Total difference too high: {}",
                total_diff
            );
        }
    }
}
