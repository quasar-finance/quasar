use std::cmp::{min, Ordering};
use std::str::FromStr;

use crate::debug;
use crate::math::liquidity::{asset0, asset1, liquidity0, liquidity1};
use crate::math::tick::tick_to_price;
use crate::rewards::CoinList;
use crate::state::{Position, ADMIN_ADDRESS, POSITIONS, STRATEGIST_REWARDS, USER_REWARDS};

use crate::vault::concentrated_liquidity::get_positions;
use crate::vault::range::move_position::SwapDirection;
use crate::{error::ContractResult, state::POOL_CONFIG, ContractError};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, Addr, CheckedMultiplyRatioError, Coin, Decimal, Decimal256, Deps, DepsMut, Env, Fraction,
    MessageInfo, Order, OverflowError, OverflowOperation, QuerierWrapper, Storage, Uint128,
    Uint256,
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

    get_one_or_two(&info.funds, denoms)
}

pub(crate) fn get_one_or_two(
    tokens: &Vec<Coin>,
    denoms: (String, String),
) -> ContractResult<(Coin, Coin)> {
    let token0 = tokens
    .clone()
    .into_iter()
    .find(|coin| coin.denom == denoms.0)
    .unwrap_or(coin(0, denoms.0));

    let token1 = tokens
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
    spot_price: Decimal,
    token0: Uint128,
    token1: Uint128,
) -> Result<Vec<(Position, Uint128, Uint128)>, ContractError> {
    // get the wanted ratio per position
    let ps = get_min_ratio_per_position(positions, spot_price)?;

    let (total0, total1) = ps.iter().try_fold(
        (Uint128::zero(), Uint128::zero()),
        |(acc0, acc1), (_, pr)| -> Result<(Uint128, Uint128), ContractError> {
            Ok((acc0.checked_add(pr.asset0)?, acc1.checked_add(pr.asset1)?))
        },
    )?;

    // assuming we have the ratio wanted_token0/total_token0 and wanted_token1/total_token1
    // ratio = min(wanted_token0/total_token0, wanted_token1/total_token1)
    // for token0 and token1, we want to allocate
    // allocate0 = token0 * ratio
    // allocate1 = token1 * ratio
    let psf: Result<Vec<(Position, Uint128, Uint128)>, ContractError> = ps
        .into_iter()
        .map(|(ps, ps_ratio)| {
            let ratio = min(
                Decimal::from_ratio(ps_ratio.asset0, total0),
                Decimal::from_ratio(ps_ratio.asset1, total1),
            );

            Ok((ps, token0 * ratio, token1 * ratio))
        })
        .collect();
    psf
}

/// per position, calculate the ratio of tokens that position needs, multiplied with the ratio of that position
/// within the vault
fn get_min_ratio_per_position(
    positions: Vec<(Position, FullPositionBreakdown)>,
    spot_price: Decimal,
) -> Result<Vec<(Position, PositionRatio)>, ContractError> {
    // per position, calculate the ratio of asset1 and asset2 a position needs

    // for each position, we take some set amount of liquidity and calculate how many asset0 and asset1 those would give us
    // we assume
    let liquidity: Decimal256 = Decimal::new(100_000_000_000_000_000_000_000_u128.into()).into();
    let ps: Result<Vec<(Position, PositionRatio)>, ContractError> = positions
        .into_iter()
        .map(|(p, fp)| {
            let pos = fp.position.unwrap();
            let upper_price_sqrt = tick_to_price(pos.upper_tick)?.sqrt();
            let lower_price_sqrt = tick_to_price(pos.lower_tick)?.sqrt();
            let spot_price_sqrt: Decimal256 = spot_price.sqrt().into();

            let amount0 =
                asset0(liquidity, spot_price_sqrt, upper_price_sqrt).unwrap_or(Uint128::zero());
            let amount1 =
                asset1(liquidity, lower_price_sqrt, spot_price_sqrt).unwrap_or(Uint128::zero());
            Ok((p, PositionRatio::new(amount0, amount1)))
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
    let positions: Result<Vec<(Position, PositionRatio)>, ContractError> = positions
        .into_iter()
        .map(|(p, internal_ratio)| {
            let external_ratio = Decimal::from_ratio(p.ratio, total_ratio);
            let total_ratio = internal_ratio.checked_mul_ratio(external_ratio)?;
            Ok((p, total_ratio))
        })
        .collect();
    positions
}

/// Calculate the total value of two assets in asset0
pub fn get_asset0_value(
    token0: Uint128,
    token1: Uint128,
    spot_price: Decimal,
) -> Result<Uint128, ContractError> {
    let total = token0
        .checked_add(token1.multiply_ratio(spot_price.denominator(), spot_price.numerator()))?;

    Ok(total)
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

#[cw_serde]
pub struct PositionRatio {
    pub asset0: Uint128,
    pub asset1: Uint128,
}

impl PositionRatio {
    pub fn new(asset0: Uint128, asset1: Uint128) -> PositionRatio {
        PositionRatio { asset0, asset1 }
    }

    pub fn simplify(&mut self) {
        // if either side is 0
        if self.asset0 >= Uint128::one() && self.asset1 == Uint128::zero() {
            self.asset0 = Uint128::one()
        }
        if self.asset0 == Uint128::zero() && self.asset1 >= Uint128::one() {
            self.asset1 = Uint128::one()
        }

        // try to simplify the ratio's between the values
        if self.asset0 > Uint128::one() && self.asset1 > Uint128::one() {
            // easiest way to simplify is using the decimal constructor and setting asset0 and asset1 to the numerator and denominator
            let d = Decimal::from_ratio(self.asset0, self.asset1);
            self.asset0 = d.numerator();
            self.asset1 = d.denominator();
        }
    }

    /// Creates a position ratio with zero on each sideUint128(0)
    #[inline]
    pub const fn zero() -> Self {
        Self {
            asset0: Uint128::zero(),
            asset1: Uint128::zero(),
        }
    }

    pub fn checked_mul(self, other: Self) -> Result<Self, OverflowError> {
        let asset0 = self.asset0.full_mul(other.asset0);
        let asset1 = self.asset1.full_mul(other.asset1);
        let pr = Self {
            asset0: asset0.try_into().map_err(|_| OverflowError {
                operation: OverflowOperation::Mul,
                operand1: self.asset0.to_string(),
                operand2: other.asset0.to_string(),
            })?,
            asset1: asset1.try_into().map_err(|_| OverflowError {
                operation: OverflowOperation::Mul,
                operand1: self.asset1.to_string(),
                operand2: other.asset1.to_string(),
            })?,
        };
        Ok(pr)
    }

    pub fn checked_mul_ratio(self, other: Decimal) -> Result<Self, CheckedMultiplyRatioError> {
        let asset0 = self
            .asset0
            .checked_multiply_ratio(other.numerator(), other.denominator())?;
        let asset1 = self
            .asset1
            .checked_multiply_ratio(other.numerator(), other.denominator())?;
        Ok(Self { asset0, asset1 })
    }

    pub fn checked_add(self, other: &Self) -> Result<Self, OverflowError> {
        let asset0 = self.asset0.checked_add(other.asset0)?;
        let asset1 = self.asset1.checked_add(other.asset1)?;
        let pr = Self { asset0, asset1 };
        Ok(pr)
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
