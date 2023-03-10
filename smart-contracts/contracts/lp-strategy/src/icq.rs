use cosmwasm_std::{to_binary, Env, IbcMsg, IbcTimeout, Storage, SubMsg, Uint128};
use osmosis_std::types::{
    cosmos::{bank::v1beta1::QueryBalanceRequest, base::v1beta1::Coin as OsmoCoin},
    osmosis::gamm::{v1beta1::QueryCalcExitPoolCoinsFromSharesRequest, v2::QuerySpotPriceRequest},
};
use prost::Message;
use quasar_types::icq::{InterchainQueryPacketData, Query};

use crate::{
    error::ContractError,
    helpers::{check_icq_channel, create_ibc_ack_submsg, get_ica_address, IbcMsgKind},
    state::{CONFIG, IBC_LOCK, ICA_CHANNEL, ICQ_CHANNEL, LP_SHARES},
};

pub fn try_icq(storage: &mut dyn Storage, env: Env) -> Result<Option<SubMsg>, ContractError> {
    if IBC_LOCK.load(storage)?.is_locked() {
        return Ok(None);
    }

    // TODO fetching ICQ channel and confirming vs handshake version can be a single function
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
        address,
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

    // path have to be set manually, should be equal to the proto_queries of osmosis-std types
    Ok(Query::new()
        .add_request(
            base_balance.encode_to_vec(),
            "/cosmos.bank.v1beta1.Query/Balance".to_string(),
        )
        .add_request(
            quote_balance.encode_to_vec(),
            "/cosmos.bank.v1beta1.Query/Balance".to_string(),
        )
        .add_request(
            lp_balance.encode_to_vec(),
            "/cosmos.bank.v1beta1.Query/Balance".to_string(),
        )
        .add_request(
            exit_pool.encode_to_vec(),
            "/osmosis.gamm.v1beta1.Query/CalcExitPoolCoinsFromShares".to_string(),
        )
        .add_request(
            spot_price.encode_to_vec(),
            "/osmosis.gamm.v2.Query/SpotPrice".to_string(),
        )
        .encode_pkt())
}

// TODO add quote denom to base denom conversion
// calculate the total balance of the vault using the query from prepare_total_balance_query()
pub fn calc_total_balance(
    storage: &mut dyn Storage,
    ica_balance: Uint128,
    exit_pool: Vec<OsmoCoin>,
    spot_price: Uint128,
) -> Result<Uint128, ContractError> {
    let config = CONFIG.load(storage)?;
    // if we receive no tokens in the response, the total balance
    if exit_pool.is_empty() {
        return Ok(ica_balance);
    }

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
        .checked_add(Uint128::new(base.amount.parse::<u128>().map_err(
            |err| ContractError::ParseIntError {
                error: err,
                value: base.amount.clone(),
            },
        )?))?
        .checked_add(
            Uint128::new(quote.amount.parse::<u128>().map_err(|err| {
                ContractError::ParseIntError {
                    error: err,
                    value: quote.amount.clone(),
                }
            })?)
            .checked_mul(spot_price)?,
        )?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use crate::{ibc_lock::Lock, state::IBC_LOCK, test_helpers::default_setup};

    use super::*;

    #[test]
    fn try_icq_unlocked_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();

        LP_SHARES
            .save(deps.as_mut().storage, &Uint128::new(100))
            .unwrap();

        // lock the ibc lock
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();

        let res = try_icq(deps.as_mut().storage, env.clone()).unwrap();

        let pkt = IbcMsg::SendPacket {
            channel_id: ICQ_CHANNEL.load(deps.as_ref().storage).unwrap(),
            data: to_binary(
                &prepare_total_balance_query(
                    deps.as_ref().storage,
                    ICA_CHANNEL.load(deps.as_ref().storage).unwrap(),
                )
                .unwrap(),
            )
            .unwrap(),
            timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
        };

        assert_eq!(
            res,
            Some(create_ibc_ack_submsg(deps.as_mut().storage, &IbcMsgKind::Icq, pkt).unwrap())
        )
    }

    #[test]
    fn try_icq_locked_bond_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();

        // lock the ibc lock
        IBC_LOCK
            .save(deps.as_mut().storage, &Lock::new().lock_bond())
            .unwrap();

        let res = try_icq(deps.as_mut().storage, env).unwrap();
        assert_eq!(res, None)
    }

    #[test]
    fn try_icq_locked_start_unbond_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();

        // lock the ibc lock
        IBC_LOCK
            .save(deps.as_mut().storage, &&Lock::new().lock_start_unbond())
            .unwrap();

        let res = try_icq(deps.as_mut().storage, env).unwrap();
        assert_eq!(res, None)
    }

    #[test]
    fn try_icq_locked_unbond_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();

        // lock the ibc lock
        IBC_LOCK
            .save(deps.as_mut().storage, &Lock::new().lock_unbond())
            .unwrap();

        let res = try_icq(deps.as_mut().storage, env).unwrap();
        assert_eq!(res, None)
    }
}
