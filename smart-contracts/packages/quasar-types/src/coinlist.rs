use std::ops::Sub;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Attribute, Coin, Decimal, Fraction, OverflowError, Uint128};

#[cw_serde]
#[derive(Default)]
/// CoinList is a wrapper around Vec<Coin>,
pub struct CoinList(Vec<Coin>);

impl CoinList {
    pub fn new(coins: Vec<Coin>) -> CoinList {
        CoinList(coins)
    }

    /// calculates the ratio of the current coins
    /// [1000uosmo, 3000uatom] * 50% = [500uosmo, 1500uatom]
    pub fn mul_ratio(&self, ratio: Decimal) -> CoinList {
        CoinList(
            self.0
                .iter()
                .map(|c| {
                    coin(
                        c.amount
                            .multiply_ratio(ratio.numerator(), ratio.denominator())
                            .u128(),
                        c.denom.clone(),
                    )
                })
                .collect(),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, coin: Coin) -> () {
        self.0.push(coin)
    }

    // TODO rename
    /// merge any values already in Rewards and append any others
    pub fn append(&mut self, coins: Vec<Coin>) -> Result<(), OverflowError> {
        // Append and merge to
        self.merge(coins)?;
        Ok(())
    }

    /// add another coinlist to self in place
    pub fn add(mut self, other: CoinList) -> Result<Self, OverflowError> {
        self.merge(other.coins())?;
        Ok(self)
    }

    pub fn merge(&mut self, coins: Vec<Coin>) -> Result<(), OverflowError> {
        for c in coins {
            let same = self.0.iter_mut().find(|c2| c.denom == c2.denom);
            if let Some(c2) = same {
                c2.amount = c.amount.checked_add(c2.amount)?
            } else {
                self.0.push(c)
            }
        }
        Ok(())
    }

    /// substract a percentage from self, mutate self and return the subtracted coins
    /// For example
    /// [1000uqsr, 20000uosmo] - 25%
    /// mutates self to [750uqsr, 1500uosmo]
    /// and returns [250uqsr, 500uosmo]
    pub fn sub_ratio(&mut self, ratio: Decimal) -> Result<CoinList, OverflowError> {
        let to_sub = self.mul_ratio(ratio);

        // actually subtract the funds
        self.checked_mut_sub(&to_sub)?;
        Ok(to_sub)
    }

    /// subtract to_sub from self, ignores any coins in to_sub that don't exist in self and vice versa
    /// every item in self is expected to be greater or equal to the amount of the coin with the same denom
    /// in to_sub
    /// thus [150uosmo, 100uatom] - [100uosmo, 200uqsr] = [50uosmo, 100uatom]
    pub fn checked_mut_sub(&mut self, to_sub: &CoinList) -> Result<(), OverflowError> {
        to_sub
            .0
            .iter()
            .try_for_each(|sub_coin| -> Result<(), OverflowError> {
                let coin = self.0.iter_mut().find(|coin| sub_coin.denom == coin.denom);
                if let Some(c) = coin {
                    c.amount = c.amount.checked_sub(sub_coin.amount)?;
                }
                Ok(())
            })
    }

    /// subtract to_sub from self, ignores any coins in to_sub that don't exist in self and vice versa
    /// every item in self is expected to be greater or equal to the amount of the coin with the same denom
    /// in to_sub
    /// thus [150uosmo, 100uatom] - [100uosmo, 200uqsr] = [50uosmo, 100uatom]
    pub fn checked_sub(&self, rhs: &CoinList) -> Result<CoinList, OverflowError> {
        let result: Result<Vec<Coin>, OverflowError> = self
            .0
            .iter()
            .map(|c| {
                let coin = rhs.0.iter().find(|rc| c.denom == rc.denom);
                // TODO these to clones are not the prettiest, see if we can make thos nicer
                if let Some(rc) = coin {
                    Ok(Coin {
                        denom: c.denom.clone(),
                        amount: c.amount.checked_sub(rc.amount)?,
                    })
                } else {
                    Ok(c.clone())
                }
            })
            .collect();

        result.map(CoinList::new)
    }

    pub fn into_attributes(self) -> Vec<Attribute> {
        self.0
            .iter()
            .map(|c| Attribute {
                key: c.denom.clone(),
                value: c.amount.to_string(),
            })
            .collect()
    }

    pub fn coins(&self) -> Vec<Coin> {
        sort_tokens(self.0.clone())
    }

    pub fn find_coin(&self, denom: String) -> Coin {
        self.0
            .clone()
            .into_iter()
            .find(|c| c.denom == denom)
            .unwrap_or(Coin {
                denom,
                amount: 0u128.into(),
            })
    }
}

impl From<Vec<Coin>> for CoinList {
    fn from(value: Vec<Coin>) -> Self {
        Self(value)
    }
}

impl ToString for CoinList {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl Sub for CoinList {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(&rhs).unwrap()
    }
}

fn sort_tokens(tokens: Vec<Coin>) -> Vec<Coin> {
    let mut sorted_tokens = tokens;
    sorted_tokens.sort_by(|a, b| a.denom.cmp(&b.denom));
    sorted_tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_tokens() {
        let tokens = vec![
            coin(100, "uatom"),
            coin(200, "uosmo"),
            coin(300, "uqsr"),
            coin(400, "ueth"),
        ];

        let expected = vec![
            coin(100, "uatom"),
            coin(400, "ueth"),
            coin(200, "uosmo"),
            coin(300, "uqsr"),
        ];

        let sorted_tokens = sort_tokens(tokens);

        assert_eq!(sorted_tokens, expected);
    }
}

#[test]
fn checked_mut_sub_works() {
    let mut coins = CoinList::default();
    coins
        .append(vec![
            coin(1000, "uosmo"),
            coin(2000, "uatom"),
            coin(3000, "uqsr"),
        ])
        .unwrap();

    assert_eq!(
        coins,
        CoinList(vec![
            coin(1000, "uosmo"),
            coin(2000, "uatom"),
            coin(3000, "uqsr")
        ])
    );

    coins
        .checked_mut_sub(&CoinList::from(vec![coin(1500, "uqsr")]))
        .unwrap();

    assert_eq!(
        coins,
        CoinList(vec![
            coin(1000, "uosmo"),
            coin(2000, "uatom"),
            coin(1500, "uqsr")
        ])
    );

    coins
        .checked_mut_sub(&CoinList::from(vec![coin(2000, "uqsr")]))
        .unwrap_err();

    coins
        .checked_mut_sub(&CoinList::from(vec![coin(999, "uqsr"), coin(999, "uosmo")]))
        .unwrap();

    assert_eq!(
        coins,
        CoinList(vec![
            coin(1, "uosmo"),
            coin(2000, "uatom"),
            coin(501, "uqsr")
        ])
    );
}

#[test]
fn percentage_works() {
    let mut rewards = CoinList::default();
    rewards
        .append(vec![
            coin(1000, "uosmo"),
            coin(2000, "uatom"),
            coin(3000, "uqsr"),
        ])
        .unwrap();

    let ratio = rewards.mul_ratio(Decimal::from_ratio(Uint128::new(10), Uint128::new(100)));
    assert_eq!(
        ratio,
        CoinList(vec![
            coin(100, "uosmo"),
            coin(200, "uatom"),
            coin(300, "uqsr")
        ])
    )
}

#[test]
fn sub_percentage_works() {
    let mut rewards = CoinList::default();
    rewards
        .append(vec![
            coin(1000, "uosmo"),
            coin(2000, "uatom"),
            coin(3000, "uqsr"),
        ])
        .unwrap();

    let ratio = rewards
        .sub_ratio(Decimal::from_ratio(Uint128::new(10), Uint128::new(100)))
        .unwrap();
    assert_eq!(
        ratio,
        CoinList(vec![
            coin(100, "uosmo"),
            coin(200, "uatom"),
            coin(300, "uqsr")
        ])
    );
    assert_eq!(
        rewards,
        CoinList(vec![
            coin(900, "uosmo"),
            coin(1800, "uatom"),
            coin(2700, "uqsr")
        ])
    )
}

#[test]
fn merge_works() {}

#[test]
fn add_works() {
    let mut rewards = CoinList::default();
    rewards
        .append(vec![
            coin(1000, "uosmo"),
            coin(2000, "uatom"),
            coin(3000, "uqsr"),
        ])
        .unwrap();
    rewards = rewards
        .add(CoinList::from(vec![
            coin(2000, "uosmo"),
            coin(2000, "uatom"),
            coin(6000, "uqsr"),
            coin(1234, "umars"),
        ]))
        .unwrap();
    assert_eq!(
        rewards,
        CoinList::from(vec![
            coin(3000, "uosmo"),
            coin(4000, "uatom"),
            coin(9000, "uqsr"),
            coin(1234, "umars")
        ])
    )
}

#[test]
fn update_rewards_works() {
    let mut rewards = CoinList::default();
    rewards
        .append(vec![
            coin(1000, "uosmo"),
            coin(2000, "uatom"),
            coin(3000, "uqsr"),
        ])
        .unwrap();

    rewards
        .append(vec![
            coin(1000, "uosmo"),
            coin(1234, "umars"),
            coin(3000, "uqsr"),
        ])
        .unwrap();

    assert_eq!(
        rewards,
        CoinList::from(vec![
            coin(2000, "uosmo"),
            coin(2000, "uatom"),
            coin(6000, "uqsr"),
            coin(1234, "umars")
        ])
    );
}
