#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use osmosis_std::types::osmosis::{poolmanager::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute}, concentratedliquidity::v1beta1::ClaimableSpreadRewardsRequest};
    use osmosis_test_tube::{Module, PoolManager, Wasm, Account};
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use crate::{msg::ExecuteMsg, test_tube::default_init};

    #[test]
    #[ignore]
    fn test_rewards_single_distribute_claim() {
        let (app, contract_address, cl_pool_id, _admin) = default_init();
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let bob = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let wasm = Wasm::new(&app);

        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[Coin::new(5_000_000, "uatom"), Coin::new(5_000_000, "uosmo")],
                &alice,
            )
            .unwrap();

        // do a bunch of swaps to get some swap fees
        PoolManager::new(&app).swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: bob.address(),
                routes: vec![SwapAmountInRoute { pool_id: cl_pool_id, token_out_denom: "uatom".to_string() }],
                token_in: Some(OsmoCoin { denom: "uosmo".to_string(), amount: "100".to_string() }),
                token_out_min_amount: "1".to_string(),
            },
            &bob,
        ).unwrap();

        PoolManager::new(&app).swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: bob.address(),
                routes: vec![SwapAmountInRoute { pool_id: cl_pool_id, token_out_denom: "uatom".to_string() }],
                token_in: Some(OsmoCoin { denom: "uosmo".to_string(), amount: "100".to_string() }),
                token_out_min_amount: "1".to_string(),
            },
            &bob,
        ).unwrap();

        PoolManager::new(&app).swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: bob.address(),
                routes: vec![SwapAmountInRoute { pool_id: cl_pool_id, token_out_denom: "uatom".to_string() }],
                token_in: Some(OsmoCoin { denom: "uosmo".to_string(), amount: "100".to_string() }),
                token_out_min_amount: "1".to_string(),
            },
            &bob,
        ).unwrap();

        PoolManager::new(&app).swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: bob.address(),
                routes: vec![SwapAmountInRoute { pool_id: cl_pool_id, token_out_denom: "uosmo".to_string() }],
                token_in: Some(OsmoCoin { denom: "uatom".to_string(), amount: "100".to_string() }),
                token_out_min_amount: "1".to_string(),
            },
            &bob,
        ).unwrap();

        PoolManager::new(&app).swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: bob.address(),
                routes: vec![SwapAmountInRoute { pool_id: cl_pool_id, token_out_denom: "uosmo".to_string() }],
                token_in: Some(OsmoCoin { denom: "uatom".to_string(), amount: "100".to_string() }),
                token_out_min_amount: "1".to_string(),
            },
            &bob,
        ).unwrap();

        PoolManager::new(&app).swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: bob.address(),
                routes: vec![SwapAmountInRoute { pool_id: cl_pool_id, token_out_denom: "uosmo".to_string() }],
                token_in: Some(OsmoCoin { denom: "uatom".to_string(), amount: "100".to_string() }),
                token_out_min_amount: "1".to_string(),
            },
            &bob,
        ).unwrap();

        let res = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::DistributeRewards { }),
                &[],
                &alice,
            )
            .unwrap();

        println!("{:?}", res.events)
    }
}
