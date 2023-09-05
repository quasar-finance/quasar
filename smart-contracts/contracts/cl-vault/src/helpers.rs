use std::str::FromStr;

use crate::math::tick::tick_to_price;
use crate::state::ADMIN_ADDRESS;
use crate::{error::ContractResult, state::POOL_CONFIG, ContractError};
use cosmwasm_std::{
    Addr, Coin, Decimal, Decimal256, Deps, Fraction, MessageInfo, QuerierWrapper, Storage, Uint128,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;

/// returns the Coin of the needed denoms in the order given in denoms

pub(crate) fn must_pay_two(
    info: &MessageInfo,
    denoms: (String, String),
) -> ContractResult<(Coin, Coin)> {
    if info.funds.len() != 2 {
        return Err(cw_utils::PaymentError::MultipleDenoms {}.into());
    }

    let token0 = info
        .funds
        .clone()
        .into_iter()
        .find(|coin| coin.denom == denoms.0 && coin.amount > Uint128::zero())
        .ok_or(cw_utils::PaymentError::MissingDenom(denoms.0))?;

    let token1 = info
        .funds
        .clone()
        .into_iter()
        .find(|coin| coin.denom == denoms.1 && coin.amount > Uint128::zero())
        .ok_or(cw_utils::PaymentError::MissingDenom(denoms.1))?;

    Ok((token0, token1))
}

/// get_spot_price
///
/// gets the spot price of the pool which this vault is managing funds in. This will always return token0 in terms of token1 (or would it be the other way around?)
pub fn get_spot_price(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
) -> Result<Decimal, ContractError> {
    let pool_config = POOL_CONFIG.load(storage)?;

    let pm_querier = PoolmanagerQuerier::new(querier);
    let spot_price =
        pm_querier.spot_price(pool_config.pool_id, pool_config.token0, pool_config.token1)?;

    Ok(Decimal::from_str(&spot_price.spot_price)?)
}

// /// get_liquidity_needed_for_tokens
// ///
// /// this function calculates the liquidity needed for depositing token0 and quote token amounts respectively and returns both.
// /// depositing both tokens would result in a refund of the token with higher needed liquidity
// ///
// /// thanks: https://github.com/osmosis-labs/osmosis/blob/main/x/concentrated-liquidity/README.md#adding-liquidity
// pub fn get_liquidity_needed_for_tokens(
//     delta_token0: String,
//     delta_token1: String,
//     lower_tick: i64,
//     upper_tick: i64,
// ) -> Result<(Uint128, Uint128), ContractError> {
//     // todo check that decimal casts are correct
//     let delta_x = Decimal256::from_atomics(Uint128::from_str(&delta_token0)?, 18)?;
//     let delta_y = Decimal256::from_atomics(Uint128::from_str(&delta_token1)?, 18)?;
//     // calc liquidity needed for token

//     // save gas and read easier by calcing ahead (gas savings prob already done by compiler)
//     let price_lower = tick_to_price(lower_tick)?;
//     let price_upper = tick_to_price(upper_tick)?;
//     let sqrt_price_lower = price_lower.sqrt();
//     let sqrt_price_upper = price_upper.sqrt();
//     let denominator = sqrt_price_upper.checked_sub(sqrt_price_lower)?;

//     // liquidity follows the formula found in the link above this function. basically this:
//     // liquidity_x = (delta_x * sqrt(price_lower) * sqrt(price_upper))/(sqrt(price_upper) - sqrt(price_lower))
//     // liquidity_7 = (delta_x)/(sqrt(price_upper) - sqrt(price_lower))
//     // overflow city?
//     let liquidity_x = delta_x
//         .checked_mul(sqrt_price_lower)?
//         .checked_mul(sqrt_price_upper)?
//         .checked_div(denominator)?;

//     let liquidity_y = delta_y.checked_div(denominator)?;

//     // todo: check this is what we want
//     Ok((
//         liquidity_x.atomics().try_into()?,
//         liquidity_y.atomics().try_into()?,
//     ))
// }

// pub fn get_deposit_amounts_for_liquidity_needed(
//     liquidity_needed_token0: Uint128,
//     liquidity_needed_token1: Uint128,
//     token0_amount: String,
//     token1_amount: String,
//     // i hate this type but it's arguably a good way to write this
// ) -> Result<((Uint128, Uint128), (Uint128, Uint128)), ContractError> {
//     // calc deposit amounts for liquidity needed
//     let amount_0 = Uint128::from_str(&token0_amount)?;
//     let amount_1 = Uint128::from_str(&token1_amount)?;

//     // one of these will be zero
//     let mut remainder_0 = Uint128::zero();
//     let mut remainder_1 = Uint128::zero();

//     let (deposit_amount_0, deposit_amount_1) = if liquidity_needed_token0 > liquidity_needed_token1
//     {
//         // scale base token amount down by L1/L0, take full amount of quote token
//         let new_amount_0 =
//             amount_0.multiply_ratio(liquidity_needed_token1, liquidity_needed_token0);
//         remainder_0 = amount_0.checked_sub(new_amount_0).unwrap();

//         (new_amount_0, amount_1)
//     } else {
//         // scale quote token amount down by L0/L1, take full amount of base token
//         let new_amount_1 =
//             amount_1.multiply_ratio(liquidity_needed_token0, liquidity_needed_token1);
//         remainder_1 = amount_1.checked_sub(new_amount_1).unwrap();

//         (amount_0, new_amount_1)
//     };

//     Ok((
//         (deposit_amount_0, deposit_amount_1),
//         (remainder_0, remainder_1),
//     ))
// }

// this math is straight from the readme
pub fn get_single_sided_deposit_0_to_1_swap_amount(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    token0_balance: Uint128,
    lower_tick: i64,
    upper_tick: i64,
) -> Result<Uint128, ContractError> {
    let spot_price = Decimal256::from(get_spot_price(storage, querier)?);
    let lower_price = tick_to_price(lower_tick)?;
    let upper_price = tick_to_price(upper_tick)?;
    let pool_metadata_constant: Uint128 = spot_price
        .checked_mul(lower_price.sqrt())?
        .checked_mul(upper_price.sqrt())?
        .to_uint_floor() // todo: this is big, so should be safe, right?
        .try_into()?;

    let swap_amount = token0_balance.checked_multiply_ratio(
        pool_metadata_constant,
        pool_metadata_constant.checked_add(Uint128::one())?,
    )?;

    Ok(swap_amount)
}

pub fn get_single_sided_deposit_1_to_0_swap_amount(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    token1_balance: Uint128,
    lower_tick: i64,
    upper_tick: i64,
) -> Result<Uint128, ContractError> {
    let spot_price = Decimal256::from(get_spot_price(storage, querier)?);
    let lower_price = tick_to_price(lower_tick)?;
    let upper_price = tick_to_price(upper_tick)?;
    let pool_metadata_constant: Uint128 = spot_price
        .checked_mul(lower_price.sqrt())?
        .checked_mul(upper_price.sqrt())?
        .to_uint_floor() // todo: this is big, so should be safe, right?
        .try_into()?;

    let swap_amount =
        token1_balance.checked_div(pool_metadata_constant.checked_add(Uint128::one())?)?;

    Ok(swap_amount)
}

pub fn with_slippage(amount: Uint128, slippage: Decimal) -> Result<Uint128, ContractError> {
    let slippage_multiplier = Decimal::one().checked_sub(slippage)?;

    let adjusted_amount = amount.checked_multiply_ratio(
        slippage_multiplier.numerator(),
        slippage_multiplier.denominator(),
    )?;

    Ok(adjusted_amount)
}

/// This function compares the address of the message sender (caller) with the current admin
/// address stored in the state. This provides a convenient way to verify if the caller
/// is the admin in a single line.
pub fn assert_admin(deps: Deps, caller: &Addr) -> Result<Addr, ContractError> {
    if ADMIN_ADDRESS.load(deps.storage)? != caller {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(caller.clone())
    }
}

#[cfg(test)]
mod tests {

    use cosmwasm_std::{coin, Addr};

    use super::*;

    #[test]
    fn must_pay_two_works_ordered() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![expected0.clone(), expected1.clone()],
        };
        let (token0, token1) =
            must_pay_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!(expected0, token0);
        assert_eq!(expected1, token1);
    }

    #[test]
    fn must_pay_two_works_unordered() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![expected1.clone(), expected0.clone()],
        };
        let (token0, token1) =
            must_pay_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!(expected0, token0);
        assert_eq!(expected1, token1);
    }

    #[test]
    fn must_pay_two_rejects_three() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![expected1, expected0, coin(200, "uqsr")],
        };
        let _err = must_pay_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap_err();
    }

    #[test]
    fn must_pay_two_rejects_one() {
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![coin(200, "uqsr")],
        };
        let _err = must_pay_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap_err();
    }
}
