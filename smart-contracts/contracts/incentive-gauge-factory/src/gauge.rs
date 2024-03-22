use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Addr, Coin, Decimal, Fraction, Uint128};
use quasar_types::coinlist::CoinList;

#[cw_serde]
pub struct Gauge {
    pub start_block: u64,
    pub end_block: u64,
    pub expiration_block: u64,
    pub total_incentives: Vec<Coin>,
    // TODO remove the fee from the gauge and move it to a secondary map so we can effciently update the fee on claiming
    pub fee: Fee,
    pub r#type: GaugeType,
}

impl Gauge {
    pub fn new(
        start_block: u64,
        end_block: u64,
        expiration_block: u64,
        total_incentives: Vec<Coin>,
        fee: Decimal,
        fee_address: Addr,
        r#type: GaugeType,
    ) -> Gauge {
        let fee = Fee::new(fee_address, fee, CoinList::new(total_incentives.clone()));

        Gauge {
            start_block,
            end_block,
            expiration_block,
            total_incentives,
            fee,
            r#type,
        }
    }
}

#[cw_serde]
pub struct Fee {
    pub fee_address: Addr,
    pub fee_ratio: Decimal,
    pub total_fees: CoinList,
    pub remaining_fees: CoinList,
}

impl Fee {
    pub fn new(fee_address: Addr, fee_ratio: Decimal, total_incentives: CoinList) -> Fee {
        let total_fees = total_incentives.mul_ratio(fee_ratio);

        Fee {
            fee_address,
            fee_ratio,
            total_fees: total_fees.clone(),
            remaining_fees: total_fees,
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
