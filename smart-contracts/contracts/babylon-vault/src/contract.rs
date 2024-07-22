use crate::error::VaultError;
use crate::msg::{ExecuteMsg, InstantiateMsg, LstInfo, QueryMsg};
use crate::state::{LSTS, OWNER};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

const CONTRACT_NAME: &str = "quasar:babylon-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type VaultResult<T = Response> = Result<T, VaultError>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> VaultResult {
    OWNER.initialize(
        deps.storage,
        deps.api,
        mars_owner::OwnerInit::SetInitialOwner { owner: msg.owner },
    )?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let msg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: msg.subdenom,
    };
    Ok(Response::default().add_message(msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> VaultResult {
    match msg {
        ExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
        ExecuteMsg::RegisterLst { denom, interface } => register_lst(deps, info, denom, interface),
        ExecuteMsg::UnregisterLst { denom } => unregister_lst(deps, info, denom),
        _ => Ok(Response::default()),
    }
}

fn register_lst(deps: DepsMut, info: MessageInfo, denom: String, interface: String) -> VaultResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    LSTS.save(deps.storage, denom, &deps.api.addr_validate(&interface)?)?;
    Ok(Response::default())
}

fn unregister_lst(deps: DepsMut, info: MessageInfo, denom: String) -> VaultResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    let interface = LSTS.may_load(deps.storage, denom.clone())?;
    if interface.is_some() {
        LSTS.remove(deps.storage, denom);
        Ok(Response::default())
    } else {
        Err(VaultError::LstNotFound { denom })
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> VaultResult<Binary> {
    match msg {
        QueryMsg::Owner {} => Ok(to_json_binary(&OWNER.query(deps.storage)?)?),
        QueryMsg::Lsts {} => Ok(to_json_binary(&query_lsts(deps)?)?),
        _ => Ok(Binary::default()),
    }
}

fn query_lsts(deps: Deps) -> StdResult<Vec<LstInfo>> {
    let lsts: StdResult<Vec<(String, Addr)>> = LSTS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect();
    let lsts = lsts?;
    let infos: Vec<LstInfo> = lsts
        .into_iter()
        .map(|(denom, interface)| -> LstInfo {
            LstInfo {
                denom,
                interface: interface.to_string(),
            }
        })
        .collect();
    Ok(infos)
}
