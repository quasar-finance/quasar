use crate::msg::LstAdapterExecuteMsgFns;
use crate::tests::ibc_setup::create_app;
use crate::LstAdapterError;
use cw_orch::{anyhow, prelude::*};
use cw_orch_interchain::prelude::*;
use quasar_types::error::FundsError;

#[test]
fn test_if_not_vault_then_claim_fails() -> anyhow::Result<()> {
    let app = create_app(vec![], Some("other".to_string()))?.app;

    let result = app.claim();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>().unwrap(),
        LstAdapterError::Owner(mars_owner::OwnerError::NotOwner {})
    );
    Ok(())
}
