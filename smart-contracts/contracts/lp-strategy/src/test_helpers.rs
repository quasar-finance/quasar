use cosmwasm_std::{IbcEndpoint, Storage};
use quasar_types::{
    ibc::{ChannelInfo, ChannelType},
    ica::handshake::IcaMetadata,
};

use crate::state::{Config, CHANNELS, CONFIG, ICA_CHANNEL, ICQ_CHANNEL, LP_SHARES};

pub fn default_setup(storage: &mut dyn Storage) -> Result<(), cosmwasm_std::StdError> {
    setup_default_icq(storage)?;
    setup_default_ica(storage)?;
    setup_default_config(storage)?;
    setup_default_lp_cache(storage)
}

pub(crate) fn setup_default_icq(storage: &mut dyn Storage) -> Result<(), cosmwasm_std::StdError> {
    let chan = "channel-1";
    CHANNELS.save(
        storage,
        chan.to_string(),
        &ChannelInfo {
            id: chan.to_string(),
            counterparty_endpoint: IbcEndpoint {
                port_id: "icqhost".to_string(),
                channel_id: chan.to_string(),
            },
            connection_id: "connection-0".to_string(),
            channel_type: ChannelType::Icq {
                channel_ty: "icq-1".to_string(),
            },
            handshake_state: quasar_types::ibc::HandshakeState::Open,
        },
    )?;
    ICQ_CHANNEL.save(storage, &chan.to_string())
}

pub(crate) fn setup_default_ica(storage: &mut dyn Storage) -> Result<(), cosmwasm_std::StdError> {
    let chan = "channel-2";
    CHANNELS.save(
        storage,
        chan.to_string(),
        &ChannelInfo {
            id: chan.to_string(),
            counterparty_endpoint: IbcEndpoint {
                port_id: "icahost".to_string(),
                channel_id: chan.to_string(),
            },
            connection_id: "connection-0".to_string(),
            channel_type: ChannelType::Ica {
                channel_ty: IcaMetadata::with_connections(
                    "connection-1".to_string(),
                    "connection-2".to_string(),
                ),
                counter_party_address: Some("osmo-address".to_string()),
            },
            handshake_state: quasar_types::ibc::HandshakeState::Open,
        },
    )?;
    ICA_CHANNEL.save(storage, &chan.to_string())
}

pub(crate) fn setup_default_config(
    storage: &mut dyn Storage,
) -> Result<(), cosmwasm_std::StdError> {
    CONFIG.save(
        storage,
        &Config {
            lock_period: 100,
            pool_id: 1,
            pool_denom: "gamm/pool/1".to_string(),
            base_denom: "uosmo".to_string(),
            quote_denom: "uqsr".to_string(),
            local_denom: "ibc/local_osmo".to_string(),
            transfer_channel: "channel-0".to_string(),
            return_source_channel: "channel-0".to_string(),
            expected_connection: "connection-0".to_string(),
        },
    )
}

pub(crate) fn setup_default_lp_cache(
    storage: &mut dyn Storage,
) -> Result<(), cosmwasm_std::StdError> {
    LP_SHARES.save(
        storage,
        &crate::state::LpCache {
            locked_shares: 100u128.into(),
            w_unlocked_shares: 100u128.into(),
            d_unlocked_shares: 100u128.into(),
        },
    )
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_dependencies;
    use quasar_types::types::{ItemShouldLoad, MapShouldLoad};

    use super::*;

    #[test]
    fn default_setup_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();

        assert_eq!(
            CONFIG.should_load(deps.as_ref().storage).unwrap(),
            Config {
                lock_period: 100,
                pool_id: 1,
                pool_denom: "gamm/pool/1".to_string(),
                base_denom: "uosmo".to_string(),
                quote_denom: "uqsr".to_string(),
                local_denom: "ibc/local_osmo".to_string(),
                transfer_channel: "channel-0".to_string(),
                return_source_channel: "channel-0".to_string(),
                expected_connection: "connection-0".to_string(),
            }
        );

        let ica_chan = ICA_CHANNEL.should_load(deps.as_ref().storage).unwrap();
        assert_eq!(
            CHANNELS
                .should_load(deps.as_ref().storage, ica_chan.clone())
                .unwrap(),
            ChannelInfo {
                id: ica_chan.clone(),
                counterparty_endpoint: IbcEndpoint {
                    port_id: "icahost".to_string(),
                    channel_id: ica_chan,
                },
                connection_id: "connection-0".to_string(),
                channel_type: ChannelType::Ica {
                    channel_ty: IcaMetadata::with_connections(
                        "connection-1".to_string(),
                        "connection-2".to_string()
                    ),
                    counter_party_address: Some("osmo-address".to_string()),
                },
                handshake_state: quasar_types::ibc::HandshakeState::Open,
            }
        );

        let icq_chan = ICQ_CHANNEL.should_load(deps.as_ref().storage).unwrap();
        assert_eq!(
            CHANNELS
                .should_load(deps.as_ref().storage, icq_chan.clone())
                .unwrap(),
            ChannelInfo {
                id: icq_chan.to_string(),
                counterparty_endpoint: IbcEndpoint {
                    port_id: "icqhost".to_string(),
                    channel_id: icq_chan,
                },
                connection_id: "connection-0".to_string(),
                channel_type: ChannelType::Icq {
                    channel_ty: "icq-1".to_string(),
                },
                handshake_state: quasar_types::ibc::HandshakeState::Open,
            }
        )
    }
}
