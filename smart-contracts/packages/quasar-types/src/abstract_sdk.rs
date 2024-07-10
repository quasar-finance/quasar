use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg<T> {
    Module { module: T },
}

#[cw_serde]
pub enum QueryMsg<T> {
    Module(T),
}
