use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::error::ContractError;

use cosmwasm_std::{
    attr, entry_point, from_binary, to_binary, BankMsg, Binary, CosmosMsg, DepsMut, Env,
    IbcAcknowledgement, IbcBasicResponse, IbcChannel, IbcEndpoint, IbcOrder, IbcPacket,
    IbcReceiveResponse, StdResult, Uint128, WasmMsg,
};

pub const STRATEGY_VERSION:&str = "strategy-1";
pub const ICS20_ORDERING: IbcOrder = IbcOrder::Unordered;

/// The funds in a StrategyPacket are mean for the contract receiving the packet and should be used
/// from there according to the
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StrategyPacket {
    /// amount of tokens to transfer is encoded as a string, but limited to u64 max
    pub amount: Uint128,
    /// the token denomination to be transferred
    pub denom: String,
    /// the sender address
    pub sender: String,
    // extend these packets with any additional information needed in your strategy
}

impl StrategyPacket {
    pub fn new<T: Into<String>>(amount: Uint128, denom: T, sender: &str, receiver: &str) -> Self {
        Ics20Packet {
            denom: denom.into(),
            amount,
            sender: sender.to_string(),
            receiver: receiver.to_string(),
        }
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        if self.amount.u128() > (u64::MAX as u128) {
            Err(ContractError::AmountOverflow {})
        } else {
            Ok(())
        }
    }
}


/// This is a generic ICS acknowledgement format.
/// Proto defined here: https://github.com/cosmos/cosmos-sdk/blob/v0.42.0/proto/ibc/core/channel/v1/channel.proto#L141-L147
/// This is compatible with the JSON serialization
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StrategyAck {
    Result(Binary),
    Error(String),
}

// TODO we probably want to ACK with the owners address, at least on withdrawals, so that if a withdraw fails, the burned tokens
//  on the host chain are returned to the user correctly
// create a serialized success message
fn ack_success() -> Binary {
    let res = Ics20Ack::Result(b"1".into());
    to_binary(&res).unwrap()
}

// create a serialized error message
fn ack_fail(err: String) -> Binary {
    let res = Ics20Ack::Error(err);
    to_binary(&res).unwrap()
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// enforces ordering and versioning constraints
pub fn ibc_channel_open(
    _deps: DepsMut,
    _env: Env,
    channel: IbcChannel,
) -> Result<(), ContractError> {
    enforce_order_and_version(&channel)?;
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// record the channel in CHANNEL_INFO
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    channel: IbcChannel,
) -> Result<IbcBasicResponse, ContractError> {
    // we need to check the counter party version in try and ack (sometimes here)
    enforce_order_and_version(&channel)?;

    let info = ChannelInfo {
        id: channel.endpoint.channel_id,
        counterparty_endpoint: channel.counterparty_endpoint,
        connection_id: channel.connection_id,
    };
    CHANNEL_INFO.save(deps.storage, &info.id, &info)?;

    Ok(IbcBasicResponse::default())
}

fn enforce_order_and_version(channel: &IbcChannel) -> Result<(), ContractError> {
    if channel.version != ICS20_VERSION {
        return Err(ContractError::InvalidIbcVersion {
            version: channel.version.clone(),
        });
    }
    if let Some(version) = &channel.counterparty_version {
        if version != ICS20_VERSION {
            return Err(ContractError::InvalidIbcVersion {
                version: version.clone(),
            });
        }
    }
    if channel.order != ICS20_ORDERING {
        return Err(ContractError::OnlyOrderedChannel {});
    }
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// Check to see if we have any balance here
/// We should not return an error if possible, but rather an acknowledgement of failure
pub fn ibc_packet_receive(
    deps: DepsMut,
    _env: Env,
    packet: IbcPacket,
) -> Result<IbcReceiveResponse, Never> {
    let res = match do_ibc_packet_receive(deps, &packet) {
        Ok(msg) => {
            // build attributes first so we don't have to clone msg below
            // similar event messages like ibctransfer module

            // This cannot fail as we parse it in do_ibc_packet_receive. Best to pass the data somehow?
            let denom = parse_voucher_denom(&msg.denom, &packet.src).unwrap();

            let attributes = vec![
                attr("action", "receive"),
                attr("sender", &msg.sender),
                attr("receiver", &msg.receiver),
                attr("denom", denom),
                attr("amount", msg.amount),
                attr("success", "true"),
            ];
            let to_send = Amount::from_parts(denom.into(), msg.amount);
            let msg = send_amount(to_send, msg.receiver);
            IbcReceiveResponse {
                acknowledgement: ack_success(),
                submessages: vec![],
                messages: vec![msg],
                attributes,
            }
        }
        Err(err) => IbcReceiveResponse {
            acknowledgement: ack_fail(err.to_string()),
            submessages: vec![],
            messages: vec![],
            attributes: vec![
                attr("action", "receive"),
                attr("success", "false"),
                attr("error", err),
            ],
        },
    };

    // if we have funds, now send the tokens to the requested recipient
    Ok(res)
}

// Returns local denom if the denom is an encoded voucher from the expected endpoint
// Otherwise, error
fn parse_voucher_denom<'a>(
    voucher_denom: &'a str,
    remote_endpoint: &IbcEndpoint,
) -> Result<&'a str, ContractError> {
    let split_denom: Vec<&str> = voucher_denom.splitn(3, '/').collect();
    if split_denom.len() != 3 {
        return Err(ContractError::NoForeignTokens {});
    }
    // a few more sanity checks
    if split_denom[0] != remote_endpoint.port_id {
        return Err(ContractError::FromOtherPort {
            port: split_denom[0].into(),
        });
    }
    if split_denom[1] != remote_endpoint.channel_id {
        return Err(ContractError::FromOtherChannel {
            channel: split_denom[1].into(),
        });
    }

    Ok(split_denom[2])
}

