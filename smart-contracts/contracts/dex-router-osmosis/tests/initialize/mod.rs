use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Attribute, Coin, Decimal, Uint128};
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    CreateConcentratedLiquidityPoolsProposal, Pool, PoolRecord, PoolsRequest,
};
use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinPool;
use osmosis_std::types::osmosis::poolmanager::v1beta1::{SpotPriceRequest, SwapAmountInRoute};
use std::str::FromStr;

use osmosis_test_tube::osmosis_std::types::osmosis::gamm::poolmodels::balancer::v1beta1::MsgCreateBalancerPool;
use osmosis_test_tube::osmosis_std::types::osmosis::gamm::v1beta1::PoolAsset;
use osmosis_test_tube::{
    cosmrs::proto::traits::Message,
    osmosis_std::types::osmosis::concentratedliquidity::{
        poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool, v1beta1::MsgCreatePosition,
    },
    Account, ConcentratedLiquidity, GovWithAppAccess, Module, OsmosisTestApp, PoolManager,
    SigningAccount, Wasm,
};
use osmosis_test_tube::{ExecuteResponse, Gamm};

use dex_router_osmosis::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

pub(crate) const ADMIN_BALANCE_AMOUNT: u128 = 3402823669209384634633746074317682114u128;
const TOKENS_PROVIDED_AMOUNT: &str = "1000000000000";
pub(crate) const FEE_DENOM: &str = "uosmo";
pub(crate) const DENOM_BASE: &str = "udydx";
pub(crate) const DENOM_QUOTE: &str = "uryeth";
pub const INTERMEDIATE_QUOTE: &str = "uwosmo";
pub(crate) const TESTUBE_BINARY: &str =
    "./test-tube-build/wasm32-unknown-unknown/release/dex_router_osmosis.wasm";

#[cw_serde]
pub struct PoolWithDenoms {
    pub pool: u64,
    pub denom0: String,
    pub denom1: String,
}

pub fn single_cl_pool_fixture(
    app: &OsmosisTestApp,
    admin: &SigningAccount,
    denoms: Vec<String>,
    mut pools: Vec<PoolWithDenoms>,
) -> Vec<PoolWithDenoms> {
    let cl_pool = MsgCreateConcentratedPool {
        sender: "overwritten".to_string(),
        denom0: denoms[0].clone(),
        denom1: denoms[1].clone(),
        tick_spacing: 100,
        spread_factor: Decimal::from_str("0.01").unwrap().atomics().to_string(),
    };

    let pm = PoolManager::new(app);
    let cl = ConcentratedLiquidity::new(app);
    let gov = GovWithAppAccess::new(&app);

    gov.propose_and_execute(
        CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
        CreateConcentratedLiquidityPoolsProposal {
            title: "CL Pool".to_string(),
            description: "So that we can trade it".to_string(),
            pool_records: vec![PoolRecord {
                denom0: cl_pool.denom0.clone(),
                denom1: cl_pool.denom1.clone(),
                tick_spacing: cl_pool.tick_spacing.clone(),
                spread_factor: cl_pool.spread_factor.clone(),
            }],
        },
        admin.address(),
        &admin,
    )
    .unwrap();

    let pools_response = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();

    // Find the pool with the specified denoms
    let matching_pool = pools_response.pools.iter().find_map(|pool_any| {
        if let Ok(pool) = Pool::decode(pool_any.value.as_slice()) {
            if pool.token0 == cl_pool.denom0 && pool.token1 == cl_pool.denom1 {
                return Some(pool);
            }
        }
        None
    });

    if let Some(pool) = matching_pool {
        let mut tokens_provided = vec![
            OsmoCoin {
                denom: pool.token0.to_string(),
                amount: TOKENS_PROVIDED_AMOUNT.to_string(),
            },
            OsmoCoin {
                denom: pool.token1.to_string(),
                amount: TOKENS_PROVIDED_AMOUNT.to_string(),
            },
        ];

        tokens_provided.sort_by(|a, b| a.denom.cmp(&b.denom));

        cl.create_position(
            MsgCreatePosition {
                pool_id: pool.id,
                sender: admin.address(),
                lower_tick: -5000000,
                upper_tick: 500000,
                tokens_provided: tokens_provided.clone(),
                token_min_amount0: "1".to_string(),
                token_min_amount1: "1".to_string(),
            },
            &admin,
        )
        .unwrap();

        let spot_price = pm
            .query_spot_price(&SpotPriceRequest {
                base_asset_denom: tokens_provided[0].denom.to_string(),
                quote_asset_denom: tokens_provided[1].denom.to_string(),
                pool_id: pool.id,
            })
            .unwrap();
        assert_eq!(spot_price.spot_price, "1.000000000000000000");

        pools.push(PoolWithDenoms {
            pool: pool.id,
            denom0: cl_pool.denom0,
            denom1: cl_pool.denom1,
        });
    } else {
        println!("Pool with the specified denoms not found.");
    }

    pools
}

