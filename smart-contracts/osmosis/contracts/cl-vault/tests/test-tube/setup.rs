#![allow(dead_code)]

use cl_vault::{helpers::generic::sort_tokens, msg::InstantiateMsg, state::VaultConfig};
use cosmwasm_std::{coin, Addr, Attribute, Coin, Decimal, Uint128};
use dex_router_osmosis::msg::{ExecuteMsg as DexExecuteMsg, InstantiateMsg as DexInstantiate};
use osmosis_std::types::{
    cosmos::{bank::v1beta1::QueryBalanceRequest, base::v1beta1},
    cosmwasm::wasm::v1::MsgExecuteContractResponse,
    osmosis::concentratedliquidity::v1beta1::{
        CreateConcentratedLiquidityPoolsProposal, Pool, PoolRecord, PoolsRequest,
    },
    osmosis::poolmanager::v1beta1::SpotPriceRequest,
};
use osmosis_test_tube::{
    cosmrs::proto::traits::Message,
    osmosis_std::types::osmosis::concentratedliquidity::{
        poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool, v1beta1::MsgCreatePosition,
    },
    Account, Bank, ConcentratedLiquidity, ExecuteResponse, Gamm, GovWithAppAccess, Module,
    OsmosisTestApp, PoolManager, SigningAccount, Wasm,
};
use std::str::FromStr;

pub const ADMIN_BALANCE_AMOUNT: u128 = 100_000_000_000_000_000_000_000_000_000u128;
pub const PERFORMANCE_FEE_DEFAULT: u64 = 20;

const TOKENS_PROVIDED_AMOUNT_HIGH: &str = "100000000000000000000";
pub const SPREAD_FACTOR_HIGH: &str = "0.1";
pub const MAX_SLIPPAGE_HIGH: u64 = 9000;

pub const DENOM_BASE: &str = "uatom";
pub const DENOM_QUOTE: &str = "ubtc";
pub const DENOM_REWARD: &str = "ustrd";

pub const ACCOUNTS_NUM: u64 = 10;
pub const ACCOUNTS_INIT_BALANCE: u128 = 1_000_000_000_000_000;
pub const DEPOSIT_AMOUNT: u128 = 5_000_000_000;

pub const INITIAL_POSITION_BURN: u128 = 1_000_000;

pub fn fixture_default(performance_fee: u64) -> (OsmosisTestApp, Addr, u64, SigningAccount, f64) {
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

// TODO: This is pub because of the proptest still not having a default_init implementation
#[allow(clippy::too_many_arguments)]
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
) -> (OsmosisTestApp, Addr, u64, SigningAccount, f64) {
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
    let vault_pool: Pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

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

    let deposit_ratio_base = calculate_deposit_ratio(
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
                    dex_router: Addr::unchecked(admin.address()),
                    swap_admin: Addr::unchecked(admin.address()),
                    twap_window_seconds: 24u64,
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
        Addr::unchecked(contract.data.address),
        vault_pool.id,
        admin,
        deposit_ratio_base,
    )
}

#[allow(clippy::too_many_arguments)]
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
                denom: DENOM_BASE.to_string(),
                amount: Uint128::from_str(TOKENS_PROVIDED_AMOUNT_HIGH).unwrap(),
            },
            Coin {
                denom: DENOM_REWARD.to_string(),
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
        let lp_pool = gm.create_basic_pool(pool_coins, &admin).unwrap();
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

    let deposit_ratio_base = calculate_deposit_ratio(
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
                    swap_admin: Addr::unchecked(admin.address()),
                    twap_window_seconds: 24u64,
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
        deposit_ratio_base,
    )
}

fn set_dex_router_paths(
    app: &OsmosisTestApp,
    dex_router: String,
    pools: &[u64],
    pools_coins: &[Vec<Coin>],
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
            admin,
        )
        .unwrap();
    }
}

