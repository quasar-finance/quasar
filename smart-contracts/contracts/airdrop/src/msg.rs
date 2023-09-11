use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    /// owner is the admin address
    pub owner: Option<String>,
    /// funding address to send back funds to
    pub quasar_funding_address: String,
}
