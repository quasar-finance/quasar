use std::env::current_dir;
use std::fs::create_dir_all;
extern crate cw_4626;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cw20::{
    AllAccountsResponse, AllAllowancesResponse, AllowanceResponse, BalanceResponse,
    TokenInfoResponse,
};
use cw20_base::msg::{ExecuteMsg, QueryMsg};
use cw_4626::msg::{
    AssetResponse, ConvertToAssetsResponse, ConvertToSharesResponse, InstantiateMsg,
    MaxDepositResponse, TotalAssetResponse, VaultInfoResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(AllowanceResponse), &out_dir);
    export_schema(&schema_for!(BalanceResponse), &out_dir);
    export_schema(&schema_for!(TokenInfoResponse), &out_dir);
    export_schema(&schema_for!(AllAllowancesResponse), &out_dir);
    export_schema(&schema_for!(AllAccountsResponse), &out_dir);
    // the cw-4626 schemas
    export_schema(&schema_for!(AssetResponse), &out_dir);
    export_schema(&schema_for!(TotalAssetResponse), &out_dir);
    export_schema(&schema_for!(ConvertToSharesResponse), &out_dir);
    export_schema(&schema_for!(ConvertToAssetsResponse), &out_dir);
    export_schema(&schema_for!(MaxDepositResponse), &out_dir);
    export_schema(&schema_for!(VaultInfoResponse), &out_dir);
}
