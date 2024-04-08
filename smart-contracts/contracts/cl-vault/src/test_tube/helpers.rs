use cosmwasm_std::Attribute;
use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse;
use osmosis_test_tube::ExecuteResponse;

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

pub fn get_event_value_amount_numeric(value: &String) -> u128 {
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
