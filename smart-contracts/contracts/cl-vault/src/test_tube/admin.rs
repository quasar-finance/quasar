#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{Coin, Decimal, Uint128};
    use osmosis_std::types::{
        cosmos::base::v1beta1,
        osmosis::concentratedliquidity::poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool,
    };

    use crate::test_tube::initialize::initialize::init_test_contract;

    #[test]
    #[ignore]
    fn range_admin_update_works() {
        let (_app, _contract_address, _cl_pool_id, _admin) = init_test_contract(
            "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
            &[
                Coin::new(340282366920938463463374607431768211455u128, "uatom"),
                Coin::new(340282366920938463463374607431768211455u128, "uosmo"),
            ],
            MsgCreateConcentratedPool {
                sender: "overwritten".to_string(),
                denom0: "uatom".to_string(),
                denom1: "uosmo".to_string(),
                tick_spacing: 100,
                spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
            },
            -5000000, // 0.5 spot price
            500000,   // 1.5 spot price
            vec![
                v1beta1::Coin {
                    denom: "uatom".to_string(),
                    amount: "1000000000000".to_string(),
                },
                v1beta1::Coin {
                    denom: "uosmo".to_string(),
                    amount: "1000000000000".to_string(),
                },
            ],
            Uint128::zero(),
            Uint128::zero(),
        );
        // change the range admin and verify that it works
    }
}
