use crate::{
    contract::{DexAdapter, DexAdapterResult},
    msg::DexAdapterMigrateMsg,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env};

pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    app: DexAdapter,
    _msg: DexAdapterMigrateMsg,
) -> DexAdapterResult {
    Ok(app.response("migrate"))
}
