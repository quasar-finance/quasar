use std::marker::PhantomData;

use cosmwasm_std::testing::{BankQuerier, MockApi, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_json, to_json_binary, Addr, BankQuery, Binary, Coin, ContractResult as CwContractResult,
    Decimal, Empty, MessageInfo, OwnedDeps, Querier, QuerierResult, QueryRequest,
};
use osmosis_std::types::cosmos::bank::v1beta1::{QuerySupplyOfRequest, QuerySupplyOfResponse};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::Pool;
use osmosis_std::types::osmosis::poolmanager::{
    v1beta1::{PoolResponse, SpotPriceResponse},
    v2::SpotPriceResponse as V2SpotPriceResponse,
};
use osmosis_std::types::osmosis::twap::v1beta1::ArithmeticTwapToNowResponse;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin,
    osmosis::concentratedliquidity::v1beta1::{
        FullPositionBreakdown, Position as OsmoPosition, PositionByIdRequest, PositionByIdResponse,
    },
};

use crate::math::tick::tick_to_price;
use crate::state::{
    PoolConfig, Position, VaultConfig, POOL_CONFIG, POSITION, RANGE_ADMIN, VAULT_CONFIG,
};

pub struct QuasarQuerier {
    position: FullPositionBreakdown,
    current_tick: i64,
    bank: BankQuerier,
}

impl QuasarQuerier {
    pub fn new(position: FullPositionBreakdown, current_tick: i64) -> QuasarQuerier {
        QuasarQuerier {
            position,
            current_tick,
            bank: BankQuerier::new(&[]),
        }
    }

    pub fn new_with_balances(
        position: FullPositionBreakdown,
        current_tick: i64,
        balances: &[(&str, &[Coin])],
    ) -> QuasarQuerier {
        QuasarQuerier {
            position,
            current_tick,
            bank: BankQuerier::new(balances),
        }
    }
}

impl Querier for QuasarQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
        let request: QueryRequest<Empty> = from_json(&Binary::from(bin_request)).unwrap();
        match request {
            QueryRequest::Stargate { path, data } => match path.as_str() {
                "/osmosis.concentratedliquidity.v1beta1.Query/PositionById" => {
                    let position_by_id_request: PositionByIdRequest =
                        prost::Message::decode(data.as_slice()).unwrap();
                    let position_id = position_by_id_request.position_id;
                    if position_id == self.position.position.clone().unwrap().position_id {
                        QuerierResult::Ok(CwContractResult::Ok(
                            to_json_binary(&PositionByIdResponse {
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
                        to_json_binary(&QuerySupplyOfResponse {
                            amount: Some(OsmoCoin {
                                denom,
                                amount: 100000.to_string(),
                            }),
                        })
                        .unwrap(),
                    ))
                }
                "/cosmos.bank.v1beta.Query/Balance" => {
                    let query: BankQuery = from_json(&Binary::from(bin_request)).unwrap();
                    self.bank.query(&query)
                }
                "/cosmos.bank.v1beta.Query/AllBalances" => {
                    let query: BankQuery = from_json(&Binary::from(bin_request)).unwrap();
                    self.bank.query(&query)
                }
                "/osmosis.poolmanager.v1beta1.Query/Pool" => {
                    QuerierResult::Ok(CwContractResult::Ok(
                        to_json_binary(&PoolResponse {
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
                        to_json_binary(&SpotPriceResponse {
                            spot_price: tick_to_price(self.current_tick).unwrap().to_string(),
                        })
                        .unwrap(),
                    ))
                }
                "/osmosis.poolmanager.v2.Query/SpotPriceV2" => {
                    QuerierResult::Ok(CwContractResult::Ok(
                        to_json_binary(&V2SpotPriceResponse {
                            spot_price: tick_to_price(self.current_tick).unwrap().to_string(),
                        })
                        .unwrap(),
                    ))
                }
                "/osmosis.twap.v1beta1.Query/ArithmeticTwapToNow" => {
                    QuerierResult::Ok(CwContractResult::Ok(
                        to_json_binary(&ArithmeticTwapToNowResponse {
                            arithmetic_twap: tick_to_price(self.current_tick).unwrap().to_string(),
                        })
                        .unwrap(),
                    ))
                }
                &_ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                    kind: format!("Unmocked stargate query path: {path:?}"),
                }),
            },
            QueryRequest::Bank(query) => self.bank.query(&query),
            _ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                kind: format!("Unmocked query type: {request:?}"),
            }),
        }
        // QuerierResult::Ok(ContractResult::Ok(to_json_binary(&"hello").unwrap()))
    }
}

