#![cfg(test)]
use crate::contract::instantiate;
use crate::ibc::{ibc_channel_connect, ibc_channel_open, ICA_ORDERING, CounterPartyIcaMetadata, VERSION, ENCODING, TX_TYPE};
use crate::state::ChannelInfo;

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{
    DepsMut, IbcChannel, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcEndpoint, OwnedDeps,
};

use crate::msg::InitMsg;

pub const DEFAULT_TIMEOUT: u64 = 3600; // 1 hour,
pub const CONTRACT_PORT: &str = "ibc:wasm1234567890abcdef";
pub const REMOTE_PORT: &str = "icahost";
pub const CONNECTION_ID: &str = "connection-2";

pub fn mock_channel(channel_id: &str) -> IbcChannel {
    IbcChannel::new(
        IbcEndpoint {
            port_id: CONTRACT_PORT.into(),
            channel_id: channel_id.into(),
        },
        IbcEndpoint {
            port_id: REMOTE_PORT.into(),
            channel_id: format!("{}5", channel_id),
        },
        ICA_ORDERING,
        r#"{
            "version":"ics27-1",
            "encoding":"proto3",
            "tx_type":"sdk_multi_msg",
            "controller_connection_id":"connection-0",
            "host_connection_id":"connection-0"
          }"#,
        CONNECTION_ID,
    )
}

pub fn mock_channel_info(channel_id: &str) -> ChannelInfo {
    ChannelInfo {
        id: channel_id.to_string(),
        counterparty_endpoint: IbcEndpoint {
            port_id: REMOTE_PORT.into(),
            channel_id: format!("{}5", channel_id),
        },
        connection_id: CONNECTION_ID.into(),
        address: "osmo1qj7gcx4m2zzcsy4y9frwd405xdm78ax48rkq5ep05k4358mdp8cskjye07".to_string(),
    }
}

// we simulate instantiate and ack here
pub fn add_channel(mut deps: DepsMut, channel_id: &str) {
    let channel = mock_channel(channel_id);
    let open_msg = IbcChannelOpenMsg::new_init(channel.clone());
    ibc_channel_open(deps.branch(), mock_env(), open_msg).unwrap();
    let connect_msg = IbcChannelConnectMsg::new_ack(channel, serde_json_wasm::to_string(&CounterPartyIcaMetadata { version: VERSION.into(), encoding: ENCODING.into(), tx_type: TX_TYPE.into(), controller_connection_id: Some("connection-0".to_string()), host_connection_id: Some("connection-0".to_string()), address: None }).unwrap());
    ibc_channel_connect(deps.branch(), mock_env(), connect_msg).unwrap();
}

pub fn setup(channels: &[&str]) -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    // instantiate an empty contract
    let instantiate_msg = InitMsg {
        default_timeout: DEFAULT_TIMEOUT,
    };
    let info = mock_info(&String::from("anyone"), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    for channel in channels {
        add_channel(deps.as_mut(), channel);
    }
    deps
}
