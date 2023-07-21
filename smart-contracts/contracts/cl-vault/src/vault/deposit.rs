use std::str::FromStr;

use cosmwasm_std::{
    from_binary, to_binary, Addr, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Order, Reply,
    Response, SubMsg, Uint128,
};
use cw_utils::PaymentError;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin,
    osmosis::{
        concentratedliquidity::v1beta1::{
            MsgCreatePosition, MsgCreatePositionResponse, PositionByIdRequest, PositionByIdResponse,
        },
        poolmanager::v1beta1::{
            EstimateSwapExactAmountInRequest, EstimateSwapExactAmountInResponse,
            MsgSwapExactAmountIn, MsgSwapExactAmountInResponse, PoolRequest, PoolResponse,
            SwapAmountInRoute, TotalPoolLiquidityRequest, TotalPoolLiquidityResponse,
        },
    },
};

use crate::{
    state::{Replies, INVESTMENT, REPLIES, STRATEGY, USER_BALANCE},
    ContractError,
};

pub fn execute_deposit(
    deps: DepsMut,
    env: Env,
    info: &MessageInfo,
    expected_amount: Uint128,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    // find recipient
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    // check that only the expected amount of base token was sent.
    let investment = INVESTMENT.load(deps.storage)?;
    let received_amount = must_pay(&info, &investment.base_denom)?;
    if expected_amount != received_amount {
        return Err(ContractError::DepositMismatch {
            expected: expected_amount,
            received: received_amount,
        });
    }

    // depending on the upper/lower tick and the current price, we need to create a position giving both tokens in the right ratio.
    // amount0 stays in base denom tokens and amount1 is swapped for quote denom tokens before creating the position
    let (amount0, amount1) = calculate_amount_to_swap(deps.as_ref(), &env, received_amount)?;

    let _swap_request = EstimateSwapExactAmountInRequest {
        pool_id: investment.pool_id,
        routes: vec![SwapAmountInRoute {
            pool_id: investment.pool_id,
            token_out_denom: investment.quote_denom.clone(),
        }],
        token_in: investment.base_denom.clone(),
    };

    // TODO: send a sync query and parse the response, if possible?
    let swap_response: EstimateSwapExactAmountInResponse = EstimateSwapExactAmountInResponse {
        token_out_amount: "0".to_string(),
    };

    // TODO: decide whether to use gamm or pool manager for the swap
    let strategy = STRATEGY.load(deps.storage)?;
    let msg_swap = MsgSwapExactAmountIn {
        sender: env.contract.address.to_string(),
        routes: vec![SwapAmountInRoute {
            pool_id: investment.pool_id,
            token_out_denom: investment.quote_denom,
        }],
        token_in: Some(Coin {
            denom: investment.base_denom,
            amount: amount1.to_string(),
        }),
        token_out_min_amount: calculate_slippage(
            Uint128::from_str(&swap_response.token_out_amount)?,
            strategy.slippage_tolerance,
        )?
        .to_string(),
    };

    let submsg_id = next_reply_id(deps.as_ref())?;
    REPLIES.save(
        deps.storage,
        submsg_id,
        &Replies::Swap {
            user_addr: recipient,
            amount0,
        },
    )?;

    let sub_msg_swap = SubMsg::reply_always(
        CosmosMsg::Stargate {
            type_url: MsgSwapExactAmountIn::TYPE_URL.to_owned(),
            value: to_binary(&msg_swap)?,
        },
        submsg_id,
    );
    Ok(Response::new().add_submessage(sub_msg_swap))
}

