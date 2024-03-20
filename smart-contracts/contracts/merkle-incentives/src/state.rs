use crate::{incentives::CoinVec, ContractError};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Addr, Coin, Decimal, Fraction};
use cw_storage_plus::{Item, Map};

pub const CLAIMED_INCENTIVES: Map<Addr, CoinVec> = Map::new("claimed_incentives");
pub const MERKLE_ROOT: Item<String> = Item::new("merkle_root");
pub const INCENTIVES_ADMIN: Item<Addr> = Item::new("incentives_admin");
pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct ClaimAccount {
    pub proof: Vec<Vec<u8>>,
    pub coins: CoinVec,
}

#[cw_serde]
pub struct InstantiateConfig {
    pub clawback_address: Addr,
    pub start_block: u64,
    pub end_block: u64,
    pub expiration_block: u64,
    pub fee: Decimal,
    pub fee_address: Addr,
    pub total_incentives: Vec<Coin>,
}

impl TryInto<Config> for InstantiateConfig {
    type Error = ContractError;

    fn try_into(self) -> Result<Config, Self::Error> {
        let effective_incentives = self
            .total_incentives
            .iter()
            .map(|c| {
                let amount = c.amount
                    - (c.amount
                        .multiply_ratio(self.fee.numerator(), self.fee.denominator()));
                coin(amount.u128(), c.denom.as_str())
            })
            .collect();

        let effective_fees = self
            .total_incentives
            .iter()
            .map(|c| {
                let amount = c
                    .amount
                    .multiply_ratio(self.fee.numerator(), self.fee.denominator());
                coin(amount.u128(), c.denom.as_str())
            })
            .collect();

        let fee = Fee {
            fee_address: self.fee_address,
            fee: self.fee,
            effective_fees,
            effective_incentives,
        };

        Ok(Config {
            clawback_address: self.clawback_address,
            start_block: self.start_block,
            end_block: self.end_block,
            expiration_block: self.expiration_block,
            fee: Some(fee),
            total_incentives: self.total_incentives,
        })
    }
}

#[cw_serde]
pub struct Config {
    pub clawback_address: Addr,
    pub start_block: u64,
    pub end_block: u64,
    pub expiration_block: u64,
    pub total_incentives: Vec<Coin>,
    pub fee: Option<Fee>,
}

#[cw_serde]
pub struct Fee {
    pub fee_address: Addr,
    pub fee: Decimal,
    pub effective_fees: Vec<Coin>,
    pub effective_incentives: Vec<Coin>,
}