pub fn get_event_attributes_by_ty_and_key(
    response: &ExecuteResponse<MsgExecuteContractResponse>,
    ty: &str,
    keys: Vec<&str>,
) -> Vec<Attribute> {
    response
        .events
        .iter()
        .filter(|event| event.ty == ty)
        .flat_map(|event| event.attributes.clone())
        .filter(|attribute| keys.contains(&attribute.key.as_str()))
        .collect()
}

pub fn get_balance_amount(app: &OsmosisTestApp, address: String, denom: String) -> u128 {
    let bm = Bank::new(app);

    bm.query_balance(&QueryBalanceRequest { address, denom })
        .unwrap()
        .balance
        .unwrap()
        .amount
        .parse::<u128>()
        .unwrap()
}

pub fn get_amount_from_denom(value: &str) -> u128 {
    // Find the position where the non-numeric part starts
    let pos = value.find(|c: char| !c.is_numeric()).unwrap_or(value.len());
    // Extract the numeric part from the string
    let numeric_part = &value[0..pos];
    // Try to parse the numeric string to u128
    numeric_part.parse::<u128>().unwrap()
}

pub fn calculate_deposit_ratio(
    spot_price: String,
    tokens_provided: Vec<v1beta1::Coin>,
    amount0_deposit: String,
    amount1_deposit: String,
    denom_base: String,
    denom_quote: String,
) -> f64 {
    // Parse the input amounts
    let amount0_deposit: u128 = amount0_deposit.parse().unwrap();
    let amount1_deposit: u128 = amount1_deposit.parse().unwrap();

    // Find the attempted amounts from the tokens_provided
    let mut provided_amount0 = 0u128;
    let mut provided_amount1 = 0u128;

    for coin in &tokens_provided {
        if coin.denom == denom_base {
            provided_amount0 = coin.amount.parse().unwrap();
        } else if coin.denom == denom_quote {
            provided_amount1 = coin.amount.parse().unwrap();
        }
    }

    // Calculate refunds
    let token0_refund = provided_amount0.saturating_sub(amount0_deposit);
    let token1_refund = provided_amount1.saturating_sub(amount1_deposit);

    // Convert token1 refund into token0 equivalent using spot price
    let spot_price_value = spot_price.parse::<f64>().unwrap();
    let token1_refund_in_token0 = (token1_refund as f64) / spot_price_value;

    // Calculate total refunds in terms of token0
    let total_refunds_in_token0 = token0_refund as f64 + token1_refund_in_token0;

    // Calculate total attempted deposits in terms of token0
    let total_attempted_deposit_in_token0 =
        provided_amount0 as f64 + (provided_amount1 as f64 / spot_price_value);

    // Calculate the ratio of total refunds in terms of token0 to total attempted deposits in terms of token0
    if total_attempted_deposit_in_token0 == 0.0 {
        0.5 // Balanced deposit
    } else {
        2.0 * total_refunds_in_token0 / total_attempted_deposit_in_token0
    }
}

pub fn calculate_expected_refunds(
    initial_amount0: u128,
    initial_amount1: u128,
    deposit_ratio_base: f64,
) -> (u128, u128) {
    if deposit_ratio_base < 0.5 {
        // More token1 to be deposited, so token0 has a higher refund
        let adjusted_amount0 = ((1.0 - deposit_ratio_base) * initial_amount0 as f64) as u128;
        let expected_refund0 = initial_amount0 - adjusted_amount0;
        (expected_refund0, 0)
    } else if deposit_ratio_base > 0.5 {
        // More token0 to be deposited, so token1 has a higher refund
        let adjusted_amount1 =
            ((1.0 - (deposit_ratio_base - 0.5) * 2.0) * initial_amount1 as f64) as u128;
        let expected_refund1 = initial_amount1 - adjusted_amount1;
        (0, expected_refund1)
    } else {
        // Balanced deposit, no refunds expected
        (0, 0)
    }
}
