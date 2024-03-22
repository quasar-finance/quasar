use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Burn {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TotalBurntResponse)]
    TotalBurntQuery {},
}

#[cw_serde]
pub struct TotalBurntResponse {
    pub amount: Vec<Coin>,
}
