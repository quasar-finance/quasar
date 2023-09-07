use cosmwasm_std::{
    from_binary, to_binary, Binary, ContractResult, ContractResult as CwContractResult, Empty,
    Querier, QuerierResult, QueryRequest,
};
use osmosis_std::types::cosmos::bank::v1beta1::{QuerySupplyOfRequest, QuerySupplyOfResponse};

use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{Pool, PoolsResponse};
use osmosis_std::types::osmosis::poolmanager::v1beta1::{PoolResponse, SpotPriceResponse};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin,
    osmosis::concentratedliquidity::v1beta1::{
        FullPositionBreakdown, PositionByIdRequest, PositionByIdResponse,
    },
};
pub struct QuasarQuerier {
    position: FullPositionBreakdown,
    current_tick: i64,
}

impl QuasarQuerier {
    pub fn new(position: FullPositionBreakdown, current_tick: i64) -> QuasarQuerier {
        QuasarQuerier {
            position,
            current_tick,
        }
    }
}

impl Querier for QuasarQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
        let request: QueryRequest<Empty> = from_binary(&Binary::from(bin_request)).unwrap();
        match request {
            QueryRequest::Stargate { path, data } => {
                println!("{}", path.as_str());
                match path.as_str() {
                    "/osmosis.concentratedliquidity.v1beta1.Query/PositionById" => {
                        let position_by_id_request: PositionByIdRequest =
                            prost::Message::decode(data.as_slice()).unwrap();
                        let position_id = position_by_id_request.position_id;
                        if position_id == self.position.position.clone().unwrap().position_id {
                            QuerierResult::Ok(CwContractResult::Ok(
                                to_binary(&PositionByIdResponse {
                                    position: Some(self.position.clone()),
                                })
                                .unwrap(),
                            ))
                        } else {
                            QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                                kind: format!("position id not found: {position_id:?}"),
                            })
                        }
                    }
                    "/cosmos.bank.v1beta1.Query/SupplyOf" => {
                        let query_supply_of_request: QuerySupplyOfRequest =
                            prost::Message::decode(data.as_slice()).unwrap();
                        let denom = query_supply_of_request.denom;
                        QuerierResult::Ok(CwContractResult::Ok(
                            to_binary(&QuerySupplyOfResponse {
                                amount: Some(OsmoCoin {
                                    denom,
                                    amount: 100.to_string(),
                                }),
                            })
                            .unwrap(),
                        ))
                    }
                    "/osmosis.poolmanager.v1beta1.Query/Pool" => {
                        QuerierResult::Ok(CwContractResult::Ok(
                            to_binary(&PoolResponse {
                                pool: Some(
                                    Pool {
                                        address: "idc".to_string(),
                                        incentives_address: "not being used".to_string(),
                                        spread_rewards_address: "not being used".to_string(),
                                        id: 1,
                                        current_tick_liquidity: "100".to_string(),
                                        token0: "uosmo".to_string(),
                                        token1: "uion".to_string(),
                                        current_sqrt_price: "not used".to_string(),
                                        current_tick: self.current_tick,
                                        tick_spacing: 100,
                                        exponent_at_price_one: -6,
                                        spread_factor: "not used".to_string(),
                                        last_liquidity_update: None,
                                    }
                                    .to_any(),
                                ),
                            })
                            .unwrap(),
                        ))
                    }
                    "/osmosis.poolmanager.v1beta1.Query/SpotPrice" => {
                        QuerierResult::Ok(CwContractResult::Ok(
                            to_binary(&SpotPriceResponse {
                                spot_price: "1.5".to_string(),
                            })
                            .unwrap(),
                        ))
                    }
                    &_ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                        kind: format!("Unmocked stargate query path: {path:?}"),
                    }),
                }
            }
            _ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                kind: format!("Unmocked query type: {request:?}"),
            }),
        }
        // QuerierResult::Ok(ContractResult::Ok(to_binary(&"hello").unwrap()))
    }
}
