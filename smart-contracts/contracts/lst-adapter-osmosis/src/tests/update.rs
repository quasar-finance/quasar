use crate::msg::{LstAdapterExecuteMsgFns, LstAdapterQueryMsgFns};
use crate::state::{Denoms, IbcConfig};
use crate::tests::ibc_setup::{create_app, DENOM, LST_DENOM, OSMOSIS};
use crate::LstAdapterError;
use cw_orch::anyhow;
use cw_orch::contract::interface_traits::CallAs;
use cw_orch_interchain::InterchainEnv;

#[test]
fn test_only_owner_can_update_ibc_config() -> anyhow::Result<()> {
    let env = create_app(vec![], None)?;
    let mut app = env.app;

    let remote_chain = "stride".to_string();
    let revision = Some(1u64);
    let block_offset = Some(2u64);
    let timeout_secs = Some(3u64);
    let channel = "channel-123".to_string();

    let other_sender = env.mock.chain(OSMOSIS)?.addr_make("other_sender");
    app.set_sender(&other_sender);
    let result = app.update_ibc_config(
        remote_chain.clone(),
        channel.clone(),
        block_offset,
        revision,
        timeout_secs,
    );

    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>().unwrap(),
        LstAdapterError::Owner(mars_owner::OwnerError::NotOwner {})
    );
    Ok(())
}

#[test]
fn test_update_ibc_config() -> anyhow::Result<()> {
    let app = create_app(vec![], None)?.app;

    let result = app.ibc_config()?;
    assert_eq!(result, IbcConfig::default());

    let remote_chain = "stride".to_string();
    let revision = Some(1u64);
    let block_offset = Some(2u64);
    let timeout_secs = Some(3u64);
    let channel = "channel-123".to_string();

    assert!(app
        .update_ibc_config(
            channel.clone(),
            remote_chain.clone(),
            block_offset,
            revision,
            timeout_secs
        )
        .is_ok());

    let result = app.ibc_config()?;
    assert_eq!(
        result,
        IbcConfig {
            remote_chain,
            channel,
            revision,
            block_offset,
            timeout_secs,
        }
    );
    Ok(())
}

#[test]
fn test_only_owner_can_update() -> anyhow::Result<()> {
    let env = create_app(vec![], None)?;
    let mut app = env.app;

    let other_sender = env.mock.chain(OSMOSIS)?.addr_make("other_sender");
    app.set_sender(&other_sender);
    let result = app.update(None, None, None, None, None);

    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>().unwrap(),
        LstAdapterError::Owner(mars_owner::OwnerError::NotOwner {})
    );
    Ok(())
}

#[test]
fn test_update() -> anyhow::Result<()> {
    let env = create_app(vec![], None)?;
    let app = env.app;

    assert_eq!(app.denoms()?.lst, LST_DENOM);
    let other_denom = "other_denom".to_string();
    assert!(app
        .update(
            Some(Denoms {
                lst: other_denom.clone(),
                underlying: DENOM.to_string()
            }),
            None,
            None,
            None,
            None
        )
        .is_ok());
    assert_eq!(app.denoms()?.lst, other_denom);

    let new_vault = env
        .mock
        .chain(OSMOSIS)?
        .addr_make("other_vault")
        .to_string();
    assert!(app
        .update(None, None, None, None, Some(new_vault.clone()))
        .is_ok());
    assert_eq!(app.vault()?, new_vault);

    let other_denom = "even_another_denom".to_string();
    let new_vault = env
        .mock
        .chain(OSMOSIS)?
        .addr_make("even_another_vault")
        .to_string();
    assert!(app
        .update(
            Some(Denoms {
                lst: other_denom.clone(),
                underlying: DENOM.to_string()
            }),
            None,
            None,
            None,
            Some(new_vault.clone())
        )
        .is_ok());
    assert_eq!(app.denoms()?.lst, other_denom);
    assert_eq!(app.vault()?, new_vault);
    Ok(())
}
