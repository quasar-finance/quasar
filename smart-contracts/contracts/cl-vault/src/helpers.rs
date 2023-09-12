use std::str::FromStr;

use crate::state::ADMIN_ADDRESS;
use crate::{error::ContractResult, state::POOL_CONFIG, ContractError};
use cosmwasm_std::{
    coin, Addr, Coin, Decimal, Deps, DepsMut, Fraction, MessageInfo, QuerierWrapper, Storage,
    Uint128,
};
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
    _deps: DepsMut,
    token0_balance: Uint128,
    lower_tick: i64,
    current_tick: i64,
    upper_tick: i64,
) -> Result<Uint128, ContractError> {
    if current_tick < lower_tick {
        return Err(ContractError::InvalidCurrentTick {}); // error for 0% swap
    }
    if current_tick > upper_tick {
        return Ok(token0_balance); // swap 100% of token0
    }

    let precision = 1_000_000u128; // six decimal of precision, TODO: remove this and convert types properly

    let current_less_lower = current_tick.checked_sub(lower_tick).unwrap() as u128;
    let upper_less_lower = upper_tick.checked_sub(lower_tick).unwrap() as u128;

    let factor = (upper_less_lower
        .checked_sub(current_less_lower)
        .unwrap()
        .checked_mul(precision)
        .unwrap())
    .checked_div(upper_less_lower)
    .unwrap();

    let swap_amount = token0_balance.checked_mul(Uint128::new(factor)).unwrap();
    let final_swap_amount = swap_amount.checked_div(Uint128::new(precision)).unwrap();

    Ok(final_swap_amount)
}

