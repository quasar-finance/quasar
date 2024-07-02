use crate::msg::LstAdapterExecuteMsgFns;
use crate::tests::ibc_setup::{create_app, LST_DENOM, OSMOSIS, STARGAZE};
use crate::LstAdapterError;
use abstract_interface::AbstractAccount;
use cosmwasm_std::{coins, Uint128};
use cw_orch::{anyhow, prelude::*};
use cw_orch_interchain::prelude::*;
use quasar_types::error::FundsError;

#[test]
fn test_if_not_vault_then_unbond_fails() -> anyhow::Result<()> {
    let app = create_app(vec![], Some("other".to_string()))?.app;

    let result = app.unbond(&[]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>().unwrap(),
        LstAdapterError::Owner(mars_owner::OwnerError::NotOwner {})
    );
    Ok(())
}

#[test]
fn test_if_missing_funds_then_unbond_fails() -> anyhow::Result<()> {
    let app = create_app(vec![], None)?.app;

    let result = app.unbond(&[]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>().unwrap(),
        LstAdapterError::Funds(FundsError::InvalidAssets(1))
    );
    Ok(())
}

#[test]
fn test_if_wrong_denom_then_unbond_fails() -> anyhow::Result<()> {
    let funds = coins(123, "wrong");
    let app = create_app(funds.clone(), None)?.app;

    let result = app.unbond(&funds);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>().unwrap(),
        LstAdapterError::Funds(FundsError::WrongDenom(LST_DENOM.to_string()))
    );
    Ok(())
}

#[test]
fn test_unbond_sends_ibc_message() -> anyhow::Result<()> {
    let funds = coins(123, LST_DENOM);
    let env = create_app(funds.clone(), None)?;
    let app = env.app;

    let ibc_action_result = app.unbond(&funds).unwrap();
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;

    let remote_account = AbstractAccount::new(&env.abstr_remote, env.remote_account_id.clone());
    let remote_denom: &str = &format!("ibc/channel-0/{}", LST_DENOM);
    let remote_balance = env
        .mock
        .chain(STARGAZE)?
        .query_balance(&remote_account.proxy.address()?, remote_denom)?;
    assert_eq!(Uint128::from(123u32), remote_balance);
    Ok(())
}
