use anyhow::{Ok, Result as AnyResult};
use serde::de::DeserializeOwned;

use crate::{
    msg::{GetMemoResponse, GetRouteResponse, ListRoutesResponse, MemoResponse},
    multitest::common::*,
    route::{Route, RouteId},
};
use cosmwasm_std::{testing::MockApi, to_binary, Addr, CosmosMsg, MemoryStorage, WasmMsg};
use cw_multi_test::{
    App, AppBuilder, BankKeeper, DistributionKeeper, FailingModule, StakeKeeper, WasmKeeper,
};

pub type QuasarMultiHopRouterApp = App<
    BankKeeper,
    MockApi,
    MemoryStorage,
    FailingModule<Empty, Empty, Empty>,
    WasmKeeper<Empty, Empty>,
    StakeKeeper,
    DistributionKeeper,
>;

use crate::msg::{ExecuteMsg, InstantiateMsg};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct QuasarVaultSuite {
    #[derivative(Debug = "ignore")]
    pub app: QuasarMultiHopRouterApp,
    // The account that deploys everything
    pub deployer: Addr,
    // executor address
    pub executor: Addr,
    // user address
    pub user: Addr,
    // router address
    pub router: Addr,
}

impl QuasarVaultSuite {
    pub fn init(init_msg: InstantiateMsg, funds: Vec<Coin>) -> Result<QuasarVaultSuite> {
        let genesis_funds = vec![coin(150000, DENOM), coin(150000, LOCAL_DENOM)];
        let deployer = Addr::unchecked(DEPLOYER);
        let executor = Addr::unchecked(EXECUTOR);
        let user = Addr::unchecked(USER);
        let mut app = AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &deployer, genesis_funds)
                .unwrap();
        });
        app.send_tokens(
            deployer.clone(),
            user.clone(),
            &[coin(50000, DENOM), coin(50000, LOCAL_DENOM)],
        )?;
        app.send_tokens(
            deployer.clone(),
            executor.clone(),
            &[coin(50000, DENOM), coin(50000, LOCAL_DENOM)],
        )?;

        let router_id = app.store_code(contract());

        let addr = app.instantiate_contract(
            router_id,
            deployer.clone(),
            &init_msg,
            &funds,
            "router-contract",
            Some(deployer.to_string()),
        )?;
        Ok(QuasarVaultSuite {
            app,
            deployer,
            executor,
            user,
            router: addr,
        })
    }

    pub fn execute(
        &mut self,
        sender: Addr,
        msg: ExecuteMsg,
        funds: Vec<Coin>,
    ) -> AnyResult<AppResponse> {
        self.app.execute(
            sender,
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: self.router.to_string(),
                msg: to_binary(&msg)?,
                funds,
            }),
        )
    }

    pub fn query<T>(&self, msg: QueryMsg) -> AnyResult<T>
    where
        T: DeserializeOwned,
    {
        let res = self
            .app
            .wrap()
            .query_wasm_smart::<T>(self.router.clone(), &msg)?;
        Ok(res)
    }

    /// the same as check_queries but panics if any query gives a different result than expected from the expected routes
    pub fn assert_queries(&self, expected: &[(RouteId, Route)]) {
        self.assert_get_memo(expected);
        self.assert_get_route(expected);
        self.assert_list_route(expected);
    }

    /// check whether all queries return values expected from the expected routes
    pub fn check_queries(&self, expected: &[(RouteId, Route)]) -> AnyResult<bool> {
        Ok(self.check_get_memo(expected)?
            && self.check_get_route(expected)?
            && self.check_list_routes(expected)?)
    }

    pub fn assert_get_memo(&self, expected: &[(RouteId, Route)]) {
        self.verify_get_memo(
            expected,
            |actual, expected| {
                panic!(
                    "a different memo was produced than expected, memo: {:?} expected {:?}",
                    actual, expected
                )
            },
            (),
        )
        .unwrap()
    }

    // do all contract queries and check that the values are the same as any of the routes in expected
    pub fn check_get_memo(&self, expected: &[(RouteId, Route)]) -> AnyResult<bool> {
        self.verify_get_memo(expected, |_, _: &[(RouteId, Route)]| false, true)
    }

    pub fn verify_get_memo<T>(
        &self,
        expected: &[(RouteId, Route)],
        on_fail: fn(GetMemoResponse, &[(RouteId, Route)]) -> T,
        on_succes: T,
    ) -> AnyResult<T> {
        let timeout = "1000";
        let retries = 3;
        let actual_memo = Some("{\"my-json\": \"myval\"}".to_string());
        for (id, route) in expected.iter() {
            let res = self.query::<GetMemoResponse>(QueryMsg::GetMemo {
                route_id: id.clone(),
                timeout: timeout.to_string(),
                retries,
                actual_memo: actual_memo.clone(),
            })?;
            if res.channel != route.channel {
                return Ok(on_fail(res, expected));
            }
            if res.port != route.port {
                return Ok(on_fail(res, expected));
            }
            if let Some(hop) = route.hop.clone() {
                if MemoResponse::Forward(hop.to_memo(
                    timeout.to_string(),
                    retries,
                    actual_memo.clone(),
                )) != res.memo
                {
                    return Ok(on_fail(res, expected));
                }
            } else if MemoResponse::Actual(actual_memo.clone()) != res.memo {
                return Ok(on_fail(res, expected));
            }
        }
        Ok(on_succes)
    }

    pub fn assert_get_route(&self, expected: &[(RouteId, Route)]) {
        self.verify_get_route(
            expected,
            |actual, expected| {
                panic!(
                    "a different memo was produced than expected, memo: {:?} expected {:?}",
                    actual, expected
                )
            },
            (),
        )
        .unwrap()
    }

    // do all contract queries and check that the values are the same as any of the routes in expected
    pub fn check_get_route(&self, expected: &[(RouteId, Route)]) -> AnyResult<bool> {
        self.verify_get_route(expected, |_, _| false, true)
    }

    pub fn verify_get_route<T>(
        &self,
        expected: &[(RouteId, Route)],
        on_fail: fn((&RouteId, &Route), (&RouteId, &Route)) -> T,
        on_succes: T,
    ) -> AnyResult<T> {
        for (id, route) in expected.iter() {
            let res = self.query::<GetRouteResponse>(QueryMsg::GetRoute {
                route_id: id.clone(),
            })?;
            if &res.route != route {
                return Ok(on_fail((id, &res.route), (id, route)));
            }
        }
        Ok(on_succes)
    }

    pub fn assert_list_route(&self, expected: &[(RouteId, Route)]) {
        self.verify_get_memo(
            expected,
            |actual, expected| {
                panic!(
                    "a different memo was produced than expected, memo: {:?} expected {:?}",
                    actual, expected
                )
            },
            (),
        )
        .unwrap()
    }

    // do all contract queries and check that the values are the same as any of the routes in expected
    pub fn check_list_routes(&self, expected: &[(RouteId, Route)]) -> AnyResult<bool> {
        self.verify_get_memo(expected, |_, _| false, true)
    }

    pub fn verify_list_routes<T>(
        &self,
        expected: &[(RouteId, Route)],
        on_fail: fn(&[(RouteId, Route)], &[(RouteId, Route)]) -> T,
        on_succes: T,
    ) -> AnyResult<T> {
        let res = self.query::<ListRoutesResponse>(QueryMsg::ListRoutes {})?;
        if res.routes.iter().all(|actual| expected.contains(actual)) {
            Ok(on_succes)
        } else {
            Ok(on_fail(res.routes.as_ref(), expected))
        }
    }
}
