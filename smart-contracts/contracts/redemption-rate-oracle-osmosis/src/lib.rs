pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
pub mod state;

pub use contract::interface::RedemptionRateOracleInterface;
pub use error::RedemptionRateOracleError;

pub const QUASAR_NAMESPACE: &str = "quasar";
pub const REDEMPTION_RATE_ORACLE_NAME: &str = "redemption-rate-oracle-osmosis";
pub const REDEMPTION_RATE_ORACLE_ID_ID: &str =
    const_format::formatcp!("{QUASAR_NAMESPACE}:{REDEMPTION_RATE_ORACLE_NAME}");
pub const REDEMPTION_RATE_ORACLE_VERSION: &str = env!("CARGO_PKG_VERSION");
