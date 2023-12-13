use std::str::FromStr;

use cosmwasm_std::{coin, Coin, Decimal, Uint128};

use osmosis_std::types::{
    cosmos::bank::v1beta1::{MsgSend, QueryAllBalancesRequest},
    osmosis::{concentratedliquidity::v1beta1::PositionByIdRequest, poolmanager::v1beta1::SpotPriceRequest},
};
use osmosis_test_tube::{Account, Bank, ConcentratedLiquidity, Module, Wasm, PoolManager};

use crate::{
    msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg},
    query::{PositionResponse, UserBalanceResponse},
    tests::{default_init, helpers::get_share_price_in_asset0},
};

#[test]
fn multi_position_deposit_works() {
    let (app, contract_address, cl_pool_id, admin) = default_init();
    let alice = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();
    let bob = app
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

    // make sure we have some fee uosmo and uatom to create the new position
    bank.send(MsgSend{ from_address: admin.address(), to_address: contract_address.to_string(), amount: vec![ Coin::new(1_000, "uatom").into(),
    Coin::new(1_000, "uosmo").into()] }, &admin).unwrap();

    let wasm = Wasm::new(&app);

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

    let spot_price: Decimal = PoolManager::new(&app).query_spot_price(&SpotPriceRequest { pool_id: cl_pool_id, base_asset_denom: "uatom".into(), quote_asset_denom: "uosmo".into() }).unwrap().spot_price.parse().unwrap();
    let share_price: Decimal = get_share_price_in_asset0(&wasm, spot_price, contract_address.as_str()).unwrap();
    println!("shares: {}", shares.balance);
    println!("share price: {}", share_price);


    // create a new position
    let _res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                crate::msg::ModifyRange::CreatePosition {
                    lower_price: Decimal::from_str("0.90").unwrap(),
                    upper_price: Decimal::from_str("1.1").unwrap(),
                    ratio: Uint128::one(),
                },
            )),
            &vec![],
            &admin,
        )
        .unwrap();

    

    // depositing
    let res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::ExactDeposit { recipient: None },
            &[Coin::new(5000, "uatom"), Coin::new(5000, "uosmo")],
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
    
        // println!("res: {:?}", res);

        let spot_price: Decimal = PoolManager::new(&app).query_spot_price(&SpotPriceRequest { pool_id: cl_pool_id, base_asset_denom: "uatom".into(), quote_asset_denom: "uosmo".into() }).unwrap().spot_price.parse().unwrap();
        let share_price: Decimal = get_share_price_in_asset0(&wasm, spot_price, contract_address.as_str()).unwrap();
        println!("shares: {}", shares.balance);
        println!("share price: {}", share_price);

        // create a new position
        // this introduction might introduce new funds into the system, and therefore change the share price
        let _res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                crate::msg::ModifyRange::CreatePosition {
                    lower_price: Decimal::from_str("0.80").unwrap(),
                    upper_price: Decimal::from_str("1.2").unwrap(),
                    ratio: Uint128::one(),
                },
            )),
            &vec![],
            &admin,
        )
        .unwrap();

         // depositing
    let res = wasm
    .execute(
        contract_address.as_str(),
        &ExecuteMsg::ExactDeposit { recipient: None },
        &[Coin::new(5000, "uatom"), Coin::new(5000, "uosmo")],
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

    // println!("res: {:?}", res);

    let spot_price: Decimal = PoolManager::new(&app).query_spot_price(&SpotPriceRequest { pool_id: cl_pool_id, base_asset_denom: "uatom".into(), quote_asset_denom: "uosmo".into() }).unwrap().spot_price.parse().unwrap();
    let share_price: Decimal = get_share_price_in_asset0(&wasm, spot_price, contract_address.as_str()).unwrap();
    println!("shares: {}", shares.balance);
    println!("share price: {}", share_price);
}
