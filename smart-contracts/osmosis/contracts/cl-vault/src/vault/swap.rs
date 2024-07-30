use cosmwasm_std::{Coin, CosmosMsg, Decimal, DepsMut, Env, Fraction, Response, Uint128};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{Pool, Position as OsmoPosition};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::helpers::getters::{
    get_position_balance, get_single_sided_deposit_0_to_1_swap_amount,
    get_single_sided_deposit_1_to_0_swap_amount, get_twap_price,
};
use crate::helpers::msgs::swap_msg;
use crate::state::{PoolConfig, POOL_CONFIG};
use crate::{state::VAULT_CONFIG, ContractError};

#[cosmwasm_schema::cw_serde]
pub enum SwapDirection {
    ZeroToOne,
    OneToZero,
    AnyToOne,
    AnyToZero,
}

// struct used by swap.rs on swap non vault funds
#[cosmwasm_schema::cw_serde]
pub struct SwapOperation {
    pub token_in_denom: String,
    pub pool_id_token_0: u64, // the osmosis pool_id as mandatory to have at least the chance to swap on CL pools
    pub pool_id_token_1: u64, // the osmosis pool_id as mandatory to have at least the chance to swap on CL pools
    pub forced_swap_route_token_0: Option<Vec<SwapAmountInRoute>>,
    pub forced_swap_route_token_1: Option<Vec<SwapAmountInRoute>>,
}

/// SwapCalculationResult holds the result of a swap calculation
pub struct SwapCalculationResult {
    pub swap_msg: CosmosMsg,
    pub token_in: Coin,
    pub min_token_out: Coin,
}

/// SwapParams holds the parameters for a swap
pub struct SwapParams {
    pub pool_id: u64, // the osmosis pool id in case of no cw_dex_router or no best/recommended route
    pub token_in: Coin,
    pub min_token_out: Coin,
    pub forced_swap_route: Option<Vec<SwapAmountInRoute>>,
}

pub fn execute_swap_non_vault_funds(
    deps: DepsMut,
    env: Env,
    swap_operations: Vec<SwapOperation>,
    twap_window_seconds: Option<u64>,
) -> Result<Response, ContractError> {
    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    if swap_operations.is_empty() {
        return Err(ContractError::EmptySwapOperations {});
    }
    let mut swap_msgs: Vec<CosmosMsg> = Vec::new();

    for swap_operation in swap_operations {
        let token_in_denom = &swap_operation.token_in_denom;

        // Assert that no BASE_DENOM or QUOTE_DENOM is trying to be swapped as token_in
        if token_in_denom == &pool_config.token0 || token_in_denom == &pool_config.token1 {
            return Err(ContractError::InvalidSwapAssets {});
        }

        // Get contract balance for the input token
        let balance_in_contract = deps
            .querier
            .query_balance(env.clone().contract.address, token_in_denom.clone())?
            .amount;
        if balance_in_contract.is_zero() {
            return Err(ContractError::InsufficientFundsForSwap {
                balance: balance_in_contract,
                needed: Uint128::new(1),
            });
        }

        // Get the current position balance ratio to compute the amount of external funds we want to swap into either token0 or token1 from the vault's pool
        // TODO: This new get_position_balance helper looks redundant with get_depositable_tokens_impl, can we reuse this instead?
        let position_balance = get_position_balance(deps.storage, &deps.querier)?;
        let to_token0_amount =
            Uint128::from((balance_in_contract.u128() as f64 * position_balance.0) as u128); // balance * ratio computed by current position balancing
        let to_token1_amount =
            Uint128::from((balance_in_contract.u128() as f64 * position_balance.1) as u128); // balance * ratio computed by current position balancing

        // TODO: Validate that the swap_operation.pool_id_0 is about token_in_denom and pool_config.token0 assets or throw error
        let twap_price_token_0 = get_twap_price(
            &deps.querier,
            env.block.time,
            twap_window_seconds.unwrap_or_default(), // default to 0 if not provided
            swap_operation.pool_id_token_0,
            token_in_denom.to_string(),
            pool_config.clone().token0,
        )?;
        swap_msgs.push(
            calculate_swap_amount(
                &deps,
                &env,
                SwapDirection::AnyToZero,
                Coin {
                    denom: token_in_denom.to_string(),
                    amount: to_token0_amount,
                },
                vault_config.swap_max_slippage,
                swap_operation.forced_swap_route_token_0,
                twap_price_token_0,
            )?
            .swap_msg,
        );

        // TODO: Validate that the swap_operation.pool_id_1 is about token_in_denom and pool_config.token1 assets or throw error
        let twap_price_token_1 = get_twap_price(
            &deps.querier,
            env.block.time,
            twap_window_seconds.unwrap_or_default(), // default to 0 if not provided
            swap_operation.pool_id_token_1,
            token_in_denom.to_string(),
            pool_config.clone().token1,
        )?;
        swap_msgs.push(
            calculate_swap_amount(
                &deps,
                &env,
                SwapDirection::AnyToOne,
                Coin {
                    denom: token_in_denom.to_string(),
                    amount: to_token1_amount,
                },
                vault_config.swap_max_slippage,
                swap_operation.forced_swap_route_token_1,
                twap_price_token_1,
            )?
            .swap_msg,
        );
    }

    Ok(Response::new()
        .add_messages(swap_msgs)
        .add_attribute("method", "execute")
        .add_attribute("action", "swap_non_vault_funds"))
}

