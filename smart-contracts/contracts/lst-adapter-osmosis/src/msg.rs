use crate::{contract::LstAdapter, state::IbcConfig};
use cosmwasm_schema::cw_serde;
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
#[impl_into(ExecuteMsg)]
pub enum LstAdapterExecuteMsg {
    // Only configured vault can execute unbonds
    #[payable]
    Unbond {},
    Claim {},
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
#[derive(cw_orch::QueryFns)]
#[impl_into(QueryMsg)]
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