pub fn handle_swap_reply(
    deps: DepsMut,
    env: Env,
    user_addr: Addr,
    amount0: Uint128,
    msg: Reply,
) -> Result<Response, ContractError> {
    // TODO: parse reply message correctly & safely handle unwraps
    let data: MsgSwapExactAmountInResponse =
        from_binary(&msg.result.into_result().unwrap().data.unwrap())?;

    // amount0 is the amount that we calculated during execute_deposit
    let amount1 = Uint128::from_str(&data.token_out_amount)?;

    let investment = INVESTMENT.load(deps.storage)?;
    let strategy = STRATEGY.load(deps.storage)?;

    let create_position = MsgCreatePosition {
        pool_id: investment.pool_id,
        sender: env.contract.address.to_string(),
        lower_tick: strategy.lower_tick,
        upper_tick: strategy.upper_tick,
        tokens_provided: vec![
            Coin {
                denom: investment.base_denom,
                amount: amount0.to_string(),
            },
            Coin {
                denom: investment.quote_denom,
                amount: amount1.to_string(),
            },
        ],
        token_min_amount0: calculate_slippage(amount0, strategy.slippage_tolerance)?.to_string(),
        token_min_amount1: calculate_slippage(amount1, strategy.slippage_tolerance)?.to_string(),
    };

    let msg: CosmosMsg<Empty> = CosmosMsg::Stargate {
        type_url: MsgCreatePosition::TYPE_URL.to_owned(),
        value: to_binary(&create_position)?,
    };

    let submsg_id = next_reply_id(deps.as_ref())?;
    let reply = Replies::CreatePosition { user_addr };
    REPLIES.save(deps.storage, submsg_id, &reply)?;

    let sub_msg = SubMsg::reply_always(msg, submsg_id);

    Ok(Response::new().add_submessage(sub_msg))
}

pub fn handle_create_position_reply(
    deps: DepsMut,
    _env: Env,
    user_addr: Addr,
    msg: Reply,
) -> Result<Response, ContractError> {
    // TODO: parse reply message correctly & safely handle unwraps
    let data: MsgCreatePositionResponse =
        from_binary(&msg.result.into_result().unwrap().data.unwrap())?;

    // TODO: amount of liquidity the user gets out (does this number change over time?)
    // should come back on the response. mind for re-deposits math
    // is "liquidity_created" taking into account previously added liquidity? We assume not
    let user_amount = Uint128::from_str(&data.liquidity_created)?;

    let _position_request = PositionByIdRequest {
        position_id: data.position_id,
    };

    // TODO: send a sync query and parse the response, if possible?
    let position_response: PositionByIdResponse = PositionByIdResponse { position: None };

    // parse posistion_response and get the amount of liquidity the vault owns (errors are dup now)
    let total_amount = Uint128::from_str(
        &position_response
            .position
            .ok_or(ContractError::PositionNotFound {
                id: data.position_id,
            })?
            .position
            .ok_or(ContractError::PositionNotFound {
                id: data.position_id,
            })?
            .liquidity,
    )?;

    // TODO: realalise that this is actually returning liquidity and not shares
    let investment = INVESTMENT.load(deps.storage)?;
    let _total_liquidity_request = TotalPoolLiquidityRequest {
        pool_id: investment.pool_id,
    };

    let _total_liquidity_response: TotalPoolLiquidityResponse;

    let _total_shares_request = PoolRequest {
        pool_id: investment.pool_id,
    };

    let _total_shares_response: PoolResponse;

    // TODO: amount of shares the vault owns
    let _query = osmosis_std::types::osmosis::poolmanager::v1beta1::PoolRequest {
        pool_id: investment.pool_id,
    };

    // TODO: calculate total amount of existing pool shares
    let total_shares = Uint128::zero();

    let user_shares = calculate_user_shares(user_amount, total_amount, total_shares)?;

    // TODO: mint vault tokens to user (user_shares)

    // TODO: should we save user address and estimate amount to state here or better in the callback?
    USER_BALANCE.update(
        deps.storage,
        user_addr,
        |balance| -> Result<_, ContractError> {
            Ok(balance.unwrap_or(Uint128::zero()) + user_shares)
        },
    )?;

    // Ok(Response::new().add_submessage(sub_msg))

    unimplemented!()

    // // Compound. Also stakes the users deposit
    // let compound_res = self.compound(deps, &env, user_deposit_amount)?;

    // // Mint vault tokens to recipient
    // let mint_res = Response::new().add_message(
    //     CallbackMsg::MintVaultToken {
    //         amount,
    //         recipient: recipient.clone(),
    //     }
    //     .into_cosmos_msg(&env)?,
    // );

    // let event = Event::new("apollo/vaults/execute_staking").add_attributes(vec![
    //     attr("action", "deposit"),
    //     attr("recipient", recipient),
    //     attr("amount", amount),
    // ]);

    // // Merge responses and add message to mint vault token
    // Ok(merge_responses(vec![receive_res, compound_res, mint_res]).add_event(event))
}