pub fn calculate_token_in_direction(
    pool_config: &PoolConfig,
    pool_details: Pool,
    position: OsmoPosition,
    tokens_provided: (Uint128, Uint128),
) -> Result<(Coin, SwapDirection, Uint128), ContractError> {
    if !tokens_provided.0.is_zero() {
        // range is above current tick
        let token_in = if pool_details.current_tick > position.upper_tick {
            Coin {
                denom: pool_config.token0.clone(),
                amount: tokens_provided.0,
            }
        } else {
            Coin {
                denom: pool_config.token0.clone(),
                amount: get_single_sided_deposit_0_to_1_swap_amount(
                    tokens_provided.0,
                    position.lower_tick,
                    pool_details.current_tick,
                    position.upper_tick,
                )?,
            }
        };
        let left_over_amount = tokens_provided.0.checked_sub(token_in.amount)?;
        Ok((token_in, SwapDirection::ZeroToOne, left_over_amount))
    } else {
        // current tick is above range
        let token_in = if pool_details.current_tick < position.lower_tick {
            Coin {
                denom: pool_config.token1.clone(),
                amount: tokens_provided.1,
            }
        } else {
            Coin {
                denom: pool_config.token1.clone(),
                amount: get_single_sided_deposit_1_to_0_swap_amount(
                    tokens_provided.1,
                    position.lower_tick,
                    pool_details.current_tick,
                    position.upper_tick,
                )?,
            }
        };
        let left_over_amount = tokens_provided.1.checked_sub(token_in.amount)?;
        Ok((token_in, SwapDirection::OneToZero, left_over_amount))
    }
}

