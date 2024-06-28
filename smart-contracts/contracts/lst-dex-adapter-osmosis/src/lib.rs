pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
mod replies;
pub mod state;

pub use contract::interface::DexAdapterInterface;
pub use error::DexAdapterError;

pub const MY_NAMESPACE: &str = "quasar";
pub const MY_APP_NAME: &str = "lst-dex-adapter-osmosis";
pub const MY_APP_ID: &str = const_format::formatcp!("{MY_NAMESPACE}:{MY_APP_NAME}");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
