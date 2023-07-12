#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use osmosis_test_tube::osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{MsgCreatePosition, LiquidityNetInDirectionRequest};
    use osmosis_test_tube::{
        osmosis_std::types::osmosis::concentratedliquidity::poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool,
        ConcentratedLiquidity,
    };
    use osmosis_test_tube::{Account, Gamm, Module, OsmosisTestApp};
}
