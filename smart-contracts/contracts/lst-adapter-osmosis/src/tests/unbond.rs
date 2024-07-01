use crate::msg::{LstAdapterExecuteMsgFns, LstAdapterInstantiateMsg};
use crate::{
    LstAdapterError, LstAdapterInterface, LST_ADAPTER_OSMOSIS_ID, LST_ADAPTER_OSMOSIS_NAMESPACE,
    LST_ADAPTER_OSMOSIS_VERSION,
};
use abstract_interface::{Abstract, AbstractAccount, AppDeployer, DeployStrategy, VCExecFns};
use abstract_std::objects::AccountId;
use abstract_std::objects::UncheckedChannelEntry;
use abstract_std::ICS20;
use cosmwasm_std::{coins, Uint128};
use cw_orch::mock::cw_multi_test::MockApiBech32;
use cw_orch::mock::MockBase;
use cw_orch::{anyhow, prelude::*};
use cw_orch_interchain::prelude::*;
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use quasar_types::error::FundsError;

use super::ibc_setup::{
    create_test_remote_account, ibc_abstract_setup, ibc_connect_polytone_and_abstract,
};

const LST_DENOM: &str = "lst_denom";
const STARGAZE: &str = "stargaze-1";
const OSMOSIS: &str = "osmosis-1";

struct TestEnv {
    pub app: LstAdapterInterface<MockBase<MockApiBech32>>,
    pub mock: MockBech32InterchainEnv,
    pub origin_account: AbstractAccount<MockBase<MockApiBech32>>,
    pub remote_account_id: AccountId,
    pub abstr_remote: Abstract<MockBase<MockApiBech32>>,
}

fn create_app(sender_balance: Vec<Coin>, vault: Option<String>) -> anyhow::Result<TestEnv> {
    let mock = MockBech32InterchainEnv::new(vec![(OSMOSIS, "osmosis"), (STARGAZE, "stargaze")]);

    let (abstr_origin, abstr_remote) = ibc_abstract_setup(&mock, OSMOSIS, STARGAZE)?;
    ibc_connect_polytone_and_abstract(&mock, STARGAZE, OSMOSIS)?;

    let (origin_account, remote_account_id) =
        create_test_remote_account(&abstr_origin, OSMOSIS, STARGAZE, &mock, None)?;
    let vault = if let Some(vault) = vault {
        mock.chain(OSMOSIS)?.addr_make(vault)
    } else {
        mock.chain(OSMOSIS)?.sender()
    };

    if !sender_balance.is_empty() {
        mock.chain(OSMOSIS)?
            .set_balance(&mock.chain(OSMOSIS)?.sender(), sender_balance)?;
    }

    let app = LstAdapterInterface::new(
        LST_ADAPTER_OSMOSIS_ID,
        abstr_origin.version_control.get_chain().clone(),
    );

    abstr_origin.version_control.claim_namespace(
        origin_account.id()?,
        LST_ADAPTER_OSMOSIS_NAMESPACE.to_owned(),
    )?;

    app.deploy(LST_ADAPTER_OSMOSIS_VERSION.parse()?, DeployStrategy::Try)?;

    origin_account.install_app(
        &app,
        &LstAdapterInstantiateMsg {
            lst_denom: LST_DENOM.to_string(),
            vault: vault.to_string(),
        },
        None,
    )?;

    let interchain_channel = mock.create_channel(
        OSMOSIS,
        STARGAZE,
        &PortId::transfer(),
        &PortId::transfer(),
        "ics20-1",
        None, // Unordered channel
    )?;

    abstract_interface::ExecuteMsgFns::update_channels(
        &abstr_origin.ans_host,
        vec![(
            UncheckedChannelEntry {
                connected_chain: "stargaze".to_string(),
                protocol: ICS20.to_string(),
            },
            interchain_channel
                .interchain_channel
                .get_chain(OSMOSIS)?
                .channel
                .unwrap()
                .to_string(),
        )],
        vec![],
    )?;

    abstract_interface::ExecuteMsgFns::update_channels(
        &abstr_remote.ans_host,
        vec![(
            UncheckedChannelEntry {
                connected_chain: "juno".to_string(),
                protocol: ICS20.to_string(),
            },
            interchain_channel
                .interchain_channel
                .get_chain(STARGAZE)?
                .channel
                .unwrap()
                .to_string(),
        )],
        vec![],
    )?;
    Ok(TestEnv {
        app,
        mock,
        origin_account,
        remote_account_id,
        abstr_remote,
    })
}

#[test]
fn test_if_not_owner_then_unbond_fails() -> anyhow::Result<()> {
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
