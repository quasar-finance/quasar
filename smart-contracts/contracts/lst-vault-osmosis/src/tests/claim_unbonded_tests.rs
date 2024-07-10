use crate::contract::{execute, instantiate};
use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::tests::util::{
    get_init_msg, mock_wasm_querier_with_lst_adapter, CREATOR, TEST_LST_ADAPTER,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coin, to_json_binary, CosmosMsg, WasmMsg};
use lst_adapter_osmosis::msg::LstAdapterExecuteMsg;

#[test]
fn claim_unbonded_fails_if_nothing_is_claimable() {
    let mut deps = mock_dependencies();
    deps.querier.update_wasm(mock_wasm_querier_with_lst_adapter(
        TEST_LST_ADAPTER.to_owned(),
        0,
        0,
        vec![],
    ));
    let env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    let info = mock_info(CREATOR, &[coin(1, "other_denom".to_string())]);
    let err = execute(deps.as_mut(), env, info, ExecuteMsg::ClaimUnbonded {}).unwrap_err();
    assert_eq!(err, ContractError::NothingToClaim {});
}

#[test]
fn claim_unbonded_claims_from_lst_adapter() {
    let mut deps = mock_dependencies();
    deps.querier.update_wasm(mock_wasm_querier_with_lst_adapter(
        TEST_LST_ADAPTER.to_owned(),
        1000,
        1000,
        vec![],
    ));
    let env = mock_env();

    let msg = get_init_msg();
    let info = mock_info(CREATOR, &[]);
    assert!(instantiate(deps.as_mut(), env.clone(), info, msg).is_ok());

    let info = mock_info(CREATOR, &[coin(1, "other_denom".to_string())]);
    let response = execute(deps.as_mut(), env, info, ExecuteMsg::ClaimUnbonded {}).unwrap();
    assert_eq!(response.messages.len(), 1);
    assert_eq!(
        response.messages[0].msg,
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: TEST_LST_ADAPTER.to_string(),
            msg: to_json_binary(&LstAdapterExecuteMsg::Claim {}).unwrap(),
            funds: vec![],
        })
    );
}
