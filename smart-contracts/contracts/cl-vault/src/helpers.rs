use std::cmp::{min, Ordering};
use std::str::FromStr;

use crate::math::liquidity::{liquidity0, liquidity1};
use crate::math::tick::tick_to_price;
use crate::rewards::CoinList;
use crate::state::{Position, ADMIN_ADDRESS, POSITIONS, STRATEGIST_REWARDS, USER_REWARDS};

use crate::vault::concentrated_liquidity::get_positions;
use crate::{error::ContractResult, state::POOL_CONFIG, ContractError};
use cosmwasm_std::{
    coin, Addr, Coin, Decimal, Decimal256, Deps, DepsMut, Env, Fraction, MessageInfo, Order,
    QuerierWrapper, StdError, Storage, Uint128, Uint256,
};

use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::FullPositionBreakdown;
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;

/// returns the Coin of the needed denoms in the order given in denoms

pub(crate) fn must_pay_one_or_two(
    info: &MessageInfo,
    denoms: (String, String),
) -> ContractResult<(Coin, Coin)> {
    if info.funds.len() != 2 && info.funds.len() != 1 {
        return Err(ContractError::IncorrectAmountFunds);
    }

    let token0 = info
        .funds
        .clone()
        .into_iter()
        .find(|coin| coin.denom == denoms.0)
        .unwrap_or(coin(0, denoms.0));

    let token1 = info
        .funds
        .clone()
        .into_iter()
        .find(|coin| coin.denom == denoms.1)
        .unwrap_or(coin(0, denoms.1));

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
    token0_balance: Uint128,
    lower_tick: i64,
    current_tick: i64,
    upper_tick: i64,
) -> Result<Uint128, ContractError> {
    // TODO error here if this condition holds
    // if current_tick < lower_tick {
    //     return ;
    // }

    let lower_price = tick_to_price(lower_tick)?;
    let current_price = tick_to_price(current_tick)?;
    let upper_price = tick_to_price(upper_tick)?;

    let cur_price_sqrt = current_price.sqrt();
    let lower_price_sqrt = lower_price.sqrt();
    let upper_price_sqrt = upper_price.sqrt();

    // let pool_metadata_constant: Decimal256 = cur_price_sqrt
    //     .checked_mul(lower_price_sqrt)?
    //     .checked_mul(cur_price_sqrt.checked_sub(lower_price_sqrt)?)?
    //     .checked_div(upper_price_sqrt.checked_sub(cur_price_sqrt)?)?;

    let pool_metadata_constant: Decimal256 = (upper_price_sqrt
        .checked_mul(cur_price_sqrt)?
        .checked_mul(cur_price_sqrt.checked_sub(lower_price_sqrt)?))?
    .checked_div(upper_price_sqrt.checked_sub(cur_price_sqrt)?)?;

    let spot_price_over_pool_metadata_constant =
        current_price.checked_div(pool_metadata_constant)?;

    let denominator = Decimal256::one().checked_add(spot_price_over_pool_metadata_constant)?;

    let swap_amount: Uint128 = Uint256::from(token0_balance)
        .multiply_ratio(denominator.denominator(), denominator.numerator())
        .try_into()?;

    Ok(swap_amount)
}

