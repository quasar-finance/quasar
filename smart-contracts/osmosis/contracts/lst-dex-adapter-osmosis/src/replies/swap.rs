use crate::{
    contract::{DexAdapter, DexAdapterResult},
    state::RECIPIENT,
};

use abstract_app::{
    sdk::{Execution, TransferInterface},
    traits::AbstractResponse,
};
use cosmwasm_std::{DepsMut, Env, Reply};

use crate::state::STATE;

pub fn swap_reply(deps: DepsMut, _env: Env, app: DexAdapter, _reply: Reply) -> DexAdapterResult {
    let state = STATE.load(deps.storage)?;
    let proxy_address = app.load_state(deps.storage)?.proxy_address;
    let balance = deps
        .querier
        .query_balance(proxy_address.to_string(), state.receive_asset.inner())?;
    let recipient = RECIPIENT.load(deps.storage)?.unwrap();
    let executor = app.executor(deps.as_ref());
    let transfer_action = app
        .bank(deps.as_ref())
        .transfer(vec![balance], &recipient)?;
    let transfer_msg = executor.execute(vec![transfer_action])?;
    Ok(app.response("swap_reply {}").add_message(transfer_msg))
}
