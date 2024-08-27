use crate::msg::{LstAdapterExecuteMsgFns, LstAdapterQueryMsgFns};
use crate::state::{UnbondInfo, UnbondStatus};
use crate::tests::fake_stride_oracle::FakeStrideOracleExecuteMsgFns;
use crate::tests::ibc_setup::{
    create_app, DENOM, LST_DENOM, OSMOSIS, REDEMPTION_RATE_PERCENT, STARGAZE, UNBOND_PERIOD,
};
use crate::LstAdapterError;
use abstract_interface::AbstractAccount;
use cosmwasm_std::{coins, Decimal, Uint128};
use cw_orch::{anyhow, prelude::*};
use cw_orch_interchain::prelude::*;
use quasar_types::error::FundsError;

#[test]
fn test_if_not_vault_then_unbond_fails() -> anyhow::Result<()> {
    let app = create_app(vec![], Some("other".to_string()))?.app;

    let result = app.unbond(&[]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>()?,
        LstAdapterError::NotVault {}
    );
    Ok(())
}

#[test]
fn test_if_missing_funds_then_unbond_fails() -> anyhow::Result<()> {
    let app = create_app(vec![], None)?.app;

    let result = app.unbond(&[]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>()?,
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
        result.unwrap_err().downcast::<LstAdapterError>()?,
        LstAdapterError::Funds(FundsError::WrongDenom(LST_DENOM.to_string()))
    );
    Ok(())
}

#[test]
fn test_unbond_sends_ibc_message() -> anyhow::Result<()> {
    let funds = coins(123, LST_DENOM);
    let env = create_app(funds.clone(), None)?;
    let app = env.app;

    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;

    let remote_account = AbstractAccount::new(&env.abstr_remote, env.remote_account_id.clone());
    let remote_denom: &str = &format!("ibc/channel-0/{}", LST_DENOM);
    let remote_balance = env
        .mock
        .chain(STARGAZE)?
        .query_balance(&remote_account.proxy.address()?, remote_denom)?;
    assert_eq!(Uint128::from(123u32), remote_balance);

    let expected_pending =
        Uint128::from(123u32).checked_mul_floor(Decimal::percent(REDEMPTION_RATE_PERCENT))?;
    let pending_unbonds = app.pending_unbonds()?;
    let start_time = env.mock.chain(OSMOSIS)?.block_info()?.time;
    assert_eq!(pending_unbonds.len(), 1);
    assert_eq!(
        pending_unbonds[0],
        UnbondInfo {
            amount: expected_pending,
            unbond_start: start_time,
            status: UnbondStatus::Unconfirmed
        }
    );

    let balance = app.balance_in_underlying()?;
    assert_eq!(balance, expected_pending);

    Ok(())
}

#[test]
fn test_unbonding_delayed_if_previous_unbond_was_not_confirmed() -> anyhow::Result<()> {
    let sender_balance = coins(123_456_789, LST_DENOM);
    let env = create_app(sender_balance, None)?;
    let app = env.app;
    let osmosis = env.mock.chain(OSMOSIS)?;

    let amount0 = Uint128::from(123u128);
    let amount1 = Uint128::from(345u128);
    let amount2 = Uint128::from(567u128);
    let total = amount0 + amount1 + amount2;
    let redemption_rate = Decimal::percent(REDEMPTION_RATE_PERCENT);
    let expected_redeem_amount0 = amount0 * redemption_rate;
    let funds = coins(amount0.u128(), LST_DENOM);
    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;

    let funds = coins(amount1.u128(), LST_DENOM);
    assert!(app.unbond(&funds).is_ok());
    let contract_balance = osmosis.query_balance(&app.address()?, LST_DENOM)?;
    assert_eq!(contract_balance, amount1);
    let total_app_balance = app.balance_in_underlying()?;
    let expected_total = expected_redeem_amount0 + amount1 * redemption_rate;
    assert_eq!(total_app_balance, expected_total);

    let new_redemption_rate = Decimal::percent(200);
    env.oracle_app.update(123, new_redemption_rate)?;

    assert!(app.confirm_unbond(expected_redeem_amount0).is_ok());
    let funds = coins(amount2.u128(), LST_DENOM);
    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;
    let contract_balance = osmosis.query_balance(&app.address()?, LST_DENOM)?;
    assert_eq!(contract_balance, Uint128::zero());
    let total_app_balance = app.balance_in_underlying()?;
    let expected_total =
        expected_redeem_amount0 + amount1 * new_redemption_rate + amount2 * new_redemption_rate;
    assert_eq!(total_app_balance, expected_total);

    let remote_account = AbstractAccount::new(&env.abstr_remote, env.remote_account_id.clone());
    let remote_denom: &str = &format!("ibc/channel-0/{}", LST_DENOM);
    let remote_balance = env
        .mock
        .chain(STARGAZE)?
        .query_balance(&remote_account.proxy.address()?, remote_denom)?;
    assert_eq!(remote_balance, total);

    Ok(())
}

