use cosmwasm_std::Decimal;
use osmosis_test_tube::{
    cosmrs::proto::prost::Message,
    osmosis_std::types::osmosis::concentratedliquidity::{
        self,
        v1beta1::{CreateConcentratedLiquidityPoolsProposal, Pool, PoolRecord, PoolsRequest},
    },
    Account, ConcentratedLiquidity, GovWithAppAccess, Module, OsmosisTestApp, SigningAccount,
};

/// Create a CL pool without any liquidity
pub fn create_cl_pool<'a>(
    app: &'a OsmosisTestApp,
    denom0: String,
    denom1: String,
    tick_spacing: u64,
    spread_factor: Decimal,
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
                spread_factor: spread_factor.atomics().to_string(),
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
