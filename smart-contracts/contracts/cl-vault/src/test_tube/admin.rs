#[cfg(test)]
mod tests {
    use crate::msg::AdminExtensionExecuteMsg::UpdateConfig;
    use crate::msg::ClQueryMsg::VaultConfigQuery;
    use crate::query::VaultConfigResponse;
    use crate::{
        msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg},
        test_tube::initialize::initialize::init_test_contract,
    };
    use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
    use osmosis_std::types::{
        cosmos::base::v1beta1,
        osmosis::concentratedliquidity::poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool,
    };
    use osmosis_test_tube::{
        Account, Bank, ConcentratedLiquidity, Module, OsmosisTestApp, SigningAccount, Wasm,
    };
    use proptest::prelude::*;

    use crate::state::VaultConfig;

    const ITERATIONS_NUMBER: usize = 1000;
    const ACCOUNTS_NUMBER: u64 = 10;
    const ACCOUNTS_INITIAL_BALANCE: u128 = 1_000_000_000_000;
    const DENOM_BASE: &str = "ZZZZZ";
    //"ibc/0CD3A0285E1341859B5E86B6AB7682F023D03E97607CCC1DC95706411D866DF7";
    const DENOM_QUOTE: &str =
        "ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858";

    fn update_config(
        wasm: &Wasm<OsmosisTestApp>,
        cl: &ConcentratedLiquidity<OsmosisTestApp>,
        contract_address: &Addr,
        new_config: VaultConfig,
        admin_account: &SigningAccount,
    ) {
        let _update_config = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Admin(UpdateConfig {
                    updates: new_config,
                })),
                &[],
                admin_account,
            )
            .unwrap();
    }

    prop_compose! {
        // TODO: evaluate if lower_tick and upper_tick are too much arbitrary
        fn get_initial_range()(lower_tick in 1i64..1_000_000, upper_tick in 1_000_001i64..2_000_000) -> (i64, i64) {
            (lower_tick, upper_tick)
        }
    }

    proptest! {
        #[test]
        fn test_update_range_admin_works(
        (initial_lower_tick, initial_upper_tick) in get_initial_range(),
        ) {
            // Creating test core
            let (app, contract_address, cl_pool_id, admin_account) = init_test_contract(
                "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
                &[
                    Coin::new(100_000_000_000_000_000_000_000, "uosmo"),
                    Coin::new(100_000_000_000_000_000_000_000, DENOM_BASE),
                    Coin::new(100_000_000_000_000_000_000_000, DENOM_QUOTE),
                ],
                MsgCreateConcentratedPool {
                    sender: "overwritten".to_string(),
                    denom0: DENOM_BASE.to_string(),
                    denom1: DENOM_QUOTE.to_string(),
                    tick_spacing: 1,
                    spread_factor: "100000000000000".to_string(),
                },
                initial_lower_tick,
                initial_upper_tick,
                vec![
                    v1beta1::Coin {
                        denom: DENOM_BASE.to_string(),
                        amount: "100000000000000000".to_string(),
                    },
                    v1beta1::Coin {
                        denom: DENOM_QUOTE.to_string(),
                        amount: "100000000000000000".to_string(),
                    },
                ],
                Uint128::zero(),
                Uint128::zero(),
            );
            let wasm = Wasm::new(&app);
            let cl = ConcentratedLiquidity::new(&app);
            let bank = Bank::new(&app);

            // initialise and update the new config
            let new_config = VaultConfig {
                performance_fee: Decimal::percent(5),
                treasury: Addr::unchecked("new_treasury_address"),
                swap_max_slippage: Decimal::percent(5),
            };
            update_config(&wasm, &cl, &contract_address, new_config, &admin_account);

            // query the new config
            let res : VaultConfigResponse = wasm.query(
                contract_address.as_str(),
                &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                    VaultConfigQuery{},
                ))
            )
                .unwrap();

            // checking against admin address as the admin is treasury while instantiating.
            assert_ne!(res.config.treasury, admin_account.address());
        }
    }
}
