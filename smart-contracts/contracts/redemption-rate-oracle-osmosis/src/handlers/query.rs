use crate::{
    contract::{RedemptionRateOracle, RedemptionRateOracleResult},
    msg::{OracleInfo, OraclesResponse, RedemptionRateOracleQueryMsg},
    state::STRIDE_ORACLE,
    RedemptionRateOracleError,
};

use cosmwasm_std::{to_json_binary, Binary, Decimal, Deps, Env};
use ica_oracle::msg::{
    QueryMsg as StrideQueryMsg, RedemptionRateResponse as StrideRedemptionRateResponse,
};

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _app: &RedemptionRateOracle,
    msg: RedemptionRateOracleQueryMsg,
) -> RedemptionRateOracleResult<Binary> {
    match msg {
        RedemptionRateOracleQueryMsg::RedemptionRate { denom } => {
            Ok(to_json_binary(&query_redemption_rate(deps, denom)?)?)
        }
        RedemptionRateOracleQueryMsg::Oracles {} => Ok(to_json_binary(&query_oracles(deps)?)?),
    }
}

fn query_redemption_rate(deps: Deps, denom: String) -> Result<Decimal, RedemptionRateOracleError> {
    let response: StrideRedemptionRateResponse = deps.querier.query_wasm_smart(
        STRIDE_ORACLE.load(deps.storage)?,
        &StrideQueryMsg::RedemptionRate {
            denom,
            params: None,
        },
    )?;
    Ok(response.redemption_rate)
}

fn query_oracles(deps: Deps) -> Result<OraclesResponse, RedemptionRateOracleError> {
    Ok(OraclesResponse {
        oracles: vec![OracleInfo {
            name: "stride".to_string(),
            address: STRIDE_ORACLE.load(deps.storage)?.to_string(),
        }],
    })
}
