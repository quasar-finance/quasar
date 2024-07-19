use crate::msg::InstantiateMsg;
use crate::tests::setup::create_test_vault;
use cosmwasm_std::Event;
use cw_orch::contract::interface_traits::CwOrchInstantiate;

#[test]
fn test_instantiate() {
    let env = create_test_vault();
    let vault = env.vault;

    let result = vault.instantiate(&InstantiateMsg {}, None, None);
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.events.len(), 1);
    assert_eq!(
        response.events[0],
        Event::new("instantiate")
            .add_attribute("_contract_address", "contract0")
            .add_attribute("code_id", "1")
    );
}
