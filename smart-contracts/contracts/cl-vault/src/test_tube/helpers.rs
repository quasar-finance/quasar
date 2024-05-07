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
