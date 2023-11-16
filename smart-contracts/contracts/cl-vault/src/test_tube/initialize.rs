#[cfg(test)]
pub mod initialize {
    use std::str::FromStr;

    use cosmwasm_std::{coin, Addr, Coin, Decimal, Uint128};
    use cw_vault_multi_standard::VaultInfoResponse;
    use osmosis_std::types::cosmos::base::v1beta1;
    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
        CreateConcentratedLiquidityPoolsProposal, Pool, PoolRecord, PoolsRequest,
    };
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SwapAmountInRoute,
    };
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::QueryDenomsFromCreatorRequest;
    use osmosis_test_tube::{
        cosmrs::proto::traits::Message,
        osmosis_std::types::osmosis::concentratedliquidity::{
            poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool, v1beta1::MsgCreatePosition,
        },
        Account, ConcentratedLiquidity, GovWithAppAccess, Module, OsmosisTestApp, PoolManager,
        SigningAccount, TokenFactory, Wasm,
    };

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
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            -200000,
            200000,
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

        // Create new account with initial funds
        let admin = app.init_account(admin_balance).unwrap();

        // `Wasm` is the module we use to interact with cosmwasm releated logic on the appchain
        // it implements `Module` trait which you will see more later.
        let wasm = Wasm::new(&app);

        // Load compiled wasm bytecode
        let wasm_byte_code = std::fs::read(filename).unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, &admin)
            .unwrap()
            .data
            .code_id;

        // Setup a dummy CL pool to work with
        let cl = ConcentratedLiquidity::new(&app);
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
            false,
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
                tokens_provided,
                token_min_amount0: token_min_amount0.to_string(),
                token_min_amount1: token_min_amount1.to_string(),
            },
            &admin,
        )
        .unwrap();

        // Instantiate
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
                sort_tokens(vec![coin(1000, pool.token0), coin(1000, pool.token1)]).as_ref(),
                &admin,
            )
            .unwrap();

        (app, Addr::unchecked(contract.data.address), pool.id, admin)
    }

    #[test]
    #[ignore]
    fn contract_default_init_works() {
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
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        // Swap some funds as Alice to move the pool's curent tick
        pm.swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: alice.address(),
                routes: vec![SwapAmountInRoute {
                    pool_id: cl_pool_id,
                    token_out_denom: "uatom".to_string(),
                }],
                token_in: Some(v1beta1::Coin {
                    denom: "uosmo".to_string(),
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
                    max_slippage: Decimal::permille(5),
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
