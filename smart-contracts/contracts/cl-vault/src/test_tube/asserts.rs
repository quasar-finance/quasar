use std::str::FromStr;

use cosmwasm_std::{Decimal, Uint128};

#[macro_export]
macro_rules! assert_eq_with_diff {
    ($left:expr, $left_tag:expr, $right:expr, $right_tag:expr, $max_rel_diff:expr, $max_absolute_diff:expr $(,)?) => {{
        $crate::test_tube::asserts::assert_eq_with_diff_impl($left, $left_tag, $right, $right_tag, $max_rel_diff, $max_absolute_diff, None);
    }};
    ($left:expr, $left_tag:expr, $right:expr, $right_tag:expr, $max_rel_diff:expr, $max_absolute_diff:expr, $($args:tt)+) => {{
        $crate::test_tube::asserts::assert_eq_with_diff_impl($left, $left_tag, $right, $right_tag, $max_rel_diff, $max_absolute_diff, Some(format!($($args)*)));
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
