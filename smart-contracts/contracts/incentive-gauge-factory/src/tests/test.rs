use crate::{
    tests::common::*,
    types::{BlockPeriod, Fee, Gauge, GaugeKind, PoolKind},
};
use cosmwasm_std::{coins, testing::mock_env, Addr};
use quasar_types::coinlist::CoinList;

#[test]
fn create_gauge() -> Result<(), anyhow::Error> {
    let admin = "admin";

    let mut app = App::default();

    let gauge_codeid = merkle_incentives_upload(&mut app);

    let (_, contract_addr) = contract_init(
        &mut app,
        admin.to_string(),
        new_init_msg(Some(admin.to_string()), Some(gauge_codeid)),
    )?;

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr,
        &get_creat_gauge_msg(),
        &[],
    );

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn update_gauge() -> Result<(), anyhow::Error> {
    let admin = "admin";

    let mut app = App::default();

    let gauge_codeid = merkle_incentives_upload(&mut app);

    let (_, contract_addr) = contract_init(
        &mut app,
        admin.to_string(),
        new_init_msg(Some(admin.to_string()), Some(gauge_codeid)),
    )?;

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr.clone(),
        &get_creat_gauge_msg(),
        &[],
    );

    assert!(res.is_ok());

    let env = mock_env();

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr.clone(),
        &crate::msg::ExecuteMsg::GaugeMsg(crate::msg::GaugeMsg::Update {
            addr: "contract1".to_string(),
            gauge: Gauge {
                period: BlockPeriod {
                    start: env.block.height + 20u64,
                    end: env.block.height + 50u64,
                    expiry: env.block.height + 100u64,
                },
                incentives: coins(5000, "ucosm"),
                clawback: "clawback".to_string(),
            },
            fees: Some(Fee::new(
                "reciever".to_string(),
                Decimal::from_ratio(Uint128::from(500u16), Uint128::one()),
                CoinList::new(vec![coin(100, "ucosm")]),
            )),
            kind: Some(GaugeKind::Pool {
                address: Addr::unchecked("pool_address"),
                kind: PoolKind::Liquidity,
                denom_a: "ucosm".to_string(),
                denom_b: "uatom".to_string(),
            }),
        }),
        &[],
    );

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn update_merkle() -> Result<(), anyhow::Error> {
    let admin = "admin";

    let mut app = App::default();

    let gauge_codeid = merkle_incentives_upload(&mut app);

    let (_, contract_addr) = contract_init(
        &mut app,
        admin.to_string(),
        new_init_msg(Some(admin.to_string()), Some(gauge_codeid)),
    )?;

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr.clone(),
        &get_creat_gauge_msg(),
        &[],
    );

    assert!(res.is_ok());

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr.clone(),
        &crate::msg::ExecuteMsg::GaugeMsg(crate::msg::GaugeMsg::MerkleUpdate {
            addr: "contract1".to_string(),
            merkle: String::from("some"),
        }),
        &[],
    );

    assert!(res.is_ok());

    Ok(())
}
