use crate::contract::DexAdapter;

use abstract_app::objects::PoolAddress;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Decimal;
use cw_asset::AssetInfo;

abstract_app::app_msg_types!(DexAdapter, DexAdapterExecuteMsg, DexAdapterQueryMsg);

#[cosmwasm_schema::cw_serde]
pub struct DexAdapterInstantiateMsg {
    pub lst_adapter: String,
    pub dex: String,
    pub offer_asset: AssetInfo,
    pub receive_asset: AssetInfo,
    pub margin: Decimal,
    pub pool: PoolAddress,
}

#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum DexAdapterExecuteMsg {
    #[payable]
    Swap { slippage: Option<Decimal> },
}

#[cosmwasm_schema::cw_serde]
pub struct DexAdapterMigrateMsg {}

#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
#[impl_into(QueryMsg)]
pub enum DexAdapterQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {
    pub lst_adapter: String,
    pub dex: String,
    pub offer_asset: AssetInfo,
    pub receive_asset: AssetInfo,
    pub margin: Decimal,
    pub pool: PoolAddress,
}
