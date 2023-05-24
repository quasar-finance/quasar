use cw_storage_plus::Map;

use crate::route::{Route, RouteId};

pub const ROUTES: Map<&RouteId, Route> = Map::new("routes");
