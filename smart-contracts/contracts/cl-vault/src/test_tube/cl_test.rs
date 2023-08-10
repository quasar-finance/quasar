#[cfg(test)]
mod tests {
    use cw_vault_multi_standard::VaultInfoResponse;
    use osmosis_std::types::osmosis::{
        concentratedliquidity::v1beta1::{Pool, PoolsRequest},
        tokenfactory::v1beta1::QueryDenomsFromCreatorRequest,
    };
    use osmosis_test_tube::{
        cosmrs::proto::traits::Message, Account, ConcentratedLiquidity, Module, TokenFactory, Wasm,
    };

    use crate::{
        msg::{ClQueryMsg, ExtensionQueryMsg, QueryMsg},
        query::PoolResponse,
        test_tube::default_init,
    };

    #[test]
    fn default_init_works() {
        let (app, contract_address, _cl_pool_id, admin) = default_init();
        let wasm = Wasm::new(&app);
        let cl = ConcentratedLiquidity::new(&app);
        let tf = TokenFactory::new(&app);

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
    }
}
