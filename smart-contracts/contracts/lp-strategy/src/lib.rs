pub mod contract;
pub mod error;
mod helpers;
mod ibc;
pub mod msg;
mod state;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
