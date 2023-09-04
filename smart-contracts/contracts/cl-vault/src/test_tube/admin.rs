#[cfg(test)]
mod tests {

    use crate::test_tube::default_init;

    #[test]
    #[ignore]
    fn range_admin_update_works() {
        let (app, contract_address, _cl_pool_id, _admin) = default_init();
        // change the range admin and verify that it works
    }
}
