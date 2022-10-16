
use cosmwasm_std::{
    DepsMut, Response,
};

use quasar_bindings::querier::{QuasarQuerier};
use quasar_bindings::query::QuasarQuery;

use crate::error::ContractError;



pub fn demo_fetch_pools(deps: DepsMut<QuasarQuery>) -> Result<Response, ContractError> {
    let querier = QuasarQuerier::new(&deps.querier);

    let pools_response = querier.osmosis_pools(Option::None)?;

    Ok(Response::new()
        .add_attribute("num_pools", pools_response.pools.len().to_string())
        .add_attribute(
            "first_pool_id",
            pools_response.pools.first().unwrap().info.id.to_string(),
        ))
}

pub fn demo_fetch_pool_info(deps: DepsMut<QuasarQuery>) -> Result<Response, ContractError> {
    let querier = QuasarQuerier::new(&deps.querier);

    let pool_info = match querier.osmosis_pool_info("1".to_string())?.pool_info {
        Some(pool_info) => pool_info,
        None => {
            return Err(ContractError::CustomError {
                val: "No pool info".into(),
            })
        }
    };

    Ok(Response::new().add_attribute("pool_info_id", pool_info.info.id.to_string()))
}

pub fn demo_fetch_oracle_prices(deps: DepsMut<QuasarQuery>) -> Result<Response, ContractError> {
    let querier = QuasarQuerier::new(&deps.querier);

    let oracle_prices = querier.oracle_prices()?;

    Ok(Response::new()
        .add_attribute("oracle_prices_length", oracle_prices.prices.len().to_string())
        .add_attribute("updated_at_height", oracle_prices.updatedAtHeight.to_string())
        .add_attribute(
            "oracle_prices",
            oracle_prices.prices.first().unwrap().denom.clone(),
        ))
}
