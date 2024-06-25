use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
#[cfg(not(target_arch = "wasm32"))]
use cw_asset::AssetInfo;
use cw_asset::{Asset, AssetInfoUnchecked};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Swap {
        routes: Vec<SwapAmountInRoute>,
        minimum_receive: Option<Uint128>,
        to: Option<String>,
    },
    SetPath {
        offer_asset: AssetInfoUnchecked,
        ask_asset: AssetInfoUnchecked,
        path: Vec<SwapAmountInRoute>,
        bidirectional: bool,
    },
}

#[cw_serde]
pub struct BestPathForPairResponse {
    /// the operations that will be executed to perform the swap
    pub operations: Vec<SwapAmountInRoute>,
    /// the amount of tokens that are expected to be received after the swap
    pub return_amount: Uint128,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint128)]
    SimulateSwapOperations {
        offer: Asset,
        operations: Vec<SwapAmountInRoute>,
    },
    /// Returns all the current path for a given (offer_asset, ask_asset) pair.
    #[returns(Vec<SwapAmountInRoute>)]
    PathsForPair {
        offer_asset: AssetInfoUnchecked,
        ask_asset: AssetInfoUnchecked,
    },
    /// finds the best path for a given (offer_asset, ask_asset) pair.
    /// if no path is found, returns None.
    #[returns(Option<BestPathForPairResponse>)]
    BestPathForPair {
        offer_asset: AssetInfoUnchecked,
        offer_amount: Uint128,
        ask_asset: AssetInfoUnchecked,
    },

    /// Returns all the assets from which there are paths to a given ask asset.
    #[returns(Vec<AssetInfo>)]
    SupportedOfferAssets { ask_asset: AssetInfoUnchecked },

    /// Returns all the assets to which there are paths from a given offer
    /// asset.
    #[returns(Vec<AssetInfo>)]
    SupportedAskAssets { offer_asset: AssetInfoUnchecked },
}

#[cw_serde]
pub struct MigrateMsg {}
