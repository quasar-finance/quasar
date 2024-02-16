#[cfg(test)]
mod tests {
    use cosmwasm_std::coin;

    use crate::test_tube::initialize::initialize::{default_init, TOKENS_PROVIDED_AMOUNT};

    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";

    #[test]
    #[ignore]
    fn range_admin_update_works() {
        let (_app, _contract, _cl_pool_id, _admin) = default_init(
            vec![
                coin(TOKENS_PROVIDED_AMOUNT, DENOM_BASE.to_string()),
                coin(TOKENS_PROVIDED_AMOUNT, DENOM_QUOTE.to_string()),
            ],
            vec![
                coin(TOKENS_PROVIDED_AMOUNT, DENOM_BASE.to_string()),
                coin(TOKENS_PROVIDED_AMOUNT, DENOM_QUOTE.to_string()),
            ],
        )
        .unwrap();
        // change the range admin and verify that it works
    }
}
