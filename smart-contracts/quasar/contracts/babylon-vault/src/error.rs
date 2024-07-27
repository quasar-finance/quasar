use crate::state::{LSTS, VAULT_DENOM};
use cosmwasm_std::{CheckedMultiplyFractionError, Coin, Order, OverflowError, StdError, Storage};
use mars_owner::OwnerError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum VaultError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Owner(#[from] OwnerError),

    #[error("{0}")]
    CheckedMultiply(#[from] CheckedMultiplyFractionError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{denom} not found")]
    DenomNotFound { denom: String },

    #[error("invalid funds")]
    InvalidFunds {},
}

fn assert_non_empty_funds(funds: &[Coin]) -> Result<(), VaultError> {
    if funds.len() != 1 {
        return Err(VaultError::InvalidFunds {});
    }

    Ok(())
}

pub fn assert_deposit_funds(storage: &dyn Storage, funds: &[Coin]) -> Result<(), VaultError> {
    assert_non_empty_funds(funds)?;

    let lsts: Result<Vec<String>, _> = LSTS.keys(storage, None, None, Order::Ascending).collect();
    if !lsts?.contains(&funds[0].denom) {
        return Err(VaultError::DenomNotFound {
            denom: funds[0].denom.clone(),
        });
    }

    Ok(())
}

pub fn assert_withdraw_funds(storage: &dyn Storage, funds: &[Coin]) -> Result<(), VaultError> {
    assert_non_empty_funds(funds)?;

    let vault_denom = VAULT_DENOM.load(storage)?;
    if vault_denom != funds[0].denom {
        return Err(VaultError::DenomNotFound {
            denom: funds[0].denom.clone(),
        });
    }

    Ok(())
}
