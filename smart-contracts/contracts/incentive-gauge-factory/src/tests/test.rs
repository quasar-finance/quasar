use crate::{
    msg::GaugeListResponse,
    tests::common::*,
    types::{BlockPeriod, Fee, Gauge, GaugeKind, PoolKind},
};
use cosmwasm_std::{coins, testing::mock_env, Addr};
use quasar_types::coinlist::CoinList;

#[test]
fn update_codeid() -> Result<(), anyhow::Error> {
    let admin = "admin";

    let mut app = App::default();

    let gauge_codeid = merkle_incentives_upload(&mut app);

    let (_, contract_addr) = contract_init(
        &mut app,
        admin.to_string(),
        new_init_msg(Some(admin.to_string()), Some(gauge_codeid)),
    )?;

    reset_time(&mut app);

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr.clone(),
        &get_creat_gauge_msg(),
        &[],
    );

    assert!(res.is_ok());

    let gauge_codeid = merkle_incentives_upload(&mut app);

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr.clone(),
        &crate::msg::ExecuteMsg::GaugeMsg(crate::msg::GaugeMsg::CodeUpdate { code: gauge_codeid }),
        &[],
    );

    assert!(res.is_ok());

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
fn create_gauge_vault() -> Result<(), anyhow::Error> {
    let admin = "admin";

    let mut app = App::default();

    let gauge_codeid = merkle_incentives_upload(&mut app);

    let (_, contract_addr) = contract_init(
        &mut app,
        admin.to_string(),
        new_init_msg(Some(admin.to_string()), Some(gauge_codeid)),
    )?;

    reset_time(&mut app);

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
fn create_gauge_pool() -> Result<(), anyhow::Error> {
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
        &get_creat_gauge_pool_msg(),
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

    reset_time(&mut app);

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
                denom_b: Some("uatom".to_string()),
            }),
        }),
        &[],
    );

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn remove_gauge() -> Result<(), anyhow::Error> {
    let admin = "admin";

    let mut app = App::default();

    let gauge_codeid = merkle_incentives_upload(&mut app);

    let (_, contract_addr) = contract_init(
        &mut app,
        admin.to_string(),
        new_init_msg(Some(admin.to_string()), Some(gauge_codeid)),
    )?;

    reset_time(&mut app);

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr.clone(),
        &get_creat_gauge_msg(),
        &[],
    );

    assert!(res.is_ok());

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr,
        &crate::msg::ExecuteMsg::GaugeMsg(crate::msg::GaugeMsg::Remove {
            addr: "contract1".to_string(),
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

    reset_time(&mut app);

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

#[test]
fn update_admin() -> Result<(), anyhow::Error> {
    let admin = "admin";

    let mut app = App::default();

    let gauge_codeid = merkle_incentives_upload(&mut app);

    let (_, contract_addr) = contract_init(
        &mut app,
        admin.to_string(),
        new_init_msg(Some(admin.to_string()), Some(gauge_codeid)),
    )?;

    reset_time(&mut app);

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
        &crate::msg::ExecuteMsg::AdminUpdate {
            addr: "new_admin".to_string(),
        },
        &[],
    );

    assert!(res.is_ok());

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr.clone(),
        &crate::msg::ExecuteMsg::AdminUpdate {
            addr: "new_admin".to_string(),
        },
        &[],
    );

    assert!(res.is_err());

    Ok(())
}

#[test]
fn fees_update() -> Result<(), anyhow::Error> {
    let admin = "admin";

    let mut app = App::default();

    let gauge_codeid = merkle_incentives_upload(&mut app);

    let (_, contract_addr) = contract_init(
        &mut app,
        admin.to_string(),
        new_init_msg(Some(admin.to_string()), Some(gauge_codeid)),
    )?;

    reset_time(&mut app);

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr.clone(),
        &get_creat_gauge_pool_msg(),
        &[],
    );

    assert!(res.is_ok());

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr,
        &crate::msg::ExecuteMsg::FeeMsg(crate::msg::FeeMsg::Update {
            addr: "contract1".to_string(),
            fees: Fee::new(
                "reciever".to_string(),
                Decimal::from_ratio(Uint128::from(500u16), Uint128::one()),
                CoinList::new(vec![coin(100, "ucosm")]),
            ),
        }),
        &[],
    );

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn fees_distribute() -> Result<(), anyhow::Error> {
    let admin = "admin";

    let mut app = App::default();

    let gauge_codeid = merkle_incentives_upload(&mut app);

    let (_, contract_addr) = contract_init(
        &mut app,
        admin.to_string(),
        new_init_msg(Some(admin.to_string()), Some(gauge_codeid)),
    )?;

    reset_time(&mut app);

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr.clone(),
        &get_creat_gauge_pool_msg(),
        &[],
    );

    assert!(res.is_ok());

    mint_native(
        &mut app,
        "contract0".to_string(),
        "ucosm".to_string(),
        1_000_000_000u128,
    );

    app.update_block(|block| {
        block.height = 210;
    });

    let res = app.execute_contract(
        Addr::unchecked(admin),
        contract_addr,
        &crate::msg::ExecuteMsg::FeeMsg(crate::msg::FeeMsg::Distribute {
            addr: "contract1".to_string(),
        }),
        &[],
    );

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn query_gauge_list() -> Result<(), anyhow::Error> {
    let admin = "admin";

    let mut app = App::default();

    let gauge_codeid = merkle_incentives_upload(&mut app);

    let (_, contract_addr) = contract_init(
        &mut app,
        admin.to_string(),
        new_init_msg(Some(admin.to_string()), Some(gauge_codeid)),
    )?;

    reset_time(&mut app);

    for _ in 0..10 {
        let res = app.execute_contract(
            Addr::unchecked(admin),
            contract_addr.clone(),
            &get_creat_gauge_msg(),
            &[],
        );

        assert!(res.is_ok());
    }

    let res = app.wrap().query_wasm_smart::<GaugeListResponse>(
        contract_addr.clone(),
        &crate::msg::QueryMsg::ListGauges {
            start_after: None,
            limit: None,
        },
    );

    assert!(res.is_ok());

    Ok(())
}
