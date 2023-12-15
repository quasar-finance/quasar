use cosmwasm_std::{Coin, Decimal, Fraction};
use osmosis_test_tube::{Runner, Wasm};

use crate::{
    msg::QueryMsg,
    query::{
        FullPosition, FullPositionResponse, TotalAssetsResponse, TotalVaultTokenSupplyResponse,
    },
};

/// get the share price of a single share in asset0, thus share/token
pub fn get_share_price_in_asset0<'a, R>(
    wasm: &Wasm<'a, R>,
    spot_price: Decimal,
    contract: &str,
) -> Result<Decimal, String>
where
    R: Runner<'a>,
{
    // get the total_vault_assets
    let (token0, token1) = get_total_assets(wasm, contract)?;

    // calculate the total vault assets in asset0
    // total = amount0 + amount1 / spotprice
    let total = token0.amount
        + token1
            .amount
            .multiply_ratio(spot_price.denominator(), spot_price.numerator());

    // get the total shares
    let total_shares: TotalVaultTokenSupplyResponse = wasm
        .query(contract, &QueryMsg::TotalVaultTokenSupply {})
        .map_err(|e| e.to_string())?;

    // calculate the price per share
    Ok(Decimal::from_ratio(total_shares.total, total))
}

pub fn get_full_positions<'a, R>(
    wasm: &Wasm<'a, R>,
    contract: &str,
) -> Result<Vec<FullPosition>, String>
where
    R: Runner<'a>,
{
    let res: FullPositionResponse = wasm
        .query(
            contract,
            &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                crate::msg::ClQueryMsg::FullPosition {},
            )),
        )
        .map_err(|e| e.to_string())?;
    Ok(res.positions)
}

pub fn get_total_assets<'a, R>(wasm: &Wasm<'a, R>, contract: &str) -> Result<(Coin, Coin), String>
where
    R: Runner<'a>,
{
    // get the total_vault_assets
    let total_assets: TotalAssetsResponse = wasm
        .query(contract, &QueryMsg::TotalAssets {})
        .map_err(|e| e.to_string())?;

    Ok((total_assets.token0, total_assets.token1))
}

/// assert that amount a and amount b
#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let eps = 99999;
        let total = 100000
        let (a, b) = (&$a, &$b);
        assert!(
            a in [(b - (b * eps / total))..(b * total / eps)],
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            (*a - *b).abs()
        );
    }};
    // ($a:expr, $b:expr, $ratio:expr) => {{
    //     let (a, b) = (&$a, &$b);
    //     let eps = $eps;
    //     assert!(
    //         (*a - *b).abs() < eps,
    //         "assertion failed: `(left !== right)` \
    //          (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
    //         *a,
    //         *b,
    //         eps,
    //         (*a - *b).abs()
    //     );
    // }};
}
