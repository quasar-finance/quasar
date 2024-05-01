use std::cmp::min;
use std::str::FromStr;

use crate::math::liquidity::{asset0, asset1};
use crate::math::tick::tick_to_price;
use crate::rewards::CoinList;
use crate::state::{Position, ADMIN_ADDRESS, STRATEGIST_REWARDS, USER_REWARDS};

use crate::{error::ContractResult, state::POOL_CONFIG, ContractError};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, Addr, CheckedMultiplyRatioError, Coin, Decimal, Decimal256, Deps, DepsMut, Env, Fraction,
    MessageInfo, OverflowError, OverflowOperation, QuerierWrapper, Storage, Uint128, Uint256,
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
    tokens: &[Coin],
    denoms: (String, String),
) -> ContractResult<(Coin, Coin)> {
    let token0 = tokens
        .iter()
        .find(|coin| coin.denom == denoms.0)
        .cloned()
        .unwrap_or(coin(0, denoms.0));

    let token1 = tokens
        .iter()
        .find(|coin| coin.denom == denoms.1)
        .cloned()
        .unwrap_or(coin(0, denoms.1));

    Ok((token0, token1))
}

pub(crate) fn get_one_or_two_coins(
    tokens: &[Coin],
    denoms: (String, String),
) -> ContractResult<Vec<Coin>> {
    let (token0, token1) = get_one_or_two(tokens, denoms)?;

    let mut tokens = vec![];

    if token0.amount > Uint128::zero() {
        tokens.push(token0)
    }

    if token1.amount > Uint128::zero() {
        tokens.push(token1)
    }

    if tokens.is_empty() {
        return Err(ContractError::IncorrectAmountFunds);
    }

    Ok(tokens)
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
    if amount.abs() < multiple {
        return 0; // Return 0 when amount is less than multiple
    }

    let remainder = amount % multiple;
    if remainder == 0 {
        amount // Amount is already a multiple of 'multiple'
    } else if amount < 0 {
        amount + remainder.abs() // Round down to the nearest multiple
    } else {
        amount + (multiple - remainder) // Round up to the nearest multiple
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

// TODO, allocating funds gives a ratio for each position, we should theb
pub fn allocate_funds_per_position(
    _deps: DepsMut,
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
    // afterwards we need to normalize all ratios to a 1 total
    let ratios = ps.into_iter().map(|(ps, ps_ratio)| {
        let r0 = Decimal::from_ratio(ps_ratio.asset0, total0);
        let r1 = Decimal::from_ratio(ps_ratio.asset1, total1);
        let ratio = min(r0, r1);

        (ps, ratio)
    });

    let total_ratio = ratios
        .clone()
        .fold(Decimal::zero(), |acc, (_, ratio)| acc + ratio);

    let psf: Result<Vec<(Position, Uint128, Uint128)>, ContractError> = ratios
        .into_iter()
        .map(|(ps, ratio)| {
            let normalized = ratio / total_ratio;

            Ok((ps, token0 * normalized, token1 * normalized))
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

    // now that we know how much tokens a positions internal ratio needs, we need to normalize these internal ratios to eachother using the positions ratios
    // Each position might get ratio/total_ratio of tokens. We want to find the effective ratio for each position then.
    // we multiply the position's internal ratio by the positions external ratio
    // The external ratio is the positions ratio divided by the total ratio of all positions
    let positions = get_final_ratio(positions);
    positions
}

/// get the final ratio of a set of positions by multiplying the internal ratio of a position with the external ratio
/// that that position has in the total set of positions
fn get_final_ratio(
    positions: Vec<(Position, PositionRatio)>,
) -> Result<Vec<(Position, PositionRatio)>, ContractError> {
    let total_ratio = positions
        .iter()
        .fold(Uint128::zero(), |acc, (p, _)| acc + p.ratio);

    let positions: Result<Vec<(Position, PositionRatio)>, ContractError> = positions
        .into_iter()
        .map(|(p, internal_ratio)| {
            let external_ratio = Decimal::from_ratio(p.ratio, total_ratio);
            let effective_ratio = internal_ratio.checked_mul_ratio(external_ratio)?;
            Ok((p, effective_ratio))
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

    use cosmwasm_std::{assert_approx_eq, coin, testing::mock_dependencies, Addr};

    use crate::math::tick::price_to_tick;

    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::Position as OsmoPosition;

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
        assert_eq!(round_up_to_nearest_multiple(2, 10), 0);
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
        assert_eq!(round_up_to_nearest_multiple(-2, 10), 0);
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

    // Helper function to create a Position with a given id and ratio
    fn create_position(id: u64, ratio: u128) -> Position {
        Position {
            position_id: id,
            ratio: Uint128::new(ratio),
        }
    }

    // Helper function to create a PositionRatio
    fn create_ratio(asset0: u128, asset1: u128) -> PositionRatio {
        PositionRatio::new(Uint128::new(asset0), Uint128::new(asset1))
    }

    #[test]
    fn test_get_final_ratio_two_positions() {
        let token0 = 150_000;
        let token1 = 300_000;

        let positions = vec![
            (create_position(0, 100), create_ratio(token0, token1)),
            (create_position(1, 200), create_ratio(token0, token1)),
        ];
        let result = get_final_ratio(positions).expect("Should not fail");

        // Check if the result is as expected, considering the logic of your get_final_ratio function
        // This is a placeholder check, you need to adjust it based on the expected behavior
        assert_eq!(result.len(), 2);
        let position_0 = result.get(0).unwrap();
        let position_1 = result.get(1).unwrap();

        // we should account for off by 1 here, since we round down in the final ratio
        // position 0 should have 1/3 of token0 and token1
        assert_approx_eq!(position_0.1.asset0, (token0 / 3).into(), "0.00002");
        assert_approx_eq!(position_0.1.asset1, (token1 / 3).into(), "0.00002");

        // position 1 should have 2/3 of token0 and token1
        assert_approx_eq!(position_1.1.asset1, (token1 * 2 / 3).into(), "0.00002");
        assert_approx_eq!(position_1.1.asset1, (token1 * 2 / 3).into(), "0.00002");
    }

    #[test]
    fn test_get_final_ratio_three_positions() {
        let token0 = 150_000;
        let token1 = 300_000;

        let positions = vec![
            (create_position(0, 100), create_ratio(token0, token1)),
            (create_position(1, 200), create_ratio(token0, token1)),
            (create_position(1, 200), create_ratio(token0, token1)),
        ];
        let result = get_final_ratio(positions).expect("Should not fail");

        // Check if the result is as expected, considering the logic of your get_final_ratio function
        // This is a placeholder check, you need to adjust it based on the expected behavior
        assert_eq!(result.len(), 3);
        let position_0 = result.get(0).unwrap();
        let position_1 = result.get(1).unwrap();

        // we should account for off by 1 here, since we round down in the final ratio
        // position 0 should have 1/5 of token0 and token1
        assert_approx_eq!(position_0.1.asset0, (token0 / 5).into(), "0.00002");
        assert_approx_eq!(position_0.1.asset1, (token1 / 5).into(), "0.00002");

        // position 1 should have 2/5 of token0 and token1
        assert_approx_eq!(position_1.1.asset1, (token1 * 2 / 5).into(), "0.00002");
        assert_approx_eq!(position_1.1.asset1, (token1 * 2 / 5).into(), "0.00002");

        // position 1 should have 2/5 of token0 and token1
        assert_approx_eq!(position_1.1.asset1, (token1 * 2 / 5).into(), "0.00002");
        assert_approx_eq!(position_1.1.asset1, (token1 * 2 / 5).into(), "0.00002");
    }

    fn osmocoin(amount: u128, denom: &str) -> OsmoCoin {
        OsmoCoin {
            denom: denom.to_string(),
            amount: amount.to_string(),
        }
    }

    #[test]
    fn test_get_min_ratio_per_position_single() {
        // test edge cases such as tokens where one has 6 decimlas and the other 18:
        let deps = mock_dependencies();

        let positions = vec![(
            // one normal position
            Position {
                position_id: 0,
                ratio: Uint128::new(50),
            },
            FullPositionBreakdown {
                position: Some(OsmoPosition {
                    position_id: 0,
                    address: "smart contract address".to_string(),
                    pool_id: 0,
                    lower_tick: -100,
                    upper_tick: 100,
                    join_time: None,
                    liquidity: "100000000".to_string(),
                }),
                asset0: Some(osmocoin(100, "token0")),
                asset1: Some(osmocoin(100, "token1")),
                claimable_spread_rewards: vec![],
                claimable_incentives: vec![],
                forfeited_incentives: vec![],
            },
        )];

        let spot_price = Decimal::from_ratio(1u128, 1_000_000u128);

        let mut result = get_min_ratio_per_position(positions, spot_price).unwrap();

        println!("{:?}", tick_to_price(-100));
        println!("{:?}", tick_to_price(100));

        println!("{:?}", result);
        result[0].1.simplify();
        println!("{:?}", result);

        assert!(result.len() == 2);
    }
}
// [(Position { position_id: 0, ratio: Uint128(50) }, PositionRatio { asset0: Uint128(998045953488830495266994297546062752), asset1: Uint128(1000049998750062496000000000000000) })]
// thread 'helpers::tests::test_get_min_ratio_per_position_single' panicked at 'assertion failed: result.len() == 2', contracts/cl-vault/src/helpers.rs:871:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// test helpers::tests::test_get_min_ratio_per_position_single ... FAILED
// [(Position { position_id: 0, ratio: Uint128(50) }, PositionRatio { asset0: Uint128(99900004), asset1: Uint128(100100) })]
// thread 'helpers::tests::test_get_min_ratio_per_position_single' panicked at 'assertion failed: result.len() == 2', contracts/cl-vault/src/helpers.rs:873:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// test helpers::tests::test_get_min_ratio_per_position_single ... FAILED
