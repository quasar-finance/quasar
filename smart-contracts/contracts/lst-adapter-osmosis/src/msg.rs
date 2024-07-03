use crate::{contract::LstAdapter, state::IbcConfig};
use cosmwasm_schema::{cw_serde, QueryResponses};
use mars_owner::OwnerUpdate;

abstract_app::app_msg_types!(LstAdapter, LstAdapterExecuteMsg, LstAdapterQueryMsg);

#[cw_serde]
pub struct LstAdapterInstantiateMsg {
    pub owner: String,
    pub vault: String,
    pub lst_denom: String,
}

#[cw_serde]
pub struct LstAdapterMigrateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum LstAdapterExecuteMsg {
    // Only configured vault can execute unbonds
    #[cw_orch(payable)]
    Unbond {},
    UpdateIbcConfig {
        channel: String,
        revision: Option<u64>,
        block_offset: Option<u64>,
        timeout_secs: Option<u64>,
    },
    Update {
        vault: Option<String>,
        lst_denom: Option<String>,
    },
    UpdateOwner(OwnerUpdate),
}

#[cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum LstAdapterQueryMsg {
    #[returns(IbcConfig)]
    IbcConfig {},
    #[returns(String)]
    Owner {},
    #[returns(String)]
    Vault {},
    #[returns(String)]
    LstDenom {},
}
