use cosmwasm_std::Attribute;
use osmosis_std::types::cosmos::base::v1beta1;
use osmosis_std::types::{
    cosmos::bank::v1beta1::QueryBalanceRequest, cosmwasm::wasm::v1::MsgExecuteContractResponse,
};
use osmosis_test_tube::{Bank, ExecuteResponse, Module, OsmosisTestApp};

pub fn get_event_attributes_by_ty_and_key(
    response: &ExecuteResponse<MsgExecuteContractResponse>,
    ty: &str,
    keys: Vec<&str>,
) -> Vec<Attribute> {
    response
        .events
        .iter()
        .filter(|event| event.ty == ty)
        .flat_map(|event| event.attributes.clone())
        .filter(|attribute| keys.contains(&attribute.key.as_str()))
        .collect()
}

pub fn get_balance_amount(app: &OsmosisTestApp, address: String, denom: String) -> u128 {
    let bm = Bank::new(app);

    bm.query_balance(&QueryBalanceRequest { address, denom })
        .unwrap()
        .balance
        .unwrap()
        .amount
        .parse::<u128>()
        .unwrap()
}

pub fn get_amount_from_denom(value: &String) -> u128 {
    // Find the position where the non-numeric part starts
    let pos = value.find(|c: char| !c.is_numeric()).unwrap_or(value.len());
    // Extract the numeric part from the string
    let numeric_part = &value[0..pos];
    // Try to parse the numeric string to u128
    numeric_part.parse::<u128>().unwrap()
}

pub fn calculate_deposit_ratio(
    spot_price: String,
    tokens_provided: Vec<v1beta1::Coin>,
    amount0_deposit: String,
    amount1_deposit: String,
    denom_base: String,
    denom_quote: String,
) -> (f64, String) {
    // Parse the input amounts
    let amount0_deposit: u128 = amount0_deposit.parse().unwrap();
    let amount1_deposit: u128 = amount1_deposit.parse().unwrap();

    // Find the attempted amounts from the tokens_provided
    let mut provided_amount0 = 0u128;
    let mut provided_amount1 = 0u128;

    for coin in &tokens_provided {
        if coin.denom == denom_base {
            provided_amount0 = coin.amount.parse().unwrap();
        } else if coin.denom == denom_quote {
            provided_amount1 = coin.amount.parse().unwrap();
        }
    }

    // Calculate refunds TODO check if this is correct
    let token0_refund = provided_amount0.saturating_sub(amount0_deposit);
    let token1_refund = provided_amount1.saturating_sub(amount1_deposit);

    // Convert token1 refund into token0 equivalent using spot price
    let spot_price_value = spot_price.parse::<f64>().unwrap();
    let token1_refund_in_token0 = (token1_refund as f64) / spot_price_value;

    // Calculate total refunds in terms of token0
    let total_refunds_in_token0 = token0_refund as f64 + token1_refund_in_token0;

    // Calculate total attempted deposits in terms of token0
    let total_attempted_deposit_in_token0 =
        provided_amount0 as f64 + (provided_amount1 as f64 / spot_price_value);

    // Calculate the ratio of total refunds in terms of token0 to total attempted deposits in terms of token0
    let ratio = if total_attempted_deposit_in_token0 == 0.0 {
        0.5 // Balanced deposit
    } else {
        2.0 * total_refunds_in_token0 / total_attempted_deposit_in_token0
    };

    // TODO: Compute this based on tokens_provided size
    let ratio_approx: String = "0.00005".to_string();

    (ratio, ratio_approx)
}

pub fn calculate_expected_refunds(
    initial_amount0: u128,
    initial_amount1: u128,
    deposit_ratio: f64,
) -> (u128, u128) {
    if deposit_ratio < 0.5 {
        // More token1 to be deposited, so token0 has a higher refund
        let adjusted_amount0 = ((1.0 - deposit_ratio) * initial_amount0 as f64) as u128;
        let expected_refund0 = initial_amount0 - adjusted_amount0;
        (expected_refund0, 0)
    } else if deposit_ratio > 0.5 {
        // More token0 to be deposited, so token1 has a higher refund
        let adjusted_amount1 =
            ((1.0 - (deposit_ratio - 0.5) * 2.0) * initial_amount1 as f64) as u128;
        let expected_refund1 = initial_amount1 - adjusted_amount1;
        (0, expected_refund1)
    } else {
        // Balanced deposit, no refunds expected
        (0, 0)
    }
}
