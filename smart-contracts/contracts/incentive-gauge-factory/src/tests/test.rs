use crate::tests::common::*;
use cosmwasm_std::Addr;

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
