use std::str::FromStr;

use crate::{state::POOL_CONFIG, ContractError};
use cosmwasm_std::{Decimal, Fraction, QuerierWrapper, Storage, Uint128};
use osmosis_std::types::osmosis::{
    concentratedliquidity::v1beta1::ConcentratedliquidityQuerier,
    poolmanager::{self, v1beta1::PoolmanagerQuerier},
};

/// get_spot_price
///
/// gets the spot price of the pool which this vault is managing funds in. This will always return token0 in terms of token1 (or would it be the other way around?)
pub fn get_spot_price(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
) -> Result<Decimal, ContractError> {
    let pool_config = POOL_CONFIG.load(storage)?;

    let pm_querier = PoolmanagerQuerier::new(querier);
    let spot_price = pm_querier.spot_price(
        pool_config.pool_id,
        pool_config.base_token,
        pool_config.quote_token,
    )?;

    Ok(Decimal::from_str(&spot_price.spot_price)?)
}

/// get_tokens_in_range
///
/// gets the total amount of each token in a given range
///
/// thanks: https://github.com/osmosis-labs/osmosis/blob/main/x/concentrated-liquidity/README.md#deducing-the-quantity-of-tokens-x-and-y-for-a-tick-range
// pub fn get_tokens_in_range(
//     storage: &dyn Storage,
//     querier: &QuerierWrapper,
//     pool_id: u64,
//     lower_tick: Uint128,
//     upper_tick: Uint128,
// ) -> Result<Uint128, ContractError> {
//     let pool_config = POOL_CONFIG.load(storage)?;

//     let cl_querier = ConcentratedliquidityQuerier::new(querier);
//     let liquidity = cl_querier.liquidity_net_in_direction(
//         pool_id,
//         pool_config.base_token,
//         lower_tick,
//         false,
//         upper_tick,
//         false,
//     )?;

//     let lower_tick_price =
//     liquidity.current_tick
// }

/// get_liquidity_needed_for_tokens
///
/// this function calculates the liquidity needed for depositing base_token and quote token amounts respectively and returns both.
/// depositing both tokens would result in a refund of the token with higher needed liquidity
///
/// thanks: https://github.com/osmosis-labs/osmosis/blob/ma * (liquidity_needed_base_token - liquidity_needed_quote_token)/ liquidity_needed_base_token
/// in(deposit_amount_0/x/concentrated-liquidity/README.md#adding-liquidity
pub fn get_liquidity_needed_for_tokens(
    delta_base_token: String,
    delta_quote_token: String,
    lower_tick: i128,
    upper_tick: i128,
) -> Result<(Uint128, Uint128), ContractError> {
    let delta_x = Uint128::from_str(&delta_base_token)?;
    let delta_y = Uint128::from_str(&delta_quote_token)?;
    // calc liquidity needed for token
    unimplemented!("get_liquidity_needed_for_tokens")
}

pub fn get_deposit_amounts_for_liquidity_needed(
    liquidity_needed_base_token: Uint128,
    liquidity_needed_quote_token: Uint128,
    base_token_amount: String,
    quote_token_amount: String,
    // i hate this type but it's arguably a good way to write this
) -> Result<((Uint128, Uint128), (Uint128, Uint128)), ContractError> {
    // calc deposit amounts for liquidity needed
    let amount_0 = Uint128::from_str(&base_token_amount)?;
    let amount_1 = Uint128::from_str(&quote_token_amount)?;

    // one of these will be zero
    let mut remainder_0 = Uint128::zero();
    let mut remainder_1 = Uint128::zero();

    let (deposit_amount_0, deposit_amount_1) =
        if (liquidity_needed_base_token > liquidity_needed_quote_token) {
            // scale base token amount down by L1/L0, take full amount of quote token
            let new_amount_0 =
                amount_0.multiply_ratio(liquidity_needed_quote_token, liquidity_needed_base_token);
            remainder_0 = amount_0.checked_sub(new_amount_0).unwrap();

            (new_amount_0, amount_1)
        } else {
            // scale quote token amount down by L0/L1, take full amount of base token
            let new_amount_1 =
                amount_1.multiply_ratio(liquidity_needed_base_token, liquidity_needed_quote_token);
            remainder_1 = amount_1.checked_sub(new_amount_1).unwrap();

            (amount_0, new_amount_1)
        };

    Ok((
        (deposit_amount_0, deposit_amount_1),
        (remainder_0, remainder_1),
    ))
}

pub fn with_slippage(amount: Uint128, slippage: Decimal) -> Result<Uint128, ContractError> {
    let slippage_multiplier = Decimal::one().checked_sub(slippage)?;

    let adjusted_amount = amount.checked_multiply_ratio(
        slippage_multiplier.numerator(),
        slippage_multiplier.denominator(),
    )?;

    Ok(adjusted_amount)
}

#[cfg(test)]
mod tests {
    use super::*;
}
