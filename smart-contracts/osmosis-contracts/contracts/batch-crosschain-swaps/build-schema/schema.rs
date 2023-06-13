use cosmwasm_schema::write_api;

use batch_crosschain_swaps::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        name: "batch-crosschain-swaps",
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    };
}
