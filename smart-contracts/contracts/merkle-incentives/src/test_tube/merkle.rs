#[cfg(test)]
mod tests {
    use crate::test_tube::initialize::initialize::default_init;

    #[test]
    #[ignore]
    fn merkle_complete_cycle_works() {
        let (app, contract, admin) = default_init();

        // TODO: Execute AdminMsg::UpdateAdmin

        // TODO: Execute AdminMsg::UpdateMerkleRoot
        // https://github.com/quasar-finance/merkle-incentives/blob/f45d842a2a6cf32d2b683f0893cae5bfaca9de3e/incentives/contracts/osmo1u4ppw4mxp00znxq5ll834dgr7ctd7jrp5hrzshch5ngfpwmp2fqsputgsx/merkle/100001.json#L3

        // TODO: Execute IncentivesMsg::Claim
        // https://github.com/quasar-finance/merkle-incentives/blob/f45d842a2a6cf32d2b683f0893cae5bfaca9de3e/incentives/contracts/osmo1u4ppw4mxp00znxq5ll834dgr7ctd7jrp5hrzshch5ngfpwmp2fqsputgsx/fetch/100001.json#L3
    }
}