pub fn mock_deps_with_querier_with_balance(
    info: &MessageInfo,
    balances: &[(&str, &[Coin])],
) -> OwnedDeps<MockStorage, MockApi, QuasarQuerier, Empty> {
    let mut deps = OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: QuasarQuerier::new_with_balances(
            FullPositionBreakdown {
                position: Some(OsmoPosition {
                    position_id: 1,
                    address: MOCK_CONTRACT_ADDR.to_string(),
                    pool_id: 1,
                    lower_tick: 100,
                    upper_tick: 1000,
                    join_time: None,
                    liquidity: "1000000.1".to_string(),
                }),
                asset0: Some(OsmoCoin {
                    denom: "token0".to_string(),
                    amount: "1000000".to_string(),
                }),
                asset1: Some(OsmoCoin {
                    denom: "token1".to_string(),
                    amount: "1000000".to_string(),
                }),
                claimable_spread_rewards: vec![
                    OsmoCoin {
                        denom: "token0".to_string(),
                        amount: "100".to_string(),
                    },
                    OsmoCoin {
                        denom: "token1".to_string(),
                        amount: "100".to_string(),
                    },
                ],
                claimable_incentives: vec![],
                forfeited_incentives: vec![],
            },
            500,
            balances,
        ),
        custom_query_type: PhantomData,
    };

    let storage = &mut deps.storage;

    RANGE_ADMIN.save(storage, &info.sender).unwrap();
    POOL_CONFIG
        .save(
            storage,
            &PoolConfig {
                pool_id: 1,
                token0: "token0".to_string(),
                token1: "token1".to_string(),
            },
        )
        .unwrap();
    VAULT_CONFIG
        .save(
            storage,
            &VaultConfig {
                performance_fee: Decimal::zero(),
                treasury: Addr::unchecked("treasure"),
                swap_max_slippage: Decimal::from_ratio(1u128, 20u128),
                dex_router: Addr::unchecked("dex_router"),
            },
        )
        .unwrap();
    POSITION
        .save(
            storage,
            &crate::state::Position {
                position_id: 1,
                join_time: 0,
                claim_after: None,
            },
        )
        .unwrap();

    deps
}

pub fn mock_deps_with_querier(
    info: &MessageInfo,
) -> OwnedDeps<MockStorage, MockApi, QuasarQuerier, Empty> {
    let position_id = 1;

    let mut deps = OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: QuasarQuerier::new(
            FullPositionBreakdown {
                position: Some(OsmoPosition {
                    position_id,
                    address: MOCK_CONTRACT_ADDR.to_string(),
                    pool_id: 1,
                    lower_tick: 100,
                    upper_tick: 1000,
                    join_time: None,
                    liquidity: "1000000.1".to_string(),
                }),
                asset0: Some(OsmoCoin {
                    denom: "token0".to_string(),
                    amount: "1000000".to_string(),
                }),
                asset1: Some(OsmoCoin {
                    denom: "token1".to_string(),
                    amount: "1000000".to_string(),
                }),
                claimable_spread_rewards: vec![
                    OsmoCoin {
                        denom: "token0".to_string(),
                        amount: "100".to_string(),
                    },
                    OsmoCoin {
                        denom: "token1".to_string(),
                        amount: "100".to_string(),
                    },
                ],
                claimable_incentives: vec![],
                forfeited_incentives: vec![],
            },
            500,
        ),
        custom_query_type: PhantomData,
    };

    let storage = &mut deps.storage;

    POSITION
        .save(
            storage,
            &Position {
                position_id,
                join_time: 0,
                claim_after: None,
            },
        )
        .unwrap();

    RANGE_ADMIN.save(storage, &info.sender).unwrap();
    POOL_CONFIG
        .save(
            storage,
            &PoolConfig {
                pool_id: 1,
                token0: "token0".to_string(),
                token1: "token1".to_string(),
            },
        )
        .unwrap();
    VAULT_CONFIG
        .save(
            storage,
            &VaultConfig {
                performance_fee: Decimal::zero(),
                treasury: Addr::unchecked("treasure"),
                swap_max_slippage: Decimal::from_ratio(1u128, 20u128),
                dex_router: Addr::unchecked("dex_router"),
            },
        )
        .unwrap();
    POSITION
        .save(
            storage,
            &crate::state::Position {
                position_id: 1,
                join_time: 0,
                claim_after: None,
            },
        )
        .unwrap();

    deps
}
