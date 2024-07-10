use crate::state::{Claim, Config};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};
use mars_owner::OwnerUpdate;
use quasar_types::denoms::LstDenom;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub dex_adapter: String,
    pub lst_adapter: String,
    pub lst_denom: LstDenom,
    pub unbonding_time_seconds: u64,
    // When swapping, we check for pending withdrawals and unbonds to determine the available funds.
    // As we don't know exactly when unbonded tokens are available, we need a buffer.
    pub unbonding_buffer_seconds: u64,
    pub subdenom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {},
    Withdraw {},
    Claim {},
    ClaimUnbonded {},
    // owner methods
    Swap {
        amount: Uint128,
        slippage: Option<Decimal>,
    },
    Update {
        dex_adapter: Option<String>,
        lst_adapter: Option<String>,
        lst_denom: Option<LstDenom>,
        unbonding_time_seconds: Option<u64>,
        unbonding_buffer_seconds: Option<u64>,
    },
    UpdateOwner(OwnerUpdate),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
    #[returns(Vec<Claim>)]
    Pending { address: String },
    #[returns(Uint128)]
    Claimable { address: String },
    #[returns(Uint128)]
    Swappable {},
}
