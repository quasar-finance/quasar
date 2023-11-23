use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

pub struct NewRange {
    pub cl_vault_address: String,
    pub lower_price: Decimal,
    pub upper_price: Decimal,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Submit a range to the range middleware
    SubmitNewRange { pub new_range: NewRange },
    /// Execute a new range
    ExecuteNewRange {
        pub cl_vault_address: String,
        pub max_slippage: Decimal,
        pub ratio_of_swappable_funds_to_use: Decimal,
        pub twap_window_seconds: u64,
        pub recommended_swap_route: SwapOperationsListUnchecked // taken from cw-dex-router 
        pub force_swap_route: bool
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
