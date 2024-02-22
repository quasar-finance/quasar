#[cfg(test)]
pub mod initialize {
    use cosmwasm_std::{Addr, Coin};
    use osmosis_test_tube::{Account, Module, OsmosisTestApp, SigningAccount, Wasm};

    use crate::msg::InstantiateMsg;

    pub fn default_init(ugauge_amount: u128) -> (OsmosisTestApp, Addr, SigningAccount) {
        init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/merkle_incentives.wasm",
            ugauge_amount,
        )
    }

    pub fn init_test_contract(
        filename: &str,
        ugauge_amount: u128,
    ) -> (OsmosisTestApp, Addr, SigningAccount) {
        // Create new osmosis appchain instance
        let app = OsmosisTestApp::new();
        let wasm = Wasm::new(&app);

        // Create new account with initial funds
        let admin = app
            .init_account(&vec![
                Coin::new(100_000_000_000_000_000_000, "uosmo"),
                Coin::new(ugauge_amount, "ugauge"),
            ])
            .unwrap();

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
                &InstantiateMsg {},
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
