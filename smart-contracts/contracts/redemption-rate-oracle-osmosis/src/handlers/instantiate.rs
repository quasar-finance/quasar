use crate::{
    contract::{RedemptionRateOracle, RedemptionRateOracleResult},
    msg::RedemptionRateOracleInstantiateMsg,
    state::{OWNER, STRIDE_ORACLE},
};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use mars_owner::OwnerInit::SetInitialOwner;

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _app: RedemptionRateOracle,
    msg: RedemptionRateOracleInstantiateMsg,
) -> RedemptionRateOracleResult {
    STRIDE_ORACLE.save(deps.storage, &deps.api.addr_validate(&msg.stride_oracle)?)?;
    OWNER.initialize(deps.storage, deps.api, SetInitialOwner { owner: msg.owner })?;
    Ok(Response::new())
}
