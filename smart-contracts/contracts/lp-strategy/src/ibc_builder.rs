use cosmwasm_std::{
    CosmosMsg, DepsMut, Empty, IbcAcknowledgement, IbcMsg, IbcPacket, Response, StdError, StdResult,
};
use cw_storage_plus::Map;
use intergamm_bindings::{
    msg::{AckResponse, IntergammMsg},
    types::IntergammAck,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

const ACKS: Map<u64, Callback> = Map::new("acks");

impl IbcBuilder {
    pub fn new() -> IbcBuilder {
        todo!()
    }

    fn execute(self, msg: IbcMsg, callback: Callback) -> (Self, CosmosMsg<IntergammMsg>) {
        todo!()
    }

    // handle takes an acknowledgement,
    fn handle(self, deps: DepsMut, ack: IbcPacket) -> StdResult<Response> {
        let callback = ACKS.load(deps.storage, ack.sequence)?;
        // here devs need to insert there functions handling
        match callback {
            Callback::One => todo!(),
            Callback::Two => todo!(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use intergamm_bindings::msg::IntergammMsg::RegisterIcaOnZone;
}
