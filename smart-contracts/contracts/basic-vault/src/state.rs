use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Timestamp, Uint128};

use cw_controllers::Claims;
use cw_storage_plus::{Item, Map};
use quasar_types::callback::{BondResponse, UnbondResponse};

use crate::{msg::PrimitiveConfig, ContractError};

// constants
pub const FALLBACK_RATIO: Decimal = Decimal::one();

// reply ids
pub const STRATEGY_BOND_ID: u64 = 80085;

// version info for migration info
pub const CONTRACT_NAME: &str = "basic-vault";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CLAIMS: Claims = Claims::new("claims");

/// Investment info is fixed at instantiation, and is used to control the function of the contract
#[cw_serde]
pub struct InvestmentInfo {
    /// Owner created the contract and takes a cut
    pub owner: Addr,
    /// This is the minimum amount we will pull out to reinvest, as well as a minimum
    /// that can be unbonded (to avoid needless staking tx)
    pub min_withdrawal: Uint128,
    /// the denom accepted by the vault
    pub deposit_denom: String,
    /// this is the array of primitives that this vault will subscribe to
    pub primitives: Vec<PrimitiveConfig>,
}

pub const CAP: Item<Cap> = Item::new("cap");

#[cw_serde]
pub struct Cap {
    cap_admin: Addr,
    total: Uint128,
    current: Uint128,
}

impl Cap {
    pub fn new(admin: Addr, total: Uint128) -> Self {
        Self {
            cap_admin: admin,
            total,
            current: Uint128::zero(),
        }
    }

    pub fn update_cap_admin(mut self, new_cap_admin: Addr) -> Self {
        self.cap_admin = new_cap_admin;
        self
    }

    pub fn update_total_cap(mut self, new_total: Uint128) -> Self {
        self.total = new_total;
        self
    }

    pub fn update_current(mut self, to_add: Uint128) -> Result<Self, ContractError> {
        let new_total = self.current.checked_add(to_add)?;
        // if we go over cap, reject
        if new_total > self.total {
            return Err(ContractError::OverCap {});
        };
        self.current = new_total;
        Ok(self)
    }
}

/// Supply is dynamic and tracks the current supply of staked and ERC20 tokens.
#[cw_serde]
#[derive(Default)]
pub struct Supply {
    /// issued is how many derivative tokens this contract has issued
    pub issued: Uint128,
}

#[cw_serde]
pub struct AdditionalTokenInfo {
    pub thesis: String,
    pub creation_time: Timestamp,
}

pub const ADDITIONAL_TOKEN_INFO: Item<AdditionalTokenInfo> = Item::new("additional_token_info");
pub const INVESTMENT: Item<InvestmentInfo> = Item::new("invest");
pub const TOTAL_SUPPLY: Item<Supply> = Item::new("total_supply");

#[cw_serde]
#[derive(Default)]
pub struct BondingStub {
    // the contract address of the primitive
    pub address: String,
    // the response of the primitive upon successful bond or error
    pub bond_response: Option<BondResponse>,
    // primitive value at the time of receiving the bond_response
    pub primitive_value: Option<Uint128>,
    // the amount sent with the Bond
    pub amount: Uint128,
}

#[cw_serde]
#[derive(Default)]
pub struct Unbond {
    pub stub: Vec<UnbondingStub>,
    pub shares: Uint128,
}

#[cw_serde]
#[derive(Default)]
pub struct UnbondingStub {
    // the contract address of the primitive
    pub address: String,
    // the response of the primitive upon successful bond or error
    pub unlock_time: Option<Timestamp>,
    // response of the unbond, if this is present then we have finished unbonding
    pub unbond_response: Option<UnbondResponse>,
    // funds attached to the unbond_response
    pub unbond_funds: Vec<Coin>,
}

// (un)bonding sequence number (to map primitive responses to the right bond action)
pub const BONDING_SEQ: Item<Uint128> = Item::new("bond_seq");
// mapping from bonding sequence number to depositor/withdrawer address
pub const BONDING_SEQ_TO_ADDR: Map<String, String> = Map::new("bond_seq_to_addr");
// current bonds pending for a user
pub const PENDING_BOND_IDS: Map<Addr, Vec<String>> = Map::new("pending_bond_ids");
// current unbonds pending for a user
pub const PENDING_UNBOND_IDS: Map<Addr, Vec<String>> = Map::new("pending_unbond_ids");
// map of bond id to bond state
pub const BOND_STATE: Map<String, Vec<BondingStub>> = Map::new("bond_state");
// map of unbond id to unbond state
pub const UNBOND_STATE: Map<String, Unbond> = Map::new("unbond_state");

pub const DEBUG_TOOL: Item<String> = Item::new("debug_tool");

// vault rewards contract
pub const VAULT_REWARDS: Item<Addr> = Item::new("vault_rewards");

impl InvestmentInfo {
    pub fn normalize_primitive_weights(&mut self) {
        let mut total_weight = Decimal::zero();
        for p in &self.primitives {
            total_weight += p.weight;
        }
        for p in &mut self.primitives {
            p.weight /= total_weight;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::msg::{PrimitiveConfig, PrimitiveInitMsg};

    #[test]
    fn test_investment_info() {
        let mut invest = InvestmentInfo {
            owner: Addr::unchecked("owner".to_string()),
            min_withdrawal: Uint128::from(1000u128),
            primitives: vec![
                PrimitiveConfig {
                    address: "primitive".to_string(),
                    weight: Decimal::percent(50), // passing in unnormalized
                    init: PrimitiveInitMsg::LP(lp_strategy::msg::InstantiateMsg {
                        lock_period: 1,
                        pool_id: 1,
                        pool_denom: "gamm/pool/1".to_string(),
                        local_denom: "uosmo".to_string(),
                        base_denom: "ibc/blah".to_string(),
                        quote_denom: "ibc/blah2".to_string(),
                        transfer_channel: "channel-0".to_string(),
                        return_source_channel: "channel-0".to_string(),
                        expected_connection: "connection-0".to_string(),
                    }),
                };
                4
            ],
            deposit_denom: "ibc/osmo".to_string(),
        };
        invest.normalize_primitive_weights();
        assert_eq!(invest.primitives[0].weight, Decimal::percent(25));
    }
}
