use cosmwasm_std::{
    coin, BankMsg, Binary, CosmosMsg, Decimal256, DepsMut, Env, MessageInfo, Response, SubMsg,
    Uint128,
};
use cw_utils::{must_pay, one_coin};
use osmosis_std::types::osmosis::{
    concentratedliquidity::v1beta1::{MsgWithdrawPosition, MsgWithdrawPositionResponse},
    tokenfactory::v1beta1::MsgBurn,
};

use crate::{
    concentrated_liquidity::{get_position, withdraw_from_position},
    reply::Replies,
    state::{CURRENT_WITHDRAWER, LOCKED_TOTAL, POOL_CONFIG, VAULT_DENOM},
    ContractError,
};

pub fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let receiver = recipient.unwrap_or(info.sender.to_string());

    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    // update the user's shares
    let shares = must_pay(&info, vault_denom.as_str())?;

    // shares sent in should equal the amount requested. This is redundant but its to comply with the vault standard
    if shares != amount {
        return Err(ContractError::IncorrectShares {});
    }

    // burn the shares
    let burn_coin = one_coin(&info)?;
    let burn: CosmosMsg = MsgBurn {
        sender: env.contract.address.clone().into_string(),
        amount: Some(burn_coin.into()),
        burn_from_address: env.contract.address.clone().into_string(),
    }
    .into();

    let addr = deps.api.addr_validate(&receiver)?;
    CURRENT_WITHDRAWER.save(deps.storage, &addr)?;

    // withdraw the user's funds from the position
    let msg = withdraw(deps, &env, shares)?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(msg, Replies::WithdrawUser as u64))
        .add_message(burn))
}

fn withdraw(
    deps: DepsMut,
    env: &Env,
    shares: Uint128,
) -> Result<MsgWithdrawPosition, ContractError> {
    let position = get_position(deps.storage, &deps.querier, env)?;
    let total_liquidity: Decimal256 = position
        .position
        .ok_or(ContractError::PositionNotFound)?
        .liquidity
        .parse()?;
    let total_shares: Uint128 = LOCKED_TOTAL.load(deps.storage)?;

    // user_liquidity = user_shares * total_liquidity / total_shares
    let user_liquidity = Decimal256::from_ratio(shares, 1_u128)
        .checked_mul(total_liquidity)?
        .checked_div(Decimal256::from_ratio(total_shares, 1_u128))?;
    withdraw_from_position(deps.storage, env, user_liquidity)
}

fn handle_withdraw_user_reply(deps: DepsMut, data: Binary) -> Result<Response, ContractError> {
    // parse the reply and instantiate the funds we want to send
    let response: MsgWithdrawPositionResponse = data.try_into()?;
    let user = CURRENT_WITHDRAWER.load(deps.storage)?;
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    let coin0 = coin(response.amount0.parse()?, pool_config.token0);
    let coin1 = coin(response.amount1.parse()?, pool_config.token1);

    // send the funds to the user
    let msg = BankMsg::Send {
        to_address: user.to_string(),
        amount: vec![coin0, coin1],
    };
    Ok(Response::new().add_message(msg))
}

#[cfg(test)]
mod tests {
    use crate::state::PoolConfig;
    use cosmwasm_std::{testing::mock_dependencies, Addr, CosmosMsg};

    use super::*;

    // the execute withdraw flow should be easiest to test in test-tube, since it requires quite a bit of Osmsosis specific information
    // we just test the handle withdraw implementation here
    #[test]
    fn handle_withdraw_user_reply_works() {
        let mut deps = mock_dependencies();
        let to_address = Addr::unchecked("bolice");
        CURRENT_WITHDRAWER
            .save(deps.as_mut().storage, &to_address)
            .unwrap();
        POOL_CONFIG
            .save(
                deps.as_mut().storage,
                &PoolConfig {
                    pool_id: 1,
                    token0: "uosmo".into(),
                    token1: "uatom".into(),
                },
            )
            .unwrap();

        let reply = MsgWithdrawPositionResponse {
            amount0: "1000".to_string(),
            amount1: "1000".to_string(),
        }
        .try_into()
        .unwrap();

        let response = handle_withdraw_user_reply(deps.as_mut(), reply).unwrap();
        assert_eq!(
            response.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: to_address.to_string(),
                amount: vec![coin(1000, "uosmo"), coin(1000, "uatom")]
            })
        )
    }
}
