use cosmwasm_std::{coin, Coin, OverflowError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PoolPairError {
    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Inconsistent denoms, received: {given}, expected: {expected}")]
    InconsistentDenoms { given: String, expected: String },
}

fn assert_denom(expected: &str, given: &str) -> Result<(), PoolPairError> {
    if expected != given {
        return Err(PoolPairError::InconsistentDenoms {
            given: given.to_string(),
            expected: expected.to_string(),
        });
    }
    Ok(())
}

#[cosmwasm_schema::cw_serde]
pub struct PoolPair<S, T> {
    pub base: S,
    pub quote: T,
}

impl<S, T> PoolPair<S, T> {
    pub fn new(base: S, quote: T) -> Self {
        Self { base, quote }
    }
}

impl PoolPair<Coin, Coin> {
    pub fn checked_sub(
        &self,
        other: &PoolPair<Coin, Coin>,
    ) -> Result<PoolPair<Coin, Coin>, PoolPairError> {
        assert_denom(&self.base.denom, &other.base.denom)?;
        assert_denom(&self.quote.denom, &other.quote.denom)?;

        Ok(PoolPair::new(
            coin(
                self.base.amount.checked_sub(other.base.amount)?.into(),
                self.base.denom.clone(),
            ),
            coin(
                self.quote.amount.checked_sub(other.quote.amount)?.into(),
                self.quote.denom.clone(),
            ),
        ))
    }
}

pub trait Contains<T> {
    fn contains(&self, value: T) -> bool;
}

impl Contains<&str> for PoolPair<String, String> {
    fn contains(&self, value: &str) -> bool {
        value == self.base || value == self.quote
    }
}

impl Contains<&str> for PoolPair<Coin, Coin> {
    fn contains(&self, value: &str) -> bool {
        value == self.base.denom || value == self.quote.denom
    }
}

#[allow(clippy::unnecessary_to_owned)]
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{coin, OverflowOperation::Sub};

    #[test]
    fn test_string_pair() {
        let base = "base".to_string();
        let quote = "quote".to_string();
        let pair = PoolPair::new(base.clone(), quote.clone());
        assert_eq!(base, pair.base);
        assert_eq!(quote, pair.quote);

        assert!(pair.contains(&base));
        assert!(pair.contains(&quote));
        assert!(pair.contains(base.as_str()));
        assert!(!pair.contains(&"other".to_string()));
        assert!(!pair.contains("other"));
    }

    #[test]
    fn test_coin_pair() {
        let base = coin(123u128, "base");
        let quote = coin(456u128, "quote");
        let pair = PoolPair::new(base.clone(), quote.clone());
        assert_eq!(base, pair.base);
        assert_eq!(quote, pair.quote);

        assert!(pair.contains(&base.denom));
        assert!(pair.contains(&quote.denom));
        assert!(pair.contains(base.denom.as_str()));
        assert!(!pair.contains(&"other".to_string()));
        assert!(!pair.contains("other"));
    }

    #[test]
    fn test_coin_pair_sub() {
        let base = coin(123u128, "base");
        let quote = coin(456u128, "quote");
        let pair = PoolPair::new(base.clone(), quote.clone());
        let other_base = coin(234u128, "base");
        let other_quote = coin(678u128, "quote");
        let other = PoolPair::new(other_base.clone(), other_quote.clone());

        let result = other.checked_sub(&pair).unwrap();
        assert_eq!(result.base.amount.u128(), 111);
        assert_eq!(result.quote.amount.u128(), 222);

        let err = pair.checked_sub(&other).unwrap_err();
        assert_eq!(
            err,
            PoolPairError::Overflow(OverflowError {
                operation: Sub,
                operand1: base.amount.to_string(),
                operand2: other_base.amount.to_string(),
            })
        );
    }

    #[test]
    fn test_coin_pair_sub_denom_check() {
        let base = coin(123u128, "base");
        let quote = coin(456u128, "quote");
        let pair = PoolPair::new(base.clone(), quote.clone());
        let invalid_quote = coin(345u128, "invalid_quote");
        let invalid = PoolPair::new(base.clone(), invalid_quote.clone());
        let err = pair.checked_sub(&invalid).unwrap_err();
        assert_eq!(
            err,
            PoolPairError::InconsistentDenoms {
                given: invalid_quote.denom,
                expected: quote.denom.clone(),
            }
        );

        let invalid_base = coin(12u128, "invalid_base");
        let invalid = PoolPair::new(invalid_base.clone(), quote.clone());
        let err = pair.checked_sub(&invalid).unwrap_err();
        assert_eq!(
            err,
            PoolPairError::InconsistentDenoms {
                given: invalid_base.denom,
                expected: base.denom,
            }
        );
    }
}
