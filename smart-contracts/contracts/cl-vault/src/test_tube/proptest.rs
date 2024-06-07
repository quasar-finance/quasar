#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
    use osmosis_std::types::cosmos::bank::v1beta1::{QueryBalanceRequest, QueryBalanceResponse};
    use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse;
    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::PositionByIdRequest;
    use osmosis_std::types::{
        cosmos::base::v1beta1,
        osmosis::concentratedliquidity::poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool,
    };
    use osmosis_test_tube::{
        Account, Bank, ConcentratedLiquidity, ExecuteResponse, Module, OsmosisTestApp,
        SigningAccount, Wasm,
    };
    use proptest::prelude::*;

    use crate::helpers::generic::sort_tokens;
    use crate::query::AssetsBalanceResponse;
    use crate::test_tube::helpers::get_event_attributes_by_ty_and_key;
    use crate::test_tube::initialize::initialize::{MAX_SLIPPAGE_HIGH, PERFORMANCE_FEE_DEFAULT};
    use crate::{
        math::tick::tick_to_price,
        msg::{ExecuteMsg, ExtensionQueryMsg, ModifyRangeMsg, QueryMsg},
        query::{PositionResponse, TotalVaultTokenSupplyResponse},
        query::{TotalAssetsResponse, UserSharesBalanceResponse},
        test_tube::initialize::initialize::init_test_contract,
    };

    const ITERATIONS_NUMBER: usize = 1000;
    const ACCOUNTS_NUMBER: u64 = 10;
    const ACCOUNTS_INITIAL_BALANCE: u128 = 100_000_000_000_000_000;
    const DENOM_BASE: &str = "ZZZZZ";
    const DENOM_QUOTE: &str =
        "ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858";

    #[derive(Clone, Copy, Debug)]
    enum Action {
        Deposit,
        Withdraw,
        //Swap,
        UpdateRange,
    }

    fn deposit(
        wasm: &Wasm<OsmosisTestApp>,
        bank: &Bank<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
        percentage: f64,
        denom0: &str,
        denom1: &str,
    ) {
        // Get user DENOM_BASE balance
        let balance_asset0 = get_user_denom_balance(bank, account, denom0);
        let balance0_str = balance_asset0.balance.unwrap().amount;
        let balance0_f64: f64 = balance0_str
            .parse()
            .expect("Failed to parse balance to f64");
        let amount0 = (balance0_f64 * (percentage / 100.0)).round() as u128;

        // Get user DENOM_QUOTE balance
        let balance_asset1 = get_user_denom_balance(bank, account, denom1);
        let balance1_str = balance_asset1.balance.unwrap().amount;
        let balance1_f64: f64 = balance1_str
            .parse()
            .expect("Failed to parse balance to f64");
        let amount1 = (balance1_f64 * (percentage / 100.0)).round() as u128;

        // Get current pool position to know token0 and token1 amounts
        let pos_assets: TotalAssetsResponse = get_vault_position_assets(wasm, contract_address);

        // Calculate the ratio between pos_asset0 and pos_asset1
        let ratio = pos_assets.token0.amount.u128() as f64 / pos_assets.token1.amount.u128() as f64;

        // Calculate the adjusted amounts to deposit
        let (adjusted_amount0, adjusted_amount1) = if ratio > 1.0 {
            // If ratio is greater than 1, adjust amount1 according to the ratio
            (amount0, (amount0 as f64 / ratio).round() as u128)
        } else {
            // If ratio is less than or equal to 1, adjust amount0 according to the ratio
            ((amount1 as f64 * ratio).round() as u128, amount1)
        };

        // Initialize an empty Vec<Coin> and push only non zero amount coins
        let mut coins_to_deposit = Vec::new();
        if adjusted_amount0 > 0 {
            coins_to_deposit.push(Coin::new(adjusted_amount0, denom0));
        }
        if adjusted_amount1 > 0 {
            coins_to_deposit.push(Coin::new(adjusted_amount1, denom1));
        }

        // Check if coins_to_deposit is not empty before proceeding or skip the iteration
        if coins_to_deposit.is_empty() {
            return;
        }

        // // Before queries
        // let vault_shares_balance_before: TotalVaultTokenSupplyResponse =
        //     get_vault_shares_balance(wasm, contract_address);
        // let vault_position_assets_before: TotalAssetsResponse =
        //     get_vault_position_assets(wasm, contract_address);
        // let user_shares_balance_before: UserBalanceResponse =
        //     get_user_shares_balance(wasm, contract_address, account);

        // Execute deposit
        let create_position: ExecuteResponse<MsgExecuteContractResponse> = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None }, // Nice to have: Make recipient random
                &sort_tokens(coins_to_deposit), // TODO: Why our contract, before adding a message/submessage cannot handle a sort? like first line of deposit::execute_exact_deposit
                account,
            )
            .unwrap();

        // Find the event with "ty": "create_position" and collect the relevant attributes
        let _create_position_attrs = get_event_attributes_by_ty_and_key(
            &create_position,
            "create_position",
            vec!["liquidity", "amount0", "amount1"],
        );
        // let create_amount0 = get_event_value_amount_numeric(&create_position_attrs[1].value);
        // let create_amount1 = get_event_value_amount_numeric(&create_position_attrs[2].value);

        // // Find the event with "ty": "tf_mint" and collect the relevant attributes
        // let tf_mint_attrs =
        //     get_event_attributes_by_ty_and_key(&create_position, "tf_mint", vec!["amount"]);
        // let tf_mint_amount = get_event_value_amount_numeric(&tf_mint_attrs[0].value);

        // // After queries
        // let vault_shares_balance_after: TotalVaultTokenSupplyResponse =
        //     get_vault_shares_balance(wasm, contract_address);
        // let vault_position_assets_after: TotalAssetsResponse =
        //     get_vault_position_assets(wasm, contract_address);
        // let user_shares_balance_after: UserBalanceResponse =
        //     get_user_shares_balance(wasm, contract_address, account);

        // // Use create_position_attrs[0].value to sum over the get_vault_shares_balance() query
        // let liquidity_created_uint_floor =
        //     Decimal256::from_str(create_position_attrs[0].value.as_str())
        //         .unwrap()
        //         .to_uint_floor();
        // let liquidity_created = Uint128::try_from(liquidity_created_uint_floor).unwrap();
        // assert_eq!(
        //     vault_shares_balance_before.total + liquidity_created,
        //     vault_shares_balance_after.total
        // );

        // Use create_amount0 to sum over the get_vault_position_assets() query
        // assert_eq!(
        //     vault_position_assets_before.token0.amount + Uint128::new(create_amount0),
        //     vault_position_assets_after.token0.amount
        // );
        // Use create_amount1 to sum over the get_vault_position_assets() query
        // assert_eq!(
        //     vault_position_assets_before.token1.amount + Uint128::new(create_amount1),
        //     vault_position_assets_after.token1.amount
        // );

        // Use tf_mint_amount to sum over the accounts_shares_balance for the depositing address
        // assert_eq!(
        //     user_shares_balance_before.balance + Uint128::new(tf_mint_amount),
        //     user_shares_balance_after.balance
        // );
    }

    fn withdraw(
        wasm: &Wasm<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
        percentage: f64,
    ) {
        let balance = get_user_shares_balance(wasm, contract_address, account);
        let amount = (balance.balance.u128() as f64 * (percentage / 100.0)).round() as u128;

        // // Before queries
        // let vault_shares_balance_before: TotalVaultTokenSupplyResponse =
        //     get_vault_shares_balance(wasm, contract_address);
        // let vault_position_assets_before: TotalAssetsResponse =
        //     get_vault_position_assets(wasm, contract_address);
        // let user_shares_balance_before: UserBalanceResponse =
        //     get_user_shares_balance(wasm, contract_address, account);

        let _user_assets_bal: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserAssetsBalance {
                        user: account.address(),
                    },
                )),
            )
            .unwrap();

        let _vault_total_shares: TotalAssetsResponse = wasm
            .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
            .unwrap();

        // Execute withdraw
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

        // Find the event with "ty": "withdraw_position" and collect the relevant attributes
        let _withdraw_position_attrs = get_event_attributes_by_ty_and_key(
            &withdraw_position,
            "withdraw_position",
            vec!["liquidity", "amount0", "amount1"],
        );
        //let withdraw_amount0 = get_event_value_amount_numeric(&withdraw_position_attrs[1].value); TODO this shouldnt pass trough get_event_value_amount_numeric as it is: -55190706220
        //let withdraw_amount1 = get_event_value_amount_numeric(&withdraw_position_attrs[2].value);

        // Find the event with "ty": "tf_burn" and collect the relevant attributes
        // let tf_burn_attrs =
        //     get_event_attributes_by_ty_and_key(&withdraw_position, "tf_burn", vec!["amount"]);
        // let tf_burn_amount = get_event_value_amount_numeric(&tf_burn_attrs[0].value);

        // // After queries
        // let vault_shares_balance_after: TotalVaultTokenSupplyResponse =
        //     get_vault_shares_balance(wasm, contract_address);
        // let vault_position_assets_after: TotalAssetsResponse =
        //     get_vault_position_assets(wasm, contract_address);
        // let user_shares_balance_after: UserBalanceResponse =
        //     get_user_shares_balance(wasm, contract_address, account);

        // Use withdraw_position_attrs[0].value to sub over the total_vault_shares_balance
        // let liquidity_withdrawn_uint_floor =
        //     Decimal256::from_str(withdraw_position_attrs[0].value.as_str())
        //         .unwrap()
        //         .to_uint_floor();
        //let liquidity_withdrawn = Uint128::try_from(liquidity_withdrawn_uint_floor).unwrap();
        // assert_eq!(
        //     vault_shares_balance_before.total + liquidity_withdrawn,
        //     vault_shares_balance_after.total
        // );

        // Use withdraw_amount0 to sub over the total_vault_denom_balance ??? maybe this is not needed
        // assert_eq!(
        //     vault_position_assets_before.token0.amount + Uint128::new(withdraw_amount0),
        //     vault_position_assets_after.token0.amount
        // );
        // Use withdraw_amount1 to sub over the total_vault_denom_balance ??? maybe this is not needed
        // assert_eq!(
        //     vault_position_assets_before.token1.amount + Uint128::new(withdraw_amount1),
        //     vault_position_assets_after.token1.amount
        // );

        // Use tf_burn_amount to sub over the accounts_shares_balance for the depositing address
        // assert_eq!(
        //     user_shares_balance_before.balance - Uint128::new(tf_burn_amount),
        //     user_shares_balance_after.balance
        // );
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
        let balance_str = balance_response.balance.unwrap().amount;
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
        let (current_lower_tick, current_upper_tick) =
            get_position_ticks(wasm, cl, contract_address);
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

        // Execute deposit and get liquidity_created from emitted events
        let _update_range = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                    ModifyRangeMsg {
                        lower_price: Decimal::new(Uint128::new(new_lower_price)),
                        upper_price: Decimal::new(Uint128::new(new_upper_price)),
                        max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH), // optimize and check how this fits in the strategy as it could trigger organic errors we dont want to test
                        ratio_of_swappable_funds_to_use: Decimal::one(),
                        twap_window_seconds: 45,
                        recommended_swap_route: None,
                        force_swap_route: false,
                        claim_after: None,
                    },
                )),
                &[],
                admin_account,
            )
            .unwrap();
    }

    // GETTERS

    fn get_user_denom_balance(
        bank: &Bank<OsmosisTestApp>,
        account: &SigningAccount,
        denom: &str,
    ) -> QueryBalanceResponse {
        bank.query_balance(&QueryBalanceRequest {
            address: account.address(),
            denom: denom.to_string(),
        })
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
                crate::msg::UserBalanceQueryMsg::UserSharesBalance {
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
        // query_position will return a Vec of position_ids
        let position_response: PositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();

        // TODO Use those to take the latest one? or what?
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
        // TODO: evaluate if lower_tick and upper_tick are too much arbitrary
        fn get_initial_range()(lower_tick in -300_000i64..0, upper_tick in 1i64..500_000) -> (i64, i64) {
            (lower_tick, upper_tick)
        }
    }

    // get_strategy_list
    prop_compose! {
        fn get_strategy_list()(list in prop::collection::vec(prop_oneof![
            Just(Action::Deposit),
            Just(Action::Withdraw),
            //Just(Action::Swap),
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

    // TESTS
    proptest! {
        // setup the config with amount of cases, usable for setting different values on ci vs local
        #![proptest_config(ProptestConfig::with_cases(get_cases()))]
        #[test]
        #[ignore]
        fn test_complete_works(
            (initial_lower_tick, initial_upper_tick) in get_initial_range(),
            actions in get_strategy_list(),
            percentages in get_percentage_list(),
            account_indexes in get_account_index_list()
        ) {
            // Creating test core
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
                        amount: "1000000000000000000".to_string(),
                    },
                    v1beta1::Coin {
                        denom: DENOM_QUOTE.to_string(),
                        amount: "1000000000000000000".to_string(),
                    },
                ],
                Uint128::zero(),
                Uint128::zero(),
                PERFORMANCE_FEE_DEFAULT
            );
            let wasm = Wasm::new(&app);
            let cl = ConcentratedLiquidity::new(&app);
            let bank = Bank::new(&app);

            // Create a fixed number of accounts using app.init_accounts() function from test-tube, and assign a fixed initial balance for all of them
            let accounts = app
                .init_accounts(&[
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, "uosmo"),
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_QUOTE),
                ], ACCOUNTS_NUMBER)
                .unwrap();

            // Make one arbitrary deposit foreach one of the created accounts using 10.00% of its balance, to avoid complications on withdrawing without any position
            for i in 0..ACCOUNTS_NUMBER {
                deposit(&wasm, &bank, &contract_address, &accounts[i as usize], 10.00, DENOM_BASE, DENOM_QUOTE);
            }

            // Iterate iterations times
            for i in 0..ITERATIONS_NUMBER {
                match actions[i] {
                    Action::Deposit => {
                        deposit(&wasm, &bank, &contract_address, &accounts[account_indexes[i] as usize], percentages[i], DENOM_BASE, DENOM_QUOTE);
                    },
                    Action::Withdraw => {
                        withdraw(&wasm, &contract_address, &accounts[account_indexes[i] as usize], percentages[i]);
                    },
                    // Action::Swap => {
                    //     swap(&wasm, &bank, &contract_address, &accounts[account_indexes[i] as usize], percentages[i], cl_pool_id);
                    // },
                    Action::UpdateRange => {
                        update_range(&wasm, &cl, &contract_address, percentages[i], &admin_account);
                    },
                }
            }

            println!("PASS");
        }
    }
}