#[test]
fn test_multiple_pending_unbonds() -> anyhow::Result<()> {
    let sender_balance = coins(123_456_789, LST_DENOM);
    let env = create_app(sender_balance, None)?;
    let app = env.app;

    let amount0 = Uint128::from(123u128);
    let amount1 = Uint128::from(345u128);
    let amount2 = Uint128::from(789u128);
    let redemption_rate = Decimal::percent(REDEMPTION_RATE_PERCENT);
    let offset_secs = 500u64;
    let start_time = env.mock.chain(OSMOSIS)?.block_info()?.time;
    let funds = coins(amount0.u128(), LST_DENOM);
    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;

    env.mock.chain(OSMOSIS)?.wait_seconds(offset_secs)?;
    let expected_redeem_amount0 = Uint128::from(10u64) + amount0 * redemption_rate;
    assert!(app.confirm_unbond(expected_redeem_amount0).is_ok());

    let funds = coins(amount1.u128(), LST_DENOM);
    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;

    env.mock.chain(OSMOSIS)?.wait_seconds(offset_secs)?;
    assert!(app.confirm_unbond(amount1 * redemption_rate).is_ok());

    let funds = coins(amount2.u128(), LST_DENOM);
    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;
    let total_redeemed =
        expected_redeem_amount0 + amount1 * redemption_rate + amount2 * redemption_rate;

    let pending_unbonds = app.pending_unbonds()?;
    assert_eq!(pending_unbonds.len(), 3);
    assert_eq!(
        pending_unbonds[0],
        UnbondInfo {
            amount: expected_redeem_amount0,
            unbond_start: start_time,
            status: UnbondStatus::Confirmed
        }
    );
    assert_eq!(
        pending_unbonds[1],
        UnbondInfo {
            amount: amount1 * redemption_rate,
            unbond_start: start_time.plus_seconds(offset_secs),
            status: UnbondStatus::Confirmed
        }
    );
    assert_eq!(
        pending_unbonds[2],
        UnbondInfo {
            amount: amount2 * redemption_rate,
            unbond_start: start_time.plus_seconds(2 * offset_secs),
            status: UnbondStatus::Unconfirmed
        }
    );

    let balance = app.balance_in_underlying()?;
    assert_eq!(balance, total_redeemed);

    Ok(())
}

#[test]
fn test_claim_multiple_deposits_and_random_donation() -> anyhow::Result<()> {
    let sender_balance = coins(123_456_789, LST_DENOM);
    let env = create_app(sender_balance, None)?;
    let app = env.app;

    let amount0 = Uint128::from(123u128);
    let amount1 = Uint128::from(345u128);
    let donation = Uint128::from(567u128);
    let redemption_rate = Decimal::percent(REDEMPTION_RATE_PERCENT);
    let osmosis = env.mock.chain(OSMOSIS)?;
    let start_time = osmosis.block_info()?.time;
    let funds = coins(amount0.u128(), LST_DENOM);
    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;
    assert!(app.confirm_unbond(amount0 * redemption_rate).is_ok());

    osmosis.wait_seconds(UNBOND_PERIOD)?;

    let funds = coins(amount1.u128(), LST_DENOM);
    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;

    let redeemable0 = amount0 * redemption_rate;
    let redeemable1 = amount1 * redemption_rate;
    let total_redeemable = redeemable0 + redeemable1;

    let balance = app.balance_in_underlying()?;
    assert_eq!(balance, total_redeemable);

    let claimable = app.claimable()?;
    assert_eq!(claimable.amount, Uint128::zero());

    let result = app.claim();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>()?,
        LstAdapterError::NothingToClaim {}
    );
    osmosis.set_balance(
        &app.address()?,
        coins((donation + redeemable0).into(), DENOM),
    )?;
    let claimable = app.claimable()?;
    assert_eq!(claimable.amount, donation);
    assert!(app.confirm_unbond_finished(start_time).is_ok());
    let cb = osmosis.query_balance(&app.address()?, DENOM)?;
    assert_eq!(cb, redeemable0 + donation);
    let pending = app.pending_unbonds()?;
    assert_eq!(pending.len(), 1);
    assert_eq!(
        pending[0],
        UnbondInfo {
            amount: redeemable1,
            unbond_start: osmosis.block_info()?.time,
            status: UnbondStatus::Unconfirmed
        }
    );
    let claimable = app.claimable()?;
    assert_eq!(claimable.amount, redeemable0 + donation);

    let expected_contract_balance = redeemable0 + redeemable1;
    assert_eq!(app.balance_in_underlying()?, expected_contract_balance);

    assert!(app.claim().is_ok());
    let claimed = osmosis.query_balance(&osmosis.sender_addr(), DENOM)?;
    assert_eq!(claimed, redeemable0 + donation);

    assert_eq!(app.balance_in_underlying()?, redeemable1);

    Ok(())
}