pub fn get_single_sided_deposit_1_to_0_swap_amount(
    _deps: DepsMut,
    token1_balance: Uint128,
    lower_tick: i64,
    current_tick: i64,
    upper_tick: i64,
) -> Result<Uint128, ContractError> {
    if current_tick < lower_tick {
        return Ok(token1_balance); // swap 100% of token1
    }
    if current_tick > upper_tick {
        return Err(ContractError::InvalidCurrentTick {}); // error for 0% swap
    }

    let precision = 1_000_000u128; // six decimal of precision, TODO: remove this and convert types properly

    let current_less_lower = current_tick.checked_sub(lower_tick).unwrap() as u128;
    let upper_less_lower = upper_tick.checked_sub(lower_tick).unwrap() as u128;

    let factor = (current_less_lower.checked_mul(precision).unwrap())
        .checked_div(upper_less_lower)
        .unwrap();

    let swap_amount = token1_balance.checked_mul(Uint128::new(factor)).unwrap();
    let final_swap_amount = swap_amount.checked_div(Uint128::new(precision)).unwrap();

    Ok(final_swap_amount)
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

#[cfg(test)]
mod tests {

    use cosmwasm_std::{coin, testing::mock_dependencies, Addr};
    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
        FullPositionBreakdown, Position,
    };

    use crate::{state::PoolConfig, test_helpers::QuasarQuerier};

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

    fn test_get_single_sided_deposit_0_to_1_swap_amount() {
        // Common setup
        let mut deps = mock_dependencies();
        let position = FullPositionBreakdown {
            position: Some(Position {
                position_id: 1,
                address: "some".to_string(),
                pool_id: 1,
                lower_tick: 100,
                upper_tick: 2000,
                join_time: None,
                liquidity: "12317361863813".to_string(),
            }),
            asset0: Some(Coin::new(1_000_000, "uatom").into()),
            asset1: Some(Coin::new(1_000_000, "uosmo").into()),
            claimable_spread_rewards: vec![Coin::new(1_000_000, "uosmo").into()], // not relevant
            claimable_incentives: vec![Coin::new(1_000_000, "uosmo").into()],     // not relevant
            forfeited_incentives: vec![Coin::new(1_000_000, "uosmo").into()],     // not relevant
        };
        let token0_balance = Uint128::new(1_000_000); // User balance

        // Mock PoolConfig
        POOL_CONFIG
            .save(
                deps.as_mut().storage,
                &PoolConfig {
                    pool_id: 1,
                    token0: "uatom".to_string(),
                    token1: "uosmo".to_string(),
                },
            )
            .unwrap();

        // Test case 1: current tick is the lowest
        let mut current_tick = 100;
        let querier1 = QuasarQuerier::new(position.clone(), current_tick);
        let qw1 = QuerierWrapper::new(&querier1);
        let mut deps_mut1 = deps.as_mut();
        deps_mut1.querier = qw1;

        let swap_amount1 = get_single_sided_deposit_0_to_1_swap_amount(
            deps_mut1,
            token0_balance,
            100,
            current_tick,
            2000,
        )
        .unwrap();
        assert_eq!(swap_amount1, Uint128::new(1000000));

        // Test case 2: current tick is within the range
        current_tick = 1050;
        let querier2 = QuasarQuerier::new(position.clone(), current_tick);
        let qw2 = QuerierWrapper::new(&querier2);
        let mut deps_mut2 = deps.as_mut();
        deps_mut2.querier = qw2;

        let swap_amount2 = get_single_sided_deposit_0_to_1_swap_amount(
            deps_mut2,
            token0_balance,
            100,
            current_tick,
            2000,
        )
        .unwrap();
        assert_eq!(swap_amount2, Uint128::new(500000));

        // Test case 3: current tick is the highest
        current_tick = 2000;
        let querier3 = QuasarQuerier::new(position, current_tick);
        let qw3 = QuerierWrapper::new(&querier3);
        let mut deps_mut3 = deps.as_mut();
        deps_mut3.querier = qw3;

        let swap_amount3 = get_single_sided_deposit_0_to_1_swap_amount(
            deps_mut3,
            token0_balance,
            100,
            current_tick,
            2000,
        )
        .unwrap();
        assert_eq!(swap_amount3, Uint128::new(0));
    }

    #[test]
    fn test_get_single_sided_deposit_1_to_0_swap_amount() {
        // Common setup
        let mut deps = mock_dependencies();
        let position = FullPositionBreakdown {
            position: Some(Position {
                position_id: 1,
                address: "some".to_string(),
                pool_id: 1,
                lower_tick: 100,
                upper_tick: 2000,
                join_time: None,
                liquidity: "12317361863813".to_string(),
            }),
            asset0: Some(Coin::new(1_000_000, "uatom").into()),
            asset1: Some(Coin::new(1_000_000, "uosmo").into()),
            claimable_spread_rewards: vec![Coin::new(1_000_000, "uosmo").into()], // not relevant
            claimable_incentives: vec![Coin::new(1_000_000, "uosmo").into()],     // not relevant
            forfeited_incentives: vec![Coin::new(1_000_000, "uosmo").into()],     // not relevant
        };
        let token1_balance = Uint128::new(1_000_000); // User balance

        // Mock PoolConfig
        POOL_CONFIG
            .save(
                deps.as_mut().storage,
                &PoolConfig {
                    pool_id: 1,
                    token0: "uatom".to_string(),
                    token1: "uosmo".to_string(),
                },
            )
            .unwrap();

        // Test case 1: current tick is the highest
        let mut current_tick = 2000;
        let querier1 = QuasarQuerier::new(position.clone(), current_tick);
        let qw1 = QuerierWrapper::new(&querier1);
        let mut deps_mut1 = deps.as_mut();
        deps_mut1.querier = qw1;

        let swap_amount1 = get_single_sided_deposit_1_to_0_swap_amount(
            deps_mut1,
            token1_balance,
            100,
            current_tick,
            2000,
        )
        .unwrap();
        assert_eq!(swap_amount1, Uint128::new(1000000));

        // Test case 2: current tick is within the range
        current_tick = 1050;
        let querier2 = QuasarQuerier::new(position.clone(), current_tick);
        let qw2 = QuerierWrapper::new(&querier2);
        let mut deps_mut2 = deps.as_mut();
        deps_mut2.querier = qw2;

        let swap_amount2 = get_single_sided_deposit_1_to_0_swap_amount(
            deps_mut2,
            token1_balance,
            100,
            current_tick,
            2000,
        )
        .unwrap();
        assert_eq!(swap_amount2, Uint128::new(500000));

        // Test case 3: current tick is the lowest
        current_tick = 100;
        let querier3 = QuasarQuerier::new(position, current_tick);
        let qw3 = QuerierWrapper::new(&querier3);
        let mut deps_mut3 = deps.as_mut();
        deps_mut3.querier = qw3;

        let swap_amount3 = get_single_sided_deposit_1_to_0_swap_amount(
            deps_mut3,
            token1_balance,
            100,
            current_tick,
            2000,
        )
        .unwrap();
        assert_eq!(swap_amount3, Uint128::new(0));
    }
}
