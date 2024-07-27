#![cfg(feature = "test-tube")]

use crate::setup::{
    get_event_attributes_by_ty_and_key, init_test_contract, MAX_SLIPPAGE_HIGH,
    PERFORMANCE_FEE_DEFAULT,
};

use cl_vault::{
    helpers::generic::sort_tokens,
    math::tick::tick_to_price,
    msg::{
        ClQueryMsg, ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, ModifyRangeMsg, QueryMsg,
        UserBalanceQueryMsg,
    },
    query::{
        AssetsBalanceResponse, PositionResponse, TotalAssetsResponse,
        TotalVaultTokenSupplyResponse, UserSharesBalanceResponse,
    },
};
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use osmosis_std::types::{
    cosmos::bank::v1beta1::QueryBalanceRequest, cosmos::base::v1beta1,
    cosmwasm::wasm::v1::MsgExecuteContractResponse,
    osmosis::concentratedliquidity::poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool,
    osmosis::concentratedliquidity::v1beta1::PositionByIdRequest,
};
use osmosis_test_tube::{
    Account, Bank, ConcentratedLiquidity, ExecuteResponse, Module, OsmosisTestApp, SigningAccount,
    Wasm,
};
use proptest::prelude::*;

const ITERATIONS_NUMBER: usize = 1000;
const ACCOUNTS_NUMBER: u64 = 10;
const ACCOUNTS_INITIAL_BALANCE: u128 = 100_000_000_000_000;
const DENOM_BASE: &str = "ZZZZZ";
const DENOM_QUOTE: &str = "ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858";

#[derive(Clone, Copy, Debug)]
enum Action {
    Deposit,
    Withdraw,
    UpdateRange,
}

fn deposit(
    wasm: &Wasm<OsmosisTestApp>,
    bank: &Bank<OsmosisTestApp>,
    contract_address: &Addr,
    account: &SigningAccount,
    percentage: f64,
    base_denom: &str,
    quote_denom: &str,
) {
    let base_asset_balance = get_user_denom_balance(bank, account, base_denom);
    let base_balance_f64: f64 = base_asset_balance
        .amount
        .parse()
        .expect("Failed to parse balance to f64");
    let base_amount = (base_balance_f64 * (percentage / 100.0)).round() as u128;

    let quote_asset_balance = get_user_denom_balance(bank, account, quote_denom);
    let quote_balance: f64 = quote_asset_balance
        .amount
        .parse()
        .expect("Failed to parse balance to f64");
    let quote_amount = (quote_balance * (percentage / 100.0)).round() as u128;

    let pos_assets: TotalAssetsResponse = get_vault_position_assets(wasm, contract_address);

    let ratio = pos_assets.token0.amount.u128() as f64 / pos_assets.token1.amount.u128() as f64;

    let (base_amount, quote_amount) = if ratio > 1.0 {
        (base_amount, (base_amount as f64 / ratio).round() as u128)
    } else {
        ((quote_amount as f64 * ratio).round() as u128, quote_amount)
    };

    let mut coins_to_deposit = Vec::new();
    if base_amount > 0 {
        coins_to_deposit.push(Coin::new(base_amount, base_denom));
    }
    if quote_amount > 0 {
        coins_to_deposit.push(Coin::new(quote_amount, quote_denom));
    }

    if coins_to_deposit.is_empty() {
        return;
    }

    let create_position: ExecuteResponse<MsgExecuteContractResponse> = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::ExactDeposit { recipient: None }, // Nice to have: Make recipient random
            &sort_tokens(coins_to_deposit), // TODO: Why our contract, before adding a message/submessage cannot handle a sort? like first line of deposit::execute_exact_deposit
            account,
        )
        .unwrap();

    let _create_position_attrs = get_event_attributes_by_ty_and_key(
        &create_position,
        "create_position",
        vec!["liquidity", "amount0", "amount1"],
    );
}

fn withdraw(
    wasm: &Wasm<OsmosisTestApp>,
    contract_address: &Addr,
    account: &SigningAccount,
    percentage: f64,
) {
    let balance = get_user_shares_balance(wasm, contract_address, account);
    let amount = (balance.balance.u128() as f64 * (percentage / 100.0)).round() as u128;

    let _user_assets_bal: AssetsBalanceResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                UserBalanceQueryMsg::UserAssetsBalance {
                    user: account.address(),
                },
            )),
        )
        .unwrap();

    let _vault_total_shares: TotalAssetsResponse = wasm
        .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
        .unwrap();

    let withdraw_position: ExecuteResponse<MsgExecuteContractResponse> = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::Redeem {
                recipient: None,
                amount: Uint128::new(amount),
            }, // Nice to have: Make recipient random
            &[],
            account,
        )
        .unwrap();

    let _withdraw_position_attrs = get_event_attributes_by_ty_and_key(
        &withdraw_position,
        "withdraw_position",
        vec!["liquidity", "amount0", "amount1"],
    );
}

