use crate::{
    contract::{RedemptionRateOracle, RedemptionRateOracleResult},
    msg::RedemptionRateOracleMigrateMsg,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env};

pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    app: RedemptionRateOracle,
    _msg: RedemptionRateOracleMigrateMsg,
) -> RedemptionRateOracleResult {
    Ok(app.response("migrate"))
}
