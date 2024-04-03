use cosmwasm_std::Uint256;

pub fn abs_diff(a: Uint256, b: Uint256) -> Uint256 {
    if a > b {
        a.checked_sub(b).unwrap()
    } else {
        b.checked_sub(a).unwrap()
    }
}