fn _swap(
    _wasm: &Wasm<OsmosisTestApp>,
    bank: &Bank<OsmosisTestApp>,
    _contract_address: &Addr,
    account: &SigningAccount,
    percentage: f64,
    _cl_pool_id: u64,
) {
    let balance_response = get_user_denom_balance(bank, account, DENOM_BASE);
    let balance_str = balance_response.amount;
    let balance_f64: f64 = balance_str.parse().expect("Failed to parse balance to f64");
    let _amount = (balance_f64 * (percentage / 100.0)).round() as u128;

    // TODO: Check user bank denom balance is not zero and enough accordingly to amount_u128

    // TODO: Implement swap strategy
}

fn update_range(
    wasm: &Wasm<OsmosisTestApp>,
    cl: &ConcentratedLiquidity<OsmosisTestApp>,
    contract_address: &Addr,
    percentage: f64,
    admin_account: &SigningAccount,
) {
    let (current_lower_tick, current_upper_tick) = get_position_ticks(wasm, cl, contract_address);
    let (current_lower_price, current_upper_price) = (
        tick_to_price(current_lower_tick).unwrap(),
        tick_to_price(current_upper_tick).unwrap(),
    );
    let clp_u128: Uint128 = current_lower_price.atomics().try_into().unwrap();
    let cup_u128: Uint128 = current_upper_price.atomics().try_into().unwrap();

    // Create new range ticks based on previous ticks by percentage variation
    // TODO: 1. Use also negative values, and maybe a random generated value for the lower and another one for upper instead of the same unique percentage
    // TODO: 2. Creating them in a range of min/max accepted by Osmosis CL module
    let percentage_factor = percentage / 100.0;
    let new_lower_price = (clp_u128.u128() as f64 * (1.0 + percentage_factor)).round() as u128;
    let new_upper_price = (cup_u128.u128() as f64 * (1.0 + percentage_factor)).round() as u128;

    // Skip equal ticks test case
    if new_lower_price == new_upper_price {
        return;
    }

    let _update_range = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                lower_price: Decimal::new(Uint128::new(new_lower_price)),
                upper_price: Decimal::new(Uint128::new(new_upper_price)),
                max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH), // optimize and check how this fits in the strategy as it could trigger organic errors we dont want to test
                ratio_of_swappable_funds_to_use: Decimal::one(),
                twap_window_seconds: 45,
                forced_swap_route: None,
                claim_after: None,
            })),
            &[],
            admin_account,
        )
        .unwrap();
}

fn get_user_denom_balance(
    bank: &Bank<OsmosisTestApp>,
    account: &SigningAccount,
    denom: &str,
) -> v1beta1::Coin {
    bank.query_balance(&QueryBalanceRequest {
        address: account.address(),
        denom: denom.to_string(),
    })
    .unwrap()
    .balance
    .unwrap()
}

fn _get_vault_shares_balance(
    wasm: &Wasm<OsmosisTestApp>,
    contract_address: &Addr,
) -> TotalVaultTokenSupplyResponse {
    wasm.query(
        contract_address.as_str(),
        &QueryMsg::TotalVaultTokenSupply {},
    )
    .unwrap()
}

fn get_vault_position_assets(
    wasm: &Wasm<OsmosisTestApp>,
    contract_address: &Addr,
) -> TotalAssetsResponse {
    wasm.query(contract_address.as_str(), &QueryMsg::TotalAssets {})
        .unwrap()
}

fn get_user_shares_balance(
    wasm: &Wasm<OsmosisTestApp>,
    contract_address: &Addr,
    account: &SigningAccount,
) -> UserSharesBalanceResponse {
    wasm.query(
        contract_address.as_str(),
        &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
            UserBalanceQueryMsg::UserSharesBalance {
                user: account.address(),
            },
        )),
    )
    .unwrap()
}

