use std::str::FromStr;

use cosmwasm_std::{
    Coin, ConversionOverflowError, Decimal, Env, Fraction, IbcMsg, IbcTimeout, StdError, Storage,
    SubMsg, Uint128,
};
use osmosis_std::{
    shim::Duration,
    types::{
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::{
            gamm::v1beta1::{MsgJoinSwapExternAmountIn, QueryCalcJoinPoolSharesResponse},
            lockup::MsgLockTokens,
        },
    },
};

use quasar_types::ica::packet::ica_send;

use crate::{
    error::ContractError,
    helpers::{create_ibc_ack_submsg, get_ica_address, IbcMsgKind, IcaMessages},
    state::{
        OngoingDeposit, PendingBond, CONFIG, ICA_CHANNEL, SIMULATED_EXIT_RESULT,
        SIMULATED_JOIN_AMOUNT, SIMULATED_JOIN_RESULT,
    },
};

pub fn do_transfer(
    storage: &mut dyn Storage,
    env: &Env,
    amount: Uint128,
    channel_id: String,
    to_address: String,
    deposits: Vec<OngoingDeposit>,
) -> Result<SubMsg, ContractError> {
    // todo check denom of funds once we have denom mapping done

    let coin = Coin {
        denom: CONFIG.load(storage)?.local_denom,
        amount,
    };

    let timeout = IbcTimeout::with_timestamp(env.block.time.plus_seconds(300));
    let transfer = IbcMsg::Transfer {
        channel_id,
        to_address,
        amount: coin,
        timeout,
    };

    Ok(create_ibc_ack_submsg(
        storage,
        IbcMsgKind::Transfer {
            pending: PendingBond { bonds: deposits },
            amount,
        },
        transfer,
    )?)
}

pub fn scale_join_pool(
    storage: &dyn Storage,
    actual: Uint128,
    join: QueryCalcJoinPoolSharesResponse,
    return_scaled: bool,
) -> Result<Uint128, ContractError> {
    let token_in = SIMULATED_JOIN_AMOUNT.load(storage)?;
    let join = join
        .share_out_amount
        .parse()
        .map_err(|err| ContractError::ParseIntError {
            error: err,
            value: join.share_out_amount,
        })?;

    if (!return_scaled) {
        return Ok(Uint128::new(join));
    } else {
        Ok(Uint128::new(join).checked_multiply_ratio(actual, token_in)?)
    }
}

pub fn consolidate_exit_pool_amount_into_local_denom(
    storage: &mut dyn Storage,
    exit_pool: &Vec<OsmoCoin>,
    spot_price: Decimal,
) -> Result<Uint128, ContractError> {
    let config = CONFIG.load(storage)?;

    // if we receive no tokens in the response, we can't exit the pool
    // todo: Should this error?
    if exit_pool.is_empty() {
        return Ok(Uint128::zero());
    }

    // consolidate exit_pool.tokens_out into a single Uint128 by using spot price to convert the quote_denom to local_denom
    let base = exit_pool
        .iter()
        .find(|coin| coin.denom == config.base_denom)
        .ok_or(ContractError::BaseDenomNotFound)?;
    let quote = exit_pool
        .iter()
        .find(|coin| coin.denom == config.quote_denom)
        .ok_or(ContractError::QuoteDenomNotFound)?;

    Ok(Uint128::new(
        base.amount
            .parse::<u128>()
            .map_err(|err| ContractError::ParseIntError {
                error: err,
                value: base.amount.clone(),
            })?,
    )
    .checked_add(
        Uint128::new(
            quote
                .amount
                .parse::<u128>()
                .map_err(|err| ContractError::ParseIntError {
                    error: err,
                    value: quote.amount.clone(),
                })?,
        )
        .checked_multiply_ratio(spot_price.numerator(), spot_price.denominator())?,
    )?)
}

pub fn calculate_share_out_min_amount(storage: &mut dyn Storage) -> Result<Uint128, ContractError> {
    let last_sim_join_pool_result = SIMULATED_JOIN_RESULT.load(storage)?;

    // todo: better dynamic slippage estimation, especially for volatile tokens
    // diminish the share_out_amount by 5 percent to allow for slippage of 5% on the swap
    Ok(last_sim_join_pool_result.checked_multiply_ratio(95u128, 100u128)?)
}

// exit shares should never be more than total shares here
pub fn calculate_token_out_min_amount(
    storage: &dyn Storage,
    exit_lp_shares: Uint128,
    total_locked_shares: Uint128,
) -> Result<Uint128, ContractError> {
    let last_sim_exit_pool_result = SIMULATED_EXIT_RESULT.load(storage)?;

    // todo: better dynamic slippage estimation, especially for volatile tokens
    // diminish the share_out_amount by 5 percent to allow for slippage of 5% on the swap
    Ok(last_sim_exit_pool_result
        .checked_multiply_ratio(exit_lp_shares, total_locked_shares)?
        .checked_multiply_ratio(95u128, 100u128)?)
}

