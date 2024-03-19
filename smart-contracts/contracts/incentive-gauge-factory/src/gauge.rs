use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

#[cw_serde]
pub struct Gauge {
    pub start_block: Uint128,
    pub end_block: Uint128,
    pub expiration_block: Uint128,
    pub total_incentives: Vec<Coin>,
    pub fee: Decimal,
    pub fee_receiver: Addr,
    pub r#type: GaugeType,
}

impl Gauge {
    pub fn new(
        start_block: Uint128,
        end_block: Uint128,
        expiration_block: Uint128,
        total_incentives: Vec<Coin>,
        fee: Decimal,
        fee_receiver: Addr,
        r#type: GaugeType,
    ) -> Gauge {
        Gauge {
            start_block,
            end_block,
            expiration_block,
            total_incentives,
            fee,
            fee_receiver,
            r#type,
        }
    }
}

#[cw_serde]
pub enum GaugeType {
    Vault { address: Addr },
}

impl GaugeType {
    pub fn new_vault_incentives(address: Addr) -> GaugeType {
        return GaugeType::Vault { address };
    }
}

pub struct VaultConfig {
    // a mint amount of shares needed to receive incentives
    minimum_shares: Option<Uint128>,
    // a maximum amount of shares
    maximum_shares: Option<Uint128>,
}
