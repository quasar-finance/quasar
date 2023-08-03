use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, Addr, Binary, Coin, Decimal, Deps, DepsMut, Env, Order, Response, SubMsg, Uint128,
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
#[cw_serde]
pub struct Rewards(Vec<Coin>);

impl Rewards {
    pub fn new() -> Rewards {
        Rewards::default()
    }

    /// calculates the percentage that the user should have
    pub fn percentage(&self, numerator: Uint128, denominator: Uint128) -> Rewards {
        // let percentage = Decimal::from_ratio(user_shares, total_shares);
        Rewards(
            self.0
                .iter()
                .map(|c| {
                    coin(
                        c.amount.multiply_ratio(numerator, denominator).u128(),
                        c.denom.clone(),
                    )
                })
                .collect(),
        )
    }

    /// merge any values already in Rewards and append any others
    pub fn update_rewards(&mut self, rewards: Vec<OsmoCoin>) -> ContractResult<()> {
        let parsed_rewards: ContractResult<Vec<Coin>> = rewards
            .into_iter()
            .map(|c| Ok(coin(c.amount.parse()?, c.denom)))
            .collect();

        // Append and merge to
        self.merge(parsed_rewards?)?;
        Ok(())
    }

    /// add rewards to self and mutate self
    pub fn add(mut self, rewards: Rewards) -> ContractResult<Self> {
        self.merge(rewards.into_coins())?;
        Ok(self)
    }

    fn merge(&mut self, coins: Vec<Coin>) -> ContractResult<()> {
        for c in coins {
            let same = self.0.iter_mut().find(|c2| c.denom == c2.denom);
            if let Some(c2) = same {
                c2.amount = c.amount + c2.amount
            } else {
                self.0.push(c)
            }
        }
        Ok(())
    }

    /// substract a percentage from self, mutate self and return the subtracted rewards
    pub fn sub_percentage(
        &mut self,
        numerator: Uint128,
        denominator: Uint128,
    ) -> ContractResult<Rewards> {
        let to_sub = self.percentage(numerator, denominator);

        // actually subtract the funds
        self.sub(&to_sub)?;
        Ok(to_sub)
    }

    /// subtract to_sub from self, ignores any coins in to_sub that don't exist in self and vice versa
    /// every item in self is expected to be greater or equal to the amount of the coin with the same denom
    /// in to_sub
    pub fn sub(&mut self, to_sub: &Rewards) -> ContractResult<()> {
        to_sub
            .0
            .iter()
            .try_for_each(|sub_coin| -> ContractResult<()> {
                let coin = self.0.iter_mut().find(|coin| sub_coin.denom == coin.denom);
                if let Some(c) = coin {
                    c.amount = c.amount.checked_sub(sub_coin.amount)?;
                }
                Ok(())
            })
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

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use super::*;

    #[test]
    fn sub_works() {
        let mut rewards = Rewards::new();
        rewards
            .update_rewards(vec![
                OsmoCoin {
                    denom: "uosmo".into(),
                    amount: "1000".into(),
                },
                OsmoCoin {
                    denom: "uatom".into(),
                    amount: "2000".into(),
                },
                OsmoCoin {
                    denom: "uqsr".into(),
                    amount: "3000".into(),
                },
            ])
            .unwrap();

        assert_eq!(rewards, Rewards(vec![coin(1000, "uosmo"), coin(2000, "uatom"), coin(3000, "uqsr")]))
    }
}
