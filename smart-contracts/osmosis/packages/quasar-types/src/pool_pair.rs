use cosmwasm_std::Coin;

#[derive(Debug)]
pub struct PoolPair<S, T> {
    pub base: S,
    pub quote: T,
}

impl<S, T> PoolPair<S, T> {
    pub fn new(base: S, quote: T) -> Self {
        Self { base, quote }
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
    use cosmwasm_std::coin;

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
}
