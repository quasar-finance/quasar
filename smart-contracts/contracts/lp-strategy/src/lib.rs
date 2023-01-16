pub mod contract;
pub mod error;
pub mod helpers;
pub mod ibc;
pub mod lock;
pub mod msg;
pub mod state;
pub mod strategy;
pub mod vault;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
