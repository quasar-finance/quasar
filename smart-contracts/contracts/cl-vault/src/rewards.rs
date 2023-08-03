use std::ops::Add;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, from_binary, Binary, Coin, Decimal, Deps, DepsMut, Env, Order, QuerierWrapper, Reply,
    Response, SubMsg, Uint128,
};
use cw_multi_test::Contract;

use crate::{
    reply::Replies,
    state::{CURRENT_REWARDS, LOCKED_TOKENS, LOCKED_TOTAL, POSITION, USER_REWARDS},
    ContractError,
};
use osmosis_std::types::{
    cosmos::{bank::v1beta1::BankQuerier, base::v1beta1::Coin as OsmoCoin},
    osmosis::concentratedliquidity::v1beta1::{
        MsgCollectIncentives, MsgCollectIncentivesResponse, MsgCollectSpreadRewards,
        MsgCollectSpreadRewardsResponse,
    },
};

#[cw_serde]
pub struct Rewards(Vec<Coin>);

impl Rewards {
    pub fn new() -> Rewards {
        Rewards::default()
    }

    /// calculates the percentage that the user should have
    pub fn percentage(&self, total_shares: Uint128, user_shares: Uint128) -> Rewards {
        // let percentage = Decimal::from_ratio(user_shares, total_shares);
        Rewards(
            self.0
                .iter()
                .map(|c| {
                    coin(
                        c.amount.multiply_ratio(user_shares, total_shares).u128(),
                        c.denom,
                    )
                })
                .collect(),
        )
    }

    /// merge any values already in Rewards and append any others
    pub fn update_rewards(&mut self, rewards: Vec<OsmoCoin>) -> Result<(), ContractError> {
        todo!()
    }

    pub fn into_coins(self) -> Vec<Coin> {
        self.0
    }
}

impl Default for Rewards {
    fn default() -> Self {
        Rewards(Vec::default())
    }
}

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
    rewards.update_rewards(response.collected_spread_rewards);

    // update the rewards map against all user's locked up vault shares
    distribute_rewards(deps, rewards)?;
    todo!()
}

fn distribute_rewards(deps: DepsMut, rewards: Rewards) -> Result<(), ContractError> {
    let total_shares = LOCKED_TOTAL.load(deps.storage)?;
    // TODO take the strategist fee here
    

    // for each user with locked tokens, we distribute
    LOCKED_TOKENS
        .range(deps.storage, None, None, Order::Ascending)
        .try_for_each(|v| -> Result<(), ContractError> {
            let (address, user_shares) = v?;
            // calculate the amount of each asset the user should get in rewards
            // we need to always round down here, so we never expect more rewards than we have
            let user_rewards = rewards.percentage(total_shares, user_shares);

            // upsert the amount into the rewards map
            USER_REWARDS.update(deps.storage, address, |old| -> Result<Rewards, ContractError> {
                if let Some(mut old_user_rewards) = old {
                    user_rewards.update_rewards(old_user_rewards)?;
                    Ok(user_rewards)
                } else {
                    Ok(user_rewards)
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
