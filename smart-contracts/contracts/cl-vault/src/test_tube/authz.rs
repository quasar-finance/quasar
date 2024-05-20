#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;

    use osmosis_test_tube::{Account, Module, Wasm};

    use crate::{
        msg::{AuthzExtension, ExecuteMsg, ExtensionQueryMsg, QueryMsg},
        query::UserSharesBalanceResponse,
        test_tube::initialize::initialize::{fixture_default, DENOM_BASE, DENOM_QUOTE},
    };

    const INITIAL_BALANCE_AMOUNT: u128 = 1_000_000_000_000_000_000_000_000_000_000;

    // check that the authz interface returns the exact same response as
    // the regular interface. Thus the actual authz functionality is out of
    // scope but contract functionality is in scope here
    #[test]
    #[ignore]
    fn deposit_withdraw_equal() {
        let (app, contract_address, _cl_pool_id, _admin, _deposit_ratio) = fixture_default();
        let wasm = Wasm::new(&app);

        // Create Alice account
        let alice = app
            .init_account(&[
                Coin::new(INITIAL_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(INITIAL_BALANCE_AMOUNT, DENOM_QUOTE),
            ])
            .unwrap();

        let deposit0 = 1_000_000_000_000_000;
        let deposit1 = 1_000_000_000_000_000;

        let deposit_response = wasm
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

        let authz_deposit_response = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Authz(
                    AuthzExtension::ExactDeposit {},
                )),
                &[
                    Coin::new(deposit0, DENOM_BASE),
                    Coin::new(deposit1, DENOM_QUOTE),
                ],
                &alice,
            )
            .unwrap();

        assert_eq!(deposit_response.data, authz_deposit_response.data);

        assert_eq!(
            deposit_response
                .events
                .iter()
                .find(|e| e.ty == "wasm".to_string()),
            authz_deposit_response
                .events
                .iter()
                .find(|e| e.ty == "wasm".to_string())
        );

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

        let to_withdraw = shares.balance.u128() / 2;

        let withdraw_response = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: to_withdraw.into(),
                },
                &[],
                &alice,
            )
            .unwrap();

        let authz_withdraw_response = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: to_withdraw.into(),
                },
                &[],
                &alice,
            )
            .unwrap();

        assert_eq!(withdraw_response.data, authz_withdraw_response.data);
        assert_eq!(
            withdraw_response
                .events
                .iter()
                .find(|e| e.ty == "wasm".to_string()),
            authz_withdraw_response
                .events
                .iter()
                .find(|e| e.ty == "wasm".to_string())
        );
    }
}
