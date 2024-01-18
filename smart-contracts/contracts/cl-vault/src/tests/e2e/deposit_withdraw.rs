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
    assert_share_price, assert_total_assets, assert_unused_funds,
    helpers::get_asset0_value,
    msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg},
    query::{PositionsResponse, UserBalanceResponse},
    rewards::CoinList,
    tests::{
        default_init,
        helpers::{
            get_event_attributes_by_ty_and_key, get_full_positions, get_share_price,
            get_share_value, get_unused_funds, get_user_shares,
        },
    },
};

use crate::tests::helpers::get_total_assets;

#[test]
fn deposit_withdraw_single_user_works() {
    let (app, contract_address, cl_pool_id, admin) = default_init();
    let init_balance = CoinList::from_coins(vec![
        Coin::new(1_000_000_000_000, "uatom"),
        Coin::new(1_000_000_000_000, "uosmo"),
    ]);

    let alice = app.init_account(&init_balance.coins()).unwrap();
    let bob = app.init_account(&init_balance.coins()).unwrap();
    let mut tx_fees = CoinList::new();

    let wasm = Wasm::new(&app);

    let bank = Bank::new(&app);

    // make sure we have some fee uosmo and uatom to create the new position
    // here we introduce new funds into the test, after this point, we'd expect the share price to no longer change
    bank.send(
        MsgSend {
            from_address: admin.address(),
            to_address: contract_address.to_string(),
            amount: vec![
                Coin::new(1_000_000, "uatom").into(),
                Coin::new(1_000_000, "uosmo").into(),
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

    // depositing
    assert_share_price!(
        &app,
        contract_address.as_str(),
        original_share_price,
        cl_pool_id
    );
    let unused_funds = get_unused_funds(&wasm, contract_address.as_str()).unwrap();

    total_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();

    // for this deposit, we have 4 deposits, 7500uatom worth of assets and the user deposits 5_000_000uatom and 5_000_000uosmo
    // since we have 3 position
    let res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::ExactDeposit { recipient: None },
            &[Coin::new(5_000_000, "uatom"), Coin::new(5_000_000, "uosmo")],
            &alice,
        )
        .unwrap();
    println!("result :{:?}", _res);
    let tx_fee: Coin = get_event_attributes_by_ty_and_key(&res, "tx", vec!["fee"])[0]
        .value
        .parse()
        .unwrap();
    tx_fees = tx_fees.add(tx_fee.into()).unwrap();

    total_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();
    assert_unused_funds!(&wasm, contract_address.as_str(), unused_funds);
    assert_total_assets!(&wasm, contract_address.as_str(), &total_assets);
    // we accept a non-default share price relative difference here. Due to the low vault value,
    // rounding causes a change in share price here, which at low vault value is a non-negligible percentual
    // increase. This is fine assuming that the relative percentual change decreases as the vault value increases
    assert_share_price!(
        &app,
        contract_address.as_str(),
        original_share_price,
        cl_pool_id
    );

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

    let share_price = get_share_price(&app, cl_pool_id, contract_address.as_str());
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

    assert_share_price!(&app, contract_address.as_str(), share_price, cl_pool_id);

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

    // depositing more funds
    let res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::ExactDeposit { recipient: None },
            &[Coin::new(5000, "uatom"), Coin::new(5000, "uosmo")],
            &alice,
        )
        .unwrap();
    total_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();
    assert_total_assets!(&wasm, contract_address.as_str(), &total_assets);

    let tx_fee: Coin = get_event_attributes_by_ty_and_key(&res, "tx", vec!["fee"])[0]
        .value
        .parse()
        .unwrap();
    tx_fees = tx_fees.add(tx_fee.into()).unwrap();

    assert_share_price!(
        &app,
        contract_address.as_str(),
        original_share_price,
        cl_pool_id
    );

    // check that alices balance + funds in positions
    let user_shares = get_user_shares(&wasm, contract_address.as_str(), alice.address()).unwrap();
    let share_value = get_share_value(&wasm, contract_address.as_str(), user_shares).unwrap();
    let current_balance = bank
        .query_all_balances(&QueryAllBalancesRequest {
            address: alice.address(),
            pagination: None,
        })
        .unwrap()
        .balances;

    total_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();

    // TODO we deposit in total 5_005_000 uatom and 5_005_000uosmo in total, our uatom value is seen as 8_011_259, so why is the atom value seen as too high
    // the total assets at this point in time: total_assets: (Coin { 5969319 "uatom" }, Coin { 1899241 "uosmo" })
    println!("total_assets: {:?}", total_assets);
    println!("share_value: {:?}", share_value);
    println!("current_balance: {:?}", current_balance);
    println!("fees: {:?}", tx_fees);

    // TODO add all paid tx fees here
    let total_value = CoinList::from_coins(share_value)
        .add(CoinList::from_coins(
            osmosis_std::try_proto_to_cosmwasm_coins(current_balance).unwrap(),
        ))
        .unwrap()
        .add(tx_fees.clone().into())
        .unwrap();

    //

    let spot_price: Decimal = PoolManager::new(&app)
        .query_spot_price(&SpotPriceRequest {
            pool_id: cl_pool_id,
            base_asset_denom: "uatom".into(),
            quote_asset_denom: "uosmo".into(),
        })
        .unwrap()
        .spot_price
        .parse()
        .unwrap();

    assert_approx_eq!(
        get_asset0_value(
            init_balance.find_coin("uatom".into()).amount,
            init_balance.find_coin("uosmo".into()).amount,
            spot_price
        )
        .unwrap(),
        get_asset0_value(
            total_value.find_coin("uatom".into()).amount,
            total_value.find_coin("uosmo".into()).amount,
            spot_price
        )
        .unwrap(),
        "0.00000001",
    );

    // withdrawing any amount of shares, we should still have the same total value
    let withdraw_shares = user_shares / Uint128::new(3);
    let res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::Redeem {
                recipient: None,
                amount: withdraw_shares,
            },
            &[],
            &alice,
        )
        .unwrap();

    let tx_fee: Coin = get_event_attributes_by_ty_and_key(&res, "tx", vec!["fee"])[0]
        .value
        .parse()
        .unwrap();
    tx_fees = tx_fees.add(tx_fee.into()).unwrap();

    let user_shares = get_user_shares(&wasm, contract_address.as_str(), alice.address()).unwrap();
    let share_value = get_share_value(&wasm, contract_address.as_str(), user_shares).unwrap();
    let current_balance = bank
        .query_all_balances(&QueryAllBalancesRequest {
            address: alice.address(),
            pagination: None,
        })
        .unwrap()
        .balances;

    let total_value = CoinList::from_coins(share_value)
        .add(CoinList::from_coins(
            osmosis_std::try_proto_to_cosmwasm_coins(current_balance).unwrap(),
        ))
        .unwrap()
        .add(tx_fees.clone())
        .unwrap();

    assert_approx_eq!(
        get_asset0_value(
            init_balance.find_coin("uatom".into()).amount,
            init_balance.find_coin("uosmo".into()).amount,
            spot_price
        )
        .unwrap(),
        get_asset0_value(
            total_value.find_coin("uatom".into()).amount,
            total_value.find_coin("uosmo".into()).amount,
            spot_price
        )
        .unwrap(),
        "0.00000001",
    );

    // withdraw the rest of the shares
    let withdraw_shares = user_shares;
    let res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::Redeem {
                recipient: None,
                amount: withdraw_shares,
            },
            &[],
            &alice,
        )
        .unwrap();

    let tx_fee: Coin = get_event_attributes_by_ty_and_key(&res, "tx", vec!["fee"])[0]
        .value
        .parse()
        .unwrap();
    tx_fees = tx_fees.add(tx_fee.into()).unwrap();

    let user_shares = get_user_shares(&wasm, contract_address.as_str(), alice.address()).unwrap();
    let share_value = get_share_value(&wasm, contract_address.as_str(), user_shares).unwrap();
    let current_balance = bank
        .query_all_balances(&QueryAllBalancesRequest {
            address: alice.address(),
            pagination: None,
        })
        .unwrap()
        .balances;

    let total_value = CoinList::from_coins(share_value)
        .add(CoinList::from_coins(
            osmosis_std::try_proto_to_cosmwasm_coins(current_balance).unwrap(),
        ))
        .unwrap()
        .add(tx_fees)
        .unwrap();

    assert_approx_eq!(
        get_asset0_value(
            init_balance.find_coin("uatom".into()).amount,
            init_balance.find_coin("uosmo".into()).amount,
            spot_price
        )
        .unwrap(),
        get_asset0_value(
            total_value.find_coin("uatom".into()).amount,
            total_value.find_coin("uosmo".into()).amount,
            spot_price
        )
        .unwrap(),
        "0.00000001",
    );
}
