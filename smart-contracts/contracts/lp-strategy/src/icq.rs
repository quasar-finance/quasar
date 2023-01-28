use cosmwasm_std::{to_binary, Env, IbcMsg, IbcTimeout, Storage, SubMsg, Uint128};
use osmosis_std::types::{
    cosmos::{bank::v1beta1::QueryBalanceRequest, base::v1beta1::Coin as OsmoCoin},
    osmosis::gamm::{v1beta1::QueryCalcExitPoolCoinsFromSharesRequest, v2::QuerySpotPriceRequest},
};
use prost::Message;
use quasar_types::icq::{InterchainQueryPacketData, Query};

use crate::{
    bond::batch_bond,
    error::ContractError,
    helpers::{check_icq_channel, create_ibc_ack_submsg, get_ica_address, IbcMsgKind},
    ibc_util::do_transfer,
    state::{
        CONFIG, ICA_CHANNEL, ICQ_CHANNEL, LP_SHARES, TRANSFER_CHANNEL,
    },
};

// after the balance query, we can calculate the amount of the claim we need to create, we update the claims and transfer the funds
pub fn handle_query_ack(
    storage: &mut dyn Storage,
    env: Env,
    query_balance: Uint128,
) -> Result<SubMsg, ContractError> {
    let transfer_chan = TRANSFER_CHANNEL.load(storage)?;
    let to_address = get_ica_address(storage, ICA_CHANNEL.load(storage)?)?;

    let (amount, deposits) = batch_bond(storage, query_balance)?;

    do_transfer(storage, env, amount, transfer_chan, to_address, deposits)
}

pub fn try_icq(storage: &mut dyn Storage, env: Env) -> Result<Option<SubMsg>, ContractError> {
    let icq_channel = ICQ_CHANNEL.load(storage)?;
    check_icq_channel(storage, icq_channel.clone())?;

    // deposit need to internally rebuild the amount of funds under the smart contract
    let packet = prepare_total_balance_query(storage, ICA_CHANNEL.load(storage)?)?;

    let send_packet_msg = IbcMsg::SendPacket {
        channel_id: icq_channel,
        data: to_binary(&packet)?,
        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
    };

    Ok(Some(create_ibc_ack_submsg(
        storage,
        &IbcMsgKind::Icq,
        send_packet_msg,
    )?))
}

pub fn prepare_total_balance_query(
    storage: &dyn Storage,
    channel: String,
) -> Result<InterchainQueryPacketData, ContractError> {
    let address = get_ica_address(storage, channel)?;
    let config = CONFIG.load(storage)?;
    // we query the current balance on our ica address
    let base_balance = QueryBalanceRequest {
        address: address.clone(),
        denom: config.base_denom.clone(),
    };
    let quote_balance = QueryBalanceRequest {
        address: address.clone(),
        denom: config.quote_denom.clone(),
    };
    let lp_balance = QueryBalanceRequest {
        address: address.clone(),
        denom: config.pool_denom,
    };
    // we simulate the result of an exit pool of our entire vault to get the total value in lp tokens
    let exit_pool = QueryCalcExitPoolCoinsFromSharesRequest {
        pool_id: config.pool_id,
        share_in_amount: LP_SHARES.load(storage)?.to_string(),
    };
    // we query the spot price of our base_denom and quote_denom so we can convert the quote_denom from exitpool to the base_denom
    let spot_price = QuerySpotPriceRequest {
        pool_id: config.pool_id,
        base_asset_denom: config.base_denom,
        quote_asset_denom: config.quote_denom,
    };
    Ok(Query::new()
        .add_request(
            base_balance.encode_to_vec(),
            QueryBalanceRequest::TYPE_URL.to_string(),
        )
        .add_request(
            quote_balance.encode_to_vec(),
            QueryBalanceRequest::TYPE_URL.to_string(),
        )
        .add_request(
            lp_balance.encode_to_vec(),
            QueryBalanceRequest::TYPE_URL.to_string(),
        )
        .add_request(
            exit_pool.encode_to_vec(),
            QueryCalcExitPoolCoinsFromSharesRequest::TYPE_URL.to_string(),
        )
        .add_request(
            spot_price.encode_to_vec(),
            QuerySpotPriceRequest::TYPE_URL.to_string(),
        )
        .encode_pkt())
}

// calculate the total balance of the vault using the query from prepare_total_balance_query()
pub fn calc_total_balance(
    storage: &mut dyn Storage,
    ica_balance: Uint128,
    exit_pool: Vec<OsmoCoin>,
    spot_price: Uint128,
) -> Result<Uint128, ContractError> {
    let config = CONFIG.load(storage)?;
    let base = exit_pool
        .iter()
        .find(|coin| coin.denom == config.base_denom)
        .ok_or(ContractError::BaseDenomNotFound)?;
    let quote = exit_pool
        .iter()
        .find(|coin| coin.denom == config.quote_denom)
        .ok_or(ContractError::QuoteDenomNotFound)?;
    // return ica_balance + base_amount + (quote_amount * spot_price)
    Ok(ica_balance
        .checked_add(Uint128::new(base.amount.parse::<u128>()?))?
        .checked_add(Uint128::new(quote.amount.parse::<u128>()?).checked_mul(spot_price)?)?)
}
