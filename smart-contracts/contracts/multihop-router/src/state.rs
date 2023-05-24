use cw_storage_plus::Map;

use crate::route::{RouteId, Route};

pub const ROUTES: Map<&RouteId, Route> = Map::new("routes");
