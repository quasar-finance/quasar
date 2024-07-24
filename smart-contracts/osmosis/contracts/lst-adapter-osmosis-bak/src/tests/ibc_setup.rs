use crate::tests::fake_stride_oracle::{FakeStrideOracle, FakeStrideOracleInstantiateMsg};
use abstract_cw_orch_polytone::Polytone;
use abstract_interface::{
    Abstract, AbstractAccount, AccountDetails, AppDeployer, DeployStrategy, ManagerExecFns,
    ManagerQueryFns, VCExecFns,
};
use abstract_polytone::handshake::POLYTONE_VERSION;
use abstract_std::ibc_client::{ExecuteMsgFns, QueryMsgFns};
use abstract_std::objects::UncheckedChannelEntry;
use abstract_std::objects::{account::AccountTrace, chain_name::ChainName, AccountId};
use abstract_std::ICS20;
use cosmwasm_std::Decimal;
use cw_orch::mock::cw_multi_test::MockApiBech32;
use cw_orch::mock::MockBase;
use cw_orch::{anyhow, prelude::*};
use cw_orch_interchain::{prelude::*, InterchainError};
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use quasar_types::denoms::LstDenom;
use std::str::FromStr;

use crate::msg::LstAdapterInstantiateMsg;
use crate::{
    LstAdapterInterface, LST_ADAPTER_OSMOSIS_ID, LST_ADAPTER_OSMOSIS_NAMESPACE,
    LST_ADAPTER_OSMOSIS_VERSION,
};

const TEST_ACCOUNT_NAME: &str = "account-test";
const TEST_ACCOUNT_DESCRIPTION: &str = "Description of an account";
const TEST_ACCOUNT_LINK: &str = "https://google.com";
pub const UNBOND_PERIOD: u64 = 1234u64;
pub const REDEMPTION_RATE_PERCENT: u64 = 123;

pub fn create_test_remote_account<Chain: IbcQueryHandler, IBC: InterchainEnv<Chain>>(
    abstr_origin: &Abstract<Chain>,
    origin_id: &str,
    remote_id: &str,
    interchain: &IBC,
    funds: Option<Vec<Coin>>,
) -> anyhow::Result<(AbstractAccount<Chain>, AccountId)> {
    let origin_name = ChainName::from_chain_id(origin_id).to_string();
    let remote_name = ChainName::from_chain_id(remote_id).to_string();

    // Create a local account for testing
    let account_name = TEST_ACCOUNT_NAME.to_string();
    let description = Some(TEST_ACCOUNT_DESCRIPTION.to_string());
    let link = Some(TEST_ACCOUNT_LINK.to_string());
    let origin_account = abstr_origin.account_factory.create_new_account(
        AccountDetails {
            name: account_name.clone(),
            description: description.clone(),
            link: link.clone(),
            base_asset: None,
            install_modules: vec![],
            namespace: None,
            account_id: None,
        },
        abstract_std::objects::gov_type::GovernanceDetails::Monarchy {
            monarch: abstr_origin
                .version_control
                .get_chain()
                .sender()
                .to_string(),
        },
        funds.as_deref(),
    )?;

    // We need to enable ibc on the account.
    origin_account.manager.update_settings(Some(true))?;

    // Now we send a message to the client saying that we want to create an account on the
    // destination chain
    let register_tx = origin_account.register_remote_account(&remote_name)?;

    let _ = interchain.wait_ibc(origin_id, register_tx)?;

    // After this is all ended, we return the account id of the account we just created on the remote chain
    let account_config = origin_account.manager.config()?;
    let remote_account_id = AccountId::new(
        account_config.account_id.seq(),
        AccountTrace::Remote(vec![ChainName::from_str(&origin_name)?]),
    )?;

    Ok((origin_account, remote_account_id))
}

pub fn abstract_ibc_connection_with<Chain: IbcQueryHandler, IBC: InterchainEnv<Chain>>(
    abstr: &Abstract<Chain>,
    interchain: &IBC,
    dest: &Abstract<Chain>,
    polytone_src: &Polytone<Chain>,
) -> Result<(), InterchainError> {
    // First we register client and host respectively
    let chain1_id = abstr.ibc.client.get_chain().chain_id();
    let chain1_name = ChainName::from_chain_id(&chain1_id);

    let chain2_id = dest.ibc.client.get_chain().chain_id();
    let chain2_name = ChainName::from_chain_id(&chain2_id);

    // First, we register the host with the client.
    // We register the polytone note with it because they are linked
    // This triggers an IBC message that is used to get back the proxy address
    let proxy_tx_result = abstr.ibc.client.register_infrastructure(
        chain2_name.to_string(),
        dest.ibc.host.address()?.to_string(),
        polytone_src.note.address()?.to_string(),
    )?;
    // We make sure the IBC execution is done so that the proxy address is saved inside the Abstract contract
    let _ = interchain.wait_ibc(&chain1_id, proxy_tx_result).unwrap();

    // Finally, we get the proxy address and register the proxy with the ibc host for the dest chain
    let proxy_address = abstr.ibc.client.host(chain2_name.to_string())?;

    abstract_std::ibc_host::ExecuteMsgFns::register_chain_proxy(
        &dest.ibc.host,
        chain1_name.to_string(),
        proxy_address.remote_polytone_proxy.unwrap(),
    )?;

    abstract_interface::AccountFactoryExecFns::update_config(
        &dest.account_factory,
        None,
        Some(dest.ibc.host.address()?.to_string()),
        None,
        None,
    )?;

    Ok(())
}

