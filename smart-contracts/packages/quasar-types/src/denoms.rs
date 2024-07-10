#[cosmwasm_schema::cw_serde]
pub struct LstDenom {
    pub denom: String,
    pub underlying: String,
}
