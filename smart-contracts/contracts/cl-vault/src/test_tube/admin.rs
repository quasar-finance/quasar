#[cfg(test)]
mod tests {
    use osmosis_std::types::cosmos::base::v1beta1;

    use crate::test_tube::initialize::initialize::default_init;

    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";

    #[test]
    #[ignore]
    fn range_admin_update_works() {
        let (_app, _contract_address, _cl_pool_id, _admin) = default_init(vec![
            v1beta1::Coin {
                denom: DENOM_BASE.to_string(),
                amount: "1000000000000".to_string(),
            },
            v1beta1::Coin {
                denom: DENOM_QUOTE.to_string(),
                amount: "1000000000000".to_string(),
            },
        ])
        .unwrap();
        // change the range admin and verify that it works
    }
}
