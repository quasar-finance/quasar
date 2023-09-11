#[cfg(test)]
mod tests {
    use cosmwasm_std::{Coin};

    use osmosis_test_tube::{Account, Module, Wasm};

    use crate::{
        msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg},
        query::UserBalanceResponse,
        test_tube::default_init,
    };

    #[test]
    #[ignore]
    fn multiple_deposit_withdraw_works() {
        let (app, contract_address, _cl_pool_id, _admin) = default_init();
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let wasm = Wasm::new(&app);

        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[Coin::new(5_000, "uatom"), Coin::new(5_000, "uosmo")],
                &alice,
            )
            .unwrap();

        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[Coin::new(5_000, "uatom"), Coin::new(5_000, "uosmo")],
                &alice,
            )
            .unwrap();

        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[Coin::new(5_000, "uatom"), Coin::new(5_000, "uosmo")],
                &alice,
            )
            .unwrap();

        let shares: UserBalanceResponse = wasm
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

        let _withdraw = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: shares.balance,
                },
                &[],
                &alice,
            )
            .unwrap();
        // verify the correct execution
    }

    #[test]
    #[ignore]
    fn single_deposit_withdraw_works() {
        let (app, contract_address, _cl_pool_id, _admin) = default_init();
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let wasm = Wasm::new(&app);

        let deposit = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[Coin::new(5_000, "uatom"), Coin::new(5_000, "uosmo")],
                &alice,
            )
            .unwrap();

        let _mint = deposit.events.iter().find(|e| e.ty == "tf_mint").unwrap();

        let shares: UserBalanceResponse = wasm
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

        let _withdraw = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: shares.balance,
                },
                &[],
                &alice,
            )
            .unwrap();
        // verify the correct execution
    }
}
