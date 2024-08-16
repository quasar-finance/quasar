pub struct PoolPair<T> {
    pub base: T,
    pub quote: T,
}

impl<T> PoolPair<T> {
    pub fn new(base: T, quote: T) -> Self {
        Self { base, quote }
    }
}
