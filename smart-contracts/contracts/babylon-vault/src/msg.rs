use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Timestamp, Uint128};
use mars_owner::{OwnerResponse, OwnerUpdate};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // permission-less methods
    Deposit {},
    Withdraw {},
    Claim {},
    // owner methods
    RegisterLst {
        denom: String,
    },
    UpdateOwner(OwnerUpdate),
}

#[cw_serde]
pub struct Claim {
    pub amount: Uint128,
    pub expiration: Timestamp,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<Claim>)]
    Pending { address: String },
    #[returns(Uint128)]
    Claimable { address: String },
    #[returns(Uint128)]
    BalanceInUnderlying {},
    #[returns(OwnerResponse)]
    Owner {},
}
