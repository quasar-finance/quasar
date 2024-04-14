pub mod adapters;
pub mod config;
pub mod contract;
mod describe;
mod error;
mod execute;
pub mod handle;
mod instantiate;
pub mod msg;
pub mod query;
mod reply;
pub mod state;
pub mod vault;

pub use crate::error::ContractError;

#[macro_export]
macro_rules! debug {
    ($deps: ident, $tag:literal, $($arg:tt)*) => {
        $deps.api.debug(format!(concat!($tag, " :{:?}"), $($arg)*).as_str())
    };
}
