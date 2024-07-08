use cosmwasm_std::{Addr, Env, QuerierWrapper, StdResult, Uint128};

pub fn query_balance<'a>(
    querier: &QuerierWrapper<'a>,
    addr: &Addr,
    denom: &str,
) -> StdResult<Uint128> {
    Ok(querier.query_balance(addr, denom)?.amount)
}

pub fn query_contract_balance<'a>(
    querier: &QuerierWrapper<'a>,
    env: &Env,
    denom: &str,
) -> StdResult<Uint128> {
    query_balance(querier, &env.contract.address, denom)
}
