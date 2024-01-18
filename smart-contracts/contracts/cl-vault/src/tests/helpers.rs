use std::str::FromStr;

use cosmwasm_std::{Attribute, Coin, Decimal, Fraction, Uint128};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SpotPriceRequest;
use osmosis_test_tube::{ExecuteResponse, Module, PoolManager, Runner, Wasm};

use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse;

use crate::{
    msg::{ClQueryMsg, ExtensionQueryMsg, QueryMsg},
    query::{
        FullPosition, FullPositionsResponse, TotalAssetsResponse, TotalVaultTokenSupplyResponse,
        UserBalanceResponse,
    },
};

pub fn get_event_attributes_by_ty_and_key(
    response: &ExecuteResponse<MsgExecuteContractResponse>,
    ty: &str,
    keys: Vec<&str>,
) -> Vec<Attribute> {
    response
        .events
        .iter()
        .filter(|event| event.ty == ty)
        .flat_map(|event| event.attributes.clone())
        .filter(|attribute| keys.contains(&attribute.key.as_str()))
        .collect()
}

pub fn get_share_price<'a, R>(app: &'a R, cl_pool_id: u64, contract_address: &str) -> Decimal
where
    R: Runner<'a>,
{
    let wasm = Wasm::new(app);

    let spot_price: Decimal = PoolManager::new(app)
        .query_spot_price(&SpotPriceRequest {
            pool_id: cl_pool_id,
            base_asset_denom: "uatom".into(),
            quote_asset_denom: "uosmo".into(),
        })
        .unwrap()
        .spot_price
        .parse()
        .unwrap();
    let share_price: Decimal =
        get_share_price_in_asset0(&wasm, spot_price, contract_address).unwrap();
    share_price
}

pub fn get_user_shares<'a, R>(
    wasm: &Wasm<'a, R>,
    contract: &str,
    user: String,
) -> Result<Uint128, String>
where
    R: Runner<'a>,
{
    let user_balance: UserBalanceResponse = wasm
        .query(
            contract,
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                crate::msg::UserBalanceQueryMsg::UserSharesBalance { user },
            )),
        )
        .map_err(|e| e.to_string())?;
    Ok(user_balance.balance)
}

pub fn get_share_value<'a, R>(
    wasm: &Wasm<'a, R>,
    contract: &str,
    amount: Uint128,
) -> Result<Vec<Coin>, String>
where
    R: Runner<'a>,
{
    let balance: Vec<Coin> = wasm
        .query(contract, &QueryMsg::ConvertToAssets { amount })
        .map_err(|e| e.to_string())?;
    Ok(balance)
}

pub fn get_unused_funds<'a, R>(
    wasm: &Wasm<'a, R>,
    contract: &str,
) -> Result<(Uint128, Uint128), String>
where
    R: Runner<'a>,
{
    let (token0, token1) = get_total_assets(wasm, contract)?;
    let full_positions: FullPositionsResponse = wasm
        .query(
            contract,
            &QueryMsg::VaultExtension(ExtensionQueryMsg::ConcentratedLiquidity(
                ClQueryMsg::FullPositions {},
            )),
        )
        .map_err(|e| e.to_string())?;

    println!("about to iterate");

    let position_funds = full_positions.positions.iter().fold(
        (Uint128::zero(), Uint128::zero()),
        |(acc0, acc1), fp| {
            let c0 = fp
                .full_breakdown
                .asset0
                .clone()
                .map(|c| c.amount.parse().unwrap())
                .unwrap_or(Uint128::zero());

            let c1 = fp
                .full_breakdown
                .asset1
                .clone()
                .map(|c| c.amount.parse().unwrap())
                .unwrap_or(Uint128::zero());

            (c0 + acc0, c1 + acc1)
        },
    );
    println!("got unused");

    Ok((
        token0.amount - position_funds.0,
        token1.amount - position_funds.1,
    ))
}

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
    let res: FullPositionsResponse = wasm
        .query(
            contract,
            &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                crate::msg::ClQueryMsg::FullPositions {},
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
