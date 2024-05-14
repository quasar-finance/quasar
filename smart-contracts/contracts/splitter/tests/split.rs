use std::str::FromStr;
use cosmwasm_std::{coin, Addr, Coin, Decimal, Empty, Uint128};
use cw_multi_test::{App, BankSudo, Contract, ContractWrapper, Executor};

use splitter::contract::{execute, instantiate, query};
use splitter::msg::{Claim, ExecuteMsg, InstantiateMsg};
use splitter::state::Receiver;

pub fn contract_fee_splitter() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new_with_empty(execute, instantiate, query);
    Box::new(contract)
}

#[test]
fn test_split() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");

    let receiver1 = Receiver::new(Addr::unchecked("user1"), Decimal::from_str("0.6").unwrap());
    let receiver2 = Receiver::new(Addr::unchecked("user2"), Decimal::from_str("0.4").unwrap());

    let code_id = app.store_code(contract_fee_splitter());
    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner.clone(),
            &InstantiateMsg {
                admin: owner.clone().to_string(),
                receivers: vec![receiver1.clone(), receiver2.clone()],
            },
            &[],
            "quasar-fee-splitter",
            Some(owner.to_string()),
        )
        .unwrap();

    app.sudo(cw_multi_test::SudoMsg::Bank(BankSudo::Mint {
        to_address: contract_addr.to_string(),
        amount: vec![coin(1200, "atom"), coin(2000, "osmo")],
    }))
    .unwrap();

    let _response = app
        .execute_contract(owner, contract_addr, &ExecuteMsg::Split {}, &[])
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(receiver1.address.clone(), "atom")
            .unwrap()
            .amount,
        Uint128::new(720)
    );
    assert_eq!(
        app.wrap()
            .query_balance(receiver2.address.clone(), "atom")
            .unwrap()
            .amount,
        Uint128::new(480)
    );

    assert_eq!(
        app.wrap()
            .query_balance(receiver1.address, "osmo")
            .unwrap()
            .amount,
        Uint128::new(1200)
    );
    assert_eq!(
        app.wrap()
            .query_balance(receiver2.address, "osmo")
            .unwrap()
            .amount,
        Uint128::new(800)
    );
}

#[test]
fn test_claim() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");

    let receiver1 = Receiver::new(Addr::unchecked("user1"), Decimal::from_str("0.6").unwrap());
    let receiver2 = Receiver::new(Addr::unchecked("user2"), Decimal::from_str("0.4").unwrap());

    let code_id = app.store_code(contract_fee_splitter());
    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner.clone(),
            &InstantiateMsg {
                admin: owner.clone().to_string(),
                receivers: vec![receiver1.clone(), receiver2.clone()],
            },
            &[],
            "quasar-fee-splitter",
            Some(owner.to_string()),
        )
        .unwrap();

    // app.sudo(cw_multi_test::SudoMsg::Bank(BankSudo::Mint {
    //     to_address: contract_addr.to_string(),
    //     amount: vec![coin(1200, "atom"), coin(2000, "osmo")],
    // }))
    // .unwrap();

    let _response = app
        .execute_contract(
            owner,
            contract_addr,
            &ExecuteMsg::Claim {
                claims: vec![Claim {
                    address: todo!(),
                    msg: todo!(),
                }],
            },
            &[],
        )
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(receiver1.address.clone(), "atom")
            .unwrap()
            .amount,
        Uint128::new(720)
    );
    assert_eq!(
        app.wrap()
            .query_balance(receiver2.address.clone(), "atom")
            .unwrap()
            .amount,
        Uint128::new(480)
    );

    assert_eq!(
        app.wrap()
            .query_balance(receiver1.address, "osmo")
            .unwrap()
            .amount,
        Uint128::new(1200)
    );
    assert_eq!(
        app.wrap()
            .query_balance(receiver2.address, "osmo")
            .unwrap()
            .amount,
        Uint128::new(800)
    );
}
