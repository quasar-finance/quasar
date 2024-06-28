use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub lst_denom: String,
}

#[cw_serde]
pub enum QueryMsg {
    Admin {},
    Config {},
    Pending {},
    RedemptionRate {},
    Claimable {},
}

#[cw_serde]
pub enum ExecuteMsg {
    Unbond {},
    Claim {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct Admin {
    pub admin: String,
}

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct Pending {
    pub pending: Uint128,
}

#[cw_serde]
pub struct Claimable {
    pub claimable: Uint128,
}

#[cw_serde]
pub struct RedemptionRate {
    pub redemption_rate: Decimal,
}
