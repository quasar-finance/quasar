use crate::setup::{
    fixture_dex_router, ACCOUNTS_INIT_BALANCE, ACCOUNTS_NUM, DENOM_BASE, DENOM_QUOTE,
    MAX_SLIPPAGE_HIGH, PERFORMANCE_FEE_DEFAULT,
};

use cl_vault::msg::ExecuteMsg;
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use osmosis_std::types::cosmos::bank::v1beta1::{
    QueryAllBalancesRequest, QueryAllBalancesResponse,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SpotPriceRequest;
use osmosis_test_tube::{
    Account, Bank, Module, OsmosisTestApp, PoolManager, RunnerError, SigningAccount, Wasm,
};
use std::str::FromStr;

#[test]
fn test_any_deposit() {
    let test_cases = vec![
        (Uint128::new(10000), Uint128::zero()),
        (Uint128::new(5000), Uint128::zero()),
        (Uint128::new(2000), Uint128::zero()),
        (Uint128::new(1500), Uint128::zero()),
        (Uint128::new(1000), Uint128::new(500)),
    ];

    for (amount_base, amount_quote) in test_cases {
        let (app, contract_address, _dex_router_addr, vault_pool_id, _pools_ids, admin, _, _) =
            fixture_dex_router(PERFORMANCE_FEE_DEFAULT);

        do_and_verify_any_deposit(
            app,
            contract_address.clone(),
            vault_pool_id,
            admin,
            amount_base,
            amount_quote,
            Decimal::bps(MAX_SLIPPAGE_HIGH),
        );
    }
}

fn do_and_verify_any_deposit(
    app: OsmosisTestApp,
    contract_address: Addr,
    vault_pool_id: u64,
    _admin: SigningAccount,
    amount_base: Uint128,
    amount_quote: Uint128,
    max_slippage: Decimal,
) {
    let (initial_balance, deposit_coins) = do_any_deposit(
        &app,
        &contract_address,
        amount_base,
        amount_quote,
        max_slippage,
    )
    .unwrap();
    let bm = Bank::new(&app);
    let pm = PoolManager::new(&app);

    if !deposit_coins.is_empty() {
        let final_balance: QueryAllBalancesResponse = bm
            .query_all_balances(&QueryAllBalancesRequest {
                address: contract_address.into_string(),
                pagination: None,
            })
            .unwrap();

        //Get Spot Price
        let spot_price = pm
            .query_spot_price(&SpotPriceRequest {
                pool_id: vault_pool_id,
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
                quote_amount * spot_price_quote_to_base
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
                quote_amount * spot_price_quote_to_base
            })
            .unwrap_or_else(Uint128::zero);

        let final_total_in_base = final_base + final_quote_in_base;

        let deposit_base_in_base = amount_base;
        let deposit_quote_in_base = amount_quote * spot_price_quote_to_base;

        let deposit_in_base = deposit_base_in_base + deposit_quote_in_base;

        let allowable_diff = (initial_total_in_base + deposit_in_base).u128() as f64
            * (1.0 - (MAX_SLIPPAGE_HIGH as f64 / 10_000.0));

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

fn do_any_deposit(
    app: &OsmosisTestApp,
    contract_address: &Addr,
    amount_base: Uint128,
    amount_quote: Uint128,
    max_slippage: Decimal,
) -> Result<(QueryAllBalancesResponse, Vec<Coin>), RunnerError> {
    let bm = Bank::new(app);
    let wasm = Wasm::new(app);

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

    let _ = wasm.execute(
        contract_address.clone().as_str(),
        &ExecuteMsg::AnyDeposit {
            amount: amount_base,
            asset: DENOM_BASE.to_string(),
            recipient: Some(accounts[0].address()),
            max_slippage,
        },
        &deposit_coins,
        &accounts[0],
    )?;
    Ok((initial_balance, deposit_coins))
}

#[test]
fn test_any_deposit_slippage_fails() {
    let test_cases = vec![
        (Uint128::new(10000), Uint128::zero()),
        (Uint128::new(5000), Uint128::zero()),
        (Uint128::new(2000), Uint128::zero()),
        (Uint128::new(1500), Uint128::zero()),
        (Uint128::new(1000), Uint128::new(500)),
    ];

    for (amount_base, amount_quote) in test_cases {
        let (app, contract_address, _dex_router_addr, _vault_pool_id, _pools_ids, _admin, _, _) =
            fixture_dex_router(PERFORMANCE_FEE_DEFAULT);

        do_any_deposit(
            &app,
            &contract_address,
            amount_base,
            amount_quote,
            Decimal::zero(),
        )
        .unwrap_err();
    }
}
