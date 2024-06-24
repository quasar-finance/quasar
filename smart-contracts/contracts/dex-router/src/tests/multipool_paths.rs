use std::str::FromStr;

use cosmwasm_std::{Addr, Attribute, Coin, Decimal, Uint128};
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    CreateConcentratedLiquidityPoolsProposal, Pool, PoolRecord, PoolsRequest,
};
use osmosis_std::types::osmosis::gamm::v1beta1::{MsgJoinPool, MsgJoinPoolResponse};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SpotPriceRequest;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgMint};

use osmosis_test_tube::osmosis_std::types::osmosis::gamm::poolmodels::balancer::v1beta1::MsgCreateBalancerPool;
use osmosis_test_tube::osmosis_std::types::osmosis::gamm::v1beta1::PoolAsset;
use osmosis_test_tube::{
    cosmrs::proto::traits::Message,
    osmosis_std::types::osmosis::concentratedliquidity::{
        poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool, v1beta1::MsgCreatePosition,
    },
    Account, ConcentratedLiquidity, GovWithAppAccess, Module, OsmosisTestApp, PoolManager,
    SigningAccount, TokenFactory, Wasm,
};
use osmosis_test_tube::{ExecuteResponse, Gamm, Runner};

use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::tests::helpers::sort_tokens;

use super::initialize::*;

pub fn multiple_pool_init() -> (OsmosisTestApp, Addr, Vec<PoolWithDenoms>, SigningAccount) {
    let app = OsmosisTestApp::new();
    let tf = TokenFactory::new(&app);

    // Create new account with initial funds
    let admin = app
        .init_account(&[Coin::new(ADMIN_BALANCE_AMOUNT, FEE_DENOM)])
        .unwrap();

    let res0 = tf
        .create_denom(
            MsgCreateDenom {
                sender: admin.address().to_string(),
                subdenom: DENOM_BASE.to_string(),
            },
            &admin,
        )
        .unwrap();
    let res1 = tf
        .create_denom(
            MsgCreateDenom {
                sender: admin.address().to_string(),
                subdenom: DENOM_QUOTE.to_string(),
            },
            &admin,
        )
        .unwrap();
    let res2 = tf
        .create_denom(
            MsgCreateDenom {
                sender: admin.address().to_string(),
                subdenom: INTERMEDIATE_BASE.to_string(),
            },
            &admin,
        )
        .unwrap();
    let res3 = tf
        .create_denom(
            MsgCreateDenom {
                sender: admin.address().to_string(),
                subdenom: INTERMEDIATE_QUOTE.to_string(),
            },
            &admin,
        )
        .unwrap();

    let denom0 = res0.data.new_token_denom;
    let denom1 = res1.data.new_token_denom;
    let denom2 = res2.data.new_token_denom;
    let denom3 = res3.data.new_token_denom;

    tf.mint(
        MsgMint {
            sender: admin.address().to_string(),
            amount: Some(OsmoCoin {
                denom: denom0.clone(),
                amount: ADMIN_BALANCE_AMOUNT.to_string(),
            }),
            mint_to_address: admin.address().to_string(),
        },
        &admin,
    )
    .unwrap();

    tf.mint(
        MsgMint {
            sender: admin.address().to_string(),
            amount: Some(OsmoCoin {
                denom: denom1.clone(),
                amount: ADMIN_BALANCE_AMOUNT.to_string(),
            }),
            mint_to_address: admin.address().to_string(),
        },
        &admin,
    )
    .unwrap();
    tf.mint(
        MsgMint {
            sender: admin.address().to_string(),
            amount: Some(OsmoCoin {
                denom: denom2.clone(),
                amount: ADMIN_BALANCE_AMOUNT.to_string(),
            }),
            mint_to_address: admin.address().to_string(),
        },
        &admin,
    )
    .unwrap();
    tf.mint(
        MsgMint {
            sender: admin.address().to_string(),
            amount: Some(OsmoCoin {
                denom: denom3.clone(),
                amount: ADMIN_BALANCE_AMOUNT.to_string(),
            }),
            mint_to_address: admin.address().to_string(),
        },
        &admin,
    )
    .unwrap();

    init_test_contract(
        app,
        admin,
        "/workspaces/quasar/smart-contracts/target/wasm32-unknown-unknown/release/dex_router.wasm",
        vec![MsgCreateConcentratedPool {
            sender: "overwritten".to_string(),
            denom0: denom0.to_string(),
            denom1: denom1.to_string(),
            tick_spacing: 100,
            spread_factor: Decimal::from_str("0.01").unwrap().atomics().to_string(),
        }],
        vec![
            MsgCreateBalancerPool {
                sender: "overwritten".to_string(),
                pool_params: None,
                pool_assets: vec![
                    PoolAsset {
                        weight: "1".to_string(),
                        token: Some(
                            Coin {
                                denom: denom0.to_string(),
                                amount: Uint128::from(1000000u128),
                            }
                            .into(),
                        ),
                    },
                    PoolAsset {
                        weight: "1".to_string(),
                        token: Some(
                            Coin {
                                denom: denom1.to_string(),
                                amount: Uint128::from(1000000u128),
                            }
                            .into(),
                        ),
                    },
                ],
                future_pool_governor: "overwritten".to_string(),
            },
            MsgCreateBalancerPool {
                sender: "overwritten".to_string(),
                pool_params: None,
                pool_assets: vec![
                    PoolAsset {
                        weight: "1".to_string(),
                        token: Some(
                            Coin {
                                denom: denom0.to_string(),
                                amount: Uint128::from(1000000u128),
                            }
                            .into(),
                        ),
                    },
                    PoolAsset {
                        weight: "1".to_string(),
                        token: Some(
                            Coin {
                                denom: denom2.to_string(),
                                amount: Uint128::from(1000000u128),
                            }
                            .into(),
                        ),
                    },
                ],
                future_pool_governor: "overwritten".to_string(),
            },
            MsgCreateBalancerPool {
                sender: "overwritten".to_string(),
                pool_params: None,
                pool_assets: vec![
                    PoolAsset {
                        weight: "1".to_string(),
                        token: Some(
                            Coin {
                                denom: denom1.to_string(),
                                amount: Uint128::from(1000000u128),
                            }
                            .into(),
                        ),
                    },
                    PoolAsset {
                        weight: "1".to_string(),
                        token: Some(
                            Coin {
                                denom: denom2.to_string(),
                                amount: Uint128::from(1000000u128),
                            }
                            .into(),
                        ),
                    },
                ],
                future_pool_governor: "overwritten".to_string(),
            },
        ],
    )
}

