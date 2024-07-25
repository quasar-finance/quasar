use cosmwasm_std::{Addr, CosmosMsg, DepsMut, Fraction, Response, Uint128};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

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
    contract_address: Addr,
    swap_operations: Vec<SwapOperation>,
) -> Result<Response, ContractError> {
    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    if swap_operations.is_empty() {
        return Err(ContractError::EmptySwapOperations {});
    }
    let mut swap_msgs = Vec::new();

    for swap_operation in swap_operations {
        let token_in_denom = &swap_operation.token_in_denom;

        // Assert that no BASE_DENOM or QUOTE_DENOM is trying to be swapped as token_in
        if token_in_denom == &pool_config.token0 || token_in_denom == &pool_config.token1 {
            return Err(ContractError::InvalidSwapAssets {});
        }

        // Get contract balance for the input token
        let balance_in_contract = deps
            .querier
            .query_balance(contract_address.clone(), token_in_denom.clone())?
            .amount;

        if balance_in_contract.is_zero() {
            return Err(ContractError::InsufficientFundsForSwap {
                balance: balance_in_contract,
                needed: Uint128::new(1),
            });
        }

        // Split the balance in contract into two parts
        // 1. tokens to be swapped in token0
        // 2. tokens to be swapped in token1
        let part_0_amount = balance_in_contract.checked_div(Uint128::new(2))?;
        let part_1_amount = balance_in_contract
            .checked_add(Uint128::new(1))?
            .checked_div(Uint128::new(2))?;

        // Calculate the minimum amount of tokens to be received
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
            contract_address.clone(),
            SwapParams {
                pool_id: swap_operation.pool_id_0,
                token_in_amount: part_0_amount,
                token_in_denom: token_in_denom.clone(),
                token_out_min_amount: token_out_min_amount_0,
                token_out_denom: pool_config.token0.clone(),
                forced_swap_route: swap_operation.forced_swap_route_token_0,
            },
        )?);
        swap_msgs.push(swap_msg(
            &deps,
            contract_address.clone(),
            SwapParams {
                pool_id: swap_operation.pool_id_1,
                token_in_amount: part_1_amount,
                token_in_denom: token_in_denom.clone(),
                token_out_min_amount: token_out_min_amount_1,
                token_out_denom: pool_config.token1.clone(),
                forced_swap_route: swap_operation.forced_swap_route_token_1,
            },
        )?);
    }

    Ok(Response::new()
        .add_messages(swap_msgs)
        .add_attribute("method", "execute")
        .add_attribute("action", "swap_non_vault_funds"))
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
            pool_id: 1,
            token_in_amount,
            token_out_min_amount,
            token_in_denom,
            token_out_denom,
            forced_swap_route: None,
        };

        let result = super::swap_msg(&deps_mut, env.contract.address.clone(), swap_params).unwrap();

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
