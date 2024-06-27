use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Swap {
        out_denom: String,
        path: Option<Vec<SwapAmountInRoute>>,
        minimum_receive: Option<Uint128>,
    },
    SetPath {
        offer_denom: String,
        ask_denom: String,
        path: Vec<u64>,
        bidirectional: bool,
    },
    RemovePath {
        offer_denom: String,
        ask_denom: String,
        path: Vec<u64>,
        bidirectional: bool,
    },
}

#[cw_serde]
pub struct BestPathForPairResponse {
    /// the path that will be used to perform the swap
    pub path: Vec<SwapAmountInRoute>,
    /// the amount of tokens that are expected to be received after the swap
    pub return_amount: Uint128,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint128)]
    SimulateSwaps {
        offer: Coin,
        path: Vec<SwapAmountInRoute>,
    },
    /// Returns all the current path for a given (offer_denom, ask_denom) pair.
    #[returns(Vec<Vec<SwapAmountInRoute>>)]
    PathsForPair {
        offer_denom: String,
        ask_denom: String,
    },
    /// finds the best path for a given (offer_denom, ask_denom) pair.
    /// if no path is found, returns None.
    #[returns(Option<BestPathForPairResponse>)]
    BestPathForPair { offer: Coin, ask_denom: String },

    /// Returns all the assets from which there are paths to a given ask asset.
    #[returns(Vec<String>)]
    SupportedOfferAssets { ask_denom: String },

    /// Returns all the assets to which there are paths from a given offer
    /// asset.
    #[returns(Vec<String>)]
    SupportedAskAssets { offer_denom: String },
}

#[cw_serde]
pub struct MigrateMsg {}