pub fn single_gamm_pool_fixture(
    app: &OsmosisTestApp,
    admin: &SigningAccount,
    denoms: Vec<String>,
    mut pools: Vec<PoolWithDenoms>,
) -> Vec<PoolWithDenoms> {
    let gamm_pool = MsgCreateBalancerPool {
        sender: "overwritten".to_string(),
        pool_params: None,
        pool_assets: vec![
            PoolAsset {
                weight: "1".to_string(),
                token: Some(
                    Coin {
                        denom: denoms[0].clone(),
                        amount: Uint128::from(1000000u128),
                    }
                    .into(),
                ),
            },
            PoolAsset {
                weight: "1".to_string(),
                token: Some(
                    Coin {
                        denom: denoms[1].clone(),
                        amount: Uint128::from(1000000u128),
                    }
                    .into(),
                ),
            },
        ],
        future_pool_governor: "overwritten".to_string(),
    };

    let gamm = Gamm::new(app);

    let response = gamm
        .create_basic_pool(
            &[
                Coin {
                    denom: gamm_pool.pool_assets[0]
                        .token
                        .as_ref()
                        .unwrap()
                        .denom
                        .to_string(),
                    amount: Uint128::from_str(
                        &gamm_pool.pool_assets[0].token.as_ref().unwrap().amount,
                    )
                    .unwrap(),
                },
                Coin {
                    denom: gamm_pool.pool_assets[1]
                        .token
                        .as_ref()
                        .unwrap()
                        .denom
                        .to_string(),
                    amount: Uint128::from_str(
                        &gamm_pool.pool_assets[1].token.as_ref().unwrap().amount,
                    )
                    .unwrap(),
                },
            ],
            &admin,
        )
        .unwrap();

    let ty = "pool_created";
    let keys = vec!["pool_id"];
    let pool_id: u64 = response
        .events
        .iter()
        .filter(|event| event.ty == ty)
        .flat_map(|event| event.attributes.clone())
        .filter(|attribute| keys.contains(&attribute.key.as_str()))
        .collect::<Vec<Attribute>>()
        .first()
        .unwrap()
        .value
        .parse()
        .unwrap();

    let _ = MsgJoinPool {
        sender: admin.address().to_string(),
        pool_id: pool_id.clone(),
        share_out_amount: "100".to_string(),
        token_in_maxs: vec![
            Coin {
                denom: gamm_pool.pool_assets[0]
                    .token
                    .as_ref()
                    .unwrap()
                    .denom
                    .to_string(),
                amount: Uint128::from_str(&gamm_pool.pool_assets[0].token.as_ref().unwrap().amount)
                    .unwrap(),
            }
            .into(),
            Coin {
                denom: gamm_pool.pool_assets[1]
                    .token
                    .as_ref()
                    .unwrap()
                    .denom
                    .to_string(),
                amount: Uint128::from_str(&gamm_pool.pool_assets[1].token.as_ref().unwrap().amount)
                    .unwrap(),
            }
            .into(),
        ],
    };

    pools.push(PoolWithDenoms {
        pool: pool_id,
        denom0: gamm_pool.pool_assets[0]
            .token
            .as_ref()
            .unwrap()
            .denom
            .to_string(),
        denom1: gamm_pool.pool_assets[1]
            .token
            .as_ref()
            .unwrap()
            .denom
            .to_string(),
    });

    pools
}

pub fn init_test_contract(app: &OsmosisTestApp, admin: &SigningAccount, filename: &str) -> Addr {
    // Create new osmosis appchain instance
    let wasm = Wasm::new(app);

    // Load compiled wasm bytecode
    let wasm_byte_code = std::fs::read(filename).unwrap();
    let code_id = wasm
        .store_code(&wasm_byte_code, None, admin)
        .unwrap()
        .data
        .code_id;

    // Instantiate dex router
    let contract = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {},
            Some(admin.address().as_str()),
            Some("dex-router-osmosis"),
            &[],
            admin,
        )
        .unwrap();

    Addr::unchecked(contract.data.address)
}

pub fn setup_paths(
    wasm: &Wasm<OsmosisTestApp>,
    contract_address: &Addr,
    path: Vec<u64>,
    offer_denom: String,
    ask_denom: String,
    admin: &SigningAccount,
) {
    wasm.execute(
        &contract_address.to_string(),
        &ExecuteMsg::SetPath {
            path,
            bidirectional: true,
            offer_denom,
            ask_denom,
        },
        &[],
        admin,
    )
    .unwrap();
}

pub fn query_paths(
    wasm: &Wasm<OsmosisTestApp>,
    contract_address: &Addr,
    offer_denom: String,
    ask_denom: String,
) -> Result<Vec<Vec<SwapAmountInRoute>>, osmosis_test_tube::RunnerError> {
    wasm.query(
        contract_address.as_str(),
        &QueryMsg::PathsForPair {
            offer_denom,
            ask_denom,
        },
    )
}
pub fn perform_swap(
    wasm: &Wasm<OsmosisTestApp>,
    contract_address: &Addr,
    offer_denom: String,
    ask_denom: String,
    path: Vec<Vec<SwapAmountInRoute>>,
    admin: &SigningAccount,
) -> Result<ExecuteResponse<MsgExecuteContractResponse>, osmosis_test_tube::RunnerError> {
    wasm.execute(
        &contract_address.to_string(),
        &ExecuteMsg::Swap {
            out_denom: ask_denom,
            path: Some(path.first().unwrap().clone()),
            minimum_receive: Some(Uint128::new(9500)),
        },
        &[Coin::new(10000u128, offer_denom)],
        admin,
    )
}
