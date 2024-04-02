// redeposit

#[cfg(test)]
mod tests {
    use apollo_cw_asset::AssetInfoBase;
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::Coin;
    use cosmwasm_std::{Decimal, Uint128};
    use cw_dex::osmosis::OsmosisPool;
    use cw_dex_router::operations::{SwapOperationBase, SwapOperationsListUnchecked};
    use cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension;
    use osmosis_std::types::cosmos::bank::v1beta1::{
        MsgSend, QueryAllBalancesRequest, QueryBalanceRequest,
    };
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SwapAmountInRoute,
    };
    use osmosis_test_tube::RunnerError::ExecuteError;
    use osmosis_test_tube::{Account, Bank, Module, PoolManager, Wasm};
    use std::str::FromStr;

    use crate::msg::ClQueryMsg::SharePrice;
    use crate::msg::UserBalanceQueryMsg::UserSharesBalance;
    use crate::msg::{AutoCompoundAsset, ExecuteMsg, ExtensionQueryMsg, ModifyRangeMsg, QueryMsg};
    use crate::query::{SharePriceResponse, UserSharesBalanceResponse};
    use crate::state::USER_REWARDS;
    use crate::test_tube::helpers::{get_amount_from_denom, get_event_attributes_by_ty_and_key};
    use crate::test_tube::initialize::initialize::{
        default_init, dex_cl_init_cl_pools, dex_cl_init_lp_pools,
    };

    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";
    const DENOM_REWARD: &str = "ustride";
    const ACCOUNTS_NUM: u64 = 10;
    const ACCOUNTS_INIT_BALANCE: u128 = 1_000_000_000_000_000;
    const DEPOSIT_AMOUNT: u128 = 5_000_000;
    const SWAPS_NUM: usize = 10;
    const SWAPS_AMOUNT: &str = "1000000000";

    #[test]
    #[ignore]
    fn test_redeposit() {
        let (app, contract_address, cl_pool_id, admin) =
            default_init();

        let wasm = Wasm::new(&app);
        let bm = Bank::new(&app);

        // Initialize accounts
        let accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                ACCOUNTS_NUM,
            )
            .unwrap();

        for account in &accounts {
            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::ExactDeposit { recipient: None },
                    &[
                        Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                        Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                    ],
                    account,
                )
                .unwrap();

            // Get shares for Alice from vault contract and assert
            let shares: UserSharesBalanceResponse = wasm
                .query(
                    contract_address.as_str(),
                    &VaultExtension(ExtensionQueryMsg::Balances(
                        UserSharesBalance {
                            user: account.address(),
                        },
                    )),
                )
                .unwrap();
            assert!(!shares.balance.is_zero());
        }

        // Declare swapper and claimer accounts
        let ops_accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                2,
            )
            .unwrap();
        let swapper = &ops_accounts[0];

        // Swaps to generate spread rewards on previously created user positions
        for _ in 0..SWAPS_NUM {
            PoolManager::new(&app)
                .swap_exact_amount_in(
                    MsgSwapExactAmountIn {
                        sender: swapper.address(),
                        routes: vec![SwapAmountInRoute {
                            pool_id: cl_pool_id,
                            token_out_denom: DENOM_BASE.to_string(),
                        }],
                        token_in: Some(OsmoCoin {
                            denom: DENOM_QUOTE.to_string(),
                            amount: SWAPS_AMOUNT.to_string(),
                        }),
                        token_out_min_amount: "1".to_string(),
                    },
                    &swapper,
                )
                .unwrap();
        }

        // check for contract balance as it has not been redeposited yet
        let balance_before = bm.query_all_balances(&QueryAllBalancesRequest{
            address: contract_address.to_string(),
            pagination: None,
        }).unwrap();

        let _result = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Redeposit{}),
                &[],
                &admin,
            )
            .unwrap();

        // check for contract balance as it has not been redeposited yet
        let balance_after = bm.query_all_balances(&QueryAllBalancesRequest{
            address: contract_address.to_string(),
            pagination: None,
        }).unwrap();

        // todo add assert on the balance
        // assert!()
    }
}
