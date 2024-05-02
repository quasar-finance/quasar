#[cfg(test)]
mod tests {
    use cosmwasm_std::{Coin, Uint128};
    use cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension;
    use osmosis_std::types::cosmos::bank::v1beta1::{QueryAllBalancesRequest, QueryBalanceRequest};
    use osmosis_test_tube::{Account, Bank, Module, Wasm};
    use std::str::FromStr;

    use crate::msg::UserBalanceQueryMsg::UserSharesBalance;
    use crate::msg::{ExecuteMsg, ExtensionQueryMsg};
    use crate::query::UserSharesBalanceResponse;
    use crate::test_tube::initialize::initialize::default_init;

    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";
    const ACCOUNTS_NUM: u64 = 100;
    const ACCOUNTS_INIT_BALANCE: u128 = 1_000_000_000_000_000;
    const DEPOSIT_AMOUNT: u128 = 5_000_000;

    #[test]
    #[ignore]
    fn test_autocompound() {
        let (app, contract_address, _cl_pool_id, admin) = default_init();

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

        let mut i = 1;
        for account in &accounts {
            if i % 2 == 0 {
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
            } else {
                let _ = wasm
                    .execute(
                        contract_address.as_str(),
                        &ExecuteMsg::AnyDeposit {
                            amount: Default::default(),
                            asset: "".to_string(),
                            recipient: None,
                        },
                        &[
                            Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                            Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                        ],
                        account,
                    )
                    .unwrap();
            }

            // Get shares for Alice from vault contract and assert
            let shares: UserSharesBalanceResponse = wasm
                .query(
                    contract_address.as_str(),
                    &VaultExtension(ExtensionQueryMsg::Balances(UserSharesBalance {
                        user: account.address(),
                    })),
                )
                .unwrap();
            assert!(!shares.balance.is_zero());

            // autocompound on every 10th deposit into the vault
            if i % 10 == 0 {
                // check for contract balance as it has not been autocompounded yet
                let balance_before = bm
                    .query_all_balances(&QueryAllBalancesRequest {
                        address: contract_address.to_string(),
                        pagination: None,
                    })
                    .unwrap();
                // assert 3 denom on balance before as it has not been autocompounded yet
                // 3 denom : vault shares, base denom, quote denom
                assert_eq!(3usize, balance_before.balances.len());

                let _result = wasm
                    .execute(
                        contract_address.as_str(),
                        &ExecuteMsg::VaultExtension(
                            crate::msg::ExtensionExecuteMsg::Autocompound {},
                        ),
                        &[],
                        &admin,
                    )
                    .unwrap();

                // check for contract balance as it has been autocompounded
                let balance_after = bm
                    .query_balance(&QueryBalanceRequest {
                        address: contract_address.to_string(),
                        denom: DENOM_QUOTE.to_string(),
                    })
                    .unwrap();

                // assert quote denom balance to be lass than 1 as sometimes the balance for
                // quote denom becomes more than zero in odd number cases
                assert!(
                    Uint128::from_str(&balance_after.balance.unwrap_or_default().amount).unwrap()
                        <= Uint128::new(1)
                );
            }

            // increment i with 1
            i += 1;
        }
    }
}
