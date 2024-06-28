use crate::{
    contract::{DexAdapter, DexAdapterResult},
    msg::DexAdapterInstantiateMsg,
    state::{State, RECIPIENT, STATE},
};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _app: DexAdapter,
    msg: DexAdapterInstantiateMsg,
) -> DexAdapterResult {
    STATE.save(
        deps.storage,
        &State {
            lst_adapter: deps.api.addr_validate(&msg.lst_adapter)?,
            dex: msg.dex,
            offer_asset: msg.offer_asset,
            receive_asset: msg.receive_asset,
            margin: msg.margin,
            pool: msg.pool,
        },
    )?;
    RECIPIENT.save(deps.storage, &None)?;
    Ok(Response::new())
}
