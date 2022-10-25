pub mod contract;
pub mod error;
mod ibc_builder;
mod helpers;
mod msg;
mod queue;
mod state;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