pub fn init_test_contract(
    app: OsmosisTestApp,
    admin: SigningAccount,
    filename: &str,
    cl_pools: Vec<MsgCreateConcentratedPool>,
    gamm_pools: Vec<MsgCreateBalancerPool>,
) -> (OsmosisTestApp, Addr, Vec<PoolWithDenoms>, SigningAccount) {
    // Create new osmosis appchain instance
    let pm = PoolManager::new(&app);
    let cl = ConcentratedLiquidity::new(&app);
    let wasm = Wasm::new(&app);

    // Load compiled wasm bytecode
    let wasm_byte_code = std::fs::read(filename).unwrap();
    let code_id = wasm
        .store_code(&wasm_byte_code, None, &admin)
        .unwrap()
        .data
        .code_id;

    let mut pools: Vec<PoolWithDenoms> = vec![];

    let gov = GovWithAppAccess::new(&app);
    for cl_pool in cl_pools {
        // Setup a dummy CL pool to work with
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

        // Get just created pool information by querying all the pools, and taking the first one
        let pools_response = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let pool: Pool = Pool::decode(pools_response.pools[0].value.as_slice()).unwrap();

        let tokens_provided = vec![
            OsmoCoin {
                denom: cl_pool.denom0.to_string(),
                amount: TOKENS_PROVIDED_AMOUNT.to_string(),
            },
            OsmoCoin {
                denom: cl_pool.denom1.to_string(),
                amount: TOKENS_PROVIDED_AMOUNT.to_string(),
            },
        ];
        // Create a first position in the pool with the admin user
        cl.create_position(
            MsgCreatePosition {
                pool_id: pool.id,
                sender: admin.address(),
                lower_tick: -5000000, // 0.5 spot price
                upper_tick: 500000,   // 1.5 spot price
                tokens_provided: tokens_provided.clone(),
                token_min_amount0: "1".to_string(),
                token_min_amount1: "1".to_string(),
            },
            &admin,
        )
        .unwrap();

        // Get and assert spot price is 1.0
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
    }

    for gamm_pool in gamm_pools {
        // Create a new pool
        let gamm = Gamm::new(&app);
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

        let add_liq = MsgJoinPool {
            sender: admin.address().to_string(),
            pool_id: pool_id.clone(),
            share_out_amount: "100".to_string(),
            token_in_maxs: sort_tokens(vec![
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
            ])
            .iter()
            .map(|c| OsmoCoin {
                denom: c.denom.clone(),
                amount: c.amount.to_string(),
            })
            .collect(),
        };

        let _res: ExecuteResponse<MsgJoinPoolResponse> =
            app.execute(add_liq, MsgJoinPool::TYPE_URL, &admin).unwrap();

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
        })
    }

    // Instantiate vault
    let contract = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {},
            Some(admin.address().as_str()),
            Some("cw-dex-router"),
            &[],
            &admin,
        )
        .unwrap();
    // // Sort tokens alphabetically by denom name or Osmosis will return an error
    // tokens_provided.sort_by(|a, b| a.denom.cmp(&b.denom)); // can't use helpers.rs::sort_tokens() due to different Coin type

    // // Increment the app time for twaps to function, this is needed to do not fail on querying a twap for a timeframe higher than the chain existence
    // app.increase_time(1000000);

    (app, Addr::unchecked(contract.data.address), pools, admin)
}

