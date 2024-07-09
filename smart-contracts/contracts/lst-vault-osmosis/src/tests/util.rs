use crate::contract::{get_factory_denom, SUBDENOM};
use crate::msg::InstantiateMsg;
use cosmwasm_std::testing::MOCK_CONTRACT_ADDR;
use cosmwasm_std::{
    from_json, to_json_binary, ContractResult, Decimal, QuerierResult, SystemError, SystemResult,
    Uint128, WasmQuery,
};
use lst_adapter_osmosis::msg::LstAdapterQueryMsg;

pub const DEPOSIT_DENOM: &str = "uosmo";
pub const LST_DENOM: &str = "ustosmo";
pub const TEST_LST_ADAPTER: &str = "test-lst-adapter";
pub const TEST_DEX_ADAPTER: &str = "test-dex-adapter";
pub const TEST_UNBONDING_PERIOD: u64 = 20000;
pub const CREATOR: &str = "creator";
pub const USER: &str = "user";

pub fn get_init_msg() -> InstantiateMsg {
    InstantiateMsg {
        dex_adapter: TEST_DEX_ADAPTER.to_string(),
        lst_adapter: TEST_LST_ADAPTER.to_string(),
        deposit_denom: DEPOSIT_DENOM.to_string(),
        lst_denom: LST_DENOM.to_string(),
        unbonding_time_seconds: TEST_UNBONDING_PERIOD,
    }
}

pub fn mock_wasm_querier_with_lst_adapter(
    lst_adapter: String,
    lst_adapter_balance: u128,
    lst_claimable: u128,
) -> Box<impl Fn(&WasmQuery) -> QuerierResult> {
    Box::from(move |request: &WasmQuery| -> QuerierResult {
        match request {
            WasmQuery::Smart { contract_addr, msg } => {
                if contract_addr == &lst_adapter {
                    let msg: LstAdapterQueryMsg = from_json(&msg).unwrap();
                    match msg {
                        LstAdapterQueryMsg::BalanceInUnderlying {} => {
                            let response = Uint128::from(lst_adapter_balance);
                            return SystemResult::Ok(ContractResult::Ok(
                                to_json_binary(&response).unwrap(),
                            ));
                        }
                        LstAdapterQueryMsg::RedemptionRate {} => {
                            let response = Decimal::percent(50);
                            return SystemResult::Ok(ContractResult::Ok(
                                to_json_binary(&response).unwrap(),
                            ));
                        }
                        LstAdapterQueryMsg::Claimable {} => {
                            let response = Uint128::from(lst_claimable);
                            return SystemResult::Ok(ContractResult::Ok(
                                to_json_binary(&response).unwrap(),
                            ));
                        }
                        _ => {
                            return SystemResult::Err(SystemError::UnsupportedRequest {
                                kind: "Not implemented".to_string(),
                            });
                        }
                    }
                }
                return SystemResult::Err(SystemError::NoSuchContract {
                    addr: contract_addr.clone(),
                });
            }
            _ => {
                return SystemResult::Err(SystemError::Unknown {});
            }
        };
    })
}

pub fn get_fund_denom() -> String {
    get_factory_denom(MOCK_CONTRACT_ADDR, SUBDENOM)
}
