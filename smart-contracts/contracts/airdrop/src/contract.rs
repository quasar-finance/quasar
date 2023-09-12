use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, Uint128};
use cw2::set_contract_version;

use crate::error::AirdropErrors;
use crate::helpers::is_contract_admin;
use crate::msg::{AdminExecuteMsg, ExecuteMsg, InstantiateMsg};
use crate::state::{Config, AIRDROP_ID, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "quasar_airdrop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, AirdropErrors> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        funding_or_refund_address: deps.api.addr_validate(&msg.funding_or_refund_address)?,
    };

    CONFIG.save(deps.storage, &config)?;
    AIRDROP_ID.save(deps.storage, &Uint128::zero())?;

    Ok(Response::default())
}

pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) {
    match msg {
        ExecuteMsg::Admin(admin_msg) => {
            is_contract_admin(&deps.querier, &env, &info.sender)?;
            match admin_msg {
                AdminExecuteMsg::AddAirdropConfig(config) => {}
                AdminExecuteMsg::UpdateAirdropConfig {
                    airdrop_id,
                    airdrop_config,
                } => {}
                AdminExecuteMsg::AddUsers {
                    airdrop_id,
                    users,
                    amounts,
                } => {}
                AdminExecuteMsg::AddUser {
                    airdrop_id,
                    user,
                    amount,
                } => {}
                AdminExecuteMsg::RemoveUsers { airdrop_id, users } => {}
                AdminExecuteMsg::RemoveUser { airdrop_id, user } => {}
                AdminExecuteMsg::WithdrawFunds { airdrop_id } => {}
            }
        }
        ExecuteMsg::ClaimAirdrop(airdrop_id) => {}
    }
}
