use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum StrategyError {
    #[error("Adaptor already exists")]
    AdaptorAlreadyExists {},

    #[error("Adaptor not found with unique_id: {unique_id}")]
    AdaptorNotFound { unique_id: String }
}
