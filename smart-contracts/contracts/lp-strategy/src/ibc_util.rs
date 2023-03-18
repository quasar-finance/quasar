use std::str::FromStr;

use cosmwasm_std::{
    Coin, ConversionOverflowError, Env, IbcMsg, IbcTimeout, StdError, Storage, SubMsg, Uint128,
};
use osmosis_std::{
    shim::Duration,
    types::{
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::{gamm::v1beta1::MsgJoinSwapExternAmountIn, lockup::MsgLockTokens},
    },
};

use quasar_types::ica::packet::ica_send;

use crate::{
    error::ContractError,
    helpers::{create_ibc_ack_submsg, get_ica_address, IbcMsgKind, IcaMessages},
    state::{OngoingDeposit, PendingBond, CONFIG, ICA_CHANNEL, SIMULATED_JOIN_RESULT},
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

pub fn calculate_share_out_min_amount(storage: &mut dyn Storage) -> Result<Uint128, ContractError> {
    let last_sim_join_pool_result = SIMULATED_JOIN_RESULT.load(storage)?;

    // todo: better dynamic slippage estimation, especially for volatile tokens
    // diminish the share_out_amount by 5 percent to allow for slippage of 5% on the swap
    Ok(
        Uint128::from_str(&last_sim_join_pool_result.share_out_amount)?
            .checked_multiply_ratio(95u128, 100u128)?,
    )
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
        Coin, Empty, IbcEndpoint, OwnedDeps, Uint128,
    };
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;

    use cw_storage_plus::Map;
    use osmosis_std::types::osmosis::gamm::v1beta1::QueryCalcJoinPoolSharesResponse;
    use quasar_types::{
        ibc::{ChannelInfo, ChannelType, HandshakeState},
        ica::handshake::IcaMetadata,
    };

    use crate::state::SIMULATED_JOIN_RESULT;

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
            .save(
                deps.as_mut().storage,
                &QueryCalcJoinPoolSharesResponse {
                    share_out_amount: "999999".to_string(),
                    tokens_out: vec![OsmoCoin {
                        denom: String::from("some-coin, does not matter"),
                        amount: "100".to_string(),
                    }],
                },
            )
            .unwrap();

        let min_amount_out = calculate_share_out_min_amount(deps.as_mut().storage).unwrap();

        assert_eq!(min_amount_out, Uint128::from(949999u128));
    }

    #[test]
    fn test_do_transfer() {}
}
