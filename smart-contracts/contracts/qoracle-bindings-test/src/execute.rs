use cosmwasm_std::{DepsMut, Response};

use quasar_bindings::querier::QuasarQuerier;
use quasar_bindings::query::QuasarQuery;

use crate::error::ContractError;

pub fn demo_fetch_pools(deps: DepsMut<QuasarQuery>) -> Result<Response, ContractError> {
    let querier = QuasarQuerier::new(&deps.querier);

    let pools_response = querier.osmosis_pools(Option::None)?;

    let pools = pools_response.pools.unwrap_or(vec![]);

    let first_pool_id = match pools.first() {
        Some(pool) => pool.pool_info.id.to_string(),
        None => "No pools found".to_string(),
    };

    Ok(Response::new()
        .add_attribute("num_pools", pools.len().to_string())
        .add_attribute("first_pool_id", first_pool_id))
}

pub fn demo_fetch_pool_info(deps: DepsMut<QuasarQuery>) -> Result<Response, ContractError> {
    let querier = QuasarQuerier::new(&deps.querier);

    let pool = match querier.osmosis_pool("1".to_string())?.pool {
        Some(pool_info) => pool_info,
        None => {
            return Err(ContractError::CustomError {
                val: "No pool info".into(),
            })
        }
    };

    Ok(Response::new().add_attribute("pool_info_id", pool.pool_info.id.to_string()))
}

pub fn demo_fetch_oracle_prices(deps: DepsMut<QuasarQuery>) -> Result<Response, ContractError> {
    let querier = QuasarQuerier::new(&deps.querier);

    let oracle_prices = querier.oracle_prices()?;

    Ok(Response::new()
        .add_attribute(
            "oracle_prices_length",
            oracle_prices.prices.len().to_string(),
        )
        .add_attribute(
            "updated_at_height",
            oracle_prices.updated_at_height.to_string(),
        )
        .add_attribute(
            "oracle_prices",
            oracle_prices.prices.first().unwrap().denom.clone(),
        ))
}
