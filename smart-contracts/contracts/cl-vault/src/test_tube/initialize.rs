#[cfg(test)]
pub mod initialize {
    use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
    use osmosis_std::types::cosmos::base::v1beta1;
    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
        CreateConcentratedLiquidityPoolsProposal, Pool, PoolRecord, PoolsRequest,
    };
    use osmosis_test_tube::SigningAccount;
    use osmosis_test_tube::{
        cosmrs::proto::traits::Message,
        osmosis_std::types::osmosis::concentratedliquidity::{
            poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool, v1beta1::MsgCreatePosition,
        },
        Account, ConcentratedLiquidity, GovWithAppAccess, Module, OsmosisTestApp, Wasm,
    };

    use crate::msg::InstantiateMsg;
    use crate::state::VaultConfig;

    pub fn default_init() -> (OsmosisTestApp, Addr, u64, SigningAccount) {
        init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: "uatom".to_string(),
                denom1: "uosmo".to_string(),
                tick_spacing: 1,
                spread_factor: "100000000000000".to_string(),
            },
            0,
            100,
            vec![
                v1beta1::Coin {
                    denom: "uatom".to_string(),
                    amount: "10000000000".to_string(),
                },
                v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "10000000000".to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        )
    }

    // admin should be on accs[0] if this is called to init
    /// this returns the testube app, contract_address, pool_id
    /// sender is overwritten by the admin
    // bad function but it's finnee
    pub fn init_test_contract(
        filename: &str,
        admin_balance: &[Coin],
        _pool: MsgCreateConcentratedPool,
        lower_tick: i64,
        upper_tick: i64,
        tokens_provided: Vec<v1beta1::Coin>,
        token_min_amount0: Uint128,
        token_min_amount1: Uint128,
    ) -> (OsmosisTestApp, Addr, u64, SigningAccount) {
        // create new osmosis appchain instance.
        let app = OsmosisTestApp::new();

        // create new account with initial funds
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

        // setup a CL pool
        let cl = ConcentratedLiquidity::new(&app);
        let gov = GovWithAppAccess::new(&app);
        gov.propose_and_execute(
            CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
            CreateConcentratedLiquidityPoolsProposal {
                title: "Create concentrated uosmo:usdc pool".to_string(),
                description: "Create concentrated uosmo:usdc pool, so that we can trade it"
                    .to_string(),
                pool_records: vec![PoolRecord {
                    denom0: "uatom".to_string(),
                    denom1: "uosmo".to_string(),
                    tick_spacing: 1,
                    exponent_at_price_one: "-6".to_string(),
                    spread_factor: "0".to_string(),
                }],
            },
            admin.address(),
            false,
            &admin,
        )
        .unwrap();

        let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
        let pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        // create a basic position on the pool
        let initial_position = MsgCreatePosition {
            pool_id: pool.id,
            sender: admin.address(),
            lower_tick,
            upper_tick,
            tokens_provided,
            token_min_amount0: token_min_amount0.to_string(),
            token_min_amount1: token_min_amount1.to_string(),
        };
        let _position = cl.create_position(initial_position, &admin).unwrap();

        let instantiate_msg = InstantiateMsg {
            admin: admin.address(),
            pool_id: pool.id,
            lockup_duration: 0,
            config: VaultConfig {
                performance_fee: Decimal::percent(5),
                treasury: Addr::unchecked(admin.address()),
                create_position_max_slippage: Decimal::percent(5),
                swap_max_slippage: Decimal::percent(5),
            },
            vault_token_subdenom: "utestvault".to_string(),
            range_admin: admin.address(),
        };
        let contract = wasm
            .instantiate(
                code_id,
                &instantiate_msg,
                Some(admin.address().as_str()),
                Some("cl-vault"),
                &[],
                &admin,
            )
            .unwrap();

        (app, Addr::unchecked(contract.data.address), pool.id, admin)
    }
}
