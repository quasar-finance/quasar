use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_json_binary, from_slice, to_json_binary, Coin, ContractResult, Empty, OwnedDeps, Querier,
    QuerierResult, QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};
use cw20::{BalanceResponse, Cw20QueryMsg, TokenInfoResponse};
use std::collections::HashMap;

pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier: WasmMockQuerier =
        WasmMockQuerier::new(MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: Default::default(),
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier,
    token_querier: TokenQuerier,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {e:?}"),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

#[derive(Clone, Default)]
pub struct TokenQuerier {
    balance: HashMap<String, Uint128>,
    supply: Uint128,
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: _,
                msg,
            }) => match from_json_binary(msg).unwrap() {
                Cw20QueryMsg::TokenInfo {} => QuerierResult::Ok(ContractResult::Ok(
                    to_json_binary(&TokenInfoResponse {
                        total_supply: self.token_querier.supply,
                        name: "vault_token".to_string(),
                        symbol: "".to_string(),
                        decimals: 0,
                    })
                    .unwrap(),
                )),
                Cw20QueryMsg::Balance { address } => QuerierResult::Ok(ContractResult::Ok(
                    to_json_binary(&BalanceResponse {
                        balance: match self.token_querier.balance.get(&address) {
                            Some(balance) => *balance,
                            None => Uint128::zero(),
                        },
                    })
                    .unwrap(),
                )),
                _ => SystemResult::Err(SystemError::UnsupportedRequest {
                    kind: "unimplemented".to_string(),
                }),
            },
            _ => self.base.handle_query(request),
        }
    }
}

impl WasmMockQuerier {
    pub fn new(base: MockQuerier<Empty>) -> Self {
        WasmMockQuerier {
            base,
            token_querier: TokenQuerier::default(),
        }
    }

    pub fn with_token_balance(&mut self, address: &str, balance: &Uint128) {
        if let Some(prev_balance) = self.token_querier.balance.get(address) {
            self.token_querier.supply -= prev_balance;
        }
        self.token_querier.supply += balance;
        self.token_querier
            .balance
            .insert(address.to_string(), *balance);
    }

    pub fn with_bank_balance(&mut self, address: &str, balance: Vec<Coin>) {
        self.base.update_balance(address, balance);
    }
}
