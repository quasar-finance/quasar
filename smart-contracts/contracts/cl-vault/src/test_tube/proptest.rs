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

    const ACCOUNTS_NUMBER: u64 = 10;
    const ACCOUNTS_INITIAL_BALANCE: u128 = 1_000_000_000_000;
    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";

    #[derive(Clone, Copy, Debug)]
    enum Action {
        Deposit(u128),
        Withdraw(u128),
        UpdateRange(u128),
        Swap(u128),
    }

    proptest! {
        /// Main test function
        #[test]
        fn test_complete_works(
            // generate a random number of deposit_amounts
            (deposit_amounts) in (0usize..100).prop_flat_map(|size|
                (
                    // to avoid overflows, we limit the amounts to u64. also force amounts to be >= 1
                    vec(any::<u64>().prop_map(|x| (x as u128).max(1)), size..=size),
                )
            ),
            (iterations) in (0usize..100).prop_flat_map(|size|
                (
                    // to avoid overflows, we limit the amounts to u64. also force amounts to be >= 1
                    vec(any::<u64>().prop_map(|x| (x as u128).max(1)), size..=size),
                )
            ),
        ) {
            // Creating test core
            let (app, contract_address, _cl_pool_id, _admin) = default_init();
            let wasm = Wasm::new(&app);

            // Creating test vars
            let mut accounts_shares_balance: HashMap<String, u128> = HashMap::new();

            // Create a fixed number of accounts using app.init_accounts() function from test-tube, and assign a fixed initial balance for all of them
            let accounts = app
                .init_accounts(&[
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_QUOTE),
                ], ACCOUNTS_NUMBER)
                .unwrap();

            // TODO: Make random deposit, random in number of deposits, and random amount in info.funds too
            // We could generate a random Vec of Uint128 that we order ASC in order to know which user,
            // how much he deposited, to check later that each user shares should be < or > than the previous one? Or how do we test that the balance is ok?
            deposit(wasm, contract_address, accounts, deposit_amounts, &accounts_shares_balance);

            // Here we know all the users have deposited. We can start executing random strategies here.

            // TODO: Iterate N times and run prop_oneof![]
            for i in iterations {
                execute_action();
            }
        }

        /// Non test functions such as execute handler with prop_oneof! This remains inside the proptest macro.

        fn execute_action(
            action in prop_oneof![
            prop::num::u128::ANY.prop_map(Action::Deposit),
            prop::num::u128::ANY.prop_map(Action::Withdraw),
            prop::num::u128::ANY.prop_map(Action::UpdateRange),
            prop::num::u128::ANY.prop_map(Action::Swap),
        ]) {
            // Perform the test logic here based on the action.
            match action {
                Action::Deposit(amount) => {
                    println!("Deposit logic here with amount: {}", amount);
                    deposit(wasm, contract_address, accounts, deposit_amounts, &accounts_shares_balance);
                },
                Action::Withdraw(amount) => {
                    println!("Withdraw logic here with amount: {}", amount);
                    withdraw(wasm, contract_address, accounts, deposit_amounts, &accounts_shares_balance);
                },
                Action::UpdateRange(amount) => {
                    println!("UpdateRange logic here with amount: {}", amount);
                },
                Action::Swap(amount) => {
                    println!("Swap logic here with amount: {}", amount);
                },
            }
        }
    }

    // Those are just reusable functions, TODO evaluate if they should be like execute_action() inside proptest! macro scope

    // TODO: Deposit strategy.
    // Generate a random number between 1 and ACCOUNTS_NUMBER and use it to iterate over accounts.
    // Foreach user query its bank denom balances, check is not zero, and generate a random percentage to calculate the amount of their remaining balance to withdraw.
    // Is very probable that all users will remain with some denom balance after this. Is what we want.
    fn deposit(
        wasm: Wasm<OsmosisTestApp>,
        contract_address: Addr,
        accounts: Vec<SigningAccount>,
        deposit_amounts: Vec<u128>,
        accounts_shares_balance: &HashMap<String, u128>,
    ) {
        for deposit_amount in deposit_amounts {
            // Generate random number between 0 and len(accounts) to pick one of the addresses to execute the deposit
            let account_index = proptest::num::u128::ANY.between(0, ACCOUNTS_NUMBER - 1); // TODO check is working

            // TODO: Get current pool position to know asset0 and asset1 as /osmosis.concentratedliquidity.v1beta1.FullPositionBreakdown
            let (amount0, amount1) = (1000u128, 1000u128); // mocked

            // TODO: Execute deposit and get liquidity_created from emitted events
            let deposit = wasm.execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None }, // TODO: Make recipient random
                &[Coin::new(amount0, DENOM_BASE), Coin::new(amount1, DENOM_QUOTE)],
                &accounts[account_index],
            ).unwrap();
            let deposit_resp: MsgCreatePositionResponse = deposit.data.try_into()?;
            let liquidity_created = deposit_resp.liquidity_created?;

            // TODO: Update map to keep track of user shares amount and make further assertions
            let mut current_shares_amount = accounts_shares_balance.get(&accounts[account_index].address());
            accounts_shares_balance.insert(
                accounts[account_index].address(),
                current_shares_amount.unwrap_or(&0u128).checked_add(liquidity_created),
            );

            assert_deposit_withdraw(wasm, contract_address, accounts, &accounts_shares_balance);
        }
    }

    // TODO: Withdraw strategy.
    // Generate a random number between 1 and ACCOUNTS_NUMBER and use it to iterate over accounts.
    // Foreach user query its shares balance, check is not zero, and generate a random percentage to calculate the amount of their total shares stack to withdraw.
    // Is very probable that all users will remain with some shares balance after this. Is what we want.
    fn withdraw(
        wasm: Wasm<OsmosisTestApp>,
        contract_address: Addr,
        accounts: Vec<SigningAccount>,
        deposit_amounts: Vec<u128>,
        accounts_shares_balance: &HashMap<String, u128>,
    ) {
        todo!();
    }


    // TODO: Update ranges strategy
    // Generate random lower and upper ticks in a specified range and execute the update ranges.
    fn update_range() {
        todo!();
    }

    // TODO: Swap assets to move prices
    // Document this
    fn swap() {
        todo!();
    }

    /// ASSERTS

    fn assert_deposit_withdraw(
        wasm: Wasm<OsmosisTestApp>,
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
