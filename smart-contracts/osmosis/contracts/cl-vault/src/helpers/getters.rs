use std::cmp::min;
use std::str::FromStr;

use osmosis_std::shim::Timestamp as OsmoTimestamp;
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use osmosis_std::types::osmosis::twap::v1beta1::TwapQuerier;

use crate::vault::concentrated_liquidity::get_position;
use crate::{
    helpers::assert::must_pay_one_or_two,
    math::tick::tick_to_price,
    state::{PoolConfig, POOL_CONFIG, RANGE_ADMIN},
    ContractError,
};
use cosmwasm_std::{
    coin, Addr, Coin, Decimal, Decimal256, Deps, DepsMut, Env, QuerierWrapper, Storage, Timestamp,
    Uint128, Uint256,
};

use super::coinlist::CoinList;

pub fn get_range_admin(deps: Deps) -> Result<Addr, ContractError> {
    Ok(RANGE_ADMIN.load(deps.storage)?)
}

/// Calculate the total value of two assets in asset0.
pub fn get_asset0_value(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    token0: Uint128,
    token1: Uint128,
) -> Result<Uint128, ContractError> {
    let pool_config = POOL_CONFIG.load(storage)?;

    let pm_querier = PoolmanagerQuerier::new(querier);
    let spot_price: Decimal = pm_querier
        .spot_price(pool_config.pool_id, pool_config.token0, pool_config.token1)?
        .spot_price
        .parse()?;

    let total = token0.checked_add(token1.checked_div_floor(spot_price)?)?;

    Ok(total)
}

pub fn get_asset_ratios(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
) -> Result<(Decimal, Decimal), ContractError> {
    let position = get_position(storage, querier)?;
    let asset0_amount = Uint128::from_str(&position.clone().asset0.unwrap_or_default().amount)?;
    let asset1_amount = Uint128::from_str(&position.clone().asset1.unwrap_or_default().amount)?;

    // Handle cases where either asset amount is zero
    if asset0_amount.is_zero() && asset1_amount.is_zero() {
        return Ok((Decimal::zero(), Decimal::zero()));
    }

    // Get the total amount of the vault's position in asset0 denom
    let asset_0_value = get_asset0_value(storage, querier, asset0_amount, asset1_amount)?;

    // Calculate the ratio of the vault's position in asset0 and asset1 using Decimal for safe division
    let asset_0_ratio = Decimal::from_ratio(asset0_amount, asset_0_value);
    let asset_1_ratio = Decimal::from_ratio(asset1_amount, asset_0_value);

    Ok((asset_0_ratio, asset_1_ratio))
}

pub fn get_twap_price(
    querier: &QuerierWrapper,
    block_time: Timestamp,
    twap_window_seconds: u64,
    pool_id: u64,
    token0_denom: String,
    token1_denom: String,
) -> Result<Decimal, ContractError> {
    let twap_querier = TwapQuerier::new(querier);
    let start_of_window = block_time.minus_seconds(twap_window_seconds);
    let twap_price = twap_querier.arithmetic_twap_to_now(
        pool_id,
        token0_denom,
        token1_denom,
        Some(OsmoTimestamp {
            seconds: start_of_window.seconds().try_into()?,
            nanos: 0,
        }),
    )?;

    Ok(Decimal::from_str(&twap_price.arithmetic_twap)?)
}

struct PoolAssets {
    pub base: Coin,
    pub quote: Coin,
}

impl PoolAssets {
    pub fn new(base: Coin, quote: Coin) -> Self {
        Self { base, quote }
    }
}

#[derive(Debug, PartialEq)]
pub struct DepositInfo {
    pub base_deposit: Uint128,
    pub quote_deposit: Uint128,
    pub base_refund: Coin,
    pub quote_refund: Coin,
}

pub fn get_depositable_tokens(
    deps: &DepsMut,
    funds: &[Coin],
    pool_config: &PoolConfig,
) -> Result<DepositInfo, ContractError> {
    let (token0, token1) = must_pay_one_or_two(
        funds,
        (pool_config.token0.clone(), pool_config.token1.clone()),
    )?;

    let position = get_position(deps.storage, &deps.querier)?;
    let assets = PoolAssets::new(
        position.asset0.unwrap_or_default().try_into()?,
        position.asset1.unwrap_or_default().try_into()?,
    );
    get_deposit_info(&assets, token0, token1)
}

fn get_deposit_info(
    assets: &PoolAssets,
    provided_base: Coin,
    provided_quote: Coin,
) -> Result<DepositInfo, ContractError> {
    let provided_base_amount: Uint256 = provided_base.amount.into();
    let provided_quote_amount: Uint256 = provided_quote.amount.into();

    let base_deposit = if assets.quote.amount.is_zero() {
        provided_base_amount
    } else {
        min(
            provided_base_amount,
            provided_quote_amount.checked_mul_floor(Decimal256::from_ratio(
                assets.base.amount,
                assets.quote.amount,
            ))?,
        )
    };
    let quote_deposit = if assets.base.amount.is_zero() {
        provided_quote_amount
    } else {
        min(
            provided_quote_amount,
            provided_base_amount.checked_mul_floor(Decimal256::from_ratio(
                assets.quote.amount,
                assets.base.amount,
            ))?,
        )
    };

    Ok(DepositInfo {
        base_deposit: base_deposit.try_into()?,
        quote_deposit: quote_deposit.try_into()?,
        base_refund: coin(
            TryInto::<Uint128>::try_into(provided_base_amount.checked_sub(base_deposit)?)?.into(),
            assets.base.denom.clone(),
        ),
        quote_refund: coin(
            TryInto::<Uint128>::try_into(provided_quote_amount.checked_sub(quote_deposit)?)?.into(),
            assets.quote.denom.clone(),
        ),
    })
}

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
        .checked_div_floor(denominator)?
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
        .checked_div_floor(denominator)?
        .try_into()?;

    Ok(swap_amount)
}

