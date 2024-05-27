use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Env, Uint128};
use quasar_types::coinlist::CoinList;

use crate::ContractError;

#[cw_serde]
pub struct BlockPeriod {
    /// start of the period
    pub start: u64,

    /// end of the period
    pub end: u64,

    /// expiration of the period
    pub expiry: u64,
}

impl BlockPeriod {
    pub fn check_conf(&self, env: Env) -> Result<(), ContractError> {
        if env.block.height.ge(&self.start)  {
            return Err(ContractError::StartTimeMustBeAhead)
        }

        if env.block.height.ge(&self.end)  {
            return Err(ContractError::EndTimeMustBeAhead)
        }

        if env.block.height.ge(&self.expiry)  {
            return Err(ContractError::ExpiryTimeMustBeAhead)
        }

        if self.end.le(&self.start) {
            return Err(ContractError::EndTimeBiggerThanStart)
        }

        if self.expiry.le(&self.start) {
            return Err(ContractError::ExpiryTimeBiggerThanStart)
        }

        Ok(())
    }
}

#[cw_serde]
pub struct GaugeInProcess {
    pub gauge: Gauge,
    pub kind: GaugeKind,
    pub fee: Fee,
}

#[cw_serde]
pub struct Gauge {
    pub period: BlockPeriod,
    pub incentives: Vec<Coin>,
    // pub kind: GaugeKind,
    pub clawback: String,
}

impl Gauge {
    pub fn new(
        period: BlockPeriod,
        incentives: Vec<Coin>,
        // kind: GaugeKind,
        clawback: String,
    ) -> Self {
        // let fee = Fee::new(fee_address, fee, CoinList::new(total_incentives.clone()));
        Self {
            period,
            incentives,
            // kind,
            clawback,
        }
    }
}

#[cw_serde]
pub struct Fee {
    /// this is the address that is configured to recieve the fee
    pub reciever: String,

    /// ratio to charge
    pub ratio: Decimal,

    /// total calculated fees
    pub total: CoinList,

    /// remaining fees to be collected
    pub remaining: CoinList,
}

impl Fee {
    pub fn new(reciever: String, ratio: Decimal, total_incentives: CoinList) -> Self {
        let total = total_incentives.mul_ratio(ratio);

        Self {
            reciever,
            ratio,
            total: total.clone(),
            remaining: total,
        }
    }
}

#[cw_serde]
pub enum PoolKind {
    Volume = 1,
    Liquidity,
}

/// The different kinds of incentive gauges supported by Quasar
/// Each kind of gauge is created in the incentive gauge factory
/// The offchain infrastructure picks up the settings from the onchain created gauge
#[cw_serde]
pub enum GaugeKind {
    /// The gauge type to incentivize a Quasar vault.
    /// address is the contract address of the corresponding Quasar vault to incentivize
    /// blacklist gives support to blacklist certain addresses, such as contracts that deposit into the vault but do not have the capability to claim any incentives
    /// min_shares is an optional setting to define a minimum amount of shares needed to earn any incentives
    /// max_shares is an optional setting to define a maximum amount of shares a user can earn any incentives over, any users over the max amount are given rewards according to the max_shares amount
    Vault {
        address: Addr,
        blacklist: Option<Vec<Addr>>,
        min_shares: Option<Uint128>,
        max_shares: Option<Uint128>,
    },

    /// Pool guage incentivization.
    /// There are two types of [PoolKind] PoolKind::Volume and PoolKind::Liquidity.
    /// To incentivize a specific token or both use denom_a and denom_b for the names of the tokens.
    Pool {
        address: Addr,
        kind: PoolKind,
        denom_a: String,
        denom_b: Option<String>,
    },
}

impl GaugeKind {
    pub fn new_vault(
        address: Addr,
        blacklist: Option<Vec<Addr>>,
        min_shares: Option<Uint128>,
        max_shares: Option<Uint128>,
    ) -> Self {
        GaugeKind::Vault {
            address,
            blacklist,
            min_shares,
            max_shares,
        }
    }

    pub fn new_pool(address: Addr, kind: PoolKind, denom_a: String, denom_b: Option<String>) -> Self {
        GaugeKind::Pool {
            address,
            kind,
            denom_a,
            denom_b,
        }
    }
}
