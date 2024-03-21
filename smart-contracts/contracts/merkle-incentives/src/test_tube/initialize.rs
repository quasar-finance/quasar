#[cfg(test)]
pub mod initialize {
    use cosmwasm_std::{Addr, Coin, Decimal};
    use osmosis_test_tube::{Account, Module, OsmosisTestApp, SigningAccount, Wasm};

    use crate::{
        msg::InstantiateMsg,
        state::Config,
    };

    pub fn default_init(gauge_coins: Vec<Coin>) -> (OsmosisTestApp, Addr, SigningAccount) {
        init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/merkle_incentives.wasm",
            gauge_coins,
        )
    }

    pub fn init_test_contract(
        filename: &str,
        gauge_coins: Vec<Coin>,
    ) -> (OsmosisTestApp, Addr, SigningAccount) {
        // Create new osmosis appchain instance
        let app = OsmosisTestApp::new();
        let wasm = Wasm::new(&app);

        // Ensure uosmo is always included by checking and adding if necessary
        let mut coins_with_uosmo = gauge_coins.clone();
        if !gauge_coins.iter().any(|coin| coin.denom == "uosmo") {
            coins_with_uosmo.push(Coin::new(100_000_000_000_000_000_000, "uosmo"));
        }

        // Create new account with initial funds
        let admin = app.init_account(&coins_with_uosmo).unwrap();

        // Load compiled wasm bytecode
        let wasm_byte_code = std::fs::read(filename).unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, &admin)
            .unwrap()
            .data
            .code_id;

        // Instantiate vault
        let contract = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    config: Config {
                        clawback_address: Addr::unchecked("bob"),
                        start_block: 1,
                        end_block: 100,
                        expiration_block: 10_000,
                    },
                },
                Some(admin.address().as_str()),
                Some("merkle-incentives"),
                &[],
                &admin,
            )
            .unwrap();

        (app, Addr::unchecked(contract.data.address), admin)
    }

    #[test]
    #[ignore]
    fn default_init_works() {
        // TODO
    }
}
