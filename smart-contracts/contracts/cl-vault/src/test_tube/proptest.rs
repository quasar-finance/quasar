#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use std::collections::HashMap;
    use cosmwasm_std::{Coin, Addr};
    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePositionResponse;
    use osmosis_test_tube::{Account, Module, OsmosisTestApp, SigningAccount, Wasm};

    use crate::{
        msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg},
        query::UserBalanceResponse,
        test_tube::default_init,
    };

    const ITERATIONS_MAX_NUMBER: usize = 100;
    const ACCOUNTS_NUMBER: u64 = 10;
    const ACCOUNTS_INITIAL_BALANCE: u128 = 1_000_000_000_000;
    const RANGE_MAX_PERCENT_DIFF: f64 = 100.00;
    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";

    #[derive(Clone, Copy, Debug)]
    enum Action {
        Deposit { account_index: u64, amount: u128 },
        Withdraw { account_index: u64, amount: u128 },
        Swap { account_index: u64, amount: u128 },
        UpdateRange { lower_tick: u128, upper_tick: u128 },
    }

    proptest! {
        /// Main test function
        #[test]
        fn test_complete_works(
            iterations in 1usize..ITERATIONS_MAX_NUMBER,
        ) {
            // Creating test core
            let (app, contract_address, _cl_pool_id, _admin) = default_init();
            let wasm = Wasm::new(&app);

            // Creating test vars
            let mut accounts_shares_balance: HashMap<String, u128> = HashMap::new();
            // TODO: Get the state lower and upper ticks
            let mut prev_lower_tick: u128 = 1; // TODO: set
            let mut prev_upper_tick: u128 = 100; // TODO: set

            // Create a fixed number of accounts using app.init_accounts() function from test-tube, and assign a fixed initial balance for all of them
            let accounts = app
                .init_accounts(&[
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_QUOTE),
                ], ACCOUNTS_NUMBER)
                .unwrap();

            // Make one arbitrary deposit foreach one of the created accounts
            for i in 0..(ACCOUNTS_NUMBER-1) {
                deposit(&wasm, &contract_address, &accounts[i as usize], 1_000, &accounts_shares_balance);
            }

            // Here we know all the users have deposited. We can start executing random strategies here.

            // Iterate iterations times
            for _ in 0..iterations {
                let action = prop_oneof![
                    // Deposit
                    (
                        any::<u64>().prop_map(|x| x % ACCOUNTS_NUMBER),
                        any::<u128>().prop_map(|x| x.max(1))
                    ).prop_map(|(account_index, amount)| Action::Deposit { account_index, amount }),
        
                    // Withdraw
                    (
                        any::<u64>().prop_map(|x| x % ACCOUNTS_NUMBER),
                        any::<u128>().prop_map(|x| {
                            let min_balance = accounts_shares_balance.values().min().unwrap_or(&0);
                            std::cmp::min(x, *min_balance)
                        })
                    ).prop_map(|(account_index, amount)| Action::Withdraw { account_index, amount }),
        
                    // Swap
                    (
                        any::<u64>().prop_map(|x| x % ACCOUNTS_NUMBER),
                        any::<u128>().prop_map(|x| x.max(1))
                    ).prop_map(|(account_index, amount)| Action::Swap { account_index, amount }),    
    
                    // Generate lower_tick and upper_tick based on previous and taking in account RANGE_MAX_PERCENT_DIFF
                    prop::collection::vec(any::<u128>(), 2..3).prop_map(move |mut vec| {
                        vec.sort();
                        let (mut lower_tick, mut upper_tick) = (vec[0], vec[1]);
                        if let (Some(prev_lower), Some(prev_upper)) = (prev_lower_tick, prev_upper_tick) {
                            let lower_diff = (prev_lower as f64) * RANGE_MAX_PERCENT_DIFF / 100.0;
                            let upper_diff = (prev_upper as f64) * RANGE_MAX_PERCENT_DIFF / 100.0;
                    
                            lower_tick = ((prev_lower as f64) - lower_diff + lower_diff * 2.0 * rand::random::<f64>()) as u128;
                            upper_tick = ((prev_upper as f64) - upper_diff + upper_diff * 2.0 * rand::random::<f64>()) as u128;
                        }
                        Action::UpdateRange { lower_tick, upper_tick }
                    }),
                ];
                
                match action {
                    Action::Deposit { account_index, amount } => {
                        println!("Deposit logic here with account_index: {} and amount: {}", account_index, amount);
                        deposit(&wasm, &contract_address, &accounts[account_index as usize], amount, &accounts_shares_balance);
                        assert_deposit_withdraw(&wasm, contract_address, accounts, &accounts_shares_balance);
                    },
                    Action::Withdraw { account_index, amount } => {
                        println!("Withdraw logic here with account_index: {} and amount: {}", account_index, amount);
                        withdraw(&wasm, &contract_address, &accounts[account_index as usize], amount, &accounts_shares_balance);
                        assert_deposit_withdraw(&wasm, contract_address, accounts, &accounts_shares_balance);
                    },
                    Action::Swap { account_index, amount } => {
                        println!("Swap logic here with account_index: {} and amount: {}", account_index, amount);
                        swap(&wasm, &contract_address, &accounts[account_index as usize], amount);
                    },
                    Action::UpdateRange { lower_tick, upper_tick } => {
                        println!("UpdateRange logic here with lower_tick: {} and upper_tick: {}", lower_tick, upper_tick);
                        update_range(&wasm, &contract_address, lower_tick, upper_tick);
                    },
                }
            }
        }
    }

    // Those are just reusable functions, TODO evaluate if they should be like execute_action() inside proptest! macro scope

    fn deposit(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
        amount: u128,
        accounts_shares_balance: &HashMap<String, u128>,
    ) {
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
        let deposit_resp: MsgCreatePositionResponse = deposit.data.try_into();
        let liquidity_created = deposit_resp.liquidity_created;

        // Update map to keep track of user shares amount and make further assertions
        let mut current_shares_amount = accounts_shares_balance.get(&account.address());
        accounts_shares_balance.insert(
            account.address(),
            current_shares_amount.unwrap_or(&0u128).checked_add(liquidity_created),
        );
    }

    fn withdraw(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
        amount: u128,
        accounts_shares_balance: &HashMap<String, u128>,
    ) {
        // TODO: Check user shares balance is not zero and enough accorindlgy to amount_u128

        // TODO: Implement withdraw strategy
    }

    fn swap(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
        amount: u128,
    ) {
        // TODO: Check user bank denom balance is not zero and enough accorindlgy to amount_u128

        // TODO: Implement swap strategy
    }

    fn update_range(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        lower_tick: u128,
        upper_tick: u128,
    ) {
        // TODO: Validate new lower_tick and upper_tick

        // TODO: Mock somehow the range_admin from contract

        // TODO: Implement update range strategy
    }

    /// ASSERTS

    fn assert_deposit_withdraw(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: Addr,
        accounts: Vec<SigningAccount>,
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
            assert_eq!(shares.balance, accounts_shares_balance.get(&account.address()));
        }
    }
}
