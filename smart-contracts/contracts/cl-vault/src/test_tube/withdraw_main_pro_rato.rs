#[cfg(test)]
mod tests {
    use cosmwasm_std::{assert_approx_eq, Coin, Decimal256, Uint128};

    use osmosis_std::types::{
        cosmos::{bank::v1beta1::QueryBalanceRequest, base::v1beta1},
        osmosis::concentratedliquidity::v1beta1::{PositionByIdRequest, PositionByIdResponse},
    };
    use osmosis_test_tube::{Account, Bank, ConcentratedLiquidity, Module, Wasm};

    use crate::{
        assert_eq_with_diff, msg::{ClQueryMsg, ExecuteMsg, ExtensionQueryMsg, QueryMsg}, query::{
            AssetsBalanceResponse, MainPositionResponse, PositionsResponse, TotalAssetsResponse, UserSharesBalanceResponse
        }, test_tube::{
            helpers::{get_event_attributes_by_ty_and_key, get_value_in_asset0},
            initialize::initialize::{
                fixture_default, DENOM_BASE, DENOM_QUOTE, PERFORMANCE_FEE_DEFAULT,
            },
        }
    };

    const INITIAL_BALANCE_AMOUNT: u128 = 1_000_000_000_000_000_000_000_000_000_000;

