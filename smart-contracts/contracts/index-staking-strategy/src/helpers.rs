use cosmwasm_std::{Addr, Deps, Order, Uint128, Reply, StdError};
use quasar_traits::traits::Curve;
use crate::{error::ContractError};


pub(crate) fn parse_seq(reply: Reply) -> Result<u64, StdError> {
    reply
    .result
    .into_result()
    .map_err(|msg| StdError::GenericErr { msg })?
    .events
    .iter()
    .find(|e| e.ty == "send_packet")
    .ok_or(StdError::NotFound {
        kind: "send_packet_event".into(),
    })?
    .attributes
    .iter()
    .find(|attr| attr.key == "packet_sequence")
    .ok_or(StdError::NotFound {
        kind: "packet_sequence".into(),
    })?
    .value
    .parse::<u64>()
    .map_err(|e| StdError::ParseErr {
        target_type: "u64".into(),
        msg: e.to_string(),
    })
}

#[cfg(test)]
mod tests {
    // TODO write some tests for helpers
}