use std::str::FromStr;

use apollo_cw_asset::AssetInfo;
use cosmwasm_std::{
    to_json_binary, Addr, Coin, CosmosMsg, DepsMut, Env, QuerierWrapper, Storage, Uint128, WasmMsg,
};
use cw_dex_router::{
    msg::{BestPathForPairResponse, ExecuteMsg, QueryMsg},
    operations::SwapOperationsListUnchecked,
};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin, osmosis::poolmanager::v1beta1::SwapAmountInRoute,
};

use crate::{
    state::{DEX_ROUTER, POOL_CONFIG},
    ContractError,
};

pub struct SwapCalculationResult {
    pub swap_msg: Option<CosmosMsg>,
    pub token_in_denom: Option<String>,
    pub swap_amount: Uint128,
    pub token_out_min_amount: Uint128,
    pub position_id: Option<u64>,
}

pub struct SwapParams {
    pub token_in_amount: Uint128,
    pub token_in_denom: String,
    pub token_out_min_amount: Uint128,
    pub token_out_denom: String,
    pub recommended_swap_route: Option<SwapOperationsListUnchecked>,
    pub force_swap_route: bool,
}
/// estimate_swap can be used to pass correct token_out_min_amount values into swap()
/// for now this function can only be used for our pool
/// this will likely be expanded once we allow arbitrary pool swaps
pub fn _estimate_swap(
    querier: &QuerierWrapper,
    storage: &mut dyn Storage,
    _env: &Env,
    token_in_amount: Uint128,
    token_in_denom: &String,
    _token_out_min_amount: Uint128,
) -> Result<Coin, ContractError> {
    let pool_config = POOL_CONFIG.load(storage)?;

    if !pool_config.pool_contains_token(token_in_denom) {
        return Err(ContractError::BadTokenForSwap {
            base_token: pool_config.token0,
            quote_token: pool_config.token1,
        });
    }

    // get token_out_denom
    let token_out_denom = if *token_in_denom == pool_config.token0 {
        pool_config.token1
    } else {
        pool_config.token0
    };

    // we will only ever have a route length of one, this will likely change once we start selecting different routes
    let pool_route = SwapAmountInRoute {
        pool_id: pool_config.pool_id,
        token_out_denom: token_out_denom.to_string(),
    };

    let pm_querier =
        osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier::new(querier);

    // todo: verify that we should be concatenating amount and denom or if we should just send token in amount as string
    let result = pm_querier.estimate_swap_exact_amount_in(
        pool_config.pool_id,
        token_in_amount.to_string() + token_in_denom,
        vec![pool_route],
    )?;

    Ok(Coin {
        denom: token_out_denom,
        amount: Uint128::from_str(&result.token_out_amount)?,
    })
}

/// swap will always swap over the CL pool. In the future we may expand the
/// feature such that it chooses best swaps over all routes
pub fn swap(deps: DepsMut, env: &Env, params: SwapParams) -> Result<CosmosMsg, ContractError> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let dex_router = DEX_ROUTER.may_load(deps.storage)?;

    let token_in_denom = params.token_in_denom.clone();

    if !pool_config.pool_contains_token(params.token_in_denom) {
        return Err(ContractError::BadTokenForSwap {
            base_token: pool_config.token0,
            quote_token: pool_config.token1,
        });
    }

    // we will only ever have a route length of one, this will likely change once we start selecting different routes
    let pool_route = SwapAmountInRoute {
        pool_id: pool_config.pool_id,
        token_out_denom: params.token_out_denom.to_string(),
    };

    let swap_msg: Result<CosmosMsg, _> = match dex_router {
        Some(dex_router_address) => {
            let offer_asset = AssetInfo::Native(token_in_denom.clone().to_string());
            let ask_asset = AssetInfo::Native(params.token_out_denom.to_string());

            let recommended_out: Uint128 = match params.recommended_swap_route.clone() {
                Some(operations) => deps.querier.query_wasm_smart(
                    dex_router_address.to_string(),
                    &QueryMsg::SimulateSwapOperations {
                        offer_amount: params.token_in_amount,
                        operations,
                    },
                )?,
                None => 0u128.into(),
            };
            let best_path: Option<BestPathForPairResponse> = deps.querier.query_wasm_smart(
                dex_router_address.to_string(),
                &QueryMsg::BestPathForPair {
                    offer_asset: offer_asset.into(),
                    ask_asset: ask_asset.into(),
                    exclude_paths: None,
                    offer_amount: params.token_in_amount,
                },
            )?;
            let best_out = match best_path.clone() {
                Some(best_path) => best_path.return_amount,
                None => 0u128.into(),
            };

            // if we need to force the route
            if params.force_swap_route {
                match params.recommended_swap_route {
                    Some(recommended_swap_route) => execute_swap_operations(
                        dex_router_address,
                        recommended_swap_route,
                        params.token_out_min_amount,
                        &token_in_denom.clone(),
                        params.token_in_amount,
                    ),
                    None => Err(ContractError::TryForceRouteWithoutRecommendedSwapRoute {}),
                }
            } else if best_out.is_zero() && recommended_out.is_zero() {
                Ok(swap_exact_amount_in(
                    env,
                    pool_route,
                    params.token_in_amount,
                    &token_in_denom.clone(),
                    params.token_out_min_amount,
                ))
            } else if best_out.ge(&recommended_out) {
                execute_swap_operations(
                    dex_router_address,
                    best_path.unwrap().operations.into(),
                    params.token_out_min_amount,
                    &token_in_denom.clone(),
                    params.token_in_amount,
                )
            } else {
                // recommended_out > best_out
                execute_swap_operations(
                    dex_router_address,
                    params.recommended_swap_route.unwrap(), // will be some here
                    params.token_out_min_amount,
                    &token_in_denom.clone(),
                    params.token_in_amount,
                )
            }
        }
        None => Ok(swap_exact_amount_in(
            env,
            pool_route,
            params.token_in_amount,
            &token_in_denom.clone(),
            params.token_out_min_amount,
        )),
    };
    swap_msg
}