fn get_position_ticks(
    wasm: &Wasm<OsmosisTestApp>,
    cl: &ConcentratedLiquidity<OsmosisTestApp>,
    contract_address: &Addr,
) -> (i64, i64) {
    let position_response: PositionResponse = wasm
        .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                ClQueryMsg::Position {},
            )),
        )
        .unwrap();

    let position = cl
        .query_position_by_id(&PositionByIdRequest {
            position_id: position_response.position_ids[0],
        })
        .unwrap()
        .position
        .unwrap()
        .position;

    match position {
        Some(position) => (position.lower_tick, position.upper_tick),
        None => panic!("Position not found"),
    }
}

// COMPOSE STRATEGY
// get_initial_range generates random lower and upper ticks for the initial position
prop_compose! {
    fn get_initial_range()(lower_tick in -300_000i64..0, upper_tick in 1i64..500_000) -> (i64, i64) {
        (lower_tick, upper_tick)
    }
}

prop_compose! {
    fn get_strategy_list()(list in prop::collection::vec(prop_oneof![
        Just(Action::Deposit),
        Just(Action::Withdraw),
        Just(Action::UpdateRange),
    ], ITERATIONS_NUMBER..ITERATIONS_NUMBER+1)) -> Vec<Action> {
        list
    }
}

// get_percentage generates a list of random percentages used to calculate deposit_amount,
// withdraw_amount, and newers lower and upper ticks based on the previous values
prop_compose! {
    fn get_percentage_list()(list in prop::collection::vec(1.0..100.0, ITERATIONS_NUMBER..ITERATIONS_NUMBER+1)) -> Vec<f64> {
        list
    }
}

// get_account_index generates a list of random numbers between 0 and the ACCOUNTS_NUMBER to use as accounts[account_index as usize]
prop_compose! {
    fn get_account_index_list()(list in prop::collection::vec(0..ACCOUNTS_NUMBER, ITERATIONS_NUMBER..ITERATIONS_NUMBER+1)) -> Vec<u64> {
        list
    }
}

fn get_cases() -> u32 {
    std::env::var("PROPTEST_CASES")
        .unwrap_or("100".to_string())
        .parse()
        .unwrap()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(get_cases()))]
    #[test]
    fn test_complete_works(
        (initial_lower_tick, initial_upper_tick) in get_initial_range(),
        actions in get_strategy_list(),
        percentages in get_percentage_list(),
        account_indexes in get_account_index_list()
    ) {
        println!("start");
        let (app, contract_address, _cl_pool_id, admin_account, _deposit_ratio, _deposit_ratio_approx) = init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(340282366920938463463374607431768211455, "uosmo"),
                Coin::new(340282366920938463463374607431768211455, DENOM_BASE),
                Coin::new(340282366920938463463374607431768211455, DENOM_QUOTE),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: DENOM_BASE.to_string(),
                denom1: DENOM_QUOTE.to_string(),
                tick_spacing: 1,
                spread_factor: "100000000000000".to_string(),
            },
            initial_lower_tick,
            initial_upper_tick,
            vec![
                v1beta1::Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: (ITERATIONS_NUMBER as u128*ACCOUNTS_INITIAL_BALANCE).to_string(),
                },
                v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: (ITERATIONS_NUMBER as u128*ACCOUNTS_INITIAL_BALANCE).to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
            PERFORMANCE_FEE_DEFAULT
        );
        println!("wasm");
        let wasm = Wasm::new(&app);
        let cl = ConcentratedLiquidity::new(&app);
        let bank = Bank::new(&app);

        let accounts = app
            .init_accounts(&[
                Coin::new(ACCOUNTS_INITIAL_BALANCE, "uosmo"),
                Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_BASE),
                Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_QUOTE),
            ], ACCOUNTS_NUMBER)
            .unwrap();

        let deposit_percentage = 10.0;
        for i in 0..ACCOUNTS_NUMBER {
            deposit(&wasm, &bank, &contract_address, &accounts[i as usize], deposit_percentage, DENOM_BASE, DENOM_QUOTE);
        }

        for i in 0..ITERATIONS_NUMBER {
            println!("iter");
            match actions[i] {
                Action::Deposit => {
                    deposit(&wasm, &bank, &contract_address, &accounts[account_indexes[i] as usize], percentages[i], DENOM_BASE, DENOM_QUOTE);
                },
                Action::Withdraw => {
                    withdraw(&wasm, &contract_address, &accounts[account_indexes[i] as usize], percentages[i]);
                },
                Action::UpdateRange => {
                    update_range(&wasm, &cl, &contract_address, percentages[i], &admin_account);
                },
            }
        }

        println!("PASS");
    }
}
