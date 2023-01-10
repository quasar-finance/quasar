use cosmwasm_std::{to_binary, Addr, Coin, Env, IbcMsg, IbcTimeout, Storage, SubMsg, Uint128};
use osmosis_std::{
    shim::Duration,
    types::{
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::{gamm::v1beta1::MsgJoinSwapExternAmountIn, lockup::MsgLockTokens},
    },
};

use quasar_types::{
    ibc::ChannelType,
    ica::{
        packet::{InterchainAccountPacketData, Type},
        traits::Pack,
    },
};

use crate::{
    error::ContractError,
    helpers::{create_ibc_ack_submsg, IbcMsgKind, IcaMessages},
    state::{PendingAck, CHANNELS, CONFIG, JOINED_FUNDS, TRANSFERRED_FUNDS},
};

pub fn do_transfer(
    storage: &mut dyn Storage,
    env: Env,
    sender: Addr,
    amount: Uint128,
    channel_id: String,
    to_address: String,
    current: Uint128,
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
        amount: coin.clone(),
        timeout,
    };

    TRANSFERRED_FUNDS.save(storage, current.u128(), &amount)?;

    Ok(create_ibc_ack_submsg(
        storage,
        PendingAck {
            kind: IbcMsgKind::Transfer,
            address: sender,
            amount: coin.amount,
            id: current,
        },
        transfer,
    )?)
}

/// prepare the submsg for joining the pool
#[allow(clippy::too_many_arguments)] // allowing this is not ideal, but for now we want to keep channel_id and pool_id in there
pub fn do_ibc_join_pool_swap_extern_amount_in(
    storage: &mut dyn Storage,
    env: Env,
    channel_id: String,
    pool_id: u64,
    denom: String,
    amount: Uint128,
    share_out_min_amount: Uint128,
    sender: Addr,
    current: Uint128,
) -> Result<SubMsg, ContractError> {
    let channel = CHANNELS.load(storage, channel_id.clone())?;
    if let ChannelType::Ica {
        channel_ty: _,
        counter_party_address,
    } = channel.channel_type
    {
        // make sure we have a counterparty address
        if counter_party_address.is_none() {
            return Err(ContractError::NoCounterpartyIcaAddress);
        }

        TRANSFERRED_FUNDS.remove(storage, current.u128());
        JOINED_FUNDS.save(storage, current.u128(), &amount)?;

        // setup the first IBC message to send, and save the entire sequence so we have acces to it on acks
        let join = MsgJoinSwapExternAmountIn {
            sender: counter_party_address.unwrap(),
            pool_id,
            token_in: Some(OsmoCoin {
                denom,
                amount: amount.to_string(),
            }),
            share_out_min_amount: share_out_min_amount.to_string(),
        };

        let packet = InterchainAccountPacketData::new(Type::ExecuteTx, vec![join.pack()], None);

        let send_packet_msg = IbcMsg::SendPacket {
            channel_id: channel_id,
            data: to_binary(&packet)?,
            timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
        };

        Ok(create_ibc_ack_submsg(
            storage,
            PendingAck {
                kind: IbcMsgKind::Ica(IcaMessages::JoinSwapExternAmountIn),
                address: sender,
                amount,
                id: current,
            },
            send_packet_msg,
        )?)
    } else {
        Err(ContractError::NoIcaChannel)
    }
}

pub fn do_ibc_lock_tokens(
    _deps: &mut dyn Storage,
    owner: String,
    coins: Vec<Coin>,
) -> Result<InterchainAccountPacketData, ContractError> {
    // denom in this case is expected to be something like gamm/pool/1
    // duration is  60 sec/min * 60 min/hr * 24hr * 14days
    // TODO move the duration to a package and make it settable
    let lock = MsgLockTokens {
        owner,
        duration: Some(Duration {
            seconds: 1209600,
            nanos: 0,
        }),
        coins: coins
            .iter()
            .map(|c| OsmoCoin {
                denom: c.denom.clone(),
                amount: c.amount.to_string(),
            })
            .collect(),
    };
    Ok(InterchainAccountPacketData::new(
        Type::ExecuteTx,
        vec![lock.pack()],
        None,
    ))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, MockApi, MockQuerier, MockStorage},
        Empty, IbcEndpoint, OwnedDeps,
    };
    use cw_storage_plus::Map;
    use quasar_types::{
        ibc::{ChannelInfo, ChannelType, HandshakeState},
        ica::handshake::IcaMetadata,
    };

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
    fn test_do_transfer() {}
}