pub fn get_unused_balances(querier: &QuerierWrapper, env: &Env) -> Result<CoinList, ContractError> {
    Ok(CoinList::from_coins(
        querier.query_all_balances(env.contract.address.to_string())?,
    ))
}

pub fn get_unused_pair_balances(
    deps: &DepsMut,
    env: &Env,
    pool_config: &PoolConfig,
) -> Result<Vec<Coin>, ContractError> {
    // Get unused balances from the contract. This is the amount of tokens that are not currently in a position.
    // This amount already includes the withdrawn amounts from previous steps as in this reply those funds already compose the contract balance.
    let unused_balances = get_unused_balances(&deps.querier, env)?;

    // Use the unused balances to get the token0 and token1 amounts that we can use to create a new position
    let base = unused_balances.find_coin(pool_config.token0.clone());
    let quote = unused_balances.find_coin(pool_config.token1.clone());

    Ok(vec![base, quote])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::tick::{build_tick_exp_cache, price_to_tick};
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::{coin, Coin, Decimal256};
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_0_to_1_swap() {
        let mut deps = mock_dependencies();
        build_tick_exp_cache(deps.as_mut().storage).unwrap();

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
        build_tick_exp_cache(deps.as_mut().storage).unwrap();

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
    fn test_position_in_both_asset() {
        let token0 = Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(1_000_000_000u128),
        };
        let token1 = Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100_000_000_000_000_000_000_000_000_000u128),
        };

        let assets = PoolAssets {
            base: token0.clone(),
            quote: token1.clone(),
        };
        let result = get_deposit_info(&assets, token0.clone(), token1.clone()).unwrap();
        assert_eq!(
            result,
            DepositInfo {
                base_deposit: Uint128::zero(),
                quote_deposit: Uint128::new(100_000_000_000_000_000_000_000_000_000u128),
                base_refund: coin(1_000_000_000u128, token0.denom),
                quote_refund: coin(0u128, token1.denom),
            }
        );
    }

    #[test]
    fn test_position_in_asset1_only() {
        let token0 = Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(50),
        };
        let token1 = Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        };

        let assets = PoolAssets {
            base: Coin {
                denom: "token0".to_string(),
                amount: Uint128::zero(),
            },
            quote: token1.clone(),
        };
        let result = get_deposit_info(&assets, token0.clone(), token1.clone()).unwrap();
        assert_eq!(
            result,
            DepositInfo {
                base_deposit: Uint128::zero(),
                quote_deposit: Uint128::new(100),
                base_refund: coin(50u128, token0.denom),
                quote_refund: coin(0u128, token1.denom),
            }
        );
    }

    #[test]
    fn test_position_in_asset0_only() {
        let token0 = Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(50),
        };
        let token1 = Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        };

        let assets = PoolAssets {
            quote: Coin {
                denom: "token1".to_string(),
                amount: Uint128::zero(),
            },
            base: token0.clone(),
        };
        let result = get_deposit_info(&assets, token0.clone(), token1.clone()).unwrap();
        assert_eq!(
            result,
            DepositInfo {
                base_deposit: Uint128::new(50),
                quote_deposit: Uint128::zero(),
                base_refund: coin(0u128, token0.denom),
                quote_refund: coin(100u128, token1.denom),
            }
        );
    }

    #[test]
    fn test_both_assets_present_token0_limiting() {
        let token0 = Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(50),
        };
        let token1 = Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        };

        let assets = PoolAssets {
            base: token0.clone(),
            quote: token1.clone(),
        };
        let result = get_deposit_info(&assets, coin(2000, "token0"), coin(5000, "token1")).unwrap();
        assert_eq!(
            result,
            DepositInfo {
                base_deposit: Uint128::new(2000),
                quote_deposit: Uint128::new(4000),
                base_refund: coin(0u128, token0.denom),
                quote_refund: coin(1000u128, token1.denom),
            }
        );
    }

    #[test]
    fn test_both_assets_present_token1_limiting() {
        let token0 = Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(50),
        };
        let token1 = Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        };

        let assets = PoolAssets {
            base: token0.clone(),
            quote: token1.clone(),
        };
        let result = get_deposit_info(&assets, coin(2000, "token0"), coin(3000, "token1")).unwrap();
        assert_eq!(
            result,
            DepositInfo {
                base_deposit: Uint128::new(1500),
                quote_deposit: Uint128::new(3000),
                base_refund: coin(500u128, token0.denom),
                quote_refund: coin(0u128, token1.denom),
            }
        );
    }
}
