#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use std::collections::HashMap;
    use cosmwasm_std::{Addr, Coin, Uint128};
    use osmosis_std::types::{osmosis::concentratedliquidity::poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool, cosmos::base::v1beta1};
    use osmosis_test_tube::{Account, Module, OsmosisTestApp, SigningAccount, Wasm};

    use crate::{
        msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg},
        query::UserBalanceResponse,
        test_tube::initialize::initialize::init_test_contract,
    };

    const ITERATIONS_NUMBER: usize = 5;
    const ACCOUNTS_NUMBER: u64 = 10;
    const ACCOUNTS_INITIAL_BALANCE: u128 = 1_000_000_000_000;
    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";

    #[derive(Clone, Copy, Debug)]
    enum Action {
        Deposit,
        Withdraw,
        Swap,
        UpdateRange,
    }

    fn deposit(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
        percentage: f64,
        accounts_shares_balance: &HashMap<String, u128>,
    ) {
        let balance = 1000; // TODO: get user asset0 balance
        let amount = (balance as f64 * (percentage / 100.0)).round() as u128;

        // TODO: Check user bank denom balance is not zero and enough accorindlgy to amount_u128

        // TODO: Get current pool position to know asset0 and asset1 as /osmosis.concentratedliquidity.v1beta1.FullPositionBreakdown
        let (amount0, amount1) = (1000u128, 1000u128);

        // TODO: Execute deposit and get liquidity_created from emitted events
        let deposit = wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::ExactDeposit { recipient: None }, // Nice to have: Make recipient random
            &[Coin::new(amount0, DENOM_BASE), Coin::new(amount1, DENOM_QUOTE)],
            &account,
        ).unwrap();
        //let deposit_resp: MsgCreatePositionResponse = deposit.data.try_into();
        //let liquidity_created = deposit_resp.liquidity_created;
        let liquidity_created = 1000 as u128;

        // TODO: Update map to keep track of user shares amount and make further assertions
        /*
        let mut current_shares_amount = accounts_shares_balance.get(&account.address());
        accounts_shares_balance.insert(
            account.address(),
            current_shares_amount.unwrap_or(&0u128).checked_add(liquidity_created),
        );
        */
    }

    fn withdraw(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
        percentage: f64,
        accounts_shares_balance: &HashMap<String, u128>,
    ) {
        let balance = 1000; // TODO: get user shares balance
        let amount = (balance as f64 * (percentage / 100.0)).round() as u128;

        // TODO: Check user shares balance is not zero and enough accorindlgy to amount_u128

        // TODO: Implement withdraw strategy
    }

    fn swap(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
        percentage: f64,
    ) {
        let balance = 1000; // TODO: get user asset0 balance
        let amount = (balance as f64 * (percentage / 100.0)).round() as u128;

        // TODO: Check user bank denom balance is not zero and enough accorindlgy to amount_u128

        // TODO: Implement swap strategy
    }

    fn update_range(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        percentage: f64
    ) {
        let (curent_lower_tick, current_upper_tick) = (1i64, 100i64); // TODO: get current ticks
        let (lower_tick, upper_tick) = (1i64, 100i64); //mocked
        // TODO: Validate new lower_tick and upper_tick

        // TODO: Mock somehow the range_admin from contract

        // TODO: Implement update range strategy
    }

    // ASSERT METHODS

    fn assert_deposit_withdraw(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        accounts: &Vec<SigningAccount>,
        accounts_shares_balance: &HashMap<String, u128>,
    ) {
        // TODO: multi-query foreach user created previously
        for account in accounts {
            let shares: UserBalanceResponse = wasm
                .query(
                    contract_address.as_str(),
                    &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                        crate::msg::UserBalanceQueryMsg::UserLockedBalance {
                            user: account.address(),
                        },
                    )),
                )
                .unwrap();
            // Check that the current account iterated shares balance is the same we expect from Hashmap
            // TODO: assert_eq!(shares.balance, accounts_shares_balance.get(&account.address()));
        }
    }

    fn assert_swap() {
        todo!()
    }

    fn assert_update_range() {
        todo!()
    }

    // COMPOSE STRATEGY

    // get_initial_range generates random lower and upper ticks for the initial position
    prop_compose! {
        fn get_initial_range()(lower_tick in 0i64..1_000_000, upper_tick in 1_000_001i64..2_000_000) -> (i64, i64) {
            (lower_tick, upper_tick)
        }
    }

    // get_strategy_list
    prop_compose! {
        fn get_strategy_list()(list in prop::collection::vec(prop_oneof![
            Just(Action::Deposit),
            Just(Action::Withdraw),
            Just(Action::Swap),
            Just(Action::UpdateRange),
        ], 0..ITERATIONS_NUMBER)) -> Vec<Action> {
            list
        }
    }

    // get_percentage generates a list of random percentages used to calculate deposit_amount,
    // withdraw_amount, and newers lower and upper ticks based on the previous values
    prop_compose! {
        fn get_percentage_list()(list in prop::collection::vec(1.0..100.0, 0..ITERATIONS_NUMBER)) -> Vec<f64> {
            list
        }
    }

    // get_account_index generates a list of random numbers between 0 and the ACCOUNTS_NUMBER-1 to use as accounts[account_index as usize]
    prop_compose! {
        fn get_account_index_list()(list in prop::collection::vec(0..(ACCOUNTS_NUMBER-1), 0..ITERATIONS_NUMBER)) -> Vec<u64> {
            list
        }
    }

    // TESTS

    proptest! {
        /// Main test function
        #[test]
        fn test_complete_works(
            (initial_lower_tick, initial_upper_tick) in get_initial_range(),
            actions in get_strategy_list(),
            percentages in get_percentage_list(),
            account_indexes in get_account_index_list()
        ) {
            // Creating test var utils
            let mut accounts_shares_balance: HashMap<String, u128> = HashMap::new();

            // Creating test core
            let (app, contract_address, _cl_pool_id, _admin) = init_test_contract(
                "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
                &[
                    Coin::new(1_000_000_000_000, "uatom"),
                    Coin::new(1_000_000_000_000, "uosmo"),
                ],
                MsgCreateConcentratedPool {
                    sender: "overwritten".to_string(),
                    denom0: "uatom".to_string(),
                    denom1: "uosmo".to_string(),
                    tick_spacing: 1,
                    spread_factor: "100000000000000".to_string(),
                },
                initial_lower_tick,
                initial_upper_tick,
                vec![
                    v1beta1::Coin {
                        denom: "uatom".to_string(),
                        amount: "10000000000".to_string(),
                    },
                    v1beta1::Coin {
                        denom: "uosmo".to_string(),
                        amount: "10000000000".to_string(),
                    },
                ],
                Uint128::zero(),
                Uint128::zero(),
            );
            let wasm = Wasm::new(&app);

            // Create a fixed number of accounts using app.init_accounts() function from test-tube, and assign a fixed initial balance for all of them
            let accounts = app
                .init_accounts(&[
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_QUOTE),
                ], ACCOUNTS_NUMBER)
                .unwrap();

            // Make one arbitrary deposit foreach one of the created accounts
            for i in 0..(ACCOUNTS_NUMBER-1) {
                deposit(&wasm, &contract_address, &accounts[i as usize], 1.00, &accounts_shares_balance);
            }

            // Here we know all the users have deposited. We can start executing random strategies here.

            // Iterate iterations times
            for i in 0..ITERATIONS_NUMBER {
                match actions[i] {
                    Action::Deposit => {
                        println!("Deposit logic here with account_index: {} and percentage: {}", account_indexes[i], percentages[i]);

                        deposit(&wasm, &contract_address, &accounts[account_indexes[i] as usize], percentages[i], &accounts_shares_balance);
                        assert_deposit_withdraw(&wasm, &contract_address, &accounts, &accounts_shares_balance);
                    },
                    Action::Withdraw => {
                        println!("Withdraw logic here with account_index: {} and percentage: {}", account_indexes[i], percentages[i]);

                        withdraw(&wasm, &contract_address, &accounts[account_indexes[i] as usize], percentages[i], &accounts_shares_balance);
                        assert_deposit_withdraw(&wasm, &contract_address, &accounts, &accounts_shares_balance);
                    },
                    Action::Swap => {
                        println!("Swap logic here with account_index: {} and percentage: {}", account_indexes[i], percentages[i]);

                        swap(&wasm, &contract_address, &accounts[account_indexes[i] as usize], percentages[i]);
                        assert_swap(); // todo!()
                    },
                    Action::UpdateRange => {
                        println!("UpdateRange logic here with percentage: {}", percentages[i]);

                        update_range(&wasm, &contract_address, percentages[i]);
                        assert_update_range(); // todo!()
                    },
                }
            }
        }
    }
}
