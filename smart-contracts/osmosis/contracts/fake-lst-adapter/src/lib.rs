pub mod contract;
mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const MY_NAMESPACE: &str = "quasar";
pub const MY_APP_NAME: &str = "fake-lst-adapter";
pub const MY_APP_ID: &str = const_format::formatcp!("{MY_NAMESPACE}:{MY_APP_NAME}");
