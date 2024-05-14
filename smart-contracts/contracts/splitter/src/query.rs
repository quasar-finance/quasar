use cosmwasm_std::{Deps, StdResult};

use crate::state::{Receivers, ADMIN, RECEIVERS};

pub fn query_admin(deps: Deps) -> StdResult<String> {
    let admin = ADMIN.load(deps.storage)?;
    Ok(admin.to_string())
}

pub fn query_receivers(deps: Deps) -> StdResult<Receivers> {
    let receivers = RECEIVERS.load(deps.storage)?;
    Ok(receivers)
}
