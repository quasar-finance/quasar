pub mod contract;
pub mod error;
mod msg;
mod queue;
mod state;
mod strategy;
mod ibc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
