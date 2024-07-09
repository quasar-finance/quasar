use crate::{
    contract::{DexAdapter, DexAdapterResult},
    msg::DexAdapterExecuteMsg,
    replies::SWAP_REPLY_ID,
    state::{RECIPIENT, STATE},
    DexAdapterError,
};
use abstract_app::{sdk::TransferInterface, traits::AbstractResponse};
use cosmwasm_std::{Decimal, DepsMut, Env, MessageInfo, SubMsg};
use cw_asset::Asset;
use lst_adapter_osmosis::msg::LstAdapterQueryMsg;
use quasar_types::{abstract_sdk::QueryMsg as AbstractQueryMsg, error::assert_funds_single_token};

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    app: DexAdapter,
    msg: DexAdapterExecuteMsg,
) -> DexAdapterResult {
    match msg {
        DexAdapterExecuteMsg::Swap { slippage } => swap(deps, info, slippage, app),
    }
}

#[allow(unused)]
fn swap(
    deps: DepsMut,
    info: MessageInfo,
    slippage: Option<Decimal>,
    app: DexAdapter,
) -> DexAdapterResult {
    let state = STATE.load(deps.storage)?;
    assert_funds_single_token(&info.funds, &state.offer_asset.inner())?;
    let offer_amount = info.funds[0].amount;
    RECIPIENT.save(deps.storage, &Some(info.sender))?;
    use abstract_dex_adapter::api::DexInterface;
    let dex = app.dex(deps.as_ref(), state.dex);
    let offer_asset = Asset::new(state.offer_asset, offer_amount);
    let simulated = dex.simulate_swap(
        offer_asset.clone(),
        state.receive_asset.clone(),
        state.pool.clone(),
    )?;
    let redemption_rate = deps.querier.query_wasm_smart::<Decimal>(
        state.lst_adapter,
        &AbstractQueryMsg::Module(LstAdapterQueryMsg::RedemptionRate {}),
    )?;
    let price = Decimal::from_ratio(offer_amount, simulated.return_amount);
    if price.checked_mul(
        Decimal::one()
            .checked_add(slippage.unwrap_or_default())?
            .checked_add(state.margin)?,
    )? > redemption_rate
    {
        return Err(DexAdapterError::InvalidPrice {});
    }

    let transfer_msgs = app.bank(deps.as_ref()).deposit(info.funds)?;
    let swap_msg = dex.swap(
        offer_asset,
        state.receive_asset,
        slippage,
        Some(price),
        state.pool,
    )?;
    Ok(app
        .response(format!("swap {}", offer_amount))
        .add_messages(transfer_msgs)
        .add_submessage(SubMsg::reply_on_success(swap_msg, SWAP_REPLY_ID)))
}
