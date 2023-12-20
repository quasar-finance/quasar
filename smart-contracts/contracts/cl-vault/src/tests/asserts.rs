use std::str::FromStr;

use cosmwasm_std::{Decimal, Coin, Uint128};
use osmosis_test_tube::{Runner, Wasm, Module};

use crate::tests::helpers::{get_share_price, get_total_assets};




#[macro_export]
macro_rules! assert_eq_with_diff {
    ($left:expr, $right:expr, $max_rel_diff:expr, $max_absolute_diff:expr $(,)?) => {{
        $crate::tests::asserts::assert_eq_with_diff_impl($left, $right, $max_rel_diff, $max_absolute_diff, None);
    }};
    ($left:expr, $right:expr, $max_rel_diff:expr, $($args:tt)+) => {{
        $crate::tests::asserts::assert_eq_with_diff_impl($left, $right, $max_rel_diff, $max_absolute_diff, Some(format!($($args)*)));
    }};
}

#[macro_export]
macro_rules! assert_share_price {
    ($app:expr, $contract_addr:expr, $expected_share_price:expr, $pool_id:expr) => {
        $crate::tests::asserts::assert_share_price_impl($app, $contract_addr, $expected_share_price, $pool_id)
    }
}

#[track_caller]
pub fn assert_share_price_impl<'a, R: Runner<'a>>(
    app: &'a R,
    contract_address: &str,
    expected_share_price: Decimal,
    pool_id: u64,
) {
    let wasm = Wasm::new(app);

    let new_share_price = get_share_price(app, pool_id, contract_address);
    assert_eq_with_diff!(
        expected_share_price.atomics(),
        new_share_price.atomics(),
        "0.000001",
        Uint128::new(1)
    );
}

#[macro_export]
macro_rules! assert_total_assets {
    ($wasm:expr, $contract_addr:expr, $expected_total_assets:expr) => {
        $crate::tests::asserts::assert_total_assets_impl($wasm, $contract_addr, $expected_total_assets)
    }
}

#[track_caller]
pub fn assert_total_assets_impl(
    wasm: &Wasm<'_, osmosis_test_tube::OsmosisTestApp>,
    contract_address: &str,
    expected_total_assets: &(Coin, Coin),
) {
    let current_assets = get_total_assets(wasm, contract_address).unwrap();
    assert_eq_with_diff!(
        expected_total_assets.0.amount,
        current_assets.0.amount,
        "0.000001",
        Uint128::new(5)
    );
    assert_eq_with_diff!(
        expected_total_assets.1.amount,
        current_assets.1.amount,
        "0.000001",
        Uint128::new(5)
    );
}

/// Implementation for the [`cosmwasm_std::assert_approx_eq`] macro. This does not provide any
/// stability guarantees and may change any time.
#[track_caller]
#[doc(hidden)]
pub fn assert_eq_with_diff_impl<U: Into<Uint128>>(
    left: U,
    right: U,
    max_rel_diff: &str,
    max_absolute_diff: U,
    panic_msg: Option<String>,
) {
    let left = left.into();
    let right = right.into();
    let max_rel_diff = Decimal::from_str(max_rel_diff).unwrap();

    let largest = std::cmp::max(left, right);
    let rel_diff = Decimal::from_ratio(left.abs_diff(right), largest);
    let abs_diff = if left > right {
        left - right
    } else {
        right - left
    };

    if rel_diff > max_rel_diff && abs_diff > max_absolute_diff.into() {
        match panic_msg {
            Some(panic_msg) => panic!(
                "assertion failed: `(left ≈ right)`\nleft: {}\nright: {}\nrelative difference: {}\nmax allowed relative difference: {}\n: {}",
                left, right, rel_diff, max_rel_diff, panic_msg
            ),
            None => panic!(
                "assertion failed: `(left ≈ right)`\nleft: {}\nright: {}\nrelative difference: {}\nmax allowed relative difference: {}\n",
                left, right, rel_diff, max_rel_diff
            ),
        }
    }
}
