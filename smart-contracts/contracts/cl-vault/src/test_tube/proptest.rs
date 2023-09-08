#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use std::collections::HashMap;
    use cosmwasm_std::{Addr, Coin, Uint128, Decimal};
    use osmosis_std::types::{
        osmosis::concentratedliquidity::poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool,
        cosmos::base::v1beta1,
    };
    use osmosis_test_tube::{Account, Module, OsmosisTestApp, SigningAccount, Wasm};

    use crate::{
        msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg, ModifyRangeMsg},
        query::{UserBalanceResponse, TotalAssetsResponse},
        test_tube::initialize::initialize::init_test_contract,
    };

    const ITERATIONS_NUMBER: usize = 100;
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
        accounts_shares_balance: &HashMap<String, Uint128>,
    ) {
         // TODO: get user DENOM_BASE balance
        let balance_asset0 = get_user_denom_balance(wasm, account, DENOM_BASE);
        let amount0 = (balance_asset0.u128() as f64 * (percentage / 100.0)).round() as u128;

         // TODO: get user DENOM_QUOTE balance
        let balance_asset1 = get_user_denom_balance(wasm, account, DENOM_QUOTE);
        let amount1 = (balance_asset1.u128() as f64 * (percentage / 100.0)).round() as u128;

        // Get current pool position to know asset0 and asset1 as /osmosis.concentratedliquidity.v1beta1.FullPositionBreakdown
        let pos_assets: TotalAssetsResponse = get_position_assets(wasm, contract_address);

        // Calculate the ratio between pos_asset0 and pos_asset1
        let ratio = pos_assets.token0.amount.u128() as f64 / pos_assets.token1.amount.u128() as f64;

        // Calculate the adjusted amounts to deposit
        let adjusted_amount0: u128;
        let adjusted_amount1: u128;
        if ratio > 1.0 {
            // If ratio is greater than 1, then asset0 has a higher amount.
            // So, adjust amount1 according to the ratio.
            adjusted_amount0 = amount0;
            adjusted_amount1 = (amount0 as f64 / ratio).round() as u128;
        } else {
            // If ratio is less than or equal to 1, then asset1 has a higher or equal amount.
            // So, adjust amount0 according to the ratio.
            adjusted_amount1 = amount1;
            adjusted_amount0 = (amount1 as f64 * ratio).round() as u128;
        }

        // TODO: Evaluate if checking that balance is not zero, as maybe a before iteration make him deposit the 100%,
        // or evaluate capping the max percentage to 90 to run indfinitely till max iterations

        println!("Deposit amounts: {}, {}", adjusted_amount0, adjusted_amount1);
        // Execute deposit and get liquidity_created from emitted events
        let deposit = wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::ExactDeposit { recipient: None }, // Nice to have: Make recipient random
            &[Coin::new(adjusted_amount0, DENOM_BASE), Coin::new(adjusted_amount1, DENOM_QUOTE)],
            &account,
        ).unwrap();
        /*
        // TODO: Get liquidity_created value from deposit response
        let deposit_resp: MsgCreatePositionResponse = deposit.data.try_into();
        let liquidity_created = deposit_resp.liquidity_created;

        // TODO: Update map to keep track of user shares amount and make further assertions
        let mut current_shares_amount = accounts_shares_balance.get(&account.address()).unwrap_or(&0u128);
        accounts_shares_balance.insert(
            account.address(),
            current_shares_amount.checked_add(liquidity_created),
        );
        */
    }

    fn withdraw(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
        percentage: f64,
        accounts_shares_balance: &HashMap<String, Uint128>,
    ) {
        let balance = get_user_shares_balance(wasm, contract_address, account); // TODO: get user shares balance
        let amount = (balance.balance.u128() as f64 * (percentage / 100.0)).round() as u128;

        println!("Withdraw amount: {}", amount);
        // Execute deposit and get liquidity_created from emitted events
        let withdraw = wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::Redeem { recipient: None, amount: Uint128::new(amount) }, // Nice to have: Make recipient random
            &[],
            &account,
        ).unwrap();

        // TODO: Update map to keep track of user shares amount and make further assertions
        /*let mut current_shares_amount = accounts_shares_balance.get(&account.address()).unwrap_or(&0u128);
        accounts_shares_balance.insert(
            account.address(),
            current_shares_amount.checked_sub(amount),
        );*/
    }

    fn swap(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
        percentage: f64,
        cl_pool_id: u64,
    ) {
        let balance = get_user_denom_balance(wasm, account, DENOM_BASE);
        let amount = (balance.u128() as f64 * (percentage / 100.0)).round() as u128;

        // TODO: Check user bank denom balance is not zero and enough accorindlgy to amount_u128
        println!("Swap amount: {}", amount);

        // TODO: Implement swap strategy
    }

    fn update_range(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        percentage: f64,
        admin_account: &SigningAccount
    ) {
        let (current_lower_tick, current_upper_tick) = get_position_ticks(wasm, contract_address);

        // Create new range ticks based on previous ticks by percentage variation
        // TODO: 1. Use also negative values, and maybe a random generated value for the lower and another one for upper instead of the same unique percentage
        // TODO: 2. Creating them in a range of min/max accepted by Osmosis CL module
        let percentage_factor = percentage / 100.0;
        let lower_tick = (current_lower_tick as f64 * (1.0 + percentage_factor)).round() as i64;
        let upper_tick = (current_upper_tick as f64 * (1.0 + percentage_factor)).round() as i64;

        println!("Update range new lower_tick: {} new upper_tick: {}", lower_tick, upper_tick);
        // Execute deposit and get liquidity_created from emitted events
        let update_range = wasm.execute(
            contract_address.as_str(),
            &ModifyRangeMsg {
                lower_price: lower_tick.to_string(),
                upper_price: upper_tick.to_string(),
                max_slippage: Decimal::new(Uint128::new(5)), // optimize and check how this fits in the strategy as it could trigger organic errors we dont want to test
            },
            &[],
            &admin_account,
        ).unwrap();
    }

    // GETTERS

    fn get_user_denom_balance(
        wasm: &Wasm<OsmosisTestApp>,
        account: &SigningAccount,
        denom: &str
    ) -> Uint128 {
        Uint128::new(1_000)
    }

    fn get_user_shares_balance(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
    ) -> UserBalanceResponse {
        wasm
            .query(
            contract_address.as_str(),
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                crate::msg::UserBalanceQueryMsg::UserSharesBalance {
                    user: account.address(),
                },
            )),
        )
        .unwrap()
    }

    fn get_position_assets(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
    ) -> TotalAssetsResponse {
        wasm
            .query(
            contract_address.as_str(),
            &QueryMsg::TotalAssets {},
        )
        .unwrap()
    }

    fn get_position_ticks(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
    ) -> (i64, i64) {
        // TODO query_position will return a Vec of position_ids

        // TODO Use those to take the latest one? or what?

        (1000, 1000)
    }

    // ASSERT METHODS

    fn assert_deposit_withdraw(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        accounts: &Vec<SigningAccount>,
        accounts_shares_balance: &HashMap<String, Uint128>,
    ) {
        // TODO: multi-query foreach user created previously
        for account in accounts {
            let shares = get_user_shares_balance(wasm, contract_address, account);

            // Check that the current account iterated shares balance is the same we expect from Hashmap
            //assert_eq!(shares.balance, accounts_shares_balance.get(&account.address()));
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
            let mut accounts_shares_balance: HashMap<String, Uint128> = HashMap::new();

            // Creating test core
            let (app, contract_address, cl_pool_id, admin_account) = init_test_contract(
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

                // Make one arbitrary deposit foreach one of the created accounts using 10.00% of its balance, to avoid complications on withdrawing without any position
            for i in 0..ACCOUNTS_NUMBER {
                println!("Making first deposit for account: {}", i);

                deposit(&wasm, &contract_address, &accounts[i as usize], 10.00, &accounts_shares_balance);
            }

            // Iterate iterations times
            for i in 0..ITERATIONS_NUMBER {
                match actions[i] {
                    Action::Deposit => {
                        println!("Deposit logic here with account_index: {} and percentage: {}", account_indexes[i], percentages[i]);

                        deposit(&wasm, &contract_address, &accounts[account_indexes[i] as usize], percentages[i], &accounts_shares_balance);
                        //assert_deposit_withdraw(&wasm, &contract_address, &accounts, &accounts_shares_balance);
                    },
                    Action::Withdraw => {
                        println!("Withdraw logic here with account_index: {} and percentage: {}", account_indexes[i], percentages[i]);

                        withdraw(&wasm, &contract_address, &accounts[account_indexes[i] as usize], percentages[i], &accounts_shares_balance);
                        //assert_deposit_withdraw(&wasm, &contract_address, &accounts, &accounts_shares_balance);
                    },
                    Action::Swap => {
                        println!("Swap logic here with account_index: {} and percentage: {}", account_indexes[i], percentages[i]);

                        swap(&wasm, &contract_address, &accounts[account_indexes[i] as usize], percentages[i], cl_pool_id);
                        //assert_swap(); // todo!()
                    },
                    Action::UpdateRange => {
                        println!("UpdateRange logic here with percentage: {}", percentages[i]);

                        update_range(&wasm, &contract_address, percentages[i], &admin_account);
                        //assert_update_range(); // todo!()
                    },
                }
            }
        }
    }
}
