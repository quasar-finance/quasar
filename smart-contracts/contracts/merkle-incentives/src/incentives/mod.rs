use core::fmt;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, OverflowError, Uint128};

use crate::ContractError;

pub mod execute;
mod helpers;
pub mod query;

#[cw_serde]
pub struct CoinVec(Vec<Coin>);

impl Default for CoinVec {
    fn default() -> Self {
        Self::new()
    }
}

impl CoinVec {
    pub fn new() -> Self {
        Self(vec![])
    }

    // A getter method to access the coins
    pub fn coins(&self) -> &Vec<Coin> {
        &self.0
    }

    // If you need to mutate the coins, you can also have a getter for mutable reference
    pub fn coins_mut(&mut self) -> &mut Vec<Coin> {
        &mut self.0
    }

    pub fn sort(&mut self) {
        self.0.sort_by(|a, b| a.denom.cmp(&b.denom));
    }

    pub fn into_bank_sends(
        &self,
        recipient: &str,
    ) -> Result<Vec<cosmwasm_std::BankMsg>, ContractError> {
        // multiple bank sends so that insufficient funds doesnt fail full tx
        Ok(self
            .0
            .iter()
            .filter(|c| c.amount.gt(&Uint128::zero()))
            .map(|coin| cosmwasm_std::BankMsg::Send {
                to_address: recipient.to_string(),
                amount: vec![coin.clone()],
            })
            .collect())
    }

    pub fn checked_sub(&self, rhs: CoinVec) -> Result<Self, ContractError> {
        let mut left_coins = self.clone();
        for right_coin in &rhs.0 {
            if let Some(left_coin) = left_coins
                .0
                .iter_mut()
                .find(|c| c.denom == right_coin.denom)
            {
                left_coin.amount = left_coin.amount.checked_sub(right_coin.amount)?;
            } else {
                return Err(ContractError::OverflowError(OverflowError::new(
                    cosmwasm_std::OverflowOperation::Sub,
                    Coin {
                        denom: right_coin.denom.clone(),
                        amount: 0u128.into(),
                    },
                    right_coin,
                )));
            }
        }
        Ok(left_coins)
    }
}

impl From<Vec<Coin>> for CoinVec {
    fn from(value: Vec<Coin>) -> Self {
        Self(value)
    }
}

impl PartialOrd for CoinVec {
    // This function compares two CoinVec instances (`self` and `other`) based on their coins.
    // It constructs HashMaps mapping denominations to amounts for both instances.
    // Then, it iterates over the HashMap of `self` and checks against the HashMap of `other`.
    // - If a coin in `self` has a corresponding coin in `other`, it compares their amounts.
    //   - If the amount of the coin in `self` is less than the amount of the corresponding coin in `other`,
    //     it sets `self_less` to true.
    //   - If the amount of the coin in `self` is greater than the amount of the corresponding coin in `other`,
    //     it sets `self_greater` to true.
    // - If a denomination exists in `other` but not in `self` and the amount is greater than zero, it sets `self_less` to true.
    // Finally, based on the flags `self_less` and `self_greater`, it returns the ordering:
    // - If `self_less` is true and `self_greater` is false, it returns `Less`.
    // - If `self_less` is false and `self_greater` is true, it returns `Greater`.
    // - If both flags are false, it returns `Equal`.
    // - If both flags are true, it returns `None`, indicating incomparability
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_map: std::collections::HashMap<_, _> = self
            .0
            .iter()
            .map(|coin| (&coin.denom, &coin.amount))
            .collect();
        let other_map: std::collections::HashMap<_, _> = other
            .0
            .iter()
            .map(|coin| (&coin.denom, &coin.amount))
            .collect();

        let mut self_less = false;
        let mut self_greater = false;

        for (denom, amount) in &self_map {
            match other_map.get(denom) {
                Some(&other_amount) => {
                    if amount < &other_amount {
                        self_less = true;
                    } else if amount > &other_amount {
                        self_greater = true;
                    }
                }
                None => self_greater = true,
            }
        }

        for (denom, amount) in &other_map {
            if self_map.get(denom).is_none() && *amount > &Uint128::zero() {
                self_less = true;
            }
        }

        match (self_less, self_greater) {
            (true, false) => Some(std::cmp::Ordering::Less),
            (false, true) => Some(std::cmp::Ordering::Greater),
            (false, false) => Some(std::cmp::Ordering::Equal),
            (true, true) => None, // Incomparable if both are true.
        }
    }
}

