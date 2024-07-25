use crate::{
    contract::{instantiate, reply, CREATE_DENOM_REPLY_ID},
    msg::{InstantiateMsg, OracleQueryMsg},
};
use cosmwasm_std::{
    from_json,
    testing::{
        mock_dependencies, mock_dependencies_with_balances, mock_env, mock_info, MockApi,
        MockQuerier, MockStorage,
    },
    to_json_binary, Coin, ContractResult, Decimal, Empty, OwnedDeps, QuerierResult, Reply,
    SubMsgResponse, SubMsgResult, SystemError, SystemResult, WasmQuery,
};
use prost::Message;
use quasar_std::quasarlabs::quasarnode::tokenfactory::v1beta1::MsgCreateDenomResponse;

pub const OWNER: &str = "owner";
pub const USER: &str = "user";
pub const SUBDENOM: &str = "subdenom";
pub const DEPOSIT_DENOM: &str = "denom1";
pub const OTHER_DEPOSIT_DENOM: &str = "denom2";
pub const VAULT_DENOM: &str = "vault_denom";

fn basic_setup(
    deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let mut deps = deps;
    let env = mock_env();
    let info = mock_info(USER, &[]);

    assert!(instantiate(
        deps.as_mut(),
        env.clone(),
        info,
        InstantiateMsg {
            owner: OWNER.to_string(),
            subdenom: SUBDENOM.to_string(),
        },
    )
    .is_ok());

    assert!(reply(
        deps.as_mut(),
        env,
        Reply {
            id: CREATE_DENOM_REPLY_ID,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: Some(
                    MsgCreateDenomResponse {
                        new_token_denom: VAULT_DENOM.to_string(),
                    }
                    .encode_to_vec()
                    .into()
                ),
            })
        }
    )
    .is_ok());
    deps
}

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let deps = mock_dependencies();
    basic_setup(deps)
}

pub fn setup_with_balances(
    balances: &[(&str, &[Coin])],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let deps = mock_dependencies_with_balances(balances);
    basic_setup(deps)
}

pub fn mock_wasm_querier(
    oracle: String,
    deposit_denom_price: Decimal,
    other_deposit_denom_price: Decimal,
) -> Box<impl Fn(&WasmQuery) -> QuerierResult> {
    Box::from(move |request: &WasmQuery| -> QuerierResult {
        match request {
            WasmQuery::Smart { contract_addr, msg } => {
                if contract_addr == &oracle {
                    let msg: OracleQueryMsg = from_json(&msg).unwrap();
                    match msg {
                        OracleQueryMsg::Price { denom } => {
                            let response = match denom.as_str() {
                                DEPOSIT_DENOM => deposit_denom_price,
                                OTHER_DEPOSIT_DENOM => other_deposit_denom_price,
                                _ => Decimal::percent(10),
                            };
                            return SystemResult::Ok(ContractResult::Ok(
                                to_json_binary(&response).unwrap(),
                            ));
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
