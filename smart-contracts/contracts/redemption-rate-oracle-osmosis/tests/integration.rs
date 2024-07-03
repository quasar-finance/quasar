mod fake_stride_oracle;

use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use cosmwasm_std::Decimal;
use cw_orch::{anyhow, prelude::*};
use fake_stride_oracle::{FakeStrideOracle, FakeStrideOracleInstantiateMsg};
use mars_owner::OwnerError;
use redemption_rate_oracle_osmosis::msg::OracleInfo;
use redemption_rate_oracle_osmosis::{
    contract::interface::RedemptionRateOracleInterface,
    msg::{
        OraclesResponse, RedemptionRateOracleExecuteMsgFns, RedemptionRateOracleInstantiateMsg,
        RedemptionRateOracleQueryMsgFns,
    },
    RedemptionRateOracleError, QUASAR_NAMESPACE,
};

struct TestEnv<Env: CwEnv> {
    mock: MockBech32,
    app: Application<Env, RedemptionRateOracleInterface<Env>>,
    oracle_app: FakeStrideOracle<Env>,
}

impl TestEnv<MockBech32> {
    fn setup(sender_is_owner: bool) -> anyhow::Result<TestEnv<MockBech32>> {
        let mock = MockBech32::new("mock");
        let sender = mock.sender();
        let namespace = Namespace::new(QUASAR_NAMESPACE)?;
        let abs_client = AbstractClient::builder(mock.clone()).build()?;

        let publisher = abs_client.publisher_builder(namespace).build()?;

        let oracle_app: FakeStrideOracle<MockBech32> =
            FakeStrideOracle::new("fake-stride-oracle", mock.clone());
        oracle_app.upload()?;
        let init_msg = FakeStrideOracleInstantiateMsg {
            redemption_rate: Decimal::percent(123),
        };
        oracle_app.instantiate(&init_msg, None, None)?;

        publisher.publish_app::<RedemptionRateOracleInterface<_>>()?;
        let owner = if sender_is_owner {
            sender.to_string()
        } else {
            mock.addr_make("other_owner").to_string()
        };
        let account = abs_client
            .account_builder()
            .install_app::<RedemptionRateOracleInterface<MockBech32>>(
                &RedemptionRateOracleInstantiateMsg {
                    owner,
                    stride_oracle: oracle_app.addr_str()?,
                },
            )?
            .build()?;
        let app = account.application::<RedemptionRateOracleInterface<MockBech32>>()?;
        Ok(TestEnv {
            mock,
            app,
            oracle_app,
        })
    }
}

#[test]
fn test_update_fails_if_not_owner() -> anyhow::Result<()> {
    let env = TestEnv::setup(false)?;
    let app = env.app;

    let addr = env.mock.addr_make("other");
    let result = app.update(Some(addr.to_string()));
    assert!(result.is_err());
    assert_eq!(
        result
            .unwrap_err()
            .downcast::<RedemptionRateOracleError>()
            .unwrap(),
        RedemptionRateOracleError::Owner(OwnerError::NotOwner {})
    );
    Ok(())
}

#[test]
fn test_oracle_works() -> anyhow::Result<()> {
    let env = TestEnv::setup(true)?;
    let app = env.app;

    let oracles = app.oracles()?;
    assert_eq!(
        oracles,
        OraclesResponse {
            oracles: vec![OracleInfo {
                name: "stride".to_string(),
                address: env.oracle_app.addr_str()?,
            }]
        }
    );
    let redemption_rate = app.redemption_rate("denom")?;
    assert_eq!(redemption_rate, Decimal::percent(123));

    let addr = env.mock.addr_make("other");
    assert!(app.update(Some(addr.to_string())).is_ok());
    let oracles = app.oracles()?;
    assert_eq!(
        oracles,
        OraclesResponse {
            oracles: vec![OracleInfo {
                name: "stride".to_string(),
                address: addr.to_string(),
            }]
        }
    );

    let result = app.redemption_rate("denom");
    assert!(result.is_err());
    Ok(())
}
