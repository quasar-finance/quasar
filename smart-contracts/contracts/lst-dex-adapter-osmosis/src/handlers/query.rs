use crate::{
    contract::{DexAdapter, DexAdapterResult},
    msg::{ConfigResponse, DexAdapterQueryMsg},
    state::STATE,
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _app: &DexAdapter,
    msg: DexAdapterQueryMsg,
) -> DexAdapterResult<Binary> {
    match msg {
        DexAdapterQueryMsg::Config {} => to_json_binary(&query_config(deps)?),
    }
    .map_err(Into::into)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(ConfigResponse {
        lst_adapter: state.lst_adapter.to_string(),
        dex: state.dex,
        offer_asset: state.offer_asset,
        receive_asset: state.receive_asset,
        margin: state.margin,
        pool: state.pool,
    })
}
