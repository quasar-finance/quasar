use cosmwasm_std::{
    to_binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response, SubMsg, Uint128,
};
use cw_utils::PaymentError;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin, osmosis::concentratedliquidity::v1beta1::MsgCreatePosition,
};

use crate::{
    state::{INVESTMENT, STRATEGY, USER_BALANCE},
    ContractError,
};

pub(crate) fn execute_deposit(
    deps: DepsMut,
    env: Env,
    info: &MessageInfo,
    expected_amount: Uint128,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    // find recipient
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let investment = INVESTMENT.load(deps.storage)?;

    // check that only the expected amount of base token was sent.
    let received_amount = must_pay(&info, &investment.base_denom)?;
    if expected_amount != received_amount {
        return Err(ContractError::DepositMismatch {
            expected: expected_amount,
            received: received_amount,
        });
    }

    let (amount0, amount1) = calculate_amount_to_swap(deps.as_ref(), &env, received_amount)?;

    // TODO: swap amount1 of base tokens for quote tokens

    // TODO: amount of liquidity the user gets out (does this number change over time?)
    let user_amount = Uint128::new(0);
    // TODO: amount of liquidity the vault owns
    let total_amount = Uint128::new(0);
    // TODO: amount of shares the vault owns
    let total_shares = Uint128::new(0);

    let user_shares = calculate_user_shares(user_amount, total_amount, total_shares)?;

    // TODO: mint vault tokens to user (user_shares)

    let strategy = STRATEGY.load(deps.storage)?;

    let cp = MsgCreatePosition {
        pool_id: investment.pool_id,
        sender: env.contract.address.to_string(),
        lower_tick: strategy.lower_tick,
        upper_tick: strategy.upper_tick,
        tokens_provided: vec![Coin {
            denom: investment.base_denom,
            amount: received_amount.into(),
        }],
        token_min_amount0: calculate_slippage(amount0, strategy.slippage_tolerance)?.to_string(),
        token_min_amount1: calculate_slippage(amount1, strategy.slippage_tolerance)?.to_string(),
    };

    let msg: CosmosMsg<Empty> = CosmosMsg::Stargate {
        type_url: MsgCreatePosition::TYPE_URL.to_owned(),
        value: to_binary(&cp)?,
    };

    let sub_msg = SubMsg::reply_always(
        msg, // TODO: think about id logic
        6969,
    );

    // TODO: should we save user address and estimate amount to state here or better in the callback?
    USER_BALANCE.update(
        deps.storage,
        recipient,
        |balance| -> Result<_, ContractError> { Ok(balance.unwrap_or(Uint128::zero()) + user_shares) },
    )?;

    Ok(Response::new().add_submessage(sub_msg))

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
    let user_shares = user_amount / total_amount * total_shares;
    Ok(user_shares)
}

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
