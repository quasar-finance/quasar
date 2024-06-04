use thiserror::Error;

#[derive(Error, Debug)]
pub enum StrategyError {
    #[error("Adaptor already exists")]
    AdaptorAlreadyExists {},
}
