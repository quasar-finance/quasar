use crate::state::Claim;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub dex_adapter: String,
    pub lst_adapter: String,
    pub deposit_denom: String,
    pub lst_denom: String,
    pub unbonding_time_seconds: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {},
    Withdraw {},
    Claim {},
    Swap {
        amount: Uint128,
        slippage: Option<Decimal>,
    },
    ClaimUnbonded {},
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Pending { address: String },
    //    Claimable { address: String },
    // simulate?
}
#[cw_serde]
pub struct PendingResponse {
    pub pending: Vec<Claim>,
}
