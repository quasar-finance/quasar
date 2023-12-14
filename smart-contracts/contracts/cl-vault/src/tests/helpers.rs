use cosmwasm_std::{Decimal, Fraction};
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
    let total_assets: TotalAssetsResponse = wasm
        .query(contract, &QueryMsg::TotalAssets {})
        .map_err(|e| e.to_string())?;

    println!("total_assets: {:?}", total_assets);

    // calculate the total vault assets in asset0
    // total = amount0 + amount1 / spotprice
    let total = total_assets.token0.amount
        + total_assets
            .token1
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
