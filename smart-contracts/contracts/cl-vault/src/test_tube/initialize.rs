#[cfg(test)]
pub mod initialize {
    use cosmwasm_std::{coin, Addr, Coin, Decimal, Uint128};
    use cw_vault_multi_standard::VaultInfoResponse;
    use dex_router_osmosis::msg::{ExecuteMsg as DexExecuteMsg, InstantiateMsg as DexInstantiate};
    use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
    use osmosis_std::types::cosmos::base::v1beta1;
    use osmosis_std::types::osmosis::concentratedliquidity;
    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
        CreateConcentratedLiquidityPoolsProposal, Pool, PoolRecord, PoolsRequest,
    };
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SpotPriceRequest, SwapAmountInRoute,
    };
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::QueryDenomsFromCreatorRequest;
    use osmosis_test_tube::Bank;
    use osmosis_test_tube::Runner;
    use osmosis_test_tube::{
        cosmrs::proto::traits::Message,
        osmosis_std::types::osmosis::concentratedliquidity::{
            poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool, v1beta1::MsgCreatePosition,
        },
        Account, ConcentratedLiquidity, Gamm, GovWithAppAccess, Module, OsmosisTestApp,
        PoolManager, SigningAccount, TokenFactory, Wasm,
    };
    use std::str::FromStr;

    use crate::helpers::generic::sort_tokens;
    use crate::math::tick::tick_to_price;
    use crate::msg::CreatePosition;
    use crate::msg::ExtensionExecuteMsg;
    use crate::msg::MovePosition;
    use crate::msg::{
        ClQueryMsg, ExecuteMsg, ExtensionQueryMsg, InstantiateMsg, ModifyRange, QueryMsg,
    };
    use crate::query::MainPositionResponse;
    use crate::query::PoolResponse;
    use crate::state::VaultConfig;
    use crate::test_tube::helpers::calculate_deposit_ratio;

    pub const ADMIN_BALANCE_AMOUNT: u128 = 100_000_000_000_000_000_000_000_000_000u128;
    pub const PERFORMANCE_FEE_DEFAULT: u64 = 20;

    // const _TOKENS_PROVIDED_AMOUNT_LOW: &str = "1000000000000000";
    // const _SPREAD_FACTOR_LOW: &str = "0.01";
    // pub const _MAX_SLIPPAGE_LOW: u64 = 9900; // this should be inline with the pool spread_factor

    const TOKENS_PROVIDED_AMOUNT_HIGH: &str = "100000000000000000000";
    pub const SPREAD_FACTOR_HIGH: &str = "0.1";
    pub const MAX_SLIPPAGE_HIGH: u64 = 9000; // this should be inline with the pool spread_factor

    pub const DENOM_BASE: &str = "uatom";
    pub const DENOM_QUOTE: &str = "ubtc";
    pub const DENOM_REWARD: &str = "ustrd";

    pub const ACCOUNTS_NUM: u64 = 10;
    pub const ACCOUNTS_INIT_BALANCE: u128 = 1_000_000_000_000_000;
    pub const DEPOSIT_AMOUNT: u128 = 5_000_000_000;

    pub const INITIAL_POSITION_BURN: u128 = 1_000_000;

    // Fixtures: Default variants

    pub fn fixture_default(
        performance_fee: u64,
    ) -> (OsmosisTestApp, Addr, u64, SigningAccount, f64, String) {
        init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(ADMIN_BALANCE_AMOUNT, "uosmo"),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: DENOM_BASE.to_string(),
                denom1: DENOM_QUOTE.to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str(SPREAD_FACTOR_HIGH)
                    .unwrap()
                    .atomics()
                    .to_string(),
            },
            -5000000, // 0.5 spot price
            500000,   // 1.5 spot price
            vec![
                v1beta1::Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT_HIGH.to_string(),
                },
                v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT_HIGH.to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
            performance_fee,
        )
    }

    // pub fn _fixture_default_less_slippage(
    // ) -> (OsmosisTestApp, Addr, u64, SigningAccount, f64, String) {
    //     init_test_contract(
    //         "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
    //         &[
    //             Coin::new(ADMIN_BALANCE_AMOUNT, "uosmo"),
    //             Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
    //             Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
    //         ],
    //         MsgCreateConcentratedPool {
    //             sender: "overwritten".to_string(),
    //             denom0: DENOM_BASE.to_string(),
    //             denom1: DENOM_QUOTE.to_string(),
    //             tick_spacing: 100,
    //             spread_factor: Decimal::from_str(SPREAD_FACTOR_LOW)
    //                 .unwrap()
    //                 .atomics()
    //                 .to_string(),
    //         },
    //         -5000000, // 0.5 spot price
    //         500000,   // 1.5 spot price
    //         vec![
    //             v1beta1::Coin {
    //                 denom: DENOM_BASE.to_string(),
    //                 amount: TOKENS_PROVIDED_AMOUNT_LOW.to_string(),
    //             },
    //             v1beta1::Coin {
    //                 denom: DENOM_QUOTE.to_string(),
    //                 amount: TOKENS_PROVIDED_AMOUNT_LOW.to_string(),
    //             },
    //         ],
    //         Uint128::zero(),
    //         Uint128::zero(),
    //     )
    // }

    pub fn fixture_dex_router(
        performance_fee: u64,
    ) -> (
        OsmosisTestApp,
        Addr,
        Addr,
        u64,
        Vec<u64>,
        SigningAccount,
        f64,
        String,
    ) {
        init_test_contract_with_dex_router_and_swap_pools(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            "../dex-router-osmosis/test-tube-build/wasm32-unknown-unknown/release/dex_router_osmosis.wasm",
            &[
                Coin::new(ADMIN_BALANCE_AMOUNT, "uosmo"),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_REWARD),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: DENOM_BASE.to_string(),
                denom1: DENOM_QUOTE.to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str(SPREAD_FACTOR_HIGH)
                    .unwrap()
                    .atomics()
                    .to_string(),
            },
            -5000000, // 0.5 spot price
            500000,   // 1.5 spot price
            vec![
                v1beta1::Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT_HIGH.to_string(),
                },
                v1beta1::Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: TOKENS_PROVIDED_AMOUNT_HIGH.to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
            performance_fee,
        )
    }

    // INIT PRIVATE METHODS

    // TODO: This is pub because of the proptest still not having a default_init implementation
    pub fn init_test_contract(
        filename: &str,
        admin_balance: &[Coin],
        pool: MsgCreateConcentratedPool,
        lower_tick: i64,
        upper_tick: i64,
        mut tokens_provided: Vec<v1beta1::Coin>,
        token_min_amount0: Uint128,
        token_min_amount1: Uint128,
        performance_fee: u64,
    ) -> (OsmosisTestApp, Addr, u64, SigningAccount, f64, String) {
        // Create new osmosis appchain instance
        let app = OsmosisTestApp::new();
        let pm = PoolManager::new(&app);
        let cl = ConcentratedLiquidity::new(&app);
        let wasm = Wasm::new(&app);
        let admin = app.init_account(admin_balance).unwrap();

        // Load compiled wasm bytecode
        let wasm_byte_code = std::fs::read(filename).unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, &admin)
            .unwrap()
            .data
            .code_id;

        let vault_pool = create_cl_pool(
            &app,
            pool.denom0,
            pool.denom1,
            pool.tick_spacing,
            pool.spread_factor,
            &admin,
        );

        // Sort tokens alphabetically by denom name or Osmosis will return an error
        tokens_provided.sort_by(|a, b| a.denom.cmp(&b.denom)); // can't use helpers.rs::sort_tokens() due to different Coin type

        // Create a first position in the pool with the admin user
        let create_position = cl
            .create_position(
                MsgCreatePosition {
                    pool_id: vault_pool.id,
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
                pool_id: vault_pool.id,
            })
            .unwrap();
        assert_eq!(spot_price.spot_price, "1.000000000000000000");

        let (deposit_ratio, deposit_ratio_approx) = calculate_deposit_ratio(
            spot_price.spot_price,
            tokens_provided,
            create_position.data.amount0,
            create_position.data.amount1,
            DENOM_BASE.to_string(),
            DENOM_QUOTE.to_string(),
        );

        // Increment the app time for twaps to function, this is needed to do not fail on querying a twap for a timeframe higher than the chain existence
        app.increase_time(1000000);

        // Instantiate vault
        let contract = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    admin: admin.address(),
                    pool_id: vault_pool.id,
                    config: VaultConfig {
                        performance_fee: Decimal::percent(performance_fee),
                        treasury: Addr::unchecked(admin.address()),
                        swap_max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                        dex_router: Addr::unchecked(admin.address()), // Just to fulfill bech32 requirement
                    },
                    vault_token_subdenom: "utestvault".to_string(),
                    range_admin: admin.address(),
                    initial_lower_tick: lower_tick,
                    initial_upper_tick: upper_tick,
                    thesis: "Provide big swap efficiency".to_string(),
                    name: "Contract".to_string(),
                },
                Some(admin.address().as_str()),
                Some("cl-vault"),
                sort_tokens(vec![
                    coin(INITIAL_POSITION_BURN, vault_pool.token0.clone()),
                    coin(INITIAL_POSITION_BURN, vault_pool.token1.clone()),
                ])
                .as_ref(),
                &admin,
            )
            .unwrap();

        let lower_price = tick_to_price(lower_tick / 2).unwrap();
        let upper_price = tick_to_price(upper_tick / 2).unwrap();

        let bank = Bank::new(&app);
        bank.send(
            MsgSend {
                from_address: admin.address(),
                to_address: contract.data.address.clone(),
                amount: osmosis_std::cosmwasm_to_proto_coins(sort_tokens(vec![
                    coin(INITIAL_POSITION_BURN, vault_pool.token0.clone()),
                    coin(INITIAL_POSITION_BURN, vault_pool.token1.clone()),
                ])),
            },
            &admin,
        )
        .unwrap();

        let _res = wasm
            .execute(
                contract.data.address.as_str(),
                &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ModifyRange(
                    ModifyRange::CreatePosition(CreatePosition {
                        lower_price: lower_price.try_into().unwrap(),
                        upper_price: upper_price.try_into().unwrap(),
                        claim_after: None,
                        max_token0: None,
                        max_token1: None,
                    }),
                )),
                &[],
                &admin,
            )
            .unwrap();

        bank.send(
            MsgSend {
                from_address: admin.address(),
                to_address: contract.data.address.clone(),
                amount: osmosis_std::cosmwasm_to_proto_coins(sort_tokens(vec![
                    coin(INITIAL_POSITION_BURN, vault_pool.token0.clone()),
                    coin(INITIAL_POSITION_BURN, vault_pool.token1.clone()),
                ])),
            },
            &admin,
        )
        .unwrap();

        let lower_price = tick_to_price(lower_tick / 3).unwrap();
        let upper_price = tick_to_price(upper_tick / 3).unwrap();

        let _res = wasm
            .execute(
                contract.data.address.as_str(),
                &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ModifyRange(
                    ModifyRange::CreatePosition(CreatePosition {
                        lower_price: lower_price.try_into().unwrap(),
                        upper_price: upper_price.try_into().unwrap(),
                        claim_after: None,
                        max_token0: None,
                        max_token1: None,
                    }),
                )),
                &[],
                &admin,
            )
            .unwrap();

        (
            app,
            Addr::unchecked(contract.data.address),
            vault_pool.id,
            admin,
            deposit_ratio,
            deposit_ratio_approx,
        )
    }

    /// Create a CL pool without any liquidity
    pub fn create_cl_pool<'a>(
        app: &'a OsmosisTestApp,
        denom0: String,
        denom1: String,
        tick_spacing: u64,
        spread_factor: String,
        admin: &SigningAccount,
    ) -> concentratedliquidity::v1beta1::Pool {
        // Setup a dummy CL pool to work with
        let gov = GovWithAppAccess::new(app);
        gov.propose_and_execute(
            CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
            CreateConcentratedLiquidityPoolsProposal {
                title: "CL Pool".to_string(),
                description: "So that we can trade it".to_string(),
                pool_records: vec![PoolRecord {
                    denom0: denom0,
                    denom1: denom1,
                    tick_spacing: tick_spacing,
                    spread_factor: spread_factor,
                }],
            },
            admin.address(),
            admin,
        )
        .unwrap();

        let cl = ConcentratedLiquidity::new(app);
        // Get just created pool information by querying all the pools, and taking the first one
        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let vault_pool: Pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();
        vault_pool
    }

    fn init_test_contract_with_dex_router_and_swap_pools(
        filename_cl: &str,
        filename_dex: &str,
        admin_balance: &[Coin],
        pool: MsgCreateConcentratedPool,
        lower_tick: i64,
        upper_tick: i64,
        mut tokens_provided: Vec<v1beta1::Coin>,
        token_min_amount0: Uint128,
        token_min_amount1: Uint128,
        performance_fee: u64,
    ) -> (
        OsmosisTestApp,
        Addr,
        Addr,
        u64,
        Vec<u64>,
        SigningAccount,
        f64,
        String,
    ) {
        // Create new osmosis appchain instance
        let app = OsmosisTestApp::new();
        let pm = PoolManager::new(&app);
        let gm = Gamm::new(&app);
        let cl = ConcentratedLiquidity::new(&app);
        let wasm = Wasm::new(&app);
        let admin = app.init_account(admin_balance).unwrap();

        // Swap pools where the first one is CL for the vault_pool
        let pools_coins = vec![
            vec![
                Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: Uint128::from_str(TOKENS_PROVIDED_AMOUNT_HIGH).unwrap(),
                },
                Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: Uint128::from_str(TOKENS_PROVIDED_AMOUNT_HIGH).unwrap(),
                },
            ],
            vec![
                Coin {
                    denom: DENOM_QUOTE.to_string(),
                    amount: Uint128::from_str(TOKENS_PROVIDED_AMOUNT_HIGH).unwrap(),
                },
                Coin {
                    denom: DENOM_REWARD.to_string(),
                    amount: Uint128::from_str(TOKENS_PROVIDED_AMOUNT_HIGH).unwrap(),
                },
            ],
            vec![
                Coin {
                    denom: DENOM_BASE.to_string(),
                    amount: Uint128::from_str(TOKENS_PROVIDED_AMOUNT_HIGH).unwrap(),
                },
                Coin {
                    denom: DENOM_REWARD.to_string(),
                    amount: Uint128::from_str(TOKENS_PROVIDED_AMOUNT_HIGH).unwrap(),
                },
            ],
        ];

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

        // Setup a dummy CL pool to work with the CL Vault contract
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
        let vault_pool: Pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap(); // we only have one here as we then create legacy gAMM Balancer pools

        // Create Balancer pools with previous vec of vec of coins
        // TODO: We could be using a mixed set of CL and gAMM pools here
        let mut lp_pools = vec![];
        for pool_coins in &pools_coins {
            let lp_pool = gm.create_basic_pool(&pool_coins, &admin).unwrap();
            lp_pools.push(lp_pool.data.pool_id);
        }

        // Here we have 4 pools in pools_ids where the index 0 is the cl_pool id

        // Sort tokens alphabetically by denom name or Osmosis will return an error
        tokens_provided.sort_by(|a, b| a.denom.cmp(&b.denom)); // can't use helpers.rs::sort_tokens() due to different Coin type

        // Create a first position in the pool with the admin user
        let create_position = cl
            .create_position(
                MsgCreatePosition {
                    pool_id: vault_pool.id,
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
                pool_id: vault_pool.id,
            })
            .unwrap();
        assert_eq!(spot_price.spot_price, "1.000000000000000000");

        let (deposit_ratio, deposit_ratio_approx) = calculate_deposit_ratio(
            spot_price.spot_price,
            tokens_provided,
            create_position.data.amount0,
            create_position.data.amount1,
            DENOM_BASE.to_string(),
            DENOM_QUOTE.to_string(),
        );

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

        // Here we pass only the 3x swap LP pools, not the Vault CL pool id 1
        set_dex_router_paths(
            &app,
            contract_dex_router.data.address.to_string(),
            &lp_pools,
            &pools_coins,
            &admin,
        );

        // Instantiate vault
        let contract_cl = wasm
            .instantiate(
                code_id_cl,
                &InstantiateMsg {
                    admin: admin.address(),
                    pool_id: vault_pool.id,
                    config: VaultConfig {
                        performance_fee: Decimal::percent(performance_fee),
                        treasury: Addr::unchecked(admin.address()),
                        swap_max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                        dex_router: Addr::unchecked(contract_dex_router.clone().data.address),
                    },
                    vault_token_subdenom: "utestvault".to_string(),
                    range_admin: admin.address(),
                    initial_lower_tick: lower_tick,
                    initial_upper_tick: upper_tick,
                    thesis: "Provide big swap efficiency".to_string(),
                    name: "Contract".to_string(),
                },
                Some(admin.address().as_str()),
                Some("cl-vault"),
                sort_tokens(vec![
                    coin(INITIAL_POSITION_BURN, vault_pool.token0),
                    coin(INITIAL_POSITION_BURN, vault_pool.token1),
                ])
                .as_ref(),
                &admin,
            )
            .unwrap();

        (
            app,
            Addr::unchecked(contract_cl.data.address),
            Addr::unchecked(contract_dex_router.data.address),
            vault_pool.id,
            lp_pools,
            admin,
            deposit_ratio,
            deposit_ratio_approx,
        )
    }

    fn set_dex_router_paths(
        app: &OsmosisTestApp,
        dex_router: String,
        pools: &Vec<u64>,
        pools_coins: &Vec<Vec<Coin>>,
        admin: &SigningAccount,
    ) {
        let wasm = Wasm::new(app);
        assert_eq!(pools.len(), pools_coins.len());

        // Set Dex Router contract paths
        for (index, pool_id) in pools.iter().enumerate() {
            wasm.execute(
                &dex_router,
                &DexExecuteMsg::SetPath {
                    offer_denom: pools_coins[index][0].denom.to_string(),
                    ask_denom: pools_coins[index][1].denom.to_string(),
                    path: vec![*pool_id],
                    bidirectional: true,
                },
                vec![].as_ref(),
                &admin,
            )
            .unwrap();
        }
    }

    // TESTS

    #[test]
    #[ignore]
    fn fixture_default_works() {
        let (app, contract_address, cl_pool_id, admin, _deposit_ratio, _deposit_ratio_approx) =
            fixture_default(PERFORMANCE_FEE_DEFAULT);
        let wasm = Wasm::new(&app);
        let cl = ConcentratedLiquidity::new(&app);
        let tf = TokenFactory::new(&app);
        let pm = PoolManager::new(&app);

        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let vault_pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        let resp = wasm
            .query::<QueryMsg, PoolResponse>(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                    ClQueryMsg::Pool {},
                )),
            )
            .unwrap();

        assert_eq!(resp.pool_config.pool_id, vault_pool.id);
        assert_eq!(resp.pool_config.token0, vault_pool.token0);
        assert_eq!(resp.pool_config.token1, vault_pool.token1);

        let resp = wasm
            .query::<QueryMsg, VaultInfoResponse>(contract_address.as_str(), &QueryMsg::Info {})
            .unwrap();

        assert_eq!(resp.tokens, vec![vault_pool.token0, vault_pool.token1]);
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
                Coin::new(1_000_000_000_000, "uosmo"),
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

        let main_position: MainPositionResponse = wasm
            .query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                    crate::msg::ClQueryMsg::MainPosition {},
                )),
            )
            .unwrap();

        // Update range of vault as Admin
        wasm.execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                ModifyRange::MovePosition(MovePosition {
                    position_id: main_position.position_id,
                    lower_price: Decimal::from_str("0.993").unwrap(),
                    upper_price: Decimal::from_str("1.002").unwrap(),
                    max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                    ratio_of_swappable_funds_to_use: Decimal::one(),
                    twap_window_seconds: 45,
                    forced_swap_route: None,
                    claim_after: None,
                }),
            )),
            &[],
            &admin,
        )
        .unwrap();
    }
}
