use crate::contract::LstAdapter;
use cosmwasm_schema::cw_serde;

abstract_app::app_msg_types!(LstAdapter, LstAdapterExecuteMsg, LstAdapterQueryMsg);

#[cw_serde]
pub struct LstAdapterInstantiateMsg {
    pub vault: String,
    pub lst_denom: String,
}

#[cw_serde]
pub struct LstAdapterMigrateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum LstAdapterExecuteMsg {
    // Only configured vault can execute unbonds
    #[payable]
    Unbond {},
    Claim {},
}

#[cw_serde]
pub struct IbcConfig {
    pub source_port: String,
    pub source_channel: String,
    pub timeout_height_offset: u64,
    pub timeout_ns_offset: u64,
}

#[cw_serde]
pub enum LstAdapterQueryMsg {
    Config {},
}
