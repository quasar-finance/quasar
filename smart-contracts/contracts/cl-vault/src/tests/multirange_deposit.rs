use std::str::FromStr;

use cosmwasm_std::{assert_approx_eq, coin, Coin, Decimal, Uint128};

use osmosis_std::types::{
    cosmos::bank::v1beta1::{MsgSend, QueryAllBalancesRequest},
    osmosis::{
        concentratedliquidity::v1beta1::PositionByIdRequest, poolmanager::v1beta1::SpotPriceRequest,
    },
};
use osmosis_test_tube::{Account, Bank, ConcentratedLiquidity, Module, PoolManager, Wasm};

use crate::{
    assert_share_price, assert_total_assets,
    msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg},
    query::{PositionResponse, UserBalanceResponse},
    tests::{
        default_init,
        helpers::{get_full_positions, get_share_price, get_unused_funds},
    }, assert_unused_funds,
};

use super::helpers::get_total_assets;

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

    let wasm = Wasm::new(&app);

    let bank = Bank::new(&app);
    // our initial balance, 89874uosmo
    let balances = bank
        .query_all_balances(&QueryAllBalancesRequest {
            address: contract_address.to_string(),
            pagination: None,
        })
        .unwrap();

    // make sure we have some fee uosmo and uatom to create the new position
    // here we introduce new funds into the test, after this point, we'd expect the share price to no longer change
    bank.send(
        MsgSend {
            from_address: admin.address(),
            to_address: contract_address.to_string(),
            amount: vec![
                Coin::new(1_000, "uatom").into(),
                Coin::new(1_000, "uosmo").into(),
            ],
        },
        &admin,
    )
    .unwrap();

    // total assets increase after we deposit
    let mut total_assets: (Coin, Coin) =
        get_total_assets(&wasm, contract_address.as_str()).unwrap();
    let original_share_price = get_share_price(&app, cl_pool_id, contract_address.as_str());

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
    assert_share_price!(
        &app,
        contract_address.as_str(),
        original_share_price,
        cl_pool_id
    );
    println!("here-1");

    // depositing
    assert_share_price!(
        &app,
        contract_address.as_str(),
        original_share_price,
        cl_pool_id
    );
    let unused_funds = get_unused_funds(&wasm, contract_address.as_str()).unwrap();
    
    total_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();

    println!("total-vault-value pre deposit: {:?}", total_assets);
    let _res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::ExactDeposit { recipient: None },
            &[Coin::new(5_000_000, "uatom"), Coin::new(5_000_000, "uosmo")],
            &alice,
        )
        .unwrap();
    println!("herehere");

    total_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();
    println!("herehear");
    assert_unused_funds!(&wasm, contract_address.as_str(), unused_funds);
    println!("what how much us unused!?!?");
    assert_total_assets!(&wasm, contract_address.as_str(), &total_assets);
    println!("total");
    assert_share_price!(
        &app,
        contract_address.as_str(),
        original_share_price,
        cl_pool_id
    );
    println!("here0");


    // create a new position
    // this introduction should not introduce new funds as long as we free up some funds first
    let positions = get_full_positions(&wasm, contract_address.as_str()).unwrap();
    let fp = positions
        .get(0)
        .unwrap()
        .full_breakdown
        .position
        .clone()
        .unwrap();

    println!("here1");
    let _res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                crate::msg::ModifyRange::DecreaseFunds {
                    position_id: fp.position_id,
                    liquidity: (Decimal::from_str(fp.liquidity.as_str()).unwrap()
                        / Decimal::from_ratio(2_u128, 1_u128))
                    .into(),
                },
            )),
            &vec![],
            &admin,
        )
        .unwrap();

    assert_share_price!(
        &app,
        contract_address.as_str(),
        original_share_price,
        cl_pool_id
    );

    println!("here2");

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

    assert_share_price!(
        &app,
        contract_address.as_str(),
        original_share_price,
        cl_pool_id
    );
    println!("here3");

    // depositing more funds
    let _res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::ExactDeposit { recipient: None },
            &[Coin::new(5000, "uatom"), Coin::new(5000, "uosmo")],
            &alice,
        )
        .unwrap();
    total_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();
    assert_total_assets!(&wasm, contract_address.as_str(), &total_assets);

    println!("here4");
    assert_share_price!(
        &app,
        contract_address.as_str(),
        original_share_price,
        cl_pool_id
    );}
