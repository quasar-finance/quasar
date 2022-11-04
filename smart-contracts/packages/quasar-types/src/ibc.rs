use cosmwasm_std::{IbcChannel, IbcOrder};

use crate::error::Error;



pub fn enforce_order_and_version(
    channel: &IbcChannel,
    counterparty_version: Option<&str>,
    contract_version: &str,
    ordering: IbcOrder
) -> Result<(), Error> {
    if channel.version != contract_version {
        return Err(Error::InvalidIbcVersion {
            version: channel.version.clone(),
        });
    }
    if let Some(version) = counterparty_version {
        if version != contract_version {
            return Err(Error::InvalidIbcVersion {
                version: version.to_string(),
            });
        }
    }
    if channel.order != ordering {
        return Err(Error::IncorrectIbcOrder { expected: ordering, got: channel.order.clone() });
    }
    Ok(())
}