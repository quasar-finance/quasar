use cosmwasm_std::StdResult;
use cw20::Logo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMarketingInfo {
    pub project: Option<String>,
    pub description: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct InstantiateMsg {
    // TODO write the instantiate msg
}

impl InstantiateMsg {
    pub fn validate(&self) -> StdResult<()> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // TODO add all wanted queries
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // We can always deposit money into the strategy
    Deposit { owner: String },
    // A request for a withdraw, this has to be a request and cannot be an immediate withdraw since funds might be locked
    WithdrawRequest {},
}
