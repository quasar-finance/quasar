use cosmwasm_std::{CosmosMsg, Empty, IbcAcknowledgement, IbcMsg, Response};
use cw_storage_plus::Map;
use intergamm_bindings::{
    msg::{AckResponse, IntergammMsg},
    types::IntergammAck,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Message {
    Intergamm { msg: IntergammMsg },
    Ibc { msg: IbcMsg },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Ack {
    Intergamm { ack: IntergammAck },
    Ibc { ack: IbcAcknowledgement },
}

// Next is an absolutely disgusting but sadly necessary enum that has to be extended by users of the vault contract
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Callback {
    One,
    Two,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct IbcBuilder {}

const acks: Map<u128, Message> = Map::new("acks");

impl IbcBuilder {
    pub fn new() -> IbcBuilder {
        todo!()
    }

    fn execute(self, msg: Message, callback: Callback) -> (Self, CosmosMsg<IntergammMsg>) {
        match msg {
            Message::Intergamm { msg } => (self, CosmosMsg::Custom(msg)),
            Message::Ibc { msg } => (self, CosmosMsg::Ibc(msg)),
        }
    }

    // handle takes an acknowledgement,
    fn handle(self, original: Message, ack: Ack) {
        // here devs need to insert there functions handling
        match ack {
            // TODO now we need to attach the outgoing sequence number to the sent messages here, so that
            // we can couple the correct origin calls to incoming acks. The map for the sequence numbers can be hidden
            // from callers, since all callers should be using ibc builder to do IBC actions
            Ack::Intergamm { ack } => todo!(),
            Ack::Ibc { ack } => todo!(),
        }
    }

    fn handle_intergamm() {}
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use intergamm_bindings::msg::IntergammMsg::RegisterIcaOnZone;

    #[test]
    fn try_ibc_builder() {
        let b = IbcBuilder::new().execute(
            Message::Intergamm {
                msg: RegisterIcaOnZone {
                    zone_id: "cosmos".to_string(),
                },
            },
            Callback::One,
        );
    }
}
