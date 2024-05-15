use cosmwasm_schema::{cw_serde, QueryResponses};
#[cfg(claim)]
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
    #[cfg(claim)]
    Claim {
        claims: Vec<Claim>,
    },
}

#[cw_serde]
pub enum AdminMsg {
    UpdateReceivers { new: Vec<Receiver> },
    UpdateAdmin { new: String },
}

#[cfg(claim)]
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
