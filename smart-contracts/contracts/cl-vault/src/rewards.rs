use std::ops::Add;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{QuerierWrapper, DepsMut, Uint128, Deps, Env, Response, SubMsg, Coin, Reply, Binary, from_binary, coin};

use crate::{ContractError, state::{POSITION, CURRENT_REWARDS}, reply::Replies};
use osmosis_std::types::{
    cosmos::{
        bank::v1beta1::BankQuerier,
        base::v1beta1::Coin as OsmoCoin
    },
    osmosis::concentratedliquidity::v1beta1::{MsgCollectIncentives, MsgCollectSpreadRewards, MsgCollectIncentivesResponse, MsgCollectSpreadRewardsResponse}
};

#[cw_serde]
pub struct Rewards(Vec<Coin>);

impl Rewards {
    pub fn new() -> Rewards {
        Rewards::default()
    }

    pub fn update_rewards(&mut self, rewards: Vec<OsmoCoin>) -> Result<(), ContractError> {
        let v: Result<Vec<Coin>, ContractError> = self.0.into_iter().map(|c| {
            let coin = rewards.iter().find(|c2| c.denom == c2.denom);
            if let Some(c3) = coin {
                c.amount + Uint128::new(c3.amount.parse()?);
                Ok(c)
            } else {
                Ok(c)
            }
        }).collect();
        self.0 = v?;
        Ok(())
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
    Ok(Response::new().add_submessage(SubMsg::reply_on_success(msg, Replies::CollectIncentives as u64)))
}

pub fn handle_collect_incentives_reply(deps: DepsMut, env: Env, data: Binary) -> Result<Response, ContractError> {
    // save the response from the collect incentives
    let response: MsgCollectIncentivesResponse = data.try_into()?;
    CURRENT_REWARDS.update(deps.storage, |mut rewards| -> Result<Rewards, ContractError> {
        rewards.update_rewards(response.collected_incentives);
        Ok(rewards)
    })?;

    // collect the spread rewards
    let msg = collect_spread_rewards(deps.as_ref(), env)?;
    Ok(Response::new().add_submessage(SubMsg::reply_on_success(msg, Replies::CollectSpreadRewards as u64)))
}

pub fn handle_collect_spread_rewards_reply(deps: DepsMut, env: Env, data: Binary) -> Result<Response, ContractError> {
    // after we have collected both the spread rewards and the incentives, we can distribute them over the share holders
    // we don't need to save the rewards here again, just pass it to update rewards
    let response: MsgCollectSpreadRewardsResponse  = data.try_into()?;
    let mut rewards = CURRENT_REWARDS.load(deps.storage)?;
    rewards.update_rewards(response.collected_spread_rewards);

    // update the rewards map against all user's locked up vault shares
    
    todo!()
}

fn update_rewards_map(deps: DepsMut, rewards: Rewards) -> Result<(), ContractError> {


    todo!()
}

fn collect_incentives(deps: Deps, env: Env) -> Result<MsgCollectIncentives, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectIncentives{ position_ids: vec![position.position_id], sender: env.contract.address.into() })
}

fn collect_spread_rewards(deps: Deps, env: Env) -> Result<MsgCollectSpreadRewards, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectSpreadRewards { position_ids: vec![position.position_id], sender: env.contract.address.into() })
}