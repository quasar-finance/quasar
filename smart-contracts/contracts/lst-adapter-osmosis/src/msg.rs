use crate::{
    contract::LstAdapter,
    state::{Denoms, IbcConfig, UnbondInfo},
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal, Timestamp, Uint128};
use mars_owner::OwnerUpdate;

abstract_app::app_msg_types!(LstAdapter, LstAdapterExecuteMsg, LstAdapterQueryMsg);

#[cw_serde]
pub struct LstAdapterInstantiateMsg {
    pub owner: String,
    pub vault: String,
    pub observer: String,
    pub denoms: Denoms,
    pub stride_oracle: String,
    pub unbond_period_secs: u64,
}

#[cw_serde]
pub struct LstAdapterMigrateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum LstAdapterExecuteMsg {
    // vault methods
    #[cw_orch(payable)]
    Unbond {},
    Claim {},
    // observer methods
    ConfirmUnbond {
        amount: Uint128,
    },
    ConfirmUnbondFinished {
        unbond_start_time: Timestamp,
    },
    // owner methods
    UpdateIbcConfig {
        channel: String,
        remote_chain: String,
        revision: Option<u64>,
        block_offset: Option<u64>,
        timeout_secs: Option<u64>,
    },
    Update {
        denoms: Option<Denoms>,
        stride_oracle: Option<String>,
        vault: Option<String>,
        observer: Option<String>,
        unbond_period_secs: Option<u64>,
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
    Oracle {},
    #[returns(Denoms)]
    Denoms {},
    #[returns(Decimal)]
    RedemptionRate {},
    #[returns(Vec<UnbondInfo>)]
    PendingUnbonds {},
    #[returns(Uint128)]
    BalanceInUnderlying {},
    #[returns(Coin)]
    Claimable {},
}
