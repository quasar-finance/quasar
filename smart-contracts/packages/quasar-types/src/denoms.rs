#[cosmwasm_schema::cw_serde]
pub struct LstDenom {
    pub denom: String,
    pub underlying: String,
}

pub fn get_factory_denom(addr: &str, subdenom: &str) -> String {
    format!("factory/{}/{}", addr, subdenom)
}
