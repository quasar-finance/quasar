use cosmwasm_schema::remove_schemas;
use lst_dex_adapter_osmosis::contract::DexAdapter;
use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    #[cfg(feature = "schema")]
    DexAdapter::export_schema(&out_dir);
}