fn swap_exact_amount_in(
    env: &Env,
    pool_route: SwapAmountInRoute,
    token_in_amount: Uint128,
    token_in_denom: &String,
    token_out_min_amount: Uint128,
) -> CosmosMsg {
    osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountIn {
        sender: env.contract.address.to_string(),
        routes: vec![pool_route],
        token_in: Some(OsmoCoin {
            denom: token_in_denom.to_string(),
            amount: token_in_amount.to_string(),
        }),
        token_out_min_amount: token_out_min_amount.to_string(),
    }
    .into()
}

fn execute_swap_operations(
    dex_router_address: Addr,
    operations: SwapOperationsListUnchecked,
    token_out_min_amount: Uint128,
    token_in_denom: &String,
    token_in_amount: Uint128,
) -> Result<CosmosMsg, ContractError> {
    let swap_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: dex_router_address.to_string(),
        msg: to_json_binary(&ExecuteMsg::ExecuteSwapOperations {
            operations,
            minimum_receive: Some(token_out_min_amount),
            to: None,
            offer_amount: None,
        })?,
        funds: vec![Coin {
            denom: token_in_denom.to_string(),
            amount: token_in_amount,
        }],
    }
    .into();

    Ok(swap_msg)
}

#[cfg(test)]
mod tests {
    use crate::vault::swap::SwapParams;
    use cosmwasm_std::{
        testing::{mock_dependencies_with_balance, mock_env},
        Coin, CosmosMsg, Uint128,
    };

    use crate::state::{PoolConfig, POOL_CONFIG};

    fn mock_pool_config() -> PoolConfig {
        PoolConfig {
            pool_id: 1,
            token0: "token0".to_string(),
            token1: "token1".to_string(),
        }
    }

    #[test]
    fn test_proper_swap() {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(1000),
        }]);
        let deps_mut = deps.as_mut();

        let env = mock_env();

        let token_in_amount = Uint128::new(100);
        let token_in_denom = "token0".to_string();
        let token_out_min_amount = Uint128::new(100);
        let token_out_denom = "token1".to_string();

        POOL_CONFIG
            .save(deps_mut.storage, &mock_pool_config())
            .unwrap();
        let swap_params = SwapParams {
            token_in_amount,
            token_out_min_amount,
            token_in_denom,
            token_out_denom,
            recommended_swap_route: None,
            force_swap_route: false,
        };

        let result = super::swap(deps_mut, &env, swap_params).unwrap();

        if let CosmosMsg::Stargate { type_url: _, value } = result {
            let msg_swap =
                osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountIn::try_from(
                    value,
                )
                .unwrap();

            assert!(msg_swap.sender == env.contract.address);
            assert!(msg_swap.routes.len() == 1);
            assert!(msg_swap.routes[0].pool_id == 1);
            assert!(msg_swap.routes[0].token_out_denom == *"token1");
            assert!(msg_swap.token_in.clone().unwrap().denom == *"token0");
            assert!(msg_swap.token_in.unwrap().amount == *"100");
            assert!(token_out_min_amount.to_string() == *"100");
        } else {
            panic!("Unexpected message type: {:?}", result);
        }
    }

    #[test]
    fn test_bad_denom_swap() {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(1000),
        }]);
        let deps_mut = deps.as_mut();

        let env = mock_env();

        let token_in_amount = Uint128::new(100);
        let token_in_denom = "token3".to_string();
        let token_out_min_amount = Uint128::new(100);
        let token_out_denom = "token1".to_string();

        let swap_params = SwapParams {
            token_in_amount,
            token_out_min_amount,
            token_in_denom,
            token_out_denom,
            recommended_swap_route: None,
            force_swap_route: false,
        };

        POOL_CONFIG
            .save(deps_mut.storage, &mock_pool_config())
            .unwrap();

        let err = super::swap(deps_mut, &env, swap_params).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Bad token out requested for swap, must be one of: \"token0\", \"token1\"".to_string()
        );
    }
}