pub fn ibc_connect_polytone_and_abstract<Chain: IbcQueryHandler, IBC: InterchainEnv<Chain>>(
    interchain: &IBC,
    origin_chain_id: &str,
    remote_chain_id: &str,
) -> anyhow::Result<()> {
    let origin_chain = interchain.chain(origin_chain_id).unwrap();
    let remote_chain = interchain.chain(remote_chain_id).unwrap();

    let abstr_origin = Abstract::load_from(origin_chain.clone())?;
    let abstr_remote = Abstract::load_from(remote_chain.clone())?;

    let origin_polytone = Polytone::load_from(origin_chain.clone())?;
    let remote_polytone = Polytone::load_from(remote_chain.clone())?;

    // Creating a connection between 2 polytone deployments
    interchain.create_contract_channel(
        &origin_polytone.note,
        &remote_polytone.voice,
        POLYTONE_VERSION,
        None, // Unordered channel
    )?;
    // Create the connection between client and host
    abstract_ibc_connection_with(&abstr_origin, interchain, &abstr_remote, &origin_polytone)?;
    Ok(())
}

pub fn ibc_abstract_setup<Chain: IbcQueryHandler, IBC: InterchainEnv<Chain>>(
    interchain: &IBC,
    origin_chain_id: &str,
    remote_chain_id: &str,
) -> anyhow::Result<(Abstract<Chain>, Abstract<Chain>)> {
    let origin_chain = interchain.chain(origin_chain_id).unwrap();
    let remote_chain = interchain.chain(remote_chain_id).unwrap();

    // Deploying abstract and the IBC abstract logic
    let abstr_origin =
        Abstract::deploy_on(origin_chain.clone(), origin_chain.sender().to_string())?;
    let abstr_remote =
        Abstract::deploy_on(remote_chain.clone(), remote_chain.sender().to_string())?;

    // Deploying polytone on both chains
    Polytone::deploy_on(origin_chain.clone(), None)?;
    Polytone::deploy_on(remote_chain.clone(), None)?;

    ibc_connect_polytone_and_abstract(interchain, origin_chain_id, remote_chain_id)?;

    Ok((abstr_origin, abstr_remote))
}

pub const LST_DENOM: &str = "lst_denom";
pub const DENOM: &str = "uosmo";
pub const STARGAZE: &str = "stargaze-1";
pub const OSMOSIS: &str = "osmosis-1";

pub struct TestEnv {
    pub app: LstAdapterInterface<MockBase<MockApiBech32>>,
    pub mock: MockBech32InterchainEnv,
    pub origin_account: AbstractAccount<MockBase<MockApiBech32>>,
    pub remote_account_id: AccountId,
    pub abstr_remote: Abstract<MockBase<MockApiBech32>>,
    pub oracle_app: FakeStrideOracle<MockBase<MockApiBech32>>,
}

pub fn create_app(sender_balance: Vec<Coin>, vault: Option<String>) -> anyhow::Result<TestEnv> {
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
    let owner = mock.chain(OSMOSIS)?.sender();

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

    let oracle_app: FakeStrideOracle<MockBech32> =
        FakeStrideOracle::new("fake-stride-oracle", mock.chain(OSMOSIS)?.clone());
    oracle_app.upload()?;
    let init_msg = FakeStrideOracleInstantiateMsg {
        redemption_rate: Decimal::percent(REDEMPTION_RATE_PERCENT),
    };
    oracle_app.instantiate(&init_msg, None, None)?;

    origin_account.install_app(
        &app,
        &LstAdapterInstantiateMsg {
            owner: owner.to_string(),
            denoms: LstDenom {
                denom: LST_DENOM.to_string(),
                underlying: DENOM.to_string(),
            },
            vault: vault.to_string(),
            observer: vault.to_string(),
            stride_oracle: oracle_app.addr_str()?,
            unbond_period_secs: UNBOND_PERIOD,
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
        oracle_app,
    })
}
