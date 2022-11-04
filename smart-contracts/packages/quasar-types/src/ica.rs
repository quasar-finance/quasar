use cosmwasm_std::{IbcChannel, IbcOrder};
use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json_wasm;

use crate::error::Error;

pub fn enforce_ica_order_and_metadata(channel: &IbcChannel, counterparty_metadata: Option<&str>) -> Result<(), Error> {
    enforce_order_and_version(channel, counterparty_metadata, &Version::Ics27_1, &Encoding::Proto3, &TxType::SdkMultiMsg, IbcOrder::Ordered)
}

// TODO add tests for all wrappers around types
pub fn enforce_order_and_version(
    channel: &IbcChannel,
    counterparty_metadata: Option<&str>,
    version: &Version,
    encoding: &Encoding,
    tx_type: &TxType,
    ordering: IbcOrder,
) -> Result<(), Error> {
    // we find the ica metadata in the version field as a string
    let metadata: IcaMetadata =
        serde_json_wasm::from_str(channel.version.as_str()).map_err(|err| {
            Error::InvalidIcaMetadata {
                raw_metadata: channel.version.clone(),
                error: err.to_string(),
            }
        })?;

    if metadata.version() != version {
        return Err(Error::InvalidIcaVersion {
            version: metadata.version().clone(),
            contract_version: version.clone(),
        });
    }
    if metadata.encoding() != encoding {
        return Err(Error::InvalidIcaEncoding {
            encoding: metadata.encoding().clone(),
            contract_encoding: encoding.clone(),
        });
    }
    if metadata.tx_type() != tx_type {
        return Err(Error::InvalidIcaTxType {
            tx_type: metadata.tx_type().clone(),
            contract_tx_type: tx_type.clone(),
        });
    }

    // TODO expand counterparty metadata parsing
    if let Some(metadata) = counterparty_metadata {
        let counterparty_metadata: CounterPartyIcaMetadata = serde_json_wasm::from_str(metadata)
            .map_err(|err| Error::InvalidCounterpartyIcaMetadata {
                raw_metadata: metadata.to_string(),
                error: err.to_string(),
            })?;
    }

    if channel.order != ordering {
        return Err(Error::IncorrectIbcOrder { expected: ordering, got: channel.order.clone() });
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct IcaMetadata {
    version: Version,
    encoding: Encoding,
    tx_type: TxType,
    controller_connection_id: Option<String>,
    host_connection_id: Option<String>,
}

impl IcaMetadata {
    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn encoding(&self) -> &Encoding {
        &self.encoding
    }

    pub fn tx_type(&self) -> &TxType {
        &self.tx_type
    }

    pub fn with_connections(
        controller_connection_id: String,
        host_connection_id: String,
    ) -> IcaMetadata {
        IcaMetadata {
            version: Version::Ics27_1,
            encoding: Encoding::Proto3,
            tx_type: TxType::SdkMultiMsg,
            controller_connection_id: Some(controller_connection_id),
            host_connection_id: Some(host_connection_id),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct CounterPartyIcaMetadata {
    version: Version,
    encoding: Encoding,
    tx_type: TxType,
    controller_connection_id: Option<String>,
    host_connection_id: Option<String>,
    address: Option<String>,
}

impl CounterPartyIcaMetadata {
    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn encoding(&self) -> &Encoding {
        &self.encoding
    }

    pub fn tx_type(&self) -> &TxType {
        &self.tx_type
    }

    pub fn with_connections(
        controller_connection_id: String,
        host_connection_id: String,
    ) -> IcaMetadata {
        IcaMetadata {
            version: Version::Ics27_1,
            encoding: Encoding::Proto3,
            tx_type: TxType::SdkMultiMsg,
            controller_connection_id: Some(controller_connection_id),
            host_connection_id: Some(host_connection_id),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Display)]
pub enum Version {
    #[serde(rename = "ics27-1")]
    #[display(fmt = "ics27-1")]
    Ics27_1,
}


impl Default for Version {
    fn default() -> Self { Version::Ics27_1 }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Display)]
pub enum Encoding {
    #[serde(rename = "proto3")]
    #[display(fmt = "proto3")]
    Proto3,
}

impl Default for Encoding {
    fn default() -> Self { Encoding::Proto3 }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Display)]
pub enum TxType {
    #[serde(rename = "sdk_multi_msg")]
    #[display(fmt = "sdk_multi_msg")]
    SdkMultiMsg,
}

impl Default for TxType {
    fn default() -> Self { TxType::SdkMultiMsg }
}

impl CounterPartyIcaMetadata {
    pub fn get_counterpary_ica_address(counterparty_version: &str) -> Result<String, Error> {
        let counterparty_metadata: CounterPartyIcaMetadata =
            serde_json_wasm::from_str(counterparty_version).map_err(|err| {
                Error::InvalidCounterpartyIcaMetadata {
                    raw_metadata: counterparty_version.to_string(),
                    error: err.to_string(),
                }
            })?;
        counterparty_metadata
            .address
            .ok_or(Error::NoCounterpartyIcaAddress {})
    }
}
