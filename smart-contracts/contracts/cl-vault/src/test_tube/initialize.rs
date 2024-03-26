#[cfg(test)]
pub mod initialize {
    use apollo_cw_asset::{AssetInfoBase, AssetInfoUnchecked};
    use cosmwasm_std::{coin, Addr, Coin, Decimal, Uint128};
    use cw_dex::osmosis::OsmosisPool;
    use cw_dex_router::msg::ExecuteMsg as DexExecuteMsg;
    use cw_dex_router::msg::InstantiateMsg as DexInstantiate;
    use cw_dex_router::operations::{SwapOperationBase, SwapOperationsListUnchecked};
    use cw_vault_multi_standard::VaultInfoResponse;
    use osmosis_std::types::cosmos::base::v1beta1;
    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
        CreateConcentratedLiquidityPoolsProposal, Pool, PoolRecord, PoolsRequest,
    };
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SpotPriceRequest, SwapAmountInRoute,
    };
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::QueryDenomsFromCreatorRequest;
    use osmosis_test_tube::{
        cosmrs::proto::traits::Message,
        osmosis_std::types::osmosis::concentratedliquidity::{
            poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool, v1beta1::MsgCreatePosition,
        },
        Account, ConcentratedLiquidity, Gamm, GovWithAppAccess, Module, OsmosisTestApp,
        PoolManager, SigningAccount, TokenFactory, Wasm,
    };
    use std::str::FromStr;

    use crate::helpers::sort_tokens;
    use crate::msg::{
        ClQueryMsg, ExecuteMsg, ExtensionQueryMsg, InstantiateMsg, ModifyRangeMsg, QueryMsg,
    };
    use crate::query::PoolResponse;
    use crate::state::VaultConfig;

    const ADMIN_BALANCE_AMOUNT: u128 = 340282366920938463463374607431768211455u128;
    const TOKENS_PROVIDED_AMOUNT: &str = "1000000000000";
    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";
    const DENOM_REWARD: &str = "ustride";

    pub fn default_init() -> (OsmosisTestApp, Addr, u64, SigningAccount) {
        init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: DENOM_BASE.to_string(),
                denom1: DENOM_QUOTE.to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.01").unwrap().atomics().to_string(),
            },
            -5000000, // 0.5 spot price
            500000,   // 1.5 spot price
            vec![
                v1beta1::Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                },
                v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        )
    }

    pub fn dex_cl_init_lp_pools() -> (OsmosisTestApp, Addr, Addr, u64, Vec<u64>, SigningAccount) {
        // TODO: We should be reusing the init_test_contract() for basic setup,
        // and init_cl_vault_with_dex_router_contract_with_lp_pools should be like init_dex_router_contract_with_lp_pools only.
        init_cl_vault_with_dex_router_contract_with_lp_pools(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            "./test-tube-build/wasm32-unknown-unknown/release/cw_dex_router.wasm",
            &[
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_REWARD),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: DENOM_BASE.to_string(),
                denom1: DENOM_QUOTE.to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.01").unwrap().atomics().to_string(),
            },
            -5000000, // 0.5 spot price
            500000,   // 1.5 spot price
            vec![
                v1beta1::Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                },
                v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        )
    }

    pub fn init_cl_vault_with_dex_router_contract_with_lp_pools(
        filename_cl: &str,
        filename_dex: &str,
        admin_balance: &[Coin],
        pool: MsgCreateConcentratedPool,
        lower_tick: i64,
        upper_tick: i64,
        mut tokens_provided: Vec<v1beta1::Coin>,
        token_min_amount0: Uint128,
        token_min_amount1: Uint128,
    ) -> (OsmosisTestApp, Addr, Addr, u64, Vec<u64>, SigningAccount) {
        // Create new osmosis appchain instance
        let app = OsmosisTestApp::new();
        let pm = PoolManager::new(&app);
        let gm = Gamm::new(&app);
        let cl = ConcentratedLiquidity::new(&app);
        let wasm = Wasm::new(&app);

        let admin = app.init_account(admin_balance).unwrap();

        // Load compiled wasm bytecode
        let wasm_byte_code_cl = std::fs::read(filename_cl).unwrap();
        let code_id_cl = wasm
            .store_code(&wasm_byte_code_cl, None, &admin)
            .unwrap()
            .data
            .code_id;

        // Load compiled wasm bytecode
        let wasm_byte_code_dex = std::fs::read(filename_dex).unwrap();
        let code_id_dex = wasm
            .store_code(&wasm_byte_code_dex, None, &admin)
            .unwrap()
            .data
            .code_id;

        // Setup a dummy CL pool to work with
        let gov = GovWithAppAccess::new(&app);
        gov.propose_and_execute(
            CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
            CreateConcentratedLiquidityPoolsProposal {
                title: "CL Pool".to_string(),
                description: "So that we can trade it".to_string(),
                pool_records: vec![PoolRecord {
                    denom0: pool.denom0,
                    denom1: pool.denom1,
                    tick_spacing: pool.tick_spacing,
                    spread_factor: pool.spread_factor,
                }],
            },
            admin.address(),
            &admin,
        )
        .unwrap();

        // Get just created pool information by querying all the pools, and taking the first one
        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let pool: Pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        // Finishing basic CL setup ... starting advanced multi-pool setup for DexRouter support and Gamm Balancer pools

        // Declare Balancer LP pools coins
        let pools_coins = vec![
            vec![
                Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: Uint128::new(10000000000000000000000),
                },
                Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: Uint128::new(10000000000000000000000),
                },
            ],
            vec![
                Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: Uint128::new(10000000000000000000000),
                },
                Coin {
                    denom: DENOM_REWARD.to_string(),
                    amount: Uint128::new(10000000000000000000000),
                },
            ],
            vec![
                Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: Uint128::new(10000000000000000000000),
                },
                Coin {
                    denom: DENOM_REWARD.to_string(),
                    amount: Uint128::new(10000000000000000000000),
                },
            ],
        ];

        // Create Balancer pools with previous vec of vec of coins
        let mut lp_pools = vec![];
        for pool_coins in &pools_coins {
            let lp_pool = gm.create_basic_pool(&pool_coins, &admin).unwrap();
            lp_pools.push(lp_pool.data.pool_id);
        }

        // Sort tokens alphabetically by denom name or Osmosis will return an error
        tokens_provided.sort_by(|a, b| a.denom.cmp(&b.denom)); // can't use helpers.rs::sort_tokens() due to different Coin type

        // Create a first position in the pool with the admin user
        cl.create_position(
            MsgCreatePosition {
                pool_id: pool.id,
                sender: admin.address(),
                lower_tick,
                upper_tick,
                tokens_provided: tokens_provided.clone(),
                token_min_amount0: token_min_amount0.to_string(),
                token_min_amount1: token_min_amount1.to_string(),
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

        // Increment the app time for twaps to function, this is needed to do not fail on querying a twap for a timeframe higher than the chain existence
        app.increase_time(1000000);

        // Instantiate Dex Router
        let contract_dex_router = wasm
            .instantiate(
                code_id_dex,
                &DexInstantiate {},
                Some(admin.address().as_str()),
                Some("dex-router"),
                sort_tokens(vec![]).as_ref(),
                &admin,
            )
            .unwrap();

        // Set Dex Router contract paths
        for (index, lp_pool_id) in lp_pools.iter().enumerate() {
            wasm.execute(
                &contract_dex_router.data.address,
                &DexExecuteMsg::SetPath {
                    offer_asset: AssetInfoUnchecked::Native(
                        pools_coins[index][0].denom.to_string(),
                    ),
                    ask_asset: AssetInfoUnchecked::Native(pools_coins[index][1].denom.to_string()),
                    path: SwapOperationsListUnchecked::new(vec![SwapOperationBase {
                        pool: cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool_id.clone())),
                        offer_asset_info: AssetInfoBase::Native(
                            pools_coins[index][0].denom.to_string(),
                        ),
                        ask_asset_info: AssetInfoBase::Native(
                            pools_coins[index][1].denom.to_string(),
                        ),
                    }]),
                    bidirectional: true,
                },
                vec![].as_ref(),
                &admin,
            )
            .unwrap();
        }

        // Set additional path
        wasm.execute(
            &contract_dex_router.data.address,
            &DexExecuteMsg::SetPath {
                offer_asset: AssetInfoUnchecked::Native(DENOM_REWARD.to_string()),
                ask_asset: AssetInfoUnchecked::Native(DENOM_BASE.to_string()),
                path: SwapOperationsListUnchecked::new(vec![
                    SwapOperationBase {
                        pool: cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pools[1])),
                        ask_asset_info: AssetInfoBase::Native(DENOM_QUOTE.to_string()),
                        offer_asset_info: AssetInfoBase::Native(DENOM_REWARD.to_string()),
                    },
                    SwapOperationBase {
                        pool: cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pools[0])),
                        offer_asset_info: AssetInfoBase::Native(DENOM_QUOTE.to_string()),
                        ask_asset_info: AssetInfoBase::Native(DENOM_BASE.to_string()),
                    },
                ]),
                bidirectional: true,
            },
            sort_tokens(vec![]).as_ref(),
            &admin,
        )
        .unwrap();

        // Instantiate vault
        let contract_cl = wasm
            .instantiate(
                code_id_cl,
                &InstantiateMsg {
                    admin: admin.address(),
                    pool_id: pool.id,
                    config: VaultConfig {
                        performance_fee: Decimal::percent(20),
                        treasury: Addr::unchecked(admin.address()),
                        swap_max_slippage: Decimal::bps(5),
                        dex_router: Addr::unchecked(contract_dex_router.clone().data.address),
                    },
                    vault_token_subdenom: "utestvault".to_string(),
                    range_admin: admin.address(),
                    initial_lower_tick: lower_tick,
                    initial_upper_tick: upper_tick,
                    thesis: "Provide big swap efficiency".to_string(),
                    name: "Contract".to_string(),
                    auto_compound_admin: admin.address(),
                },
                Some(admin.address().as_str()),
                Some("cl-vault"),
                sort_tokens(vec![coin(1000, pool.token0), coin(1000, pool.token1)]).as_ref(),
                &admin,
            )
            .unwrap();

        (
            app,
            Addr::unchecked(contract_cl.data.address),
            Addr::unchecked(contract_dex_router.data.address),
            pool.id,
            lp_pools,
            admin,
        )
    }

    pub fn dex_cl_init_cl_pools() -> (
        OsmosisTestApp,
        Addr,
        Addr,
        u64,
        u64,
        u64,
        u64,
        SigningAccount,
    ) {
        init_cl_vault_with_dex_router_contract_cl_pools(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            "./test-tube-build/wasm32-unknown-unknown/release/cw_dex_router.wasm",
            &[
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_REWARD),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: DENOM_BASE.to_string(),
                denom1: DENOM_QUOTE.to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.01").unwrap().atomics().to_string(),
            },
            -5000000, // 0.5 spot price
            500000,   // 1.5 spot price
            vec![
                v1beta1::Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                },
                v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT.to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        )
    }

    pub fn init_cl_vault_with_dex_router_contract_cl_pools(
        filename_cl: &str,
        filename_dex: &str,
        admin_balance: &[Coin],
        pool: MsgCreateConcentratedPool,
        lower_tick: i64,
        upper_tick: i64,
        mut tokens_provided: Vec<v1beta1::Coin>,
        token_min_amount0: Uint128,
        token_min_amount1: Uint128,
    ) -> (
        OsmosisTestApp,
        Addr,
        Addr,
        u64,
        u64,
        u64,
        u64,
        SigningAccount,
    ) {
        // Create new osmosis appchain instance
        let app = OsmosisTestApp::new();
        let pm = PoolManager::new(&app);
        let cl = ConcentratedLiquidity::new(&app);
        let wasm = Wasm::new(&app);

        let admin = app.init_account(admin_balance).unwrap();

        // Load compiled wasm bytecode
        let wasm_byte_code_cl = std::fs::read(filename_cl).unwrap();
        let code_id_cl = wasm
            .store_code(&wasm_byte_code_cl, None, &admin)
            .unwrap()
            .data
            .code_id;

        // Load compiled wasm bytecode
        let wasm_byte_code_dex = std::fs::read(filename_dex).unwrap();
        let code_id_dex = wasm
            .store_code(&wasm_byte_code_dex, None, &admin)
            .unwrap()
            .data
            .code_id;

        // Setup a dummy CL pool to work with
        let gov = GovWithAppAccess::new(&app);
        gov.propose_and_execute(
            CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
            CreateConcentratedLiquidityPoolsProposal {
                title: "CL Pool".to_string(),
                description: "So that we can trade it".to_string(),
                pool_records: vec![PoolRecord {
                    denom0: pool.clone().denom0,
                    denom1: pool.clone().denom1,
                    tick_spacing: pool.clone().tick_spacing,
                    spread_factor: pool.clone().spread_factor,
                }],
            },
            admin.address(),
            &admin,
        )
        .unwrap();

        gov.propose_and_execute(
            CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
            CreateConcentratedLiquidityPoolsProposal {
                title: "CL Pool".to_string(),
                description: "So that we can trade it".to_string(),
                pool_records: vec![PoolRecord {
                    denom0: DENOM_BASE.to_string(),
                    denom1: DENOM_QUOTE.to_string(),
                    tick_spacing: pool.clone().tick_spacing,
                    spread_factor: pool.clone().spread_factor,
                }],
            },
            admin.address(),
            &admin,
        )
        .unwrap();

        gov.propose_and_execute(
            CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
            CreateConcentratedLiquidityPoolsProposal {
                title: "CL Pool".to_string(),
                description: "So that we can trade it".to_string(),
                pool_records: vec![PoolRecord {
                    denom0: DENOM_QUOTE.to_string(),
                    denom1: DENOM_REWARD.to_string(),
                    tick_spacing: pool.clone().tick_spacing,
                    spread_factor: pool.clone().spread_factor,
                }],
            },
            admin.address(),
            &admin,
        )
        .unwrap();

        gov.propose_and_execute(
            CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
            CreateConcentratedLiquidityPoolsProposal {
                title: "CL Pool".to_string(),
                description: "So that we can trade it".to_string(),
                pool_records: vec![PoolRecord {
                    denom0: DENOM_BASE.to_string(),
                    denom1: DENOM_REWARD.to_string(),
                    tick_spacing: pool.clone().tick_spacing,
                    spread_factor: pool.clone().spread_factor,
                }],
            },
            admin.address(),
            &admin,
        )
        .unwrap();

        let pool_1_coins = vec![
            v1beta1::Coin {
                denom: DENOM_BASE.to_string(),
                amount: "10000000000000000000000".to_string(),
            },
            v1beta1::Coin {
                denom: DENOM_QUOTE.to_string(),
                amount: "10000000000000000000000".to_string(),
            },
        ];
        // let lp_pool1 = gm.create_basic_pool(&pool_1_coins, &admin).unwrap();
        //
        let pool_2_coins = vec![
            v1beta1::Coin {
                denom: DENOM_QUOTE.to_string(),
                amount: "10000000000000000000000".to_string(),
            },
            v1beta1::Coin {
                denom: DENOM_REWARD.to_string(),
                amount: "10000000000000000000000".to_string(),
            },
        ];
        // let lp_pool2 = gm.create_basic_pool(&pool_2_coins, &admin).unwrap();
        //
        let pool_3_coins = vec![
            v1beta1::Coin {
                denom: DENOM_BASE.to_string(),
                amount: "10000000000000000000000".to_string(),
            },
            v1beta1::Coin {
                denom: DENOM_REWARD.to_string(),
                amount: "10000000000000000000000".to_string(),
            },
        ];
        // let lp_pool3 = gm.create_basic_pool(&pool_3_coins, &admin).unwrap();

        // Get just created pool information by querying all the pools, and taking the first one
        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let pool: Pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();
        let lp_pool1: Pool = Pool::decode(pools.pools[1].value.as_slice()).unwrap();
        let lp_pool2: Pool = Pool::decode(pools.pools[2].value.as_slice()).unwrap();
        let lp_pool3: Pool = Pool::decode(pools.pools[3].value.as_slice()).unwrap();

        cl.create_position(
            MsgCreatePosition {
                pool_id: lp_pool1.id,
                sender: admin.address(),
                lower_tick,
                upper_tick,
                tokens_provided: pool_1_coins,
                token_min_amount0: token_min_amount0.to_string(),
                token_min_amount1: token_min_amount1.to_string(),
            },
            &admin,
        )
        .unwrap();
        cl.create_position(
            MsgCreatePosition {
                pool_id: lp_pool2.id,
                sender: admin.address(),
                lower_tick,
                upper_tick,
                tokens_provided: pool_2_coins,
                token_min_amount0: token_min_amount0.to_string(),
                token_min_amount1: token_min_amount1.to_string(),
            },
            &admin,
        )
        .unwrap();
        cl.create_position(
            MsgCreatePosition {
                pool_id: lp_pool3.id,
                sender: admin.address(),
                lower_tick,
                upper_tick,
                tokens_provided: pool_3_coins,
                token_min_amount0: token_min_amount0.to_string(),
                token_min_amount1: token_min_amount1.to_string(),
            },
            &admin,
        )
        .unwrap();

        // Sort tokens alphabetically by denom name or Osmosis will return an error
        tokens_provided.sort_by(|a, b| a.denom.cmp(&b.denom)); // can't use helpers.rs::sort_tokens() due to different Coin type

        // Create a first position in the pool with the admin user
        cl.create_position(
            MsgCreatePosition {
                pool_id: pool.id,
                sender: admin.address(),
                lower_tick,
                upper_tick,
                tokens_provided: tokens_provided.clone(),
                token_min_amount0: token_min_amount0.to_string(),
                token_min_amount1: token_min_amount1.to_string(),
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

        // Increment the app time for twaps to function, this is needed to do not fail on querying a twap for a timeframe higher than the chain existence
        app.increase_time(1000000);

        let contract_dex_router = wasm
            .instantiate(
                code_id_dex,
                &DexInstantiate {},
                Some(admin.address().as_str()),
                Some("dex-router"),
                sort_tokens(vec![]).as_ref(),
                &admin,
            )
            .unwrap();

        // Instantiate vault
        let contract_cl = wasm
            .instantiate(
                code_id_cl,
                &InstantiateMsg {
                    admin: admin.address(),
                    pool_id: pool.id,
                    config: VaultConfig {
                        performance_fee: Decimal::percent(20),
                        treasury: Addr::unchecked(admin.address()),
                        swap_max_slippage: Decimal::bps(5),
                        dex_router: Addr::unchecked(contract_dex_router.clone().data.address),
                    },
                    vault_token_subdenom: "utestvault".to_string(),
                    range_admin: admin.address(),
                    initial_lower_tick: lower_tick,
                    initial_upper_tick: upper_tick,
                    thesis: "Provide big swap efficiency".to_string(),
                    name: "Contract".to_string(),
                    auto_compound_admin: admin.address(),
                },
                Some(admin.address().as_str()),
                Some("cl-vault"),
                sort_tokens(vec![coin(1000, pool.token0), coin(1000, pool.token1)]).as_ref(),
                &admin,
            )
            .unwrap();

        wasm.execute(
            &contract_dex_router.data.address,
            &DexExecuteMsg::SetPath {
                offer_asset: AssetInfoUnchecked::Native(DENOM_BASE.to_string()),
                ask_asset: AssetInfoUnchecked::Native(DENOM_QUOTE.to_string()),
                path: SwapOperationsListUnchecked::new(vec![SwapOperationBase {
                    pool: cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool1.id)),
                    offer_asset_info: AssetInfoBase::Native(DENOM_BASE.to_string()),
                    ask_asset_info: AssetInfoBase::Native(DENOM_QUOTE.to_string()),
                }]),
                bidirectional: true,
            },
            sort_tokens(vec![]).as_ref(),
            &admin,
        )
        .unwrap();

        wasm.execute(
            &contract_dex_router.data.address,
            &DexExecuteMsg::SetPath {
                offer_asset: AssetInfoUnchecked::Native(DENOM_QUOTE.to_string()),
                ask_asset: AssetInfoUnchecked::Native(DENOM_REWARD.to_string()),
                path: SwapOperationsListUnchecked::new(vec![SwapOperationBase {
                    pool: cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool2.id)),
                    offer_asset_info: AssetInfoBase::Native(DENOM_QUOTE.to_string()),
                    ask_asset_info: AssetInfoBase::Native(DENOM_REWARD.to_string()),
                }]),
                bidirectional: true,
            },
            sort_tokens(vec![]).as_ref(),
            &admin,
        )
        .unwrap();

        wasm.execute(
            &contract_dex_router.data.address,
            &DexExecuteMsg::SetPath {
                offer_asset: AssetInfoUnchecked::Native(DENOM_REWARD.to_string()),
                ask_asset: AssetInfoUnchecked::Native(DENOM_BASE.to_string()),
                path: SwapOperationsListUnchecked::new(vec![SwapOperationBase {
                    pool: cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool3.id)),
                    offer_asset_info: AssetInfoBase::Native(DENOM_REWARD.to_string()),
                    ask_asset_info: AssetInfoBase::Native(DENOM_BASE.to_string()),
                }]),
                bidirectional: true,
            },
            sort_tokens(vec![]).as_ref(),
            &admin,
        )
        .unwrap();

        wasm.execute(
            &contract_dex_router.data.address,
            &DexExecuteMsg::SetPath {
                offer_asset: AssetInfoUnchecked::Native(DENOM_REWARD.to_string()),
                ask_asset: AssetInfoUnchecked::Native(DENOM_BASE.to_string()),
                path: SwapOperationsListUnchecked::new(vec![
                    SwapOperationBase {
                        pool: cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool2.id)),
                        ask_asset_info: AssetInfoBase::Native(DENOM_QUOTE.to_string()),
                        offer_asset_info: AssetInfoBase::Native(DENOM_REWARD.to_string()),
                    },
                    SwapOperationBase {
                        pool: cw_dex::Pool::Osmosis(OsmosisPool::unchecked(lp_pool1.id)),
                        offer_asset_info: AssetInfoBase::Native(DENOM_QUOTE.to_string()),
                        ask_asset_info: AssetInfoBase::Native(DENOM_BASE.to_string()),
                    },
                ]),
                bidirectional: true,
            },
            sort_tokens(vec![]).as_ref(),
            &admin,
        )
        .unwrap();

        (
            app,
            Addr::unchecked(contract_cl.data.address),
            Addr::unchecked(contract_dex_router.data.address),
            pool.id,
            lp_pool1.id,
            lp_pool2.id,
            lp_pool3.id,
            admin,
        )
    }

    pub fn init_test_contract(
        filename: &str,
        admin_balance: &[Coin],
        pool: MsgCreateConcentratedPool,
        lower_tick: i64,
        upper_tick: i64,
        mut tokens_provided: Vec<v1beta1::Coin>,
        token_min_amount0: Uint128,
        token_min_amount1: Uint128,
    ) -> (OsmosisTestApp, Addr, u64, SigningAccount) {
        // Create new osmosis appchain instance
        let app = OsmosisTestApp::new();
        let pm = PoolManager::new(&app);
        let cl = ConcentratedLiquidity::new(&app);
        let wasm = Wasm::new(&app);

        // Create new account with initial funds
        let admin = app.init_account(admin_balance).unwrap();

        // Load compiled wasm bytecode
        let wasm_byte_code = std::fs::read(filename).unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, &admin)
            .unwrap()
            .data
            .code_id;

        // Setup a dummy CL pool to work with
        let gov = GovWithAppAccess::new(&app);
        gov.propose_and_execute(
            CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
            CreateConcentratedLiquidityPoolsProposal {
                title: "CL Pool".to_string(),
                description: "So that we can trade it".to_string(),
                pool_records: vec![PoolRecord {
                    denom0: pool.denom0,
                    denom1: pool.denom1,
                    tick_spacing: pool.tick_spacing,
                    spread_factor: pool.spread_factor,
                }],
            },
            admin.address(),
            &admin,
        )
        .unwrap();

        // Get just created pool information by querying all the pools, and taking the first one
        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let pool: Pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        // Sort tokens alphabetically by denom name or Osmosis will return an error
        tokens_provided.sort_by(|a, b| a.denom.cmp(&b.denom)); // can't use helpers.rs::sort_tokens() due to different Coin type

        // Create a first position in the pool with the admin user
        cl.create_position(
            MsgCreatePosition {
                pool_id: pool.id,
                sender: admin.address(),
                lower_tick,
                upper_tick,
                tokens_provided: tokens_provided.clone(),
                token_min_amount0: token_min_amount0.to_string(),
                token_min_amount1: token_min_amount1.to_string(),
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

        // Increment the app time for twaps to function, this is needed to do not fail on querying a twap for a timeframe higher than the chain existence
        app.increase_time(1000000);

        // Instantiate vault
        let contract = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    admin: admin.address(),
                    pool_id: pool.id,
                    config: VaultConfig {
                        performance_fee: Decimal::percent(20),
                        treasury: Addr::unchecked(admin.address()),
                        swap_max_slippage: Decimal::bps(5),
                        dex_router: Addr::unchecked(admin.address()), // Just to fulfill bech32 requirement
                    },
                    vault_token_subdenom: "utestvault".to_string(),
                    range_admin: admin.address(),
                    initial_lower_tick: lower_tick,
                    initial_upper_tick: upper_tick,
                    thesis: "Provide big swap efficiency".to_string(),
                    name: "Contract".to_string(),
                    auto_compound_admin: admin.address(),
                },
                Some(admin.address().as_str()),
                Some("cl-vault"),
                sort_tokens(vec![coin(1000, pool.token0), coin(1000, pool.token1)]).as_ref(),
                &admin,
            )
            .unwrap();

        (app, Addr::unchecked(contract.data.address), pool.id, admin)
    }

    #[test]
    #[ignore]
    fn default_init_works() {
        let (app, contract_address, cl_pool_id, admin) = default_init();
        let wasm = Wasm::new(&app);
        let cl = ConcentratedLiquidity::new(&app);
        let tf = TokenFactory::new(&app);
        let pm = PoolManager::new(&app);

        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        let resp = wasm
            .query::<QueryMsg, PoolResponse>(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                    ClQueryMsg::Pool {},
                )),
            )
            .unwrap();

        assert_eq!(resp.pool_config.pool_id, pool.id);
        assert_eq!(resp.pool_config.token0, pool.token0);
        assert_eq!(resp.pool_config.token1, pool.token1);

        let resp = wasm
            .query::<QueryMsg, VaultInfoResponse>(contract_address.as_str(), &QueryMsg::Info {})
            .unwrap();

        assert_eq!(resp.tokens, vec![pool.token0, pool.token1]);
        assert_eq!(
            resp.vault_token,
            tf.query_denoms_from_creator(&QueryDenomsFromCreatorRequest {
                creator: contract_address.to_string()
            })
            .unwrap()
            .denoms[0]
        );

        // Create Alice account
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, DENOM_BASE),
                Coin::new(1_000_000_000_000, DENOM_QUOTE),
            ])
            .unwrap();

        // Swap some funds as Alice to move the pool's curent tick
        pm.swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: alice.address(),
                routes: vec![SwapAmountInRoute {
                    pool_id: cl_pool_id,
                    token_out_denom: DENOM_BASE.to_string(),
                }],
                token_in: Some(v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: "1000".to_string(),
                }),
                token_out_min_amount: "1".to_string(),
            },
            &alice,
        )
        .unwrap();

        // Increment the app time for twaps to function
        app.increase_time(1000000);

        // Update range of vault as Admin
        wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                ModifyRangeMsg {
                    lower_price: Decimal::from_str("0.993").unwrap(),
                    upper_price: Decimal::from_str("1.002").unwrap(),
                    max_slippage: Decimal::bps(9500),
                    ratio_of_swappable_funds_to_use: Decimal::one(),
                    twap_window_seconds: 45,
                },
            )),
            &[],
            &admin,
        )
        .unwrap();
    }
}