// Implement Display for CoinVec
impl fmt::Display for CoinVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut coins = self.0.clone();
        coins.sort_by(|a, b| a.denom.cmp(&b.denom));

        let sorted_coins: Vec<String> = coins
            .iter()
            .map(|coin| format!("{}{}", coin.amount, coin.denom))
            .collect();
        write!(f, "{}", sorted_coins.join(""))
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Uint128;

    use super::*;

    #[test]
    fn test_partial_order_failing() {
        let coin_vec = CoinVec(vec![Coin {
            denom: "uusd".to_string(),
            amount: Uint128::from(50u128),
        }]);

        let coin_vec2 = CoinVec(vec![Coin {
            denom: "uluna".to_string(),
            amount: Uint128::from(100u128),
        }]);

        assert_eq!(false, coin_vec2.le(&coin_vec));
        assert_eq!(false, coin_vec2.ge(&coin_vec));

        let coin_vec = CoinVec(vec![Coin {
            denom: "uusd".to_string(),
            amount: Uint128::from(100u128),
        }]);

        let coin_vec2 = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(150u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(50u128),
            },
        ]);

        assert!(coin_vec.le(&coin_vec2));

        let coin_vec = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(50u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(50u128),
            },
        ]);

        let coin_vec2 = CoinVec(vec![Coin {
            denom: "uluna".to_string(),
            amount: Uint128::from(100u128),
        }]);

        // coin vec should not be gt or lt coin vec 2 as this case should not pass through
        assert_eq!(false, coin_vec.lt(&coin_vec2));
        assert_eq!(false, coin_vec.gt(&coin_vec2));
    }

    #[test]
    fn test_partial_order() {
        let coin_vec = CoinVec(vec![Coin {
            denom: "uusd".to_string(),
            amount: Uint128::from(100u128),
        }]);

        let coin_vec2 = CoinVec(vec![Coin {
            denom: "uusd".to_string(),
            amount: Uint128::from(150u128),
        }]);

        assert!(coin_vec.le(&coin_vec2));
    }

    #[test]
    fn test_sort() {
        let mut coin_vec = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(100u128),
            },
        ]);

        coin_vec.sort();

        assert_eq!(
            coin_vec,
            CoinVec(vec![
                Coin {
                    denom: "uluna".to_string(),
                    amount: Uint128::from(100u128),
                },
                Coin {
                    denom: "uusd".to_string(),
                    amount: Uint128::from(100u128),
                },
            ])
        );
    }

    #[test]
    fn test_coin_vec_checked_sub() {
        let coin_vec = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(100u128),
            },
        ]);

        let coin_vec2 = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(50u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(50u128),
            },
        ]);

        let result = coin_vec.checked_sub(coin_vec2).unwrap();
        assert_eq!(
            result,
            CoinVec(vec![
                Coin {
                    denom: "uusd".to_string(),
                    amount: Uint128::from(50u128),
                },
                Coin {
                    denom: "uluna".to_string(),
                    amount: Uint128::from(50u128),
                },
            ])
        );
    }

    #[test]
    fn test_coin_vec_checked_sub_error() {
        let coin_vec = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(100u128),
            },
        ]);

        let coin_vec2 = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(150u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(50u128),
            },
        ]);

        let result = coin_vec.checked_sub(coin_vec2);
        assert!(result.is_err());
    }

    #[test]
    fn test_coin_vec_ordering() {
        let coin_vec = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(100u128),
            },
        ]);

        let coin_vec2 = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(50u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(50u128),
            },
        ]);

        assert_eq!(
            coin_vec.partial_cmp(&coin_vec2),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(
            coin_vec2.partial_cmp(&coin_vec),
            Some(std::cmp::Ordering::Less)
        );

        let coin_vec3 = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(25u128),
            },
        ]);

        assert_eq!(
            coin_vec.partial_cmp(&coin_vec3),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(
            coin_vec3.partial_cmp(&coin_vec),
            Some(std::cmp::Ordering::Less)
        );

        // test with different length of coins
        let coin_vec4 = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "uapple".to_string(),
                amount: Uint128::from(25u128),
            },
        ]);

        assert_eq!(
            coin_vec4.partial_cmp(&coin_vec),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(
            coin_vec.partial_cmp(&coin_vec4),
            Some(std::cmp::Ordering::Less)
        );

        let coin_vec5 = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(120u128),
            },
            Coin {
                denom: "uapple".to_string(),
                amount: Uint128::from(25u128),
            },
        ]);

        assert_eq!(
            coin_vec.partial_cmp(&coin_vec5),
            Some(std::cmp::Ordering::Less) // technically only one is less, but we are stricter with greater for our use-case
        );
        assert_eq!(
            coin_vec5.partial_cmp(&coin_vec),
            Some(std::cmp::Ordering::Greater)
        );

        let coin_vec6 = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(50u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(150u128),
            },
        ]);

        assert_eq!(coin_vec.partial_cmp(&coin_vec6), None);
        assert_eq!(
            coin_vec6.partial_cmp(&coin_vec),
            None // in this case both greater because we have to guard against attacks (It is not safe to switch the greter than check in helpers.rs)
        );
    }

    #[test]
    fn test_coin_vec_display() {
        let coin_vec = CoinVec(vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(100u128),
            },
        ]);

        assert_eq!(coin_vec.to_string(), "100uluna100uusd");

        let coin_vec2 = CoinVec(vec![
            Coin {
                denom: "uapple".to_string(),
                amount: Uint128::from(1000u128),
            },
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "uaaaapple".to_string(),
                amount: Uint128::from(100u128),
            },
        ]);

        assert_eq!(
            coin_vec2.to_string(),
            "100uaaaapple1000uapple100uluna100uusd"
        );
    }
}