fn calculate_amount_to_swap(
    _deps: Deps,
    _env: &Env,
    _user_deposit_amount: Uint128,
) -> Result<(Uint128, Uint128), ContractError> {
    // TODO: set the two sides of liquidity equal to each other won't work
    todo!()
}

// this function returns the minimum amount of tokens that the position will accept when providing liquidity
// TODO: evaluate if we should use Decimal
fn calculate_slippage(
    expected_amount: Uint128,
    slippage_tolerance: Uint128,
) -> Result<Uint128, ContractError> {
    if slippage_tolerance.is_zero() {
        return Ok(expected_amount);
    }
    if slippage_tolerance > Uint128::new(10000) {
        return Err(ContractError::InvalidSlippageTolerance { slippage_tolerance });
    }
    let slippage = expected_amount
        .checked_mul(slippage_tolerance)?
        .checked_div(Uint128::new(10000))?; // 10000 because slippage_tolerance is in basis points

    Ok(expected_amount.checked_sub(slippage)?)
}

// TODO: precision and safe math?
fn calculate_user_shares(
    user_amount: Uint128,
    total_amount: Uint128,
    total_shares: Uint128,
) -> Result<Uint128, ContractError> {
    // TODO: figure if we need to use Uint256 here to not overflow
    let user_shares = user_amount / total_amount * total_shares;
    Ok(user_shares)
}

pub fn next_reply_id(deps: Deps) -> Result<u64, ContractError> {
    let last = REPLIES
        .range(deps.storage, None, None, Order::Descending)
        .next();
    Ok(last.map_or(0, |Ok((k, _))| k + 1))
}

// this is called from the reply hook
// don't forget to cleanup the REPLIES state item
// REPLIES.remove(deps.storage, msg.id);
fn execute_deposit_after_swap() {}
fn execute_mint_after_join() {}

/// If exactly one coin was sent, returns it regardless of denom.
/// Returns error if 0 or 2+ coins were sent
pub fn one_coin(info: &MessageInfo) -> Result<cosmwasm_std::Coin, PaymentError> {
    match info.funds.len() {
        0 => Err(PaymentError::NoFunds {}),
        1 => {
            let coin = &info.funds[0];
            if coin.amount.is_zero() {
                Err(PaymentError::NoFunds {})
            } else {
                Ok(coin.clone())
            }
        }
        _ => Err(PaymentError::MultipleDenoms {}),
    }
}

/// Requires exactly one denom sent, which matches the requested denom.
/// Returns the amount if only one denom and non-zero amount. Errors otherwise.
pub fn must_pay(info: &MessageInfo, denom: &str) -> Result<Uint128, PaymentError> {
    let coin = one_coin(info)?;
    if coin.denom != denom {
        Err(PaymentError::MissingDenom(denom.to_string()))
    } else {
        Ok(coin.amount)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Uint128;

    use crate::ContractError;

    use super::calculate_slippage;

    #[test]
    fn slippage_calculations() {
        assert_eq!(
            calculate_slippage(Uint128::new(100), Uint128::new(100)).unwrap(),
            Uint128::new(99)
        );
        assert_eq!(
            calculate_slippage(Uint128::new(1000), Uint128::new(200)).unwrap(),
            Uint128::new(980)
        );
        assert_eq!(
            calculate_slippage(Uint128::new(1000), Uint128::new(10000)).unwrap(),
            Uint128::new(0)
        );
        assert_eq!(
            calculate_slippage(Uint128::new(1000), Uint128::new(0)).unwrap(),
            Uint128::new(1000)
        );
        assert_eq!(
            calculate_slippage(Uint128::new(1000), Uint128::new(11111)).unwrap_err(),
            ContractError::InvalidSlippageTolerance {
                slippage_tolerance: Uint128::new(11111)
            }
        );
    }
}
