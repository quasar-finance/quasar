#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use osmosis_std::types::ibc::applications::interchain_accounts::controller::v1::MsgRegisterInterchainAccount;

use crate::error::ContractError;
use crate::msg::{
    FakeLstExecuteMsg, FakeLstInstantiateMsg, FakeLstQueryMsg, RedemptionRateResponse,
};
use crate::state::STATE;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: FakeLstInstantiateMsg,
) -> Result<Response, ContractError> {
    STATE.save(deps.storage, &msg.redemption_rate)?;

    let reg_msg = MsgRegisterInterchainAccount {
        owner: env.contract.address.to_string(),
        connection_id: "connection-3027".to_string(),
        version: "ics27-1".to_string(),
    };
    // let reg_msg = MsgRegisterInterchainAccount {
    //     owner: Signer::from_str(env.contract.address.as_ref()).unwrap(),
    //     connection_id: ConnectionId::new(3027),
    //     version: Version::new("0.0.1".to_string()),
    // };
    Ok(Response::new().add_message(reg_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: FakeLstExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        FakeLstExecuteMsg::Update { redemption_rate } => {
            STATE.update(deps.storage, |_| -> Result<_, ContractError> {
                Ok(redemption_rate)
            })?;
        }
    }
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: FakeLstQueryMsg) -> Result<Binary, ContractError> {
    match msg {
        FakeLstQueryMsg::RedemptionRate { .. } => Ok(to_json_binary(&RedemptionRateResponse {
            redemption_rate: STATE.load(deps.storage)?,
            update_time: 0u64,
        })?),
    }
}
