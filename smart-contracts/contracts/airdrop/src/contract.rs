use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw2::set_contract_version;

use crate::error::AirdropErrors;
use crate::msg::InstantiateMsg;
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

    let owner = if let Some(owner) = msg.owner {
        deps.api.addr_validate(&owner)?
    } else {
        info.sender
    };

    let config = Config {
        owner,
        quasar_funding_address: deps.api.addr_validate(&msg.quasar_funding_address)?,
    };

    CONFIG.save(deps.storage, &config)?;
    AIRDROP_ID.save(deps.storage, &Uint128::zero())?;

    Ok(Response::default())
}
