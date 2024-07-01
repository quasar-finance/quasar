pub mod contract;
mod error;
pub mod msg;
pub mod state;

pub use crate::error::LstAdapterError;

#[cfg(test)]
mod tests;

pub const LST_ADAPTER_OSMOSIS_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use contract::interface::LstAdapterInterface;

pub const LST_ADAPTER_OSMOSIS_NAMESPACE: &str = "quasar";
pub const LST_ADAPTER_OSMOSIS_NAME: &str = "lst-adapter-osmosis";
pub const LST_ADAPTER_OSMOSIS_ID: &str =
    const_format::formatcp!("{LST_ADAPTER_OSMOSIS_NAMESPACE}:{LST_ADAPTER_OSMOSIS_NAME}");