pub fn get_single_sided_deposit_1_to_0_swap_amount(
    token1_balance: Uint128,
    lower_tick: i64,
    current_tick: i64,
    upper_tick: i64,
) -> Result<Uint128, ContractError> {
    let lower_price = tick_to_price(lower_tick)?;
    let current_price = tick_to_price(current_tick)?;
    let upper_price = tick_to_price(upper_tick)?;

    let cur_price_sqrt = current_price.sqrt();
    let lower_price_sqrt = lower_price.sqrt();
    let upper_price_sqrt = upper_price.sqrt();

    let pool_metadata_constant: Decimal256 = (upper_price_sqrt
        .checked_mul(cur_price_sqrt)?
        .checked_mul(cur_price_sqrt.checked_sub(lower_price_sqrt)?))?
    .checked_div(upper_price_sqrt.checked_sub(cur_price_sqrt)?)?;

    let pool_metadata_constant_over_spot_price: Decimal256 =
        pool_metadata_constant.checked_div(current_price)?;

    let denominator = Decimal256::one().checked_add(pool_metadata_constant_over_spot_price)?;

    let swap_amount: Uint128 = Uint256::from(token1_balance)
        .multiply_ratio(denominator.denominator(), denominator.numerator())
        .try_into()?;

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

pub fn round_up_to_nearest_multiple(amount: i64, multiple: i64) -> i64 {
    let remainder = amount % multiple;
    if remainder == 0 {
        amount
    } else if amount < 0 {
        amount - remainder
    } else {
        amount + multiple - remainder
    }
}

pub fn sort_tokens(tokens: Vec<Coin>) -> Vec<Coin> {
    let mut sorted_tokens = tokens;
    sorted_tokens.sort_by(|a, b| a.denom.cmp(&b.denom));
    sorted_tokens
}

/// this function subtracts out anything from the raw contract balance that isn't dedicated towards user or strategist rewards.
/// this function is expensive.
pub fn get_unused_balances(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    env: &Env,
) -> Result<CoinList, ContractError> {
    let mut balances =
        CoinList::from_coins(querier.query_all_balances(env.contract.address.to_string())?);

    // subtract out strategist rewards and all user rewards
    let strategist_rewards = STRATEGIST_REWARDS.load(storage)?;

    balances.sub(&strategist_rewards)?;

    for user_reward in USER_REWARDS.range(storage, None, None, cosmwasm_std::Order::Ascending) {
        balances.sub(&user_reward?.1)?;
    }

    Ok(balances)
}

pub fn get_max_utilization_for_ratio(
    token0: Uint256,
    token1: Uint256,
    ratio: Decimal256,
) -> Result<(Uint256, Uint256), ContractError> {
    // maxdep1 = T0 / R
    let max_deposit1_from_0 =
        token0.checked_multiply_ratio(ratio.denominator(), ratio.numerator())?;
    // maxdep0 = T1 * R
    let max_deposit0_from_1 =
        token1.checked_multiply_ratio(ratio.numerator(), ratio.denominator())?;

    if max_deposit0_from_1 > token0 {
        Ok((token0, max_deposit1_from_0))
    } else if max_deposit1_from_0 > token1 {
        Ok((max_deposit0_from_1, token1))
    } else {
        Ok((token0, token1))
    }
}

// TODO, do we allocate to top up
pub fn allocate_funds_per_position(
    positions: Vec<(Position, FullPositionBreakdown)>,
    asset0: Uint128,
    asset1: Uint128,
) -> Result<Vec<(Position, Uint128, Uint128)>, ContractError> {
    // get the wanted ratio per position
    let ps = get_min_ratio_per_position(positions)?;

    // divide the assets over the positions
    let total = get_total_ratio(&ps)?;

    // for token0, we want to deposit position_ratio.numerator/total.numerator * total_token_1
    // for token1, we want to deposit position_ratio.denominator/total.denominator
    let psf: Result<Vec<(Position, Uint128, Uint128)>, ContractError> = ps
        .into_iter()
        .map(|(ps, ps_ratio)| {
            Ok((
                ps,
                ps_ratio
                    .numerator()
                    .checked_div(total.numerator())?
                    .checked_mul(asset0)?,
                ps_ratio
                    .denominator()
                    .checked_div(total.denominator())?
                    .checked_mul(asset1)?,
            ))
        })
        .collect();
    psf
}

/// Calculate the total ratio after a set if positions
fn get_total_ratio(positions_ratios: &Vec<(Position, Decimal)>) -> Result<Decimal, ContractError> {
    let total = positions_ratios
        .iter()
        .try_fold(Decimal::zero(), |acc, (_, r)| acc.checked_add(*r))?;
    Ok(total)
}

/// per position, calculate the ratio of tokens that position needs, multiplied with the ratio of that position
/// within the vault
fn get_min_ratio_per_position(
    positions: Vec<(Position, FullPositionBreakdown)>,
) -> Result<Vec<(Position, Decimal)>, ContractError> {
    // per position, calculate the ratio of asset1 and asset2 a position needs
    // TODO this should use the current price and the ranges instead of token, since a position might contain a small amount of tokens
    let ps: Result<Vec<(Position, Decimal)>, ContractError> = positions
        .into_iter()
        .map(|(p, fp)| {
            let amount0 = fp
                .asset0
                .map(|c| -> Result<Uint128, ContractError> { Ok(c.amount.parse()?) })
                .unwrap_or(Ok(Uint128::zero()))?;
            let amount1 = fp
                .asset1
                .map(|c| -> Result<Uint128, ContractError> { Ok(c.amount.parse()?) })
                .unwrap_or(Ok(Uint128::zero()))?;
            Ok((p, Decimal::from_ratio(amount0, amount1)))
        })
        .collect();

    let positions = ps?;

    let total_ratio = positions
        .iter()
        .fold(Uint128::zero(), |acc, (p, _)| acc + p.ratio);

    // now that we know how much tokens a positions internal ratio needs, we need to normalize these internal ratios to eachother using the positions ratios
    // Each position might get ratio/total_ratio of tokens. We want to find the effective ratio for each position then.
    // we multiply the position's internal ratio by the positions external ratio
    // The external ratio is the positions ratio divided by the total ratio of all positions
    let positions: Result<Vec<(Position, Decimal)>, ContractError> = positions
        .into_iter()
        .map(|(p, internal_ratio)| {
            let external_ratio = Decimal::from_ratio(p.ratio, total_ratio);
            let total_ratio = internal_ratio.checked_mul(external_ratio)?;
            Ok((p, total_ratio))
        })
        .collect();
    positions
}

/// get_liquidity_amount_for_unused_funds basically simulates an any deposit against the vault
pub fn get_liquidity_amount_for_unused_funds(
    deps: DepsMut,
    env: &Env,
    token0: Coin,
    token1: Coin,
    additional_excluded_funds: (Uint128, Uint128),
    spot_price: Decimal256,
) -> Result<Decimal256, ContractError> {
    let tokens = get_unused_balances(deps.storage, &deps.querier, env)?;
    let unused_t0 = tokens
        .find_coin(token0.denom)
        .amount
        .checked_sub(additional_excluded_funds.0)?
        .into();
    let unused_t1 = tokens
        .find_coin(token1.denom)
        .amount
        .checked_sub(additional_excluded_funds.1)?
        .into();

    let double_bleh = get_positions(deps.storage, &deps.querier)?;
    let bleh = get_min_ratio_per_position(double_bleh.clone())?;
    let total_ratio = get_total_ratio(&bleh)?;

    // using the spot price, convert our unused_t0 and unused_t1 to funds in the correct ratio of our current positions
    let current_ratio = Decimal256::from_ratio(unused_t0, unused_t1);

    // for now call token0 x and token1 y, we are going to calculate which amounts we have per token assuming a perfect swap before we calculate the liquidity resulting from that
    // when calculating a swap from x to y, we calculate x''=x/((1+P_c*a)/b)
    // knowing x'', we can simply calculate x'/y, since if we swap x'', x'/y=a/b
    // when calculating a swap from y to x, we calculate y''=a*P_c*y/(b+1)
    // if current_ratio > total_ratio, we need to swap token0 to token1, if current_ratio < total_ratio, we swap token1 to token0

    // our formulas assume that we have no assets of the other token outstanding. Since we are converting using perfect swaps anyway, we can just convert all of token0 to token1 and vice versa when needed and do the math

    let (t0, t1): (Uint128, Uint128) = match current_ratio.cmp(&total_ratio.into()) {
        Ordering::Less => {
            let x = Decimal256::from_ratio(unused_t0, 1_u128)
                + Decimal256::from_ratio(unused_t1, 1_u128) / spot_price;
            let x_to_swap = x
                / ((Decimal256::one()
                    + spot_price * Decimal256::from_ratio(total_ratio.numerator(), 1_u128))
                    / Decimal256::from_ratio(total_ratio.denominator(), 1_u128));
            let leftover_x = x - x_to_swap;
            (
                leftover_x.to_uint_floor().try_into()?,
                (x_to_swap * spot_price).to_uint_floor().try_into()?,
            )
        } //swap token1 to token0
        Ordering::Equal => (unused_t0, unused_t1), // no swap is needed
        Ordering::Greater => {
            let y = Decimal256::from_ratio(unused_t1, 1_u128)
                + Decimal256::from_ratio(unused_t0, 1_u128) * spot_price;
            let y_to_swap = Decimal256::from_ratio(total_ratio.numerator(), 1_u128)
                * spot_price
                * y
                / (Decimal256::from_ratio(total_ratio.denominator(), 1_u128) + Decimal256::one());
            let leftover_y = y - y_to_swap;
            (
                leftover_y.to_uint_floor().try_into()?,
                (y_to_swap / spot_price).to_uint_floor().try_into()?,
            )
        } //swap token0 to token1
    };

    // calculate the liquidity
    let liquidity = get_liquidity_for_positions(double_bleh, spot_price, t0, t1)?;

    Ok(liquidity)
}

/// given a set of positions with external ratios between the positions, calculate the total liquidity gained by depositing asset01 and asset1 into this set of positions
pub fn get_liquidity_for_positions(
    positions: Vec<(Position, FullPositionBreakdown)>,
    spot_price: Decimal256,
    asset0: Uint128,
    asset1: Uint128,
) -> Result<Decimal256, ContractError> {
    let ps = allocate_funds_per_position(positions.clone(), asset0, asset1)?;

    let total_liquidity = ps
        .into_iter()
        .zip(positions)
        .map(|((p1, asset0, asset1), (p2, fp))| {
            if p1.position_id != p2.position_id {
                todo!()
            }

            get_liquidity_for_position(fp, spot_price, asset0, asset1)
        })
        .try_fold(
            Decimal256::zero(),
            |acc, val| -> Result<Decimal256, ContractError> { Ok(acc.checked_add(val?)?) },
        )?;

    Ok(total_liquidity)
}

/// given a position, a spot price, calculate the max amount of liquidity that can be used by that position
fn get_liquidity_for_position(
    position: FullPositionBreakdown,
    spot_price: Decimal256,
    asset0: Uint128,
    asset1: Uint128,
) -> Result<Decimal256, ContractError> {
    let p = position.position.unwrap();
    let lower_price = tick_to_price(p.lower_tick)?;
    let upper_price = tick_to_price(p.upper_tick)?;

    // decide if our position is onesided,
    // if so return liquidity of that side
    // else return the min liquidity of liquidity0 and liquidity1
    //
    if spot_price < lower_price {
        liquidity1(
            Decimal256::from_ratio(asset1, 1_u128),
            lower_price.sqrt(),
            spot_price.sqrt(),
        )
    } else if upper_price < spot_price {
        liquidity0(
            Decimal256::from_ratio(asset0, 1_u128),
            spot_price.sqrt(),
            upper_price.sqrt(),
        )
    } else {
        // liq0 ranges from spot price to upper price
        let liq0 = liquidity0(
            Decimal256::from_ratio(asset0, 1_u128),
            spot_price.sqrt(),
            upper_price.sqrt(),
        )?;
        let liq1 = liquidity1(
            Decimal256::from_ratio(asset1, 1_u128),
            lower_price.sqrt(),
            spot_price.sqrt(),
        )?;
        Ok(min(liq0, liq1))
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use cosmwasm_std::{coin, testing::mock_dependencies, Addr};

    use crate::math::tick::price_to_tick;

    use super::*;

    #[test]
    fn must_pay_one_or_two_works_ordered() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![expected0.clone(), expected1.clone()],
        };
        let (token0, token1) =
            must_pay_one_or_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!(expected0, token0);
        assert_eq!(expected1, token1);
    }

    #[test]
    fn must_pay_one_or_two_works_unordered() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![expected1.clone(), expected0.clone()],
        };
        let (token0, token1) =
            must_pay_one_or_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!(expected0, token0);
        assert_eq!(expected1, token1);
    }

    #[test]
    fn must_pay_one_or_two_rejects_three() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![expected1, expected0, coin(200, "uqsr")],
        };
        let _err =
            must_pay_one_or_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap_err();
    }

    #[test]
    fn must_pay_one_or_two_accepts_second_token() {
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![coin(200, "uosmo")],
        };
        let res = must_pay_one_or_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!((coin(0, "uatom"), coin(200, "uosmo")), res)
    }

    #[test]
    fn must_pay_one_or_two_accepts_first_token() {
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![coin(200, "uatom")],
        };
        let res = must_pay_one_or_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!((coin(200, "uatom"), coin(0, "uosmo")), res)
    }

    #[test]
    fn test_0_to_1_swap() {
        let mut deps = mock_dependencies();

        let lower_price = "4500";
        let upper_price = "5500";
        let token0amt = 200000u128;

        // prices and expected amounts taken from https://docs.google.com/spreadsheets/d/1xPsKsQkM0apTZQPBBwVlEyB5Sk31sw6eE8U0FgnTWUQ/edit?usp=sharing
        let mut prices = HashMap::new();
        prices.insert("4501", Uint128::new(232));
        prices.insert("4600", Uint128::new(22674));
        prices.insert("4700", Uint128::new(44304));
        prices.insert("4800", Uint128::new(65099));
        prices.insert("4900", Uint128::new(85241));
        prices.insert("5000", Uint128::new(104884));
        prices.insert("5100", Uint128::new(124166));
        prices.insert("5200", Uint128::new(143210));
        prices.insert("5300", Uint128::new(162128));
        prices.insert("5400", Uint128::new(181025));
        prices.insert("5499", Uint128::new(199809));

        let lower_tick = price_to_tick(
            deps.as_mut().storage,
            Decimal256::from_str(lower_price).unwrap(),
        )
        .unwrap()
        .try_into()
        .unwrap();

        let upper_tick = price_to_tick(
            deps.as_mut().storage,
            Decimal256::from_str(upper_price).unwrap(),
        )
        .unwrap()
        .try_into()
        .unwrap();

        for (price, result) in prices.into_iter() {
            let curr_tick =
                price_to_tick(deps.as_mut().storage, Decimal256::from_str(price).unwrap())
                    .unwrap()
                    .try_into()
                    .unwrap();

            let swap_amount = get_single_sided_deposit_0_to_1_swap_amount(
                token0amt.into(),
                lower_tick,
                curr_tick,
                upper_tick,
            )
            .unwrap();

            assert_eq!(swap_amount, result);
        }
    }

    #[test]
    fn test_1_to_0_swap() {
        let mut deps = mock_dependencies();

        let lower_price = "4500";
        let upper_price = "5500";

        let token1amt = 200000u128;

        // prices and expected amounts taken from https://docs.google.com/spreadsheets/d/1xPsKsQkM0apTZQPBBwVlEyB5Sk31sw6eE8U0FgnTWUQ/edit?usp=sharing
        let mut prices = HashMap::new();
        prices.insert("4501", Uint128::new(199767));
        prices.insert("4600", Uint128::new(177325));
        prices.insert("4700", Uint128::new(155695));
        prices.insert("4800", Uint128::new(134900));
        prices.insert("4900", Uint128::new(114758));
        prices.insert("5000", Uint128::new(95115));
        prices.insert("5100", Uint128::new(75833));
        prices.insert("5200", Uint128::new(56789));
        prices.insert("5300", Uint128::new(37871));
        prices.insert("5400", Uint128::new(18974));
        prices.insert("5499", Uint128::new(190));

        let lower_tick = price_to_tick(
            deps.as_mut().storage,
            Decimal256::from_str(lower_price).unwrap(),
        )
        .unwrap()
        .try_into()
        .unwrap();

        let upper_tick: i64 = price_to_tick(
            deps.as_mut().storage,
            Decimal256::from_str(upper_price).unwrap(),
        )
        .unwrap()
        .try_into()
        .unwrap();

        for (price, result) in prices.into_iter() {
            let curr_tick =
                price_to_tick(deps.as_mut().storage, Decimal256::from_str(price).unwrap())
                    .unwrap()
                    .try_into()
                    .unwrap();

            let swap_amount = get_single_sided_deposit_1_to_0_swap_amount(
                token1amt.into(),
                lower_tick,
                curr_tick,
                upper_tick,
            )
            .unwrap();

            assert_eq!(swap_amount, result);
        }
    }

    #[test]
    fn test_round_up_to_nearest_multiple() {
        assert_eq!(round_up_to_nearest_multiple(10, 5), 10);
        assert_eq!(round_up_to_nearest_multiple(11, 5), 15);
        assert_eq!(round_up_to_nearest_multiple(12, 5), 15);
        assert_eq!(round_up_to_nearest_multiple(13, 5), 15);
        assert_eq!(round_up_to_nearest_multiple(14, 5), 15);
        assert_eq!(round_up_to_nearest_multiple(15, 5), 15);
        assert_eq!(round_up_to_nearest_multiple(16, 5), 20);
        assert_eq!(round_up_to_nearest_multiple(17, 5), 20);
        assert_eq!(round_up_to_nearest_multiple(18, 5), 20);
        assert_eq!(round_up_to_nearest_multiple(19, 5), 20);
        assert_eq!(round_up_to_nearest_multiple(20, 5), 20);
        // does it also work for negative inputs?
        assert_eq!(round_up_to_nearest_multiple(-10, 5), -10);
        assert_eq!(round_up_to_nearest_multiple(-11, 5), -10);
        assert_eq!(round_up_to_nearest_multiple(-12, 5), -10);
        assert_eq!(round_up_to_nearest_multiple(-13, 5), -10);
        assert_eq!(round_up_to_nearest_multiple(-14, 5), -10);
        assert_eq!(round_up_to_nearest_multiple(-15, 5), -15);
        assert_eq!(round_up_to_nearest_multiple(-16, 5), -15);
        assert_eq!(round_up_to_nearest_multiple(-17, 5), -15);
        assert_eq!(round_up_to_nearest_multiple(-18, 5), -15);
        assert_eq!(round_up_to_nearest_multiple(-19, 5), -15);
        assert_eq!(round_up_to_nearest_multiple(-20, 5), -20);
    }

    #[test]
    fn test_sort_tokens() {
        let tokens = vec![
            coin(100, "uatom"),
            coin(200, "uosmo"),
            coin(300, "uqsr"),
            coin(400, "ueth"),
        ];

        let expected = vec![
            coin(100, "uatom"),
            coin(400, "ueth"),
            coin(200, "uosmo"),
            coin(300, "uqsr"),
        ];

        let sorted_tokens = sort_tokens(tokens);

        assert_eq!(sorted_tokens, expected);
    }
}
