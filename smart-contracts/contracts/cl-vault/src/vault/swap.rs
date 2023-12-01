use std::str::FromStr;

use apollo_cw_asset::AssetInfo;
use cosmwasm_std::{Coin, CosmosMsg, DepsMut, Env, QuerierWrapper, Storage, Uint128, WasmMsg, WasmQuery};
use cw_dex_router::{operations::SwapOperationsListUnchecked, msg::QueryMsg};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin, osmosis::poolmanager::v1beta1::SwapAmountInRoute,
};

use crate::{state::{POOL_CONFIG, DEX_ROUTER}, ContractError};

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
pub fn swap(
    deps: DepsMut,
    env: &Env,
    token_in_amount: Uint128,
    token_in_denom: &String,
    token_out_min_amount: Uint128,
    token_out_denom: &String,
    recommended_swap_route: Option<SwapOperationsListUnchecked>,
    force_swap_route: bool,
) -> Result<CosmosMsg, ContractError> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let dex_router = DEX_ROUTER.load(deps.storage)?;

    
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
        token_out_denom,
    };

    let swap_msg: CosmosMsg = match dex_router {
        Some(dex_router_address) => {
            let offer_asset = AssetInfo::Native(token_in_denom.to_string());
            let recommended_out = deps.querier.query_wasm_smart(dex_router_address, &QueryMsg::BestPathForPair { offer_asset: (), ask_asset: (), exclude_paths: () })?;

            Ok()
        },
        None => todo!(),
    }
        osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountIn {
            sender: env.contract.address.to_string(),
            routes: vec![pool_route],
            token_in: Some(OsmoCoin {
                denom: token_in_denom.to_string(),
                amount: token_in_amount.to_string(),
            }),
            token_out_min_amount: token_out_min_amount.to_string(),
        }
        .into();

    Ok(swap_msg)
}

#[cfg(test)]
mod tests {
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

        let result = super::swap(
            deps_mut,
            &env,
            token_in_amount,
            &token_in_denom,
            token_out_min_amount,
            &token_out_denom,
            None,
            false,
        )
        .unwrap();

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

        POOL_CONFIG
            .save(deps_mut.storage, &mock_pool_config())
            .unwrap();

        let err = super::swap(
            deps_mut,
            &env,
            token_in_amount,
            &token_in_denom,
            token_out_min_amount,
            &token_out_denom,
            None,
            false,
        )
        .unwrap_err();

        assert_eq!(
            err.to_string(),
            "Bad token out requested for swap, must be one of: \"token0\", \"token1\"".to_string()
        );
    }
}
