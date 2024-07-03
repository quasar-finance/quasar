use crate::msg::{LstAdapterExecuteMsgFns, LstAdapterQueryMsgFns};
use crate::state::IbcConfig;
use crate::tests::ibc_setup::{create_app, LST_DENOM, OSMOSIS};
use crate::LstAdapterError;
use cw_orch::anyhow;
use cw_orch_interchain::InterchainEnv;

#[test]
fn test_only_owner_can_update_ibc_config() -> anyhow::Result<()> {
    let app = create_app(vec![], None, Some("other_owner".to_string()))?.app;

    let revision = Some(1u64);
    let block_offset = Some(2u64);
    let timeout_secs = Some(3u64);
    let channel = "channel-123".to_string();

    let result = app.update_ibc_config(channel.clone(), block_offset, revision, timeout_secs);

    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>().unwrap(),
        LstAdapterError::Owner(mars_owner::OwnerError::NotOwner {})
    );
    Ok(())
}

#[test]
fn test_update_ibc_config() -> anyhow::Result<()> {
    let app = create_app(vec![], None, None)?.app;

    let result = app.ibc_config()?;
    assert_eq!(result, IbcConfig::default());

    let revision = Some(1u64);
    let block_offset = Some(2u64);
    let timeout_secs = Some(3u64);
    let channel = "channel-123".to_string();

    assert!(app
        .update_ibc_config(channel.clone(), block_offset, revision, timeout_secs)
        .is_ok());

    let result = app.ibc_config()?;
    assert_eq!(
        result,
        IbcConfig {
            revision,
            block_offset,
            timeout_secs,
            channel,
        }
    );
    Ok(())
}

#[test]
fn test_only_owner_can_update() -> anyhow::Result<()> {
    let app = create_app(vec![], None, Some("other_owner".to_string()))?.app;

    let result = app.update(None, None);

    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>().unwrap(),
        LstAdapterError::Owner(mars_owner::OwnerError::NotOwner {})
    );
    Ok(())
}

#[test]
fn test_update() -> anyhow::Result<()> {
    let env = create_app(vec![], None, None)?;
    let app = env.app;

    assert_eq!(app.lst_denom()?, LST_DENOM);
    let other_denom = "other_denom".to_string();
    assert!(app.update(Some(other_denom.clone()), None).is_ok());
    assert_eq!(app.lst_denom()?, other_denom);

    let new_vault = env
        .mock
        .chain(OSMOSIS)?
        .addr_make("other_vault")
        .to_string();
    assert!(app.update(None, Some(new_vault.clone())).is_ok());
    assert_eq!(app.vault()?, new_vault);

    let other_denom = "even_another_denom".to_string();
    let new_vault = env
        .mock
        .chain(OSMOSIS)?
        .addr_make("even_another_vault")
        .to_string();
    assert!(app
        .update(Some(other_denom.clone()), Some(new_vault.clone()))
        .is_ok());
    assert_eq!(app.lst_denom()?, other_denom);
    assert_eq!(app.vault()?, new_vault);
    Ok(())
}
