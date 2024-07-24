use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Decimal};

#[cw_serde]
pub struct FakeLstInstantiateMsg {
    pub redemption_rate: Decimal,
}

#[cw_serde]
pub struct FakeLstMigrateMsg {}

#[cw_serde]
pub enum FakeLstExecuteMsg {
    Update { redemption_rate: Decimal },
}

#[cw_serde]
pub enum FakeLstQueryMsg {
    RedemptionRate {
        denom: String,
        params: Option<Binary>,
    },
}

#[cw_serde]
pub struct RedemptionRateResponse {
    pub redemption_rate: Decimal,
    pub update_time: u64,
}
