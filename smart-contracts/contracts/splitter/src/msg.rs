use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

use crate::state::{Receiver, Receivers};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub receivers: Vec<Receiver>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Admin(AdminMsg),
    Split {},
    Claim { claims: Vec<Claim> },
}

#[cw_serde]
pub enum AdminMsg {
    UpdateReceivers { new: Vec<Receiver> },
    UpdateAdmin { new: String },
}

#[cw_serde]
pub struct Claim {
    pub address: String,
    pub msg: Binary,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Receivers)]
    GetReceivers {},
    #[returns(String)]
    GetAdmin {},
}
