use cosmwasm_std::attr;
use cosmwasm_std::Addr;
use cosmwasm_std::Attribute;
use cosmwasm_std::Event;

use crate::msg::InstantiateMsg;
use crate::multitest::common::*;
use crate::multitest::suite::*;
use crate::route::Destination;
use crate::route::Hop;
use crate::route::Route;
use crate::route::RouteId;

#[test]
fn create_route() {
    // initialize the suite
    let mut suite = QuasarVaultSuite::init(InstantiateMsg {}, vec![]).unwrap();

    // create some mock routes
    let osmo_route = (
        RouteId::new(Destination::new("osmosis"), "uosmo".to_string()),
        Route::new("channel-12", "transfer", None),
    );

    // add a route as admin
    let res = suite
        .execute(
            Addr::unchecked(DEPLOYER),
            ExecuteMsg::AddRoute {
                route_id: osmo_route.0.clone(),
                route: osmo_route.1,
            },
            vec![],
        )
        .unwrap();

    let e = Event::new("wasm").add_attributes(vec![
        attr("action", "add_route"),
        attr("route_id", "destination: osmosis, asset: uosmo"),
        attr("route", "channel: channel-12, port: transfer"),
    ]);
    res.assert_event(&e);

    // mutate a route as admin
    let res = suite
        .execute(
            Addr::unchecked(DEPLOYER),
            ExecuteMsg::MutateRoute {
                route_id: osmo_route.0,
                new_route: Route::new(
                    "channel-13",
                    "transfer",
                    Some(Hop::new("channel-11", "transfer", "cosmos123", None)),
                ),
            },
            vec![],
        )
        .unwrap();
}
