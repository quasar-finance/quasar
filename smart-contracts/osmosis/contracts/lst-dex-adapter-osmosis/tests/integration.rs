mod fake_lst_adapter;
use cw_asset::AssetInfo;
use fake_lst_adapter::{interface::FakeLstInterface, FakeLstExecuteMsgFns, FakeLstInstantiateMsg};
use lst_dex_adapter_osmosis::{
    contract::interface::DexAdapterInterface,
    msg::{
        ConfigResponse, DexAdapterExecuteMsgFns, DexAdapterInstantiateMsg, DexAdapterQueryMsgFns,
    },
    DexAdapterError, MY_NAMESPACE,
};

use abstract_app::objects::{namespace::Namespace, PoolAddress};
use abstract_client::{AbstractClient, Application};
use abstract_dex_adapter::{interface::DexAdapter, msg::DexInstantiateMsg, DEX_ADAPTER_ID};
use cosmwasm_std::{Decimal, Uint128};
use cw_orch::{anyhow, prelude::*};
use quasar_types::error::FundsError;
use wyndex_bundle::{WynDex, WYNDEX};

struct TestEnv<Env: CwEnv> {
    mock: MockBech32,
    app: Application<Env, DexAdapterInterface<Env>>,
    lst_app: Application<Env, FakeLstInterface<Env>>,
    dex: WynDex,
}

impl TestEnv<MockBech32> {
    fn setup(
        sender_offer_fund_amount: u128,
        mut funds: Vec<Coin>,
    ) -> anyhow::Result<TestEnv<MockBech32>> {
        let mock = MockBech32::new_with_chain_id("mock", "juno-1");
        let sender = mock.sender_addr();
        let namespace = Namespace::new(MY_NAMESPACE)?;

        let abs_client = AbstractClient::builder(mock.clone()).build()?;

        let wyndex = wyndex_bundle::WynDex::deploy_on(mock.clone(), Empty {})?;
        if sender_offer_fund_amount != 0u128 {
            funds.push(Coin::new(
                sender_offer_fund_amount,
                wyndex.eur_token.to_string(),
            ));
        }
        if !funds.is_empty() {
            abs_client.set_balance(sender, &funds)?;
        }
        let abstract_publisher = abs_client
            .publisher_builder(Namespace::from_id(DEX_ADAPTER_ID)?)
            .build()?;
        let _dex_adapter: DexAdapter<MockBech32> =
            abstract_publisher.publish_adapter(DexInstantiateMsg {
                swap_fee: Decimal::percent(1),
                recipient_account: 0,
            })?;
        let publisher = abs_client.publisher_builder(namespace).build()?;
        publisher.publish_app::<FakeLstInterface<_>>()?;
        let account = abs_client
            .account_builder()
            .install_app_with_dependencies::<FakeLstInterface<MockBech32>>(
                &FakeLstInstantiateMsg {
                    redemption_rate: Decimal::one(),
                },
                Empty {},
            )?
            .build()?;
        let lst_app = account.application::<FakeLstInterface<MockBech32>>()?;
        publisher.publish_app::<DexAdapterInterface<_>>()?;
        let account = abs_client
            .account_builder()
            .install_app_with_dependencies::<DexAdapterInterface<MockBech32>>(
                &DexAdapterInstantiateMsg {
                    lst_adapter: lst_app.addr_str()?,
                    dex: WYNDEX.to_string(),
                    offer_asset: AssetInfo::native(wyndex.eur_token.to_string()),
                    receive_asset: AssetInfo::native(wyndex.usd_token.to_string()),
                    margin: Decimal::percent(1),
                    pool: PoolAddress::Contract(wyndex.eur_usd_pair.clone()),
                },
                Empty {},
            )?
            .build()?;
        let dex_app = account.application::<DexAdapterInterface<MockBech32>>()?;
        dex_app.authorize_on_adapters(&[DEX_ADAPTER_ID])?;
        Ok(TestEnv {
            mock,
            app: dex_app,
            dex: wyndex,
            lst_app,
        })
    }
}

#[test]
fn successful_install() -> anyhow::Result<()> {
    let env = TestEnv::setup(0, vec![])?;
    let app = env.app;

    let config = app.config()?;
    assert_eq!(
        config,
        ConfigResponse {
            lst_adapter: env.lst_app.addr_str()?,
            dex: WYNDEX.to_string(),
            offer_asset: AssetInfo::native(env.dex.eur_token.to_string()),
            receive_asset: AssetInfo::native(env.dex.usd_token.to_string()),
            margin: Decimal::percent(1),
            pool: PoolAddress::Contract(env.dex.eur_usd_pair),
        }
    );
    Ok(())
}

#[test]
fn missing_funds() -> anyhow::Result<()> {
    let env = TestEnv::setup(0, vec![])?;
    let app = env.app;

    let result = app.swap(None, &[]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<DexAdapterError>().unwrap(),
        DexAdapterError::Funds(FundsError::InvalidAssets(1))
    );
    Ok(())
}

#[test]
fn too_many_funds() -> anyhow::Result<()> {
    let mut funds = vec![Coin::new(100_000u128, "uother")];
    let env = TestEnv::setup(100_000u128, funds.clone())?;
    funds.push(Coin::new(100_000u128, env.dex.eur_token.to_string()));
    let app = env.app;

    let result = app.swap(None, &funds);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<DexAdapterError>().unwrap(),
        DexAdapterError::Funds(FundsError::InvalidAssets(1))
    );
    Ok(())
}

#[test]
fn wrong_denom() -> anyhow::Result<()> {
    let funds = vec![Coin::new(100_000u128, "wrong_denom")];
    let env = TestEnv::setup(0, funds.clone())?;
    let app = env.app;

    let result = app.swap(None, &funds);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<DexAdapterError>().unwrap(),
        DexAdapterError::Funds(FundsError::WrongDenom(env.dex.eur_token.to_string()))
    );
    Ok(())
}

#[test]
fn swap_gets_rejected_if_price_above_redemption_rate() -> anyhow::Result<()> {
    let env = TestEnv::setup(100_000u128, vec![])?;
    let funds = vec![Coin::new(1_000u128, env.dex.eur_token.to_string())];
    let app = env.app;

    let result = app.swap(None, &funds);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<DexAdapterError>().unwrap(),
        DexAdapterError::InvalidPrice {}
    );
    Ok(())
}

#[test]
fn swap_succeeds_if_price_below_redemption_rate() -> anyhow::Result<()> {
    let env = TestEnv::setup(100_000u128, vec![])?;
    let funds = vec![Coin::new(1_000u128, env.dex.eur_token.to_string())];
    let app = env.app;
    env.lst_app.update(Decimal::from_ratio(2u8, 1u8))?;

    assert!(app.swap(None, &funds).is_ok());
    let received_amount = env
        .mock
        .query_balance(&env.mock.sender, &env.dex.usd_token.to_string())?;
    assert_eq!(received_amount, Uint128::from(900u128));
    Ok(())
}
