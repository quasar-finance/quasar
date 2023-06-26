use cosmos_sdk_proto::tendermint::abci::ResponseQuery;
use cosmwasm_std::{IbcEndpoint, Storage};
use prost::bytes::Bytes;
use quasar_types::{
    ibc::{ChannelInfo, ChannelType},
    ica::handshake::IcaMetadata,
};

use crate::{
    bond::Bond,
    state::{
        Config, PendingBond, RawAmount, CHANNELS, CONFIG, ICA_CHANNEL, ICQ_CHANNEL, LP_SHARES,
    },
};

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

pub fn pending_bond_to_bond(pending: &PendingBond) -> Vec<Bond> {
    pending
        .bonds
        .iter()
        .map(|bond| Bond {
            amount: match &bond.raw_amount {
                RawAmount::LocalDenom(amount) => *amount,
                RawAmount::LpShares(_) => panic!("unexpected lp shares"),
            },
            owner: bond.owner.clone(),
            bond_id: bond.bond_id.clone(),
        })
        .collect()
}

pub fn create_query_response(response: Vec<u8>) -> ResponseQuery {
    ResponseQuery {
        code: 1,
        log: "".to_string(),
        info: "".to_string(),
        index: 1,
        key: Bytes::from("0"),
        value: response.into(),
        proof_ops: None,
        height: 0,
        codespace: "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_dependencies, Addr, Uint128};

    use crate::state::OngoingDeposit;

    use super::*;

    #[test]
    fn default_setup_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();

        assert_eq!(
            CONFIG.load(deps.as_ref().storage).unwrap(),
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

        let ica_chan = ICA_CHANNEL.load(deps.as_ref().storage).unwrap();
        assert_eq!(
            CHANNELS
                .load(deps.as_ref().storage, ica_chan.clone())
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

        let icq_chan = ICQ_CHANNEL.load(deps.as_ref().storage).unwrap();
        assert_eq!(
            CHANNELS
                .load(deps.as_ref().storage, icq_chan.clone())
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

    #[test]
    fn test_pending_bond_to_bond_works() {
        let pb = PendingBond {
            bonds: vec![
                OngoingDeposit {
                    claim_amount: Uint128::new(100),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(1000)),
                    owner: Addr::unchecked("address"),
                    bond_id: "1".to_string(),
                },
                OngoingDeposit {
                    claim_amount: Uint128::new(99),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(999)),
                    owner: Addr::unchecked("address"),
                    bond_id: "2".to_string(),
                },
                OngoingDeposit {
                    claim_amount: Uint128::new(101),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(1000)),
                    owner: Addr::unchecked("address"),
                    bond_id: "3".to_string(),
                },
            ],
        };

        let bonds = pending_bond_to_bond(&pb);
        assert_eq!(bonds[0].amount, Uint128::new(1000));
        assert_eq!(bonds[1].amount, Uint128::new(999));
        assert_eq!(bonds[2].amount, Uint128::new(1000));
        assert_eq!(bonds[0].owner, Addr::unchecked("address"));
        assert_eq!(bonds[1].owner, Addr::unchecked("address"));
        assert_eq!(bonds[2].owner, Addr::unchecked("address"));
        assert_eq!(bonds[0].bond_id, "1");
        assert_eq!(bonds[1].bond_id, "2");
        assert_eq!(bonds[2].bond_id, "3");
    }
}
