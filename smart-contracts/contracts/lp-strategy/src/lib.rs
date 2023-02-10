pub mod bond;
pub mod contract;
pub mod error;
pub mod helpers;
pub mod ibc;
pub mod ibc_lock;
mod ibc_util;
pub mod icq;
pub mod msg;
pub mod reply;
pub mod start_unbond;
pub mod state;
pub mod unbond;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
