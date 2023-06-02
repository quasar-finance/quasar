use cosmwasm_std::attr;
use cosmwasm_std::Addr;
use cosmwasm_std::Event;

use crate::msg::InstantiateMsg;
use crate::multitest::common::*;
use crate::multitest::suite::*;
use crate::route::Destination;
use crate::route::Hop;
use crate::route::Route;
use crate::route::RouteId;

#[test]
fn route_lifecycle_works() {
    // initialize the suite
    let mut suite = QuasarVaultSuite::init(InstantiateMsg {}, vec![]).unwrap();

    // create some mock routes
    let mut osmo_routes = vec![(
        RouteId::new(Destination::new("osmosis"), "uosmo".to_string()),
        Route::new("channel-12", "transfer", None),
    )];

    // add the routes as admin
    for route in osmo_routes.iter() {
        let res = suite
            .execute(
                Addr::unchecked(DEPLOYER),
                ExecuteMsg::AddRoute {
                    route_id: route.0.clone(),
                    route: route.1.clone(),
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
    }

    suite.assert_queries(&osmo_routes);
    // mutate the first route in our vec
    let osmo_routes: Vec<(RouteId, Route)> = osmo_routes.iter_mut().map(|val| {
        (val.0.clone(), Route::new(
            "channel-13",
            "transfer",
            Some(Hop::new("channel-11", "transfer", "cosmos123", None))))
    }).collect();

    // mutate the route in our contract
    let res = suite
        .execute(
            Addr::unchecked(DEPLOYER),
            ExecuteMsg::MutateRoute {
                route_id: osmo_routes[0].0.clone(),
                new_route: osmo_routes[0].1.clone(),
            },
            vec![],
        )
        .unwrap();

    let e = Event::new("wasm").add_attributes(vec![
        attr("action", "mutate_route"),
        attr("route_id", "destination: osmosis, asset: uosmo"),
        attr(
            "route",
            "channel: channel-13, port: transfer, hop: (channel: channel-11, port: transfer, receiver: cosmos123)",
        ),
    ]);
    res.assert_event(&e);

    suite.assert_queries(&osmo_routes);
    let res = suite
        .execute(
            Addr::unchecked(DEPLOYER),
            ExecuteMsg::RemoveRoute {
                route_id: osmo_routes[0].0.clone(),
            },
            vec![],
        )
        .unwrap();
    let e = Event::new("wasm").add_attributes(vec![
        attr("action", "remove_route"),
        attr("route_id", "destination: osmosis, asset: uosmo"),
    ]);
    res.assert_event(&e);
    suite.assert_queries(&vec![]);
}
