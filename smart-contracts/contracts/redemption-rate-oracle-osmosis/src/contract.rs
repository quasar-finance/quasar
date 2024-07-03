use crate::{
    error::RedemptionRateOracleError,
    handlers,
    msg::{
        RedemptionRateOracleExecuteMsg, RedemptionRateOracleInstantiateMsg,
        RedemptionRateOracleMigrateMsg, RedemptionRateOracleQueryMsg,
    },
    REDEMPTION_RATE_ORACLE_ID_ID, REDEMPTION_RATE_ORACLE_VERSION,
};

use abstract_app::AppContract;
use cosmwasm_std::Response;

pub type RedemptionRateOracleResult<T = Response> = Result<T, RedemptionRateOracleError>;

pub type RedemptionRateOracle = AppContract<
    RedemptionRateOracleError,
    RedemptionRateOracleInstantiateMsg,
    RedemptionRateOracleExecuteMsg,
    RedemptionRateOracleQueryMsg,
    RedemptionRateOracleMigrateMsg,
>;

const APP: RedemptionRateOracle = RedemptionRateOracle::new(
    REDEMPTION_RATE_ORACLE_ID_ID,
    REDEMPTION_RATE_ORACLE_VERSION,
    None,
)
.with_instantiate(handlers::instantiate_handler)
.with_execute(handlers::execute_handler)
.with_query(handlers::query_handler)
.with_migrate(handlers::migrate_handler);

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, RedemptionRateOracle);

abstract_app::cw_orch_interface!(APP, RedemptionRateOracle, RedemptionRateOracleInterface);
