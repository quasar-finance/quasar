use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use quasar_types::coinlist::CoinList;

#[cw_serde]
pub struct Gauge {
    pub start_block: u64,
    pub end_block: u64,
    pub expiration_block: u64,
    pub total_incentives: Vec<Coin>,
    // TODO remove the fee from the gauge and move it to a secondary map so we can effciently update the fee on claiming
    pub fee: Fee,
    pub kind: GaugeKind,
}

impl Gauge {
    pub fn new(
        start_block: u64,
        end_block: u64,
        expiration_block: u64,
        total_incentives: Vec<Coin>,
        fee: Decimal,
        fee_address: Addr,
        kind: GaugeKind,
    ) -> Gauge {
        let fee = Fee::new(fee_address, fee, CoinList::new(total_incentives.clone()));

        Gauge {
            start_block,
            end_block,
            expiration_block,
            total_incentives,
            fee,
            kind,
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
        shares: Option<VaultConfig>,
    },

    Pool {
        address: Addr,
    }
}

impl GaugeKind {
    pub fn new_vault_incentives(
        address: Addr,
        blacklist: Option<Vec<Addr>>,
        shares: Option<VaultConfig>,
    ) -> GaugeKind {
        GaugeKind::Vault {
            address,
            blacklist,
            shares,
        }
    }
}

#[cw_serde]
pub struct VaultConfig {
    /// a mint amount of shares needed to receive incentives
    min: Option<Uint128>,
    /// a maximum amount of shares
    max: Option<Uint128>,
}
