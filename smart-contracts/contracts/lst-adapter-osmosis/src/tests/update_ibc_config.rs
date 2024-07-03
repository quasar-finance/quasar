use crate::msg::{LstAdapterExecuteMsgFns, LstAdapterQueryMsgFns};
use crate::state::IbcConfig;
use crate::tests::ibc_setup::create_app;
use cw_orch::anyhow;

#[test]
fn test_update_ibc_config() -> anyhow::Result<()> {
    let app = create_app(vec![], None)?.app;

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
    // let result = app.claim();
    // assert!(result.is_err());
    // assert_eq!(
    //     result.unwrap_err().downcast::<LstAdapterError>().unwrap(),
    //     LstAdapterError::Owner(mars_owner::OwnerError::NotOwner {})
    // );
    Ok(())
}