#[test]
fn multiple_pools_work() {
    let (app, contract_address, pools, admin) = multiple_pool_init();
    let wasm = Wasm::new(&app);

    let _ = app
        .init_account(&[
            Coin::new(100_000_000u128, DENOM_BASE),
            Coin::new(ADMIN_BALANCE_AMOUNT, FEE_DENOM),
        ])
        .unwrap();

    for pool in pools.clone() {
        let _ = wasm
            .execute(
                &contract_address.to_string(),
                &ExecuteMsg::SetPath {
                    offer_asset: cw_asset::AssetInfo::native(pool.denom0.clone()).into(),
                    ask_asset: cw_asset::AssetInfo::native(pool.denom1.clone()).into(),
                    path: vec![SwapAmountInRoute {
                        pool_id: pool.pool,
                        token_out_denom: pool.denom1,
                    }],
                    bidirectional: true,
                },
                &[],
                &admin,
            )
            .unwrap();
    }

    // let resp: BestPathForPairResponse = wasm
    //     .query(
    //         &contract_address.to_string(),
    //         &QueryMsg::BestPathForPair {
    //             offer_asset: apollo_cw_asset::AssetInfoBase::Native(
    //                 pools.first().unwrap().denom0.clone(),
    //             ),
    //             ask_asset: apollo_cw_asset::AssetInfoBase::Native(
    //                 pools.first().unwrap().denom1.clone(),
    //             ),
    //             offer_amount: Uint128::from(10000u128),
    //             exclude_paths: None,
    //         },
    //     )
    //     .unwrap();

    // println!(
    //     "operationS: {:?} going from {:?} to {:?}",
    //     resp.operations,
    //     pools.first().unwrap().denom0.clone(),
    //     pools.first().unwrap().denom1.clone()
    // );

    // let pool_manager = PoolManager::new(&app);
    // let pool_resp = pool_manager
    //     .query_pool(&PoolRequest { pool_id: 1 })
    //     .unwrap();
    // let pool = Pool::decode(pool_resp.pool.unwrap().value.as_ref()).unwrap();
    // println!("pool: {:?}", pool);

    // let mut iter = resp.operations.clone().into_iter();
    // // the first swap should be over pool 1
    // assert_eq!(iter.next().unwrap().pool_id, 1);
    // assert!(iter.next().is_none());

    // let _ = wasm
    //     .execute(
    //         &contract_address.to_string(),
    //         &ExecuteMsg::ExecuteSwapOperations {
    //             operations: resp.operations.into(),
    //             minimum_receive: Some(Uint128::one()),
    //             to: None,
    //         },
    //         &[Coin::new(10000u128, pools.first().unwrap().denom0.clone())],
    //         &admin,
    //     )
    //     .unwrap();
}
