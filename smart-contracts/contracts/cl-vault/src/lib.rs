pub mod contract;
mod error;
pub mod helpers;
mod instantiate;
mod math;
pub mod msg;
pub mod query;
mod reply;
mod rewards;
pub mod state;
mod vault;

pub use crate::error::ContractError;

#[cfg(test)]
mod test_tube;

#[cfg(test)]
mod test_helpers;

#[macro_export]
macro_rules! debug {
    ($deps: ident, $tag:literal, $($arg:tt)*) => {
        $deps.api.debug(format!(concat!($tag, " :{:?}"), $($arg)*).as_str())
    };
    ($deps: ident, $tag:literal) => {
        $deps.api.debug($tag)
    };
}
