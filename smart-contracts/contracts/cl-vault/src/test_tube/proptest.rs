#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{assert_approx_eq, Addr, Attribute, Coin, Decimal, Uint128};
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

    use crate::{
        helpers::sort_tokens,
        math::tick::tick_to_price,
        msg::{ExecuteMsg, ExtensionQueryMsg, ModifyRangeMsg, QueryMsg},
        query::{PositionResponse, TotalVaultTokenSupplyResponse},
        query::{TotalAssetsResponse, UserBalanceResponse},
        test_tube::initialize::initialize::init_test_contract,
    };

    // SETTINGS

    const ITERATIONS_NUMBER: usize = 1000;
    const ACCOUNTS_NUMBER: u64 = 10;
    const MAX_PERCENTAGE: f64 = 100.0;
    const ACCOUNTS_INITIAL_BALANCE: u128 = 1_000_000_000_000;
    const DENOM_BASE: &str = "ZZZZZ";
    const DENOM_QUOTE: &str =
        "ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858";
    const DENOM_FEE: &str = "uosmo"; // You cant change that

    // WORKFLOWS

    fn deposit(
        wasm: &Wasm<OsmosisTestApp>,
        bank: &Bank<OsmosisTestApp>,
        contract_address: &Addr,
        account: &SigningAccount,
        percentage: f64,
    ) {
        // Get user DENOM_BASE balance
        let balance_asset0 = get_user_denom_balance(bank, account, DENOM_BASE);
        let balance0_str = balance_asset0.balance.unwrap().amount;
        let balance0_f64: f64 = balance0_str
            .parse()
            .expect("Failed to parse balance to f64");
        let amount0 = (balance0_f64 * (percentage / 100.0)).round() as u128;

        // Get user DENOM_QUOTE balance
        let balance_asset1 = get_user_denom_balance(bank, account, DENOM_QUOTE);
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
            coins_to_deposit.push(Coin::new(adjusted_amount0, DENOM_BASE));
        }
        if adjusted_amount1 > 0 {
            coins_to_deposit.push(Coin::new(adjusted_amount1, DENOM_QUOTE));
        }

        // Check if coins_to_deposit is not empty before proceeding or skip the iteration
        if coins_to_deposit.is_empty() {
            return;
        }

        // Before queries
        let vault_shares_balance_before: TotalVaultTokenSupplyResponse =
            get_vault_shares_balance(wasm, contract_address);
        let vault_position_assets_before: TotalAssetsResponse =
            get_vault_position_assets(wasm, contract_address);
        let user_shares_balance_before: UserBalanceResponse =
            get_user_shares_balance(wasm, contract_address, account);

        // Execute deposit
        let create_position: ExecuteResponse<MsgExecuteContractResponse> = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None }, // Nice to have: Make recipient random
                &sort_tokens(coins_to_deposit), // TODO: Why our contract, before adding a message/submessage cannot handle a sort? like first line of deposit.rs::execute_exact_deposit
                account,
            )
            .unwrap();

        // After queries
        let vault_shares_balance_after: TotalVaultTokenSupplyResponse =
            get_vault_shares_balance(wasm, contract_address);
        let vault_position_assets_after: TotalAssetsResponse =
            get_vault_position_assets(wasm, contract_address);
        let user_shares_balance_after: UserBalanceResponse =
            get_user_shares_balance(wasm, contract_address, account);

        // Assert Total Vault Shares using tf_mint_amount to sum over the get_vault_shares_balance() query before and after
        let tf_mint_attrs =
            get_event_attributes_by_ty_and_key(&create_position, "tf_mint", vec!["amount"]);
        let tf_mint_amount = get_event_value_amount_numeric(&tf_mint_attrs[0].value);
        assert_eq!(
            vault_shares_balance_before.total + Uint128::new(tf_mint_amount),
            vault_shares_balance_after.total
        );

        // Find the event with "ty": "create_position" and collect the relevant attributes
        let create_position_attrs = get_event_attributes_by_ty_and_key(
            &create_position,
            "create_position",
            vec!["amount0", "amount1"], // TODO: Maybe we should assert also the "liquidity"
        );

        // Use create_amount0 to sum over the get_vault_position_assets() query
        let create_amount0 = Uint128::from_str(&create_position_attrs[0].value).unwrap();
        // TODO: Optimize this condition
        if vault_position_assets_before.token0.amount != Uint128::zero()
            && vault_position_assets_after.token0.amount != Uint128::zero()
        {
            assert_approx_eq!(
                vault_position_assets_before
                    .token0
                    .amount
                    .checked_add(create_amount0)
                    .unwrap(),
                vault_position_assets_after.token0.amount,
                "0.002" // TODO: Optimize this assert
            );
        }

        // Use create_amount1 to sum over the get_vault_position_assets() query
        let create_amount1 = Uint128::from_str(&create_position_attrs[1].value).unwrap();
        // TODO: Optimize this condition
        if vault_position_assets_before.token1.amount != Uint128::zero()
            && vault_position_assets_after.token1.amount != Uint128::zero()
        {
            assert_approx_eq!(
                vault_position_assets_before
                    .token1
                    .amount
                    .checked_add(create_amount1)
                    .unwrap(),
                vault_position_assets_after.token1.amount,
                "0.002" // TODO: Optimize this assert
            );
        }

        // Assert User Shares Balance using tf_mint_amount to sum with previous balance and compare to current
        assert_eq!(
            user_shares_balance_before.balance + Uint128::new(tf_mint_amount),
            user_shares_balance_after.balance
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

        // Before queries
        let vault_shares_balance_before: TotalVaultTokenSupplyResponse =
            get_vault_shares_balance(wasm, contract_address);
        let vault_position_assets_before: TotalAssetsResponse =
            get_vault_position_assets(wasm, contract_address);
        let user_shares_balance_before: UserBalanceResponse =
            get_user_shares_balance(wasm, contract_address, account);

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

        // After queries
        let vault_shares_balance_after: TotalVaultTokenSupplyResponse =
            get_vault_shares_balance(wasm, contract_address);
        let vault_position_assets_after: TotalAssetsResponse =
            get_vault_position_assets(wasm, contract_address);
        let user_shares_balance_after: UserBalanceResponse =
            get_user_shares_balance(wasm, contract_address, account);

        // Assert Total Vault Shares using tf_burn_amount to sub over the total_vault_shares_balance
        let tf_burn_attrs =
            get_event_attributes_by_ty_and_key(&withdraw_position, "tf_burn", vec!["amount"]);
        let tf_burn_amount = get_event_value_amount_numeric(&tf_burn_attrs[0].value);
        assert_eq!(
            vault_shares_balance_before.total - Uint128::new(tf_burn_amount),
            vault_shares_balance_after.total
        );

        // Find the event with "ty": "withdraw_position" and collect the relevant attributes
        let withdraw_position_attrs = get_event_attributes_by_ty_and_key(
            &withdraw_position,
            "withdraw_position",
            vec!["amount0", "amount1"], // TODO: Maybe we should assert also the "liquidity"
        );

        // Use withdraw_amount0 to sub over the total_vault_denom_balance
        let withdraw_amount0 = Uint128::from_str(
            &withdraw_position_attrs[0]
                .value
                .trim_start_matches("-")
                .to_string(),
        )
        .unwrap();
        let left = vault_position_assets_before
            .token0
            .amount
            .checked_sub(withdraw_amount0)
            .unwrap();
        let right = vault_position_assets_after.token0.amount;
        // TODO: Optimize this assert
        assert!(
            (left >= right && left <= right + Uint128::from(1u128))
                || (left <= right && left + Uint128::from(1u128) >= right),
            "Left and Right are not within the +1/-1 range. Left: {}, Right: {}",
            left,
            right
        );

        // Use withdraw_amount1 to sub over the total_vault_denom_balance
        let withdraw_amount1 = Uint128::from_str(
            &withdraw_position_attrs[1]
                .value
                .trim_start_matches("-")
                .to_string(),
        )
        .unwrap();
        let left = vault_position_assets_before
            .token1
            .amount
            .checked_sub(withdraw_amount1)
            .unwrap();
        let right = vault_position_assets_after.token1.amount;
        // TODO: Optimize this assert
        assert!(
            (left >= right && left <= right + Uint128::from(1u128))
                || (left <= right && left + Uint128::from(1u128) >= right),
            "Left and Right are not within the +1/-1 range. Left: {}, Right: {}",
            left,
            right
        );

        // Assert User Shares Balance using tf_burn_amount to subtract from previous balance and compare to current
        assert_eq!(
            user_shares_balance_before
                .balance
                .checked_sub(Uint128::new(tf_burn_amount))
                .unwrap(),
            user_shares_balance_after.balance
        );
    }

    // fn swap(
    //     _wasm: &Wasm<OsmosisTestApp>,
    //     bank: &Bank<OsmosisTestApp>,
    //     _contract_address: &Addr,
    //     account: &SigningAccount,
    //     percentage: f64,
    //     _cl_pool_id: u64,
    // ) {
    //     let balance_response = get_user_denom_balance(bank, account, DENOM_BASE);
    //     let balance_str = balance_response.balance.unwrap().amount;
    //     let balance_f64: f64 = balance_str.parse().expect("Failed to parse balance to f64");
    //     let _amount = (balance_f64 * (percentage / 100.0)).round() as u128;

    //     // TODO: Check user bank denom balance is not zero and enough accordingly to amount_u128

    //     // TODO: Implement swap strategy
    // }

    fn update_range(
        wasm: &Wasm<OsmosisTestApp>,
        cl: &ConcentratedLiquidity<OsmosisTestApp>,
        contract_address: &Addr,
        percentage: f64,
        admin_account: &SigningAccount,
    ) {
        // Before queries
        let (lower_tick_before, upper_tick_before) = get_position_ticks(wasm, cl, contract_address);
        let (lower_price_before, upper_price_before) = (
            tick_to_price(lower_tick_before).unwrap(),
            tick_to_price(upper_tick_before).unwrap(),
        );
        let clp_u128: Uint128 = lower_price_before.atomics().try_into().unwrap();
        let cup_u128: Uint128 = upper_price_before.atomics().try_into().unwrap();

        // Create new range ticks based on previous ticks by percentage variation
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
                        max_slippage: Decimal::bps(5),
                    },
                )),
                &[],
                admin_account,
            )
            .unwrap();

        // After queries
        let (lower_tick_after, upper_tick_after) = get_position_ticks(wasm, cl, contract_address);
        let (lower_price_after, upper_price_after) = (
            tick_to_price(lower_tick_after).unwrap(),
            tick_to_price(upper_tick_after).unwrap(),
        );
        let alp_u128: Uint128 = lower_price_after.atomics().try_into().unwrap();
        let aup_u128: Uint128 = upper_price_after.atomics().try_into().unwrap();

        // As we do before we get ticks, convert to price and compare the before with after
        assert_approx_eq!(alp_u128, Uint128::new(new_lower_price), "0.000001"); // TODO: Optimize this, maybe changing with assert_eq with rounding on the left arg
        assert_approx_eq!(aup_u128, Uint128::new(new_upper_price), "0.000001"); // TODO: Optimize this
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

    fn get_vault_shares_balance(
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
    ) -> UserBalanceResponse {
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
        let current_position: PositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::Position {},
                )),
            )
            .unwrap();

        // Assert we have only one positoin
        assert!(current_position.position_ids.len() == 1);

        let single_position = cl
            .query_position_by_id(&PositionByIdRequest {
                position_id: current_position.position_ids[0],
            })
            .unwrap()
            .position
            .unwrap()
            .position
            .unwrap();

        (single_position.lower_tick, single_position.upper_tick)
    }

    // HELPERS

    fn get_event_attributes_by_ty_and_key(
        response: &ExecuteResponse<MsgExecuteContractResponse>,
        ty: &str,
        keys: Vec<&str>,
    ) -> Vec<Attribute> {
        response
            .events
            .iter()
            .filter(|event| event.ty == ty)
            .flat_map(|event| event.attributes.clone())
            .filter(|attribute| keys.contains(&attribute.key.as_str()))
            .collect()
    }

    fn get_event_value_amount_numeric(value: &String) -> u128 {
        // Find the position where the non-numeric part starts
        let pos = value.find(|c: char| !c.is_numeric()).unwrap_or(value.len());
        // Extract the numeric part from the string
        let numeric_part = &value[0..pos];
        // Try to parse the numeric string to u128
        numeric_part.parse::<u128>().unwrap()
    }

    // COMPOSE STRATEGY

    // Actions enum
    #[derive(Clone, Copy, Debug)]
    enum Action {
        Deposit,
        Withdraw,
        // Swap,
        UpdateRange,
    }

    // get_initial_range generates random lower and upper ticks for the initial position
    prop_compose! {
        fn get_initial_range()(lower_tick in -1_000_000i64..1_000_000, upper_tick in 1_000_001i64..2_000_000) -> (i64, i64) {
            (lower_tick, upper_tick)
        }
    }

    // get_strategy_list
    prop_compose! {
        fn get_strategy_list()(list in prop::collection::vec(prop_oneof![
            Just(Action::Deposit),
            Just(Action::Withdraw),
            // Just(Action::Swap),
            Just(Action::UpdateRange),
        ], ITERATIONS_NUMBER..ITERATIONS_NUMBER+1)) -> Vec<Action> {
            list
        }
    }

    // get_percentage generates a list of random percentages used to calculate deposit_amount,
    // withdraw_amount, and newers lower and upper ticks based on the previous values
    prop_compose! {
        fn get_percentage_list()(list in prop::collection::vec(1.0..MAX_PERCENTAGE, ITERATIONS_NUMBER..ITERATIONS_NUMBER+1)) -> Vec<f64> {
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
        #[test]
        #[ignore]
        fn test_complete_works(
            (initial_lower_tick, initial_upper_tick) in get_initial_range(),
            actions in get_strategy_list(),
            percentages in get_percentage_list(),
            account_indexes in get_account_index_list()
        ) {
            // Creating test core
            let (app, contract_address, _cl_pool_id, admin_account) = init_test_contract(
                "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
                &[
                    Coin::new(100_000_000_000_000_000_000_000, "uosmo"),
                    Coin::new(100_000_000_000_000_000_000_000, DENOM_BASE),
                    Coin::new(100_000_000_000_000_000_000_000, DENOM_QUOTE),
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
                        amount: "100000000000000000".to_string(),
                    },
                    v1beta1::Coin {
                        denom: DENOM_QUOTE.to_string(),
                        amount: "100000000000000000".to_string(),
                    },
                ],
                Uint128::zero(),
                Uint128::zero(),
            );
            let wasm = Wasm::new(&app);
            let cl = ConcentratedLiquidity::new(&app);
            let bank = Bank::new(&app);

            // Create a fixed number of accounts using app.init_accounts() function from test-tube, and assign a fixed initial balance for all of them
            let accounts = app
                .init_accounts(&[
                    Coin::new(100_000_000_000_000_000_000_000, DENOM_FEE),
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_QUOTE),
                ], ACCOUNTS_NUMBER)
                .unwrap();

            // Make one arbitrary deposit foreach one of the created accounts using 10.00% of its balance, to avoid complications on withdrawing without any position
            for i in 0..ACCOUNTS_NUMBER {
                deposit(&wasm, &bank, &contract_address, &accounts[i as usize], 10.00);
            }

            // Iterate ITERATIONS_NUMBER times
            for i in 0..ITERATIONS_NUMBER {
                match actions[i] {
                    Action::Deposit => {
                        deposit(&wasm, &bank, &contract_address, &accounts[account_indexes[i] as usize], percentages[i]);
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