/// prepare the submsg for joining the pool
#[allow(clippy::too_many_arguments)] // allowing this is not ideal, but for now we want to keep channel_id and pool_id in there
pub fn do_ibc_join_pool_swap_extern_amount_in(
    storage: &mut dyn Storage,
    env: Env,
    pool_id: u64,
    denom: String,
    amount: Uint128,
    share_out_min_amount: Uint128,
    deposits: Vec<OngoingDeposit>,
) -> Result<SubMsg, ContractError> {
    let ica_channel = ICA_CHANNEL.load(storage)?;
    let ica_address = get_ica_address(storage, ica_channel.clone())?;

    // setup the first IBC message to send, and save the entire sequence so we have acces to it on acks
    let msg = MsgJoinSwapExternAmountIn {
        sender: ica_address,
        pool_id,
        token_in: Some(OsmoCoin {
            denom,
            amount: amount.to_string(),
        }),
        share_out_min_amount: share_out_min_amount.to_string(),
    };

    let pkt = ica_send::<MsgJoinSwapExternAmountIn>(
        msg,
        ica_channel,
        IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
    )?;

    Ok(create_ibc_ack_submsg(
        storage,
        IbcMsgKind::Ica(IcaMessages::JoinSwapExternAmountIn(PendingBond {
            bonds: deposits,
        })),
        pkt,
    )?)
}

pub fn do_ibc_lock_tokens(
    storage: &mut dyn Storage,
    owner: String,
    coins: Vec<Coin>,
) -> Result<MsgLockTokens, ContractError> {
    let lock_period = CONFIG.load(storage)?.lock_period;

    // TODO move the duration to a package and make it settable
    Ok(MsgLockTokens {
        owner,
        duration: Some(Duration {
            // TODO clean up this conversion a bit
            seconds: i64::try_from(lock_period).map_err(|_| {
                ContractError::Std(StdError::ConversionOverflow {
                    source: ConversionOverflowError {
                        source_type: "u64",
                        target_type: "i64",
                        value: lock_period.to_string(),
                    },
                })
            })?,
            nanos: 0,
        }),
        coins: coins
            .iter()
            .map(|c| OsmoCoin {
                denom: c.denom.clone(),
                amount: c.amount.to_string(),
            })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, MockApi, MockQuerier, MockStorage},
        Empty, IbcEndpoint, OwnedDeps, Uint128,
    };
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;

    use cw_storage_plus::Map;
    use osmosis_std::types::osmosis::gamm::v1beta1::QueryCalcJoinPoolSharesResponse;
    use quasar_types::{
        ibc::{ChannelInfo, ChannelType, HandshakeState},
        ica::handshake::IcaMetadata,
    };

    use crate::{
        ibc_util::calculate_token_out_min_amount,
        state::{SIMULATED_EXIT_RESULT, SIMULATED_JOIN_RESULT},
    };

    use super::calculate_share_out_min_amount;

    fn default_instantiate(
        channels: &Map<String, ChannelInfo>,
    ) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
        let mut deps = mock_dependencies();

        // load an ICA channel into the deps
        let (chan_id, channel_info) = default_ica_channel();
        // unwrap here since this is a test function
        channels
            .save(deps.as_mut().storage, chan_id, &channel_info)
            .unwrap();
        deps
    }

    fn default_ica_channel() -> (String, ChannelInfo) {
        let chan_id = String::from("channel-0");
        (
            chan_id.clone(),
            ChannelInfo {
                id: chan_id,
                counterparty_endpoint: IbcEndpoint {
                    port_id: String::from("ica-host"),
                    channel_id: String::from("channel-0"),
                },
                connection_id: String::from("connection-0"),
                channel_type: ChannelType::Ica {
                    channel_ty: IcaMetadata::with_connections(
                        String::from("connection-0"),
                        String::from("connection-0"),
                    ),
                    counter_party_address: Some(String::from("osmo")),
                },
                handshake_state: HandshakeState::Open,
            },
        )
    }

    #[test]
    fn default_instantiate_works() {
        let channels = Map::new("channels");
        let deps = default_instantiate(&channels);

        let _chan = channels
            .load(deps.as_ref().storage, "channel-0".to_string())
            .unwrap();
    }

    #[test]
    fn test_calculate_share_out_min_amount() {
        let mut deps = mock_dependencies();
        SIMULATED_JOIN_RESULT
            .save(deps.as_mut().storage, &Uint128::new(999999))
            .unwrap();

        let min_amount_out = calculate_share_out_min_amount(deps.as_mut().storage).unwrap();

        assert_eq!(min_amount_out, Uint128::from(949999u128));
    }

    #[test]
    fn test_calculate_token_out_min_amount() {
        let mut deps = mock_dependencies();
        SIMULATED_EXIT_RESULT
            .save(deps.as_mut().storage, &Uint128::from(999999u128))
            .unwrap();

        let exit_shares_amount = Uint128::from(100u128);
        let total_shares_amount = Uint128::from(100u128);

        let min_amount_out = calculate_token_out_min_amount(
            deps.as_mut().storage,
            exit_shares_amount,
            total_shares_amount,
        )
        .unwrap();

        assert_eq!(min_amount_out, Uint128::from(949999u128));

        // now lets test with a different amount of exit shares and total shares
        let exit_shares_amount = Uint128::from(900u128);
        let total_shares_amount = Uint128::from(100000u128);

        let min_amount_out = calculate_token_out_min_amount(
            deps.as_mut().storage,
            exit_shares_amount,
            total_shares_amount,
        )
        .unwrap();

        assert_eq!(min_amount_out, Uint128::from(8549u128));
    }

    #[test]
    fn test_do_transfer() {}
}