    // TODO assert that this withdraw hits both main_position aswell as pro rato withdraw
    #[test]
    #[ignore]
    fn withdraw_main_and_pro_rato_return_same_value() {
        let (app, contract_address, cl_pool_id, admin, _deposit_ratio, _deposit_ratio_approx) =
            fixture_default(PERFORMANCE_FEE_DEFAULT);
        let wasm = Wasm::new(&app);
        let bank = Bank::new(&app);
        let cl = ConcentratedLiquidity::new(&app);

        // Create Alice account
        let alice = app
            .init_account(&[
                Coin::new(INITIAL_BALANCE_AMOUNT, "uosmo"),
                Coin::new(INITIAL_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(INITIAL_BALANCE_AMOUNT, DENOM_QUOTE),
            ])
            .unwrap();

        let vault_assets_before: TotalAssetsResponse = wasm
            .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
            .unwrap();

        // Get user_assets for Alice from vault contract and assert
        let _user_assets: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserAssetsBalance {
                        user: alice.address(),
                    },
                )),
            )
            .unwrap();

        // TODO: Check this -> Certain deposit amounts do not work here due to an off by one error in Osmosis cl code. The value here is chosen to specifically work
        /*
        user:assets: AssetsBalanceResponse { balances: [Coin { 281243579389884 "uatom" }, Coin { 448554353093648 "uosmo" }] }
        1_000_000_000_000_000
        0_448_554_353_093_648
        0_281_243_579_389_884
        so these tokens could 2x easily
         */

        let deposit0 = 1_000_000_000_000_000;
        let deposit1 = 1_000_000_000_000_000;

        let response = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(deposit0, DENOM_BASE),
                    Coin::new(deposit1, DENOM_QUOTE),
                ],
                &alice,
            )
            .unwrap();

            wasm.execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Autocompound {}),
                &[],
                &admin,
            )
            .unwrap();

        let _vault_assets_after: TotalAssetsResponse = wasm
            .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
            .unwrap();

        // assert that the refund + used funds are equal to what we deposited
        let refund0: u128 =
            get_event_attributes_by_ty_and_key(&response, "wasm", vec!["refund0_amount"])
                .get(0)
                .map(|attr| attr.value.parse().unwrap())
                .unwrap_or(0);
        let refund1: u128 =
            get_event_attributes_by_ty_and_key(&response, "wasm", vec!["refund1_amount"])
                .get(0)
                .map(|attr| attr.value.parse().unwrap())
                .unwrap_or(0);

        let deposited0: u128 =
            get_event_attributes_by_ty_and_key(&response, "wasm", vec!["amount0"])
                .get(0)
                .map(|attr| attr.value.parse().unwrap())
                .unwrap_or(0);
        let deposited1: u128 =
            get_event_attributes_by_ty_and_key(&response, "wasm", vec!["amount1"])
                .get(0)
                .map(|attr| attr.value.parse().unwrap())
                .unwrap_or(0);

        assert_eq!(
            deposit0 + deposit1,
            refund0 + refund1 + deposited0 + deposited1
        );

        // Get shares for Alice from vault contract and assert
        let shares: UserSharesBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserSharesBalance {
                        user: alice.address(),
                    },
                )),
            )
            .unwrap();
        assert!(!shares.balance.is_zero());

        // TODO should we calc from shares or userAssetsBalance
        let user_value: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::ConvertToAssets {
                    amount: shares.balance,
                },
            )
            .unwrap();

        assert_approx_eq!(
            user_value.balances[0].amount,
            Uint128::from(deposited0),
            "0.000001"
        );
        assert_approx_eq!(
            user_value.balances[1].amount,
            Uint128::from(deposited1),
            "0.000001"
        );

        // Get user_assets for Alice from vault contract and assert
        let user_assets: AssetsBalanceResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                    crate::msg::UserBalanceQueryMsg::UserAssetsBalance {
                        user: alice.address(),
                    },
                )),
            )
            .unwrap();

        // assert the token0 deposited by alice by checking the balance of alice
        // we expect sent - refunded here, or 627_000_000_000_000
        // TODO, The UserAssetsBalance query here returns too little, so either we mint too little or the query works incorrect
        assert_approx_eq!(
            user_assets.balances[0].amount,
            Uint128::from(deposited0),
            "0.000001"
        );

        // assert the token1 deposited by alice
        assert_approx_eq!(
            user_assets.balances[1].amount,
            Uint128::from(deposited1),
            "0.000001"
        );

        // Get vault assets and assert
        let vault_assets: TotalAssetsResponse = wasm
            .query(contract_address.as_str(), &QueryMsg::TotalAssets {})
            .unwrap();
        assert_approx_eq!(
            vault_assets.token0.amount,
            vault_assets_before
                .token0
                .amount
                .checked_add(Uint128::from(deposited0)) // TODO: remove hardcoded
                .unwrap(),
            "0.000001"
        );

        // Assert vault assets taking in account the refunded amount to Alice, so we only expect around 500 to deposit here
        assert_approx_eq!(
            vault_assets.token1.amount,
            vault_assets_before
                .token1
                .amount
                .checked_add(Uint128::from(deposited1)) // TODO: remove hardcoded
                .unwrap(),
            "0.000001"
        );

        let shares_to_withdraw = shares.balance / Uint128::new(2_u128);

        // withdraw funds from the main position
        let token0 = bank
            .query_balance(&QueryBalanceRequest {
                address: alice.address(),
                denom: DENOM_BASE.to_string(),
            })
            .unwrap();
        let token1 = bank
            .query_balance(&QueryBalanceRequest {
                address: alice.address(),
                denom: DENOM_QUOTE.to_string(),
            })
            .unwrap();

        let token0 = token0.balance.unwrap_or(v1beta1::Coin {
            denom: DENOM_BASE.to_string(),
            amount: 0.to_string(),
        });
        let token1 = token1.balance.unwrap_or(v1beta1::Coin {
            denom: DENOM_QUOTE.to_string(),
            amount: 0.to_string(),
        });
        let alice_balance_value_before = get_value_in_asset0(
            &app,
            cl_pool_id,
            token0.try_into().unwrap(),
            token1.try_into().unwrap(),
        )
        .unwrap();
        let withdraw = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: shares_to_withdraw,
                },
                &[],
                &alice,
            )
            .unwrap();
        let source = get_event_attributes_by_ty_and_key(&withdraw, "wasm", vec!["source"]);
        assert_eq!(source[0].value, "main_position");

        let token0 = bank
            .query_balance(&QueryBalanceRequest {
                address: alice.address(),
                denom: DENOM_BASE.to_string(),
            })
            .unwrap();
        let token1 = bank
            .query_balance(&QueryBalanceRequest {
                address: alice.address(),
                denom: DENOM_QUOTE.to_string(),
            })
            .unwrap();
        let alice_balance_value_after = get_value_in_asset0(
            &app,
            cl_pool_id,
            token0.balance.unwrap().try_into().unwrap(),
            token1.balance.unwrap().try_into().unwrap(),
        )
        .unwrap();

        let first_difference = alice_balance_value_after - alice_balance_value_before;

        let main_position: MainPositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                    ClQueryMsg::MainPosition,
                )),
            )
            .unwrap();
        let position: PositionByIdResponse = cl
            .query_position_by_id(&PositionByIdRequest {
                position_id: main_position.position_id,
            })
            .unwrap();
        
        // println!("position {:?}", position);
        // let liquidity: Decimal256 = position
        //     .position
        //     .unwrap()
        //     .position
        //     .unwrap()
        //     .liquidity
        //     .parse()
        //     .unwrap();

        // // withdraw funds from the main position into the free balance such that we force a ratio withdraw
        // let _res = wasm
        //     .execute(
        //         contract_address.as_str(),
        //         &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
        //             crate::msg::ModifyRange::DecreaseFunds {
        //                 position_id: main_position.position_id,
        //                 liquidity: liquidity
        //                     .checked_mul(Decimal256::from_ratio(9_u128, 10_u128))
        //                     .unwrap(),
        //             },
        //         )),
        //         &[],
        //         &admin,
        //     )
        //     .unwrap();

        // withdraw funds from the main position
        let token0 = bank
            .query_balance(&QueryBalanceRequest {
                address: alice.address(),
                denom: DENOM_BASE.to_string(),
            })
            .unwrap();
        let token1 = bank
            .query_balance(&QueryBalanceRequest {
                address: alice.address(),
                denom: DENOM_QUOTE.to_string(),
            })
            .unwrap();
        let alice_balance_value_before = get_value_in_asset0(
            &app,
            cl_pool_id,
            token0.balance.unwrap().try_into().unwrap(),
            token1.balance.unwrap().try_into().unwrap(),
        )
        .unwrap();

        let withdraw = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: shares_to_withdraw,
                },
                &[],
                &alice,
            )
            .unwrap();
            let source = get_event_attributes_by_ty_and_key(&withdraw, "wasm", vec!["source"]);
            assert_eq!(source[0].value, "all_positions");

        let token0 = bank
            .query_balance(&QueryBalanceRequest {
                address: alice.address(),
                denom: DENOM_BASE.to_string(),
            })
            .unwrap();
        let token1 = bank
            .query_balance(&QueryBalanceRequest {
                address: alice.address(),
                denom: DENOM_QUOTE.to_string(),
            })
            .unwrap();
        let alice_balance_value_after = get_value_in_asset0(
            &app,
            cl_pool_id,
            token0.balance.unwrap().try_into().unwrap(),
            token1.balance.unwrap().try_into().unwrap(),
        )
        .unwrap();

        let second_difference = alice_balance_value_after - alice_balance_value_before;


        let positions: PositionsResponse = wasm.query(contract_address.as_str(), &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(ClQueryMsg::Positions {}))).unwrap();
        // since we are withdrawing from multiple different positions here in the second difference, we might encounter multiple
        // moments where we round down, hence we can be off by 1 * the amount of positions + 1 for the free funds.
        // since each of those calculations is a potential round down
        let allowed_absolute_diff = positions.positions.len() as u128 + 1_u128;
        assert_eq_with_diff!(first_difference, "main position withdraw value", second_difference, "all positions withdraw value", "0", Uint128::new(allowed_absolute_diff), "difference between withdraws through the main position is too big")
    }
}
