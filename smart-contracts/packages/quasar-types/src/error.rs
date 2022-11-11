use cosmwasm_std::IbcOrder;
use thiserror::Error;

use crate::ica::{Encoding, TxType, Version};

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("No Counterparty Ica Address")]
    NoCounterpartyIcaAddress,

    #[error("Could not deserialize counterparty ica metadata, got {raw_metadata}, error: {error}")]
    InvalidCounterpartyIcaMetadata { raw_metadata: String, error: String },

    #[error("Could not deserialize ica metadata, got {raw_metadata}, error: {error}")]
    InvalidIcaMetadata { raw_metadata: String, error: String },

    #[error("Incorrect ICA version, got {version}, want {contract_version}")]
    InvalidIcaVersion {
        version: Version,
        contract_version: Version,
    },

    #[error("Incorrect ICA version, got {encoding}, want {contract_encoding}")]
    InvalidIcaEncoding {
        encoding: Encoding,
        contract_encoding: Encoding,
    },

    #[error("Incorrect ICA version, got {tx_type}, want {contract_tx_type}")]
    InvalidIcaTxType {
        tx_type: TxType,
        contract_tx_type: TxType,
    },

    #[error("Incorrect IbcOrder")]
    IncorrectIbcOrder { expected: IbcOrder, got: IbcOrder },

    #[error("invalid Ibc version")]
    InvalidIbcVersion { version: String },
}
