use std::ops::Add;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Uint128, coin, Fraction};

#[cw_serde]
pub struct Gauge {
    pub start_block: Uint128,
    pub end_block: Uint128,
    pub expiration_block: Uint128,
    pub total_incentives: Vec<Coin>,
    pub fee: Fee,
    pub r#type: GaugeType,
}

impl Gauge {
    pub fn new(
        start_block: Uint128,
        end_block: Uint128,
        expiration_block: Uint128,
        total_incentives: Vec<Coin>,
        fee: Decimal,
        fee_address: Addr,
        r#type: GaugeType,
    ) -> Gauge {
        let fee =  Fee::new(fee_address, fee, &total_incentives);

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
    pub total_fees: Vec<Coin>,
    pub remaining_fees: Vec<Coin>,
}

impl Fee {
    pub fn new(fee_address: Addr, fee_ratio: Decimal, total_incentives: &[Coin]) -> Fee {
        let total_fees: Vec<Coin> = total_incentives
        .iter()
        .map(|c| {
            let amount = c
                .amount
                .multiply_ratio(fee_ratio.numerator(), fee_ratio.denominator());
            coin(amount.u128(), c.denom.as_str())
        })
        .collect();

        Fee { fee_address, fee_ratio, total_fees: total_fees.clone(), remaining_fees: total_fees }
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
