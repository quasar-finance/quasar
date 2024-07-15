use cosmwasm_std::{CosmosMsg, DepsMut, Env, Fraction, MessageInfo, Response, Uint128};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::helpers::assert::assert_range_admin;
use crate::helpers::msgs::swap_msg;
use crate::msg::SwapOperation;
use crate::state::POOL_CONFIG;
use crate::{state::VAULT_CONFIG, ContractError};

/// SwapCalculationResult holds the result of a swap calculation
pub struct SwapCalculationResult {
    pub swap_msg: CosmosMsg,
    pub token_in_denom: String,
    pub token_in_amount: Uint128,
    pub token_out_min_amount: Uint128,
    pub position_id: Option<u64>,
}

/// SwapParams holds the parameters for a swap
pub struct SwapParams {
    pub pool_id: u64, // the osmosis pool id in case of no cw_dex_router or no best/recommended route
    pub token_in_amount: Uint128,
    pub token_in_denom: String,
    pub token_out_min_amount: Uint128,
    pub token_out_denom: String,
    pub forced_swap_route: Option<Vec<SwapAmountInRoute>>,
}

pub fn execute_swap_non_vault_funds(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    swap_operations: Vec<SwapOperation>,
) -> Result<Response, ContractError> {
    // validate auto compound admin as the purpose of swaps are mainly around autocompound non-vault assets into assets that can be actually compounded.
    assert_range_admin(deps.storage, &info.sender)?;

    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    if swap_operations.is_empty() {
        return Err(ContractError::EmptySwapOperations {});
    }

    let mut swap_msgs: Vec<CosmosMsg> = vec![];

    for swap_operation in swap_operations {
        let token_in_denom = swap_operation.token_in_denom.clone();
        let pool_token_0 = pool_config.token0.clone();
        let pool_token_1 = pool_config.token1.clone();

        // Assert that no BASE_DENOM or QUOTE_DENOM is trying to be swapped as token_in
        if token_in_denom == pool_token_0 || token_in_denom == pool_token_1 {
            return Err(ContractError::InvalidSwapAssets {});
        }

        // Throw an Error if contract balance for the wanted denom is 0
        let balance_in_contract = deps
            .querier
            .query_balance(
                env.clone().contract.address,
                swap_operation.clone().token_in_denom,
            )?
            .amount;

        // TODO_FUTURE: This could be a <= condition against a threshold value mayube in dollars to avoid dust swaps
        if balance_in_contract == Uint128::zero() {
            // TODO: Use InsufficientFundsForSwap instead, this has been removed after STRATEGIST_REWARDS state eval removal
            return Err(ContractError::InsufficientFunds {});
        }

        // TODO_FUTURE: We could be swapping into the actual vault balance so we could prepend_swap() the autocompound entrypoint.
        let part_0_amount = balance_in_contract.checked_div(Uint128::new(2))?;
        let part_1_amount = balance_in_contract
            .checked_add(Uint128::new(1))?
            .checked_div(Uint128::new(2))?;

        // TODO_FUTURE: We should be passing the max_slippage from outside as we do during ModifyRange
        let token_out_min_amount_0 = part_0_amount.checked_multiply_ratio(
            vault_config.swap_max_slippage.numerator(),
            vault_config.swap_max_slippage.denominator(),
        )?;
        let token_out_min_amount_1 = part_1_amount.checked_multiply_ratio(
            vault_config.swap_max_slippage.numerator(),
            vault_config.swap_max_slippage.denominator(),
        )?;

        swap_msgs.push(swap_msg(
            &deps,
            &env,
            SwapParams {
                pool_id: swap_operation.pool_id_0,
                token_in_amount: part_0_amount,
                token_in_denom: token_in_denom.clone(),
                token_out_min_amount: token_out_min_amount_0,
                token_out_denom: pool_token_0,
                forced_swap_route: swap_operation.forced_swap_route_token_0,
            },
        )?);
        swap_msgs.push(swap_msg(
            &deps,
            &env,
            SwapParams {
                pool_id: swap_operation.pool_id_1,
                token_in_amount: part_1_amount,
                token_in_denom: token_in_denom.clone(),
                token_out_min_amount: token_out_min_amount_1,
                token_out_denom: pool_token_1,
                forced_swap_route: swap_operation.forced_swap_route_token_1,
            },
        )?);
    }

    Ok(Response::new()
        .add_messages(swap_msgs)
        .add_attribute("method", "execute")
        .add_attribute("action", "swap_non_vault_funds"))
}

/// estimate_swap can be used to pass correct token_out_min_amount values into swap()
/// for now this function can only be used for our pool
/// this will likely be expanded once we allow arbitrary pool swaps
// pub fn _estimate_swap(
//     querier: &QuerierWrapper,
//     storage: &mut dyn Storage,
//     _env: &Env,
//     token_in_amount: Uint128,
//     token_in_denom: &String,
//     _token_out_min_amount: Uint128,
// ) -> Result<Coin, ContractError> {
//     let pool_config = POOL_CONFIG.load(storage)?;

//     if !pool_config.pool_contains_token(token_in_denom) {
//         return Err(ContractError::BadTokenForSwap {
//             base_token: pool_config.token0,
//             quote_token: pool_config.token1,
//         });
//     }

//     // get token_out_denom
//     let token_out_denom = if *token_in_denom == pool_config.token0 {
//         pool_config.token1
//     } else {
//         pool_config.token0
//     };

//     // we will only ever have a route length of one, this will likely change once we start selecting different routes
//     let pool_route = SwapAmountInRoute {
//         pool_id: pool_config.pool_id,
//         token_out_denom: token_out_denom.to_string(),
//     };

//     let pm_querier =
//         osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier::new(querier);

//     let result = pm_querier.estimate_swap_exact_amount_in(
//         pool_config.pool_id,
//         token_in_amount.to_string() + token_in_denom,
//         vec![pool_route],
//     )?;

//     Ok(Coin {
//         denom: token_out_denom,
//         amount: Uint128::from_str(&result.token_out_amount)?,
//     })
// }

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
            pool_id: 1,
            token_in_amount,
            token_out_min_amount,
            token_in_denom,
            token_out_denom,
            forced_swap_route: None,
        };

        let result = super::swap_msg(&deps_mut, &env, swap_params).unwrap();

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

    // TODO: Move this test logic into any invoker of swap_msg tests as now its their concern to
    // validate the token_in_denom based on the context we swap (either non vault funds or during a rerange / anydeposit)
    // #[test]
    // fn test_bad_denom_swap() {
    //     let mut deps = mock_dependencies_with_balance(&[Coin {
    //         denom: "token0".to_string(),
    //         amount: Uint128::new(1000),
    //     }]);
    //     let deps_mut = deps.as_mut();

    //     let env = mock_env();

    //     let token_in_amount = Uint128::new(100);
    //     let token_in_denom = "token3".to_string();
    //     let token_out_min_amount = Uint128::new(100);
    //     let token_out_denom = "token1".to_string();

    //     let swap_params = SwapParams {
    //         token_in_amount,
    //         token_out_min_amount,
    //         token_in_denom,
    //         token_out_denom,
    //         recommended_swap_route: None,
    //         force_swap_route: false,
    //     };

    //     POOL_CONFIG
    //         .save(deps_mut.storage, &mock_pool_config())
    //         .unwrap();

    //     let err = super::swap_msg(&deps_mut, &env, swap_params).unwrap_err();

    //     assert_eq!(
    //         err.to_string(),
    //         "Bad token out requested for swap, must be one of: \"token0\", \"token1\"".to_string()
    //     );
    // }
}
