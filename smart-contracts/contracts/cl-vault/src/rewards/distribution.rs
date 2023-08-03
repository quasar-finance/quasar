use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, Addr, Binary, Coin, Decimal, Deps, DepsMut, Env, Fraction, Order, Response, SubMsg,
    Uint128,
};

use crate::{
    error::ContractResult,
    reply::Replies,
    state::{
        CURRENT_REWARDS, LOCKED_TOKENS, LOCKED_TOTAL, POSITION, STRATEGIST_REWARDS, USER_REWARDS,
        VAULT_CONFIG,
    },
    ContractError,
};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin,
    osmosis::concentratedliquidity::v1beta1::{
        MsgCollectIncentives, MsgCollectIncentivesResponse, MsgCollectSpreadRewards,
        MsgCollectSpreadRewardsResponse,
    },
};

use super::rewards::Rewards;

/// claim_rewards claims rewards from Osmosis and update the rewards map to reflect each users rewards
pub fn claim_rewards(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    CURRENT_REWARDS.save(deps.storage, &Rewards::new())?;
    let msg = collect_incentives(deps.as_ref(), env)?;
    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        msg,
        Replies::CollectIncentives as u64,
    )))
}

pub fn handle_collect_incentives_reply(
    deps: DepsMut,
    env: Env,
    data: Binary,
) -> Result<Response, ContractError> {
    // save the response from the collect incentives
    let response: MsgCollectIncentivesResponse = data.try_into()?;
    CURRENT_REWARDS.update(
        deps.storage,
        |mut rewards| -> Result<Rewards, ContractError> {
            rewards.update_rewards(response.collected_incentives);
            Ok(rewards)
        },
    )?;

    // collect the spread rewards
    let msg = collect_spread_rewards(deps.as_ref(), env)?;
    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        msg,
        Replies::CollectSpreadRewards as u64,
    )))
}

pub fn handle_collect_spread_rewards_reply(
    deps: DepsMut,
    env: Env,
    data: Binary,
) -> Result<Response, ContractError> {
    // after we have collected both the spread rewards and the incentives, we can distribute them over the share holders
    // we don't need to save the rewards here again, just pass it to update rewards
    let response: MsgCollectSpreadRewardsResponse = data.try_into()?;
    let mut rewards = CURRENT_REWARDS.load(deps.storage)?;
    rewards.update_rewards(response.collected_spread_rewards)?;

    // update the rewards map against all user's locked up vault shares
    distribute_rewards(deps, rewards)?;
    todo!()
}

fn distribute_rewards(mut deps: DepsMut, mut rewards: Rewards) -> Result<(), ContractError> {
    let vault_config = VAULT_CONFIG.load(deps.storage)?;

    // calculate the strategist fee
    let strategist_fee = rewards.sub_percentage(
        vault_config.performance_fee.numerator(),
        vault_config.performance_fee.denominator(),
    )?;
    STRATEGIST_REWARDS.update(deps.storage, |mut old| old.add(strategist_fee))?;

    let total_shares = LOCKED_TOTAL.load(deps.storage)?;
    // for each user with locked tokens, we distribute some part of the rewards to them
    let user_rewards: Result<Vec<(Addr, Rewards)>, ContractError> = LOCKED_TOKENS
        .range(deps.branch().storage, None, None, Order::Ascending)
        .map(|v| -> Result<(Addr, Rewards), ContractError> {
            let (address, user_shares) = v?;
            // calculate the amount of each asset the user should get in rewards
            // we need to always round down here, so we never expect more rewards than we have
            let user_rewards = rewards.percentage(total_shares, user_shares);
            Ok((address, user_rewards))
        })
        .collect();

    user_rewards?
        .into_iter()
        .try_for_each(|(addr, reward)| -> ContractResult<()> {
            USER_REWARDS.update(deps.storage, addr, |old| -> ContractResult<Rewards> {
                if let Some(old_user_rewards) = old {
                    Ok(reward.add(old_user_rewards)?)
                } else {
                    Ok(reward)
                }
            })?;
            Ok(())
        })?;

    Ok(())
}

fn collect_incentives(deps: Deps, env: Env) -> Result<MsgCollectIncentives, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectIncentives {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}

fn collect_spread_rewards(deps: Deps, env: Env) -> Result<MsgCollectSpreadRewards, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectSpreadRewards {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use crate::state::Position;

    use super::*;

    #[test]
    fn test_collect_incentives() {
        let mut deps = mock_dependencies();
        let position = Position { position_id: 1 };
        POSITION.save(deps.as_mut().storage, &position).unwrap();
        let env = mock_env();

        let res = collect_incentives(deps.as_ref(), env.clone()).unwrap();

        // Check that the correct message type is returned
        assert_eq!(
            res,
            MsgCollectIncentives {
                position_ids: vec![1], // Check that the correct position_id is included in the message
                sender: env.contract.address.into(),
            }
        );
    }

    #[test]
    fn test_collect_spread_rewards() {
        let mut deps = mock_dependencies();
        let position = Position { position_id: 1 };
        POSITION.save(deps.as_mut().storage, &position).unwrap();
        let env = mock_env();

        let res = collect_spread_rewards(deps.as_ref(), env.clone()).unwrap();

        // Check that the correct message type is returned
        assert_eq!(
            res,
            MsgCollectSpreadRewards {
                position_ids: vec![1], // Check that the correct position_id is included in the message
                sender: env.contract.address.into(),
            }
        );
    }
}
