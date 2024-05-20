use cosmwasm_std::Attribute;
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

pub fn _extract_attribute_value_by_ty_and_key(
    events: Vec<cosmwasm_std::Event>,
    ty: &str,
    key: &str,
) -> Option<String> {
    events
        .iter()
        .find(|event| event.ty == ty)
        .and_then(|event| {
            event
                .attributes
                .iter()
                .find(|attr| attr.key == key)
                .map(|attr| attr.value.clone())
        })
}

// pub fn adjust_deposit_amounts(
//     initial_amount0: u128,
//     initial_amount1: u128,
//     deposit_ratio: f64,
// ) -> (u128, u128) {
//     if deposit_ratio < 0.5 {
//         // More token1 to be deposited, reduce token0 amount
//         let adjusted_amount0 = ((1.0 - deposit_ratio) * initial_amount0 as f64) as u128;
//         println!("adjusted_amount0: {:?}", adjusted_amount0);
//         (adjusted_amount0, initial_amount1)
//     } else if deposit_ratio > 0.5 {
//         // More token0 to be deposited, reduce token1 amount
//         let adjusted_amount1 = (deposit_ratio * initial_amount1 as f64) as u128;
//         (initial_amount0, adjusted_amount1)
//     } else {
//         // Balanced deposit
//         (initial_amount0, initial_amount1)
//     }
// }

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
