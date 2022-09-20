use crate::msg::IntergammMsg;
use cw_storage_plus::Map;

pub const REPLIES: Map<u64, IntergammMsg> = Map::new("intergamm-replies");
