use cw_storage_plus::Map;

use crate::route::{Route, RouteId};

/// ROUTES represents complete routes to a destination for a certain asset.
/// The value of Route represents the route to take to 
pub const ROUTES: Map<&RouteId, Route> = Map::new("routes");
