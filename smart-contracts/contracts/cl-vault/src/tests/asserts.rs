use std::str::FromStr;

use cosmwasm_std::{Coin, Decimal, Uint128};
use osmosis_test_tube::{Module, Runner, Wasm};

use crate::{
    helpers::get_unused_balances,
    tests::helpers::{get_share_price, get_total_assets},
};

use super::helpers::get_unused_funds;

#[macro_export]
macro_rules! assert_eq_with_diff {
    ($left:expr, $left_tag:expr, $right:expr, $right_tag:expr, $max_rel_diff:expr, $max_absolute_diff:expr $(,)?) => {{
        $crate::tests::asserts::assert_eq_with_diff_impl($left, $left_tag, $right, $right_tag, $max_rel_diff, $max_absolute_diff, None);
    }};
    ($left:expr, $left_tag:expr, $right:expr, $right_tag:expr, $max_rel_diff:expr, $($args:tt)+) => {{
        $crate::tests::asserts::assert_eq_with_diff_impl($left, $left_tag, $right, $right_tag, $max_rel_diff, $max_absolute_diff, Some(format!($($args)*)));
    }};
}

/// assert that some two values are at most both the relative difference and absolute difference away from each other
/// useful in comparing small Uint128 values to eachother
#[track_caller]
#[doc(hidden)]
pub fn assert_eq_with_diff_impl<U: Into<Uint128>>(
    left: U,
    left_tag: &str,
    right: U,
    right_tag: &str,
    max_rel_diff: &str,
    max_absolute_diff: U,
    panic_msg: Option<String>,
) {
    let left = left.into();
    let right = right.into();
    let max_rel_diff = Decimal::from_str(max_rel_diff).unwrap();

    let largest = std::cmp::max(left, right);
    // we need to handle the case where left or right are 0
    let rel_diff =
        Decimal::checked_from_ratio(left.abs_diff(right), largest).unwrap_or(max_rel_diff);
    let abs_diff = if left > right {
        left - right
    } else {
        right - left
    };

    if rel_diff > max_rel_diff && abs_diff > max_absolute_diff.into() {
        match panic_msg {
            Some(panic_msg) => panic!(
                "assertion failed: `({left_tag} ≈ {right_tag})`\n{left_tag}: {left}\n{right_tag}: {right}\nrelative difference: {rel_diff}\nmax allowed relative difference: {max_rel_diff}\n: {panic_msg}"
            ),
            None => panic!(
                "assertion failed: `({left_tag} ≈ {right_tag})`\n{left_tag}: {left}\n{right_tag}: {right}\nrelative difference: {rel_diff}\nmax allowed relative difference: {max_rel_diff}\n"
            ),
        }
    }
}

#[macro_export]
macro_rules! assert_share_price {
    ($app:expr, $contract_addr:expr, $expected_share_price:expr, $pool_id:expr) => {
        $crate::tests::asserts::assert_share_price_impl(
            $app,
            $contract_addr,
            $expected_share_price,
            $pool_id,
            "0.0001",
        )
    };
    ($app:expr, $contract_addr:expr, $expected_share_price:expr, $pool_id:expr, $max_rel_diff:expr) => {
        $crate::tests::asserts::assert_share_price_impl(
            $app,
            $contract_addr,
            $expected_share_price,
            $pool_id,
            $max_rel_diff,
        )
    };
}

#[track_caller]
pub fn assert_share_price_impl<'a, R: Runner<'a>>(
    app: &'a R,
    contract_address: &str,
    expected_share_price: Decimal,
    pool_id: u64,
    max_rel_diff: &str,
) {
    let new_share_price = get_share_price(app, pool_id, contract_address);
    assert_eq_with_diff!(
        expected_share_price.atomics(),
        "expected_share_price",
        new_share_price.atomics(),
        "new_share_price",
        max_rel_diff,
        Uint128::new(1)
    );
}

#[macro_export]
macro_rules! assert_total_assets {
    ($wasm:expr, $contract_addr:expr, $expected_total_assets:expr) => {
        $crate::tests::asserts::assert_total_assets_impl(
            $wasm,
            $contract_addr,
            $expected_total_assets,
        )
    };
}

#[track_caller]
pub fn assert_total_assets_impl<'a, R>(
    wasm: &Wasm<'a, R>,
    contract_address: &str,
    expected_total_assets: &(Coin, Coin),
) where
    R: Runner<'a>,
{
    let current_assets = get_total_assets(wasm, contract_address).unwrap();
    assert_eq_with_diff!(
        expected_total_assets.0.amount,
        "expected_total_assets token0",
        current_assets.0.amount,
        "current_total_assets token0",
        "0.000001",
        Uint128::new(5)
    );
    assert_eq_with_diff!(
        expected_total_assets.1.amount,
        "expected_total_assets token1",
        current_assets.1.amount,
        "current_total_assets token1",
        "0.000001",
        Uint128::new(5)
    );
}

#[macro_export]
macro_rules! assert_unused_funds {
    ($wasm:expr, $contract_addr:expr, $actual:expr) => {
        $crate::tests::asserts::assert_unused_funds_impl($wasm, $contract_addr, $actual)
    };
}

#[track_caller]
pub fn assert_unused_funds_impl<'a, R>(
    wasm: &Wasm<'a, R>,
    contract_address: &str,
    (actual0, actual1): (Uint128, Uint128),
) where
    R: Runner<'a>,
{
    let (expected0, expected1) = get_unused_funds(wasm, contract_address).unwrap();
    assert_eq_with_diff!(
        actual0,
        "actual token0",
        expected0,
        "expected token0",
        "0.00000001",
        Uint128::new(4)
    );
    assert_eq_with_diff!(
        actual1,
        "actual token1",
        expected1,
        "expected token1",
        "0.00000001",
        Uint128::new(4)
    )
}
