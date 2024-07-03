use crate::{
    contract::{RedemptionRateOracle, RedemptionRateOracleResult},
    msg::RedemptionRateOracleExecuteMsg,
    state::{OWNER, STRIDE_ORACLE},
};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _app: RedemptionRateOracle,
    msg: RedemptionRateOracleExecuteMsg,
) -> RedemptionRateOracleResult {
    match msg {
        RedemptionRateOracleExecuteMsg::Update { stride_oracle } => {
            update(deps, info, stride_oracle)
        }
        RedemptionRateOracleExecuteMsg::UpdateOwner(update) => {
            Ok(OWNER.update(deps, info, update)?)
        }
    }
}

fn update(
    deps: DepsMut,
    info: MessageInfo,
    stride_oracle: Option<String>,
) -> RedemptionRateOracleResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    if let Some(stride_oracle) = stride_oracle {
        STRIDE_ORACLE.save(deps.storage, &deps.api.addr_validate(&stride_oracle)?)?;
    }
    Ok(Response::default())
}