#[allow(clippy::too_many_arguments)]
pub fn calculate_swap_amount(
    deps: &DepsMut,
    env: &Env,
    swap_direction: SwapDirection,
    token_in: Coin, // this is a coin so we can pass external funds as token_in
    max_slippage: Decimal,
    forced_swap_route: Option<Vec<SwapAmountInRoute>>,
    twap_price: Decimal,
) -> Result<SwapCalculationResult, ContractError> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    // Determine the target token and amount based on swap direction
    let (denom_out, amount_out_ratio) = match swap_direction {
        SwapDirection::ZeroToOne | SwapDirection::AnyToOne => (
            &pool_config.token1,
            (twap_price.numerator(), twap_price.denominator()), // TODO: Check if this is correct order for AnyToOne
        ),
        SwapDirection::OneToZero | SwapDirection::AnyToZero => (
            &pool_config.token0,
            (twap_price.denominator(), twap_price.numerator()), // TODO: Check if this is correct order for AnyToZero
        ),
    };

    // Compute the ideal amount
    let token_out_amount = token_in
        .amount
        .checked_multiply_ratio(amount_out_ratio.0, amount_out_ratio.1)?;

    // Compute the minimum amount based on the max slippage
    let min_token_out = Coin {
        denom: denom_out.clone(),
        amount: token_out_amount
            .checked_multiply_ratio(max_slippage.numerator(), max_slippage.denominator())?,
    };

    let swap_msg = swap_msg(
        &deps,
        env.clone().contract.address,
        SwapParams {
            pool_id: pool_config.pool_id,
            token_in: token_in.clone(),
            min_token_out: min_token_out.clone(),
            forced_swap_route,
        },
    )?;

    Ok(SwapCalculationResult {
        swap_msg,
        token_in,
        min_token_out,
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        state::{VaultConfig, VAULT_CONFIG},
        vault::swap::{execute_swap_non_vault_funds, SwapOperation, SwapParams},
    };
    use cosmwasm_std::{
        testing::{mock_dependencies_with_balance, mock_env, mock_info},
        Addr, Coin, CosmosMsg, Decimal, Uint128,
    };

    use crate::state::{PoolConfig, POOL_CONFIG};

    fn mock_pool_config() -> PoolConfig {
        PoolConfig {
            pool_id: 1,
            token0: "token0".to_string(),
            token1: "token1".to_string(),
        }
    }

    // Mock vault configuration
    fn mock_vault_config() -> VaultConfig {
        VaultConfig {
            swap_max_slippage: Decimal::from_str("0.005").unwrap(),
            performance_fee: Decimal::from_str("0.2").unwrap(),
            treasury: Addr::unchecked("treasury"),
            dex_router: Addr::unchecked("dex_router"),
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

        let token_in = Coin {
            denom: "token0".to_string(),
            amount: Uint128::new(100),
        };
        let min_token_out = Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        };

        POOL_CONFIG
            .save(deps_mut.storage, &mock_pool_config())
            .unwrap();
        let swap_params = SwapParams {
            pool_id: 1,
            token_in,
            min_token_out: min_token_out.clone(),
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
            assert!(min_token_out.amount.to_string() == *"100");
        } else {
            panic!("Unexpected message type: {:?}", result);
        }
    }

    #[test]
    fn test_execute_swap_non_vault_funds() {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "uscrt".to_string(),
            amount: Uint128::new(100),
        }]);
        let env = mock_env();
        let info = mock_info("tester", &[]);

        // Save the mock configurations
        POOL_CONFIG
            .save(deps.as_mut().storage, &mock_pool_config())
            .unwrap();
        VAULT_CONFIG
            .save(deps.as_mut().storage, &mock_vault_config())
            .unwrap();

        let swap_operations = vec![SwapOperation {
            token_in_denom: "uscrt".to_string(),
            pool_id_token_0: 1,
            pool_id_token_1: 1,
            forced_swap_route_token_0: None,
            forced_swap_route_token_1: None,
        }];

        let response =
            execute_swap_non_vault_funds(deps.as_mut(), env, swap_operations, None).unwrap();

        // Check response attributes
        assert_eq!(response.attributes[0].value, "execute");
        assert_eq!(response.attributes[1].value, "swap_non_vault_funds");

        // Check messages
        assert_eq!(response.messages.len(), 2);

        let token_out_min_amount_expected = Uint128::new(4975); // Expected minimum amount after slippage adjustment (49.75 from 50)

        println!("{:?}", response.messages[0].msg);
        println!("{:?}", response.messages[1].msg);

        // if let CosmosMsg::Stargate { type_url: _, value } = &response.messages[0].msg {
        //     let msg_swap = osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountIn::try_from(value).unwrap();
        //     assert_eq!(msg_swap.token_in.clone().unwrap().denom, "uscrt");
        //     assert_eq!(msg_swap.token_in.unwrap().amount, "50");
        //     assert_eq!(msg_swap.routes[0].pool_id, 1);
        //     assert_eq!(msg_swap.routes[0].token_out_denom, "token0");
        //     assert_eq!(msg_swap.token_out_min_amount, token_out_min_amount_expected.to_string());
        // } else {
        //     panic!("Unexpected message type: {:?}", response.messages[0].msg);
        // }

        // if let CosmosMsg::Stargate { type_url: _, value } = &response.messages[1].msg {
        //     let msg_swap = osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountIn::try_from(value).unwrap();
        //     assert_eq!(msg_swap.token_in.clone().unwrap().denom, "uscrt");
        //     assert_eq!(msg_swap.token_in.unwrap().amount, "50");
        //     assert_eq!(msg_swap.routes[0].pool_id, 1);
        //     assert_eq!(msg_swap.routes[0].token_out_denom, "token1");
        //     assert_eq!(msg_swap.token_out_min_amount, token_out_min_amount_expected.to_string());
        // } else {
        //     panic!("Unexpected message type: {:?}", response.messages[1].msg);
        // }
    }
}
