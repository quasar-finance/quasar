#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, Coin, Uint128};

    use osmosis_std::types::cosmos::bank::v1beta1::{MsgSend, QueryAllBalancesRequest};
    use osmosis_test_tube::{Account, Bank, Module, Wasm};

    use crate::{
        msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg},
        query::UserBalanceResponse,
        test_tube::default_init,
    };

    #[test]
    #[ignore]
    fn multiple_deposit_withdraw_unused_funds_works() {
        let (app, contract_address, _cl_pool_id, _admin) = default_init();
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let bank = Bank::new(&app);
        // our initial balance, 89874uosmo
        let balances = bank
            .query_all_balances(&QueryAllBalancesRequest {
                address: contract_address.to_string(),
                pagination: None,
            })
            .unwrap();
        println!("{:?}", balances);

        let wasm = Wasm::new(&app);

        // depositing 5000 each
        let res = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[Coin::new(5_000, "uatom"), Coin::new(5_000, "uosmo")],
                &alice,
            )
            .unwrap();
        // The contract right now has 89874 free uosmo, if we send another 89874 free uosmo, we double the amount of free
        // liquidity
        // This amount should decrease the amount of shares we get back
        println!("{:?}", res);

        bank.send(
            MsgSend {
                from_address: alice.address(),
                to_address: contract_address.to_string(),
                amount: vec![coin(89874, "uosmo").into()],
            },
            &alice,
        )
        .unwrap();

        let res = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[Coin::new(5_000, "uatom"), Coin::new(5_000, "uosmo")],
                &alice,
            )
            .unwrap();
        println!("{:?}", res);
        
        // 2766182566501133149875859 before banksend,
        // 1926137978194597565946694 after banksend
        // does this make sense?
        // when we withdraw 2766182566501133149875859 shares, we should get our original amount back + 
        // 2766182566501133149875859 / total_shares * 89874 back, remember we had original free osmo
        // and sent free osmo
        // the second share amount should only get it's original amount back

        // let _ = wasm
        //     .execute(
        //         contract_address.as_str(),
        //         &ExecuteMsg::ExactDeposit { recipient: None },
        //         &[Coin::new(5_000, "uatom"), Coin::new(5_000, "uosmo")],
        //         &alice,
        //     )
        //     .unwrap();

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

        let withdraw = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::Redeem {
                    recipient: None,
                    amount: Uint128::new(1926137978194597565946694),
                },
                &[],
                &alice,
            )
            .unwrap();
        println!("{:?}", withdraw);
        // we receive "token0_amount", value: "2018" }, Attribute { key: "token1_amount", value: "3503
        // we used 5000uatom to deposit and 507 uosmo, thus we are down 3000 uatom and up 2996 uosmo

    }

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