#[test]
fn test_claim_works_unbond_is_finished_and_funds_are_available() -> anyhow::Result<()> {
    let sender_balance = coins(123_456_789, LST_DENOM);
    let env = create_app(sender_balance, None)?;
    let app = env.app;

    let amount0 = Uint128::from(123u128);
    let redemption_rate = Decimal::percent(REDEMPTION_RATE_PERCENT);
    let osmosis = env.mock.chain(OSMOSIS)?;
    let start_time = osmosis.block_info()?.time;
    let funds = coins(amount0.u128(), LST_DENOM);
    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;
    assert!(app.confirm_unbond(amount0 * redemption_rate).is_ok());

    osmosis.wait_seconds(UNBOND_PERIOD)?;

    let total_redeem_amount = amount0 * redemption_rate;

    let underlying_balance = total_redeem_amount;
    osmosis.set_balance(&app.address()?, coins(underlying_balance.into(), DENOM))?;
    assert!(app.confirm_unbond_finished(start_time).is_ok());

    assert_eq!(app.balance_in_underlying()?, total_redeem_amount);

    assert!(app.claim().is_ok());
    let balance = osmosis.query_balance(&osmosis.sender_addr(), DENOM)?;
    assert_eq!(balance, underlying_balance);

    let expected_contract_balance = Uint128::zero();
    assert_eq!(app.balance_in_underlying()?, expected_contract_balance);

    Ok(())
}

#[test]
fn test_confirm_finished_fails_before_expiration() -> anyhow::Result<()> {
    let sender_balance = coins(123_456_789, LST_DENOM);
    let env = create_app(sender_balance, None)?;
    let app = env.app;

    let amount0 = Uint128::from(123u128);
    let redemption_rate = Decimal::percent(REDEMPTION_RATE_PERCENT);
    let osmosis = env.mock.chain(OSMOSIS)?;
    let start_time = osmosis.block_info()?.time;
    let funds = coins(amount0.u128(), LST_DENOM);
    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;
    assert!(app.confirm_unbond(amount0 * redemption_rate).is_ok());
    osmosis.wait_seconds(UNBOND_PERIOD / 2)?;
    let result = app.confirm_unbond_finished(start_time);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>()?,
        LstAdapterError::UnbondNotFinished {}
    );

    Ok(())
}
#[test]
fn test_confirm_finished_fails_if_funds_are_not_yet_available() -> anyhow::Result<()> {
    let sender_balance = coins(123_456_789, LST_DENOM);
    let env = create_app(sender_balance, None)?;
    let app = env.app;

    let amount0 = Uint128::from(123u128);
    let redemption_rate = Decimal::percent(REDEMPTION_RATE_PERCENT);
    let osmosis = env.mock.chain(OSMOSIS)?;
    let start_time = osmosis.block_info()?.time;
    let funds = coins(amount0.u128(), LST_DENOM);
    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;
    assert!(app.confirm_unbond(amount0 * redemption_rate).is_ok());
    osmosis.wait_seconds(UNBOND_PERIOD)?;

    let result = app.confirm_unbond_finished(start_time);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>()?,
        LstAdapterError::StillWaitingForFunds {}
    );

    Ok(())
}

#[test]
fn test_claim_fails_unbond_is_not_finished_and_funds_are_available() -> anyhow::Result<()> {
    let sender_balance = coins(123_456_789, LST_DENOM);
    let env = create_app(sender_balance, None)?;
    let app = env.app;

    let amount0 = Uint128::from(123u128);
    let redemption_rate = Decimal::percent(REDEMPTION_RATE_PERCENT);
    let osmosis = env.mock.chain(OSMOSIS)?;
    let funds = coins(amount0.u128(), LST_DENOM);
    let ibc_action_result = app.unbond(&funds)?;
    let _ = env.mock.wait_ibc(OSMOSIS, ibc_action_result)?;
    assert!(app.confirm_unbond(amount0 * redemption_rate).is_ok());

    osmosis.wait_seconds(UNBOND_PERIOD)?;

    let total_redeem_amount = amount0 * redemption_rate;

    osmosis.set_balance(&app.address()?, coins(total_redeem_amount.into(), DENOM))?;

    assert_eq!(app.balance_in_underlying()?, total_redeem_amount);

    let result = app.claim();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().downcast::<LstAdapterError>()?,
        LstAdapterError::NothingToClaim {}
    );

    Ok(())
}