// this does the work of ibc_packet_receive, we wrap it to turn errors into acknowledgements
fn do_ibc_packet_receive(deps: DepsMut, packet: &IbcPacket) -> Result<Ics20Packet, ContractError> {
    let msg: Ics20Packet = from_binary(&packet.data)?;
    let channel = packet.dest.channel_id.clone();

    // If the token originated on the remote chain, it looks like "ucosm".
    // If it originated on our chain, it looks like "port/channel/ucosm".
    let denom = parse_voucher_denom(&msg.denom, &packet.src)?;

    let amount = msg.amount;
    CHANNEL_STATE.update(
        deps.storage,
        (&channel, denom),
        |orig| -> Result<_, ContractError> {
            // this will return error if we don't have the funds there to cover the request (or no denom registered)
            let mut cur = orig.ok_or(ContractError::PaymentError(cw_utils) )?;
            cur.outstanding = cur
                .outstanding
                .checked_sub(amount)
                .or(Err(ContractError::InsufficientFunds {}))?;
            Ok(cur)
        },
    )?;
    Ok(msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// check if success or failure and update balance, or return funds
pub fn ibc_packet_ack(
    deps: DepsMut,
    _env: Env,
    ack: IbcAcknowledgement,
) -> Result<IbcBasicResponse, ContractError> {
    // TODO: trap error like in receive?
    let msg: Ics20Ack = from_binary(&ack.acknowledgement)?;
    match msg {
        Ics20Ack::Result(_) => on_packet_success(deps, ack.original_packet),
        Ics20Ack::Error(err) => on_packet_failure(deps, ack.original_packet, err),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// return fund to original sender (same as failure in ibc_packet_ack)
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    packet: IbcPacket,
) -> Result<IbcBasicResponse, ContractError> {
    // TODO: trap error like in receive?
    on_packet_failure(deps, packet, "timeout".to_string())
}

// update the balance stored on this (channel, denom) index
fn on_packet_success(deps: DepsMut, packet: IbcPacket) -> Result<IbcBasicResponse, ContractError> {
    let msg: Ics20Packet = from_binary(&packet.data)?;
    // similar event messages like ibctransfer module
    let attributes = vec![
        attr("action", "acknowledge"),
        attr("sender", &msg.sender),
        attr("receiver", &msg.receiver),
        attr("denom", &msg.denom),
        attr("amount", msg.amount),
        attr("success", "true"),
    ];

    let channel = packet.src.channel_id;
    let denom = msg.denom;
    let amount = msg.amount;
    CHANNEL_STATE.update(deps.storage, (&channel, &denom), |orig| -> StdResult<_> {
        let mut state = orig.unwrap_or_default();
        state.outstanding += amount;
        state.total_sent += amount;
        Ok(state)
    })?;

    Ok(IbcBasicResponse {
        submessages: vec![],
        messages: vec![],
        attributes,
    })
}

// return the tokens to sender
fn on_packet_failure(
    _deps: DepsMut,
    packet: IbcPacket,
    err: String,
) -> Result<IbcBasicResponse, ContractError> {
    let msg: Ics20Packet = from_binary(&packet.data)?;
    // similar event messages like ibctransfer module
    let attributes = vec![
        attr("action", "acknowledge"),
        attr("sender", &msg.sender),
        attr("receiver", &msg.receiver),
        attr("denom", &msg.denom),
        attr("amount", &msg.amount),
        attr("success", "false"),
        attr("error", err),
    ];

    let amount = Amount::from_parts(msg.denom, msg.amount);
    let msg = send_amount(amount, msg.sender);
    let res = IbcBasicResponse {
        submessages: vec![],
        messages: vec![msg],
        attributes,
    };
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::*;

    use crate::contract::query_channel;
    use cosmwasm_std::testing::mock_env;
    use cosmwasm_std::{coins, to_vec, IbcEndpoint};

    #[test]
    fn check_ack_json() {
        let success = StrategyAck::Result(b"1".into());
        let fail = StrategyAck::Error("bad coin".into());

        let success_json = String::from_utf8(to_vec(&success).unwrap()).unwrap();
        assert_eq!(r#"{"result":"MQ=="}"#, success_json.as_str());

        let fail_json = String::from_utf8(to_vec(&fail).unwrap()).unwrap();
        assert_eq!(r#"{"error":"bad coin"}"#, fail_json.as_str());
    }

    #[test]
    fn check_packet_json() {
        let packet = StrategyPacket::new(
            Uint128(12345),
            "ucosm",
            "cosmos1zedxv25ah8fksmg2lzrndrpkvsjqgk4zt5ff7n",
            "wasm1fucynrfkrt684pm8jrt8la5h2csvs5cnldcgqc",
        );
        // Example message generated from the SDK
        let expected = r#"{"amount":"12345","denom":"ucosm","receiver":"wasm1fucynrfkrt684pm8jrt8la5h2csvs5cnldcgqc","sender":"cosmos1zedxv25ah8fksmg2lzrndrpkvsjqgk4zt5ff7n"}"#;

        let encdoded = String::from_utf8(to_vec(&packet).unwrap()).unwrap();
        assert_eq!(expected, encdoded.as_str());
    }
}