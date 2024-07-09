use crate::error::{assert_observer, assert_vault};
use crate::msg::{
    LstAdapterExecuteMsg, LstAdapterInstantiateMsg, LstAdapterMigrateMsg, LstAdapterQueryMsg,
};
use crate::state::{
    Denoms, IbcConfig, UnbondInfo, UnbondStatus, DENOMS, IBC_CONFIG, OBSERVER, ORACLE, OWNER,
    REDEEMED_BALANCE, TOTAL_BALANCE, UNBONDING, UNBOND_PERIOD_SECS, VAULT,
};
use crate::{LstAdapterError, LST_ADAPTER_OSMOSIS_ID, LST_ADAPTER_OSMOSIS_VERSION};
#[cfg(not(target_arch = "wasm32"))]
use abstract_app::abstract_interface::AbstractInterfaceError;
use abstract_app::{abstract_interface, AppContract};
use abstract_sdk::{AbstractResponse, IbcInterface, TransferInterface};
#[cfg(not(target_arch = "wasm32"))]
use abstract_std::manager::ModuleInstallConfig;
use abstract_std::objects::chain_name::ChainName;
use cosmwasm_std::{
    coin, coins, to_json_binary, BankMsg, Binary, Coin, Decimal, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult, Storage, Timestamp, Uint128,
};
use ica_oracle::msg::{
    QueryMsg as StrideQueryMsg, RedemptionRateResponse as StrideRedemptionRateResponse,
};
use mars_owner::OwnerInit::SetInitialOwner;
// use osmosis_std::types::ibc::applications::transfer::v1::MsgTransfer;
// use osmosis_std::types::ibc::core::client::v1::Height;
// use prost::Message;
use quasar_types::{
    error::assert_funds_single_token,
    query::query_contract_balance,
    // stride::{get_autopilot_msg, Action},
};

pub type LstAdapterResult<T = Response> = Result<T, LstAdapterError>;

pub type LstAdapter = AppContract<
    LstAdapterError,
    LstAdapterInstantiateMsg,
    LstAdapterExecuteMsg,
    LstAdapterQueryMsg,
    LstAdapterMigrateMsg,
>;

const APP: LstAdapter = LstAdapter::new(LST_ADAPTER_OSMOSIS_ID, LST_ADAPTER_OSMOSIS_VERSION, None)
    .with_instantiate(instantiate_)
    .with_execute(execute_)
    .with_query(query_)
    .with_migrate(migrate_);

#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, LstAdapter);

abstract_app::cw_orch_interface!(APP, LstAdapter, LstAdapterInterface);

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::LstAdapterInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;

    fn dependency_install_configs(
        _configuration: Self::DependenciesConfig,
    ) -> Result<Vec<ModuleInstallConfig>, AbstractInterfaceError> {
        Ok(vec![])
    }
}

pub fn instantiate_(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _app: LstAdapter,
    msg: LstAdapterInstantiateMsg,
) -> LstAdapterResult {
    OWNER.initialize(deps.storage, deps.api, SetInitialOwner { owner: msg.owner })?;
    VAULT.save(deps.storage, &deps.api.addr_validate(&msg.vault)?)?;
    OBSERVER.save(deps.storage, &deps.api.addr_validate(&msg.observer)?)?;
    DENOMS.save(deps.storage, &msg.denoms)?;
    ORACLE.save(deps.storage, &deps.api.addr_validate(&msg.stride_oracle)?)?;
    UNBONDING.save(deps.storage, &vec![])?;
    UNBOND_PERIOD_SECS.save(deps.storage, &msg.unbond_period_secs)?;
    REDEEMED_BALANCE.save(deps.storage, &Uint128::zero())?;
    TOTAL_BALANCE.save(deps.storage, &Uint128::zero())?;
    Ok(Response::default())
}

pub fn execute_(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    app: LstAdapter,
    msg: LstAdapterExecuteMsg,
) -> LstAdapterResult {
    match msg {
        LstAdapterExecuteMsg::Unbond {} => unbond(deps, env, info, app),
        LstAdapterExecuteMsg::ConfirmUnbond { amount } => confirm_unbond(deps, info, app, amount),
        LstAdapterExecuteMsg::ConfirmUnbondFinished { unbond_start_time } => {
            confirm_unbond_finished(deps, env, info, app, unbond_start_time)
        }
        LstAdapterExecuteMsg::Claim {} => claim(deps, env, info, app),
        LstAdapterExecuteMsg::UpdateIbcConfig {
            remote_chain,
            channel,
            revision,
            block_offset,
            timeout_secs,
        } => update_ibc_config(
            deps,
            info,
            app,
            remote_chain,
            channel,
            revision,
            block_offset,
            timeout_secs,
        ),
        LstAdapterExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
        LstAdapterExecuteMsg::Update {
            vault,
            denoms,
            observer,
            stride_oracle,
            unbond_period_secs: unbond_period,
        } => update(
            deps,
            info,
            app,
            vault,
            observer,
            denoms,
            stride_oracle,
            unbond_period,
        ),
    }
}

fn record_pending_unbond(
    storage: &mut dyn Storage,
    amount: Uint128,
    time: Timestamp,
) -> Result<bool, LstAdapterError> {
    let mut unbonding = UNBONDING.load(storage)?;
    let pending = unbonding
        .last()
        .map(|info| info.status.clone())
        .unwrap_or(UnbondStatus::Confirmed)
        == UnbondStatus::Unconfirmed;
    if !pending {
        unbonding.push(UnbondInfo {
            amount,
            unbond_start: time,
            status: UnbondStatus::Unconfirmed,
        });
        UNBONDING.save(storage, &unbonding)?;
        TOTAL_BALANCE.update(storage, |balance| -> StdResult<Uint128> {
            balance
                .checked_add(amount)
                .map_err(|err| StdError::GenericErr {
                    msg: err.to_string(),
                })
        })?;
    }
    Ok(pending)
}

fn unbond(deps: DepsMut, env: Env, info: MessageInfo, app: LstAdapter) -> LstAdapterResult {
    assert_vault(&info.sender, &VAULT.load(deps.storage)?)?;
    let denoms = DENOMS.load(deps.storage)?;
    assert_funds_single_token(&info.funds, &denoms.lst)?;
    let redemption_rate = query_redemption_rate(deps.as_ref())?;
    let unbond_amount = query_contract_balance(&deps.querier, &env, &denoms.lst)?;
    let previous_unbond_pending = record_pending_unbond(
        deps.storage,
        unbond_amount.checked_mul_floor(redemption_rate)?,
        env.block.time,
    )?;

    let response = app.response("unbond");
    if previous_unbond_pending {
        return Ok(response);
    }

    let unbond_funds = coins(unbond_amount.into(), denoms.lst);
    let mut transfer_msgs = app.bank(deps.as_ref()).deposit(unbond_funds.clone())?;
    let ibc_client = app.ibc_client(deps.as_ref());
    let ibc_msg = ibc_client.ics20_transfer(
        ChainName::from_chain_id("stargaze-1").to_string(),
        unbond_funds,
    )?;
    transfer_msgs.push(ibc_msg);

    // let ibc_config = IBC_CONFIG.load(deps.storage)?;
    // let remote_addr = app
    //     .ibc_client(deps.as_ref())
    //     .remote_proxy_addr(&ibc_config.remote_chain)?;
    // if remote_addr.is_none() {
    //     return Err(LstAdapterError::MissingRemoteAddress {
    //         chain: ibc_config.remote_chain,
    //     });
    // }
    // let remote_addr = remote_addr.unwrap();
    // let autopilot_redeem_msg = get_autopilot_msg(
    //     Action::RedeemStake,
    //     remote_addr.as_ref(),
    //     Some(env.contract.address.to_string()),
    // );
    // let timeout_timestamp = if let Some(timeout_secs) = ibc_config.timeout_secs { env.block.time.nanos() + timeout_secs * 1_000_000_000 } else { 0u64 };
    // let msg = MsgTransfer {
    //     source_port: "transfer".to_string(),
    //     source_channel: ibc_config.channel,
    //     token: Some(info.funds[0].clone().into()),
    //     sender: env.contract.address.to_string(),
    //     receiver: remote_addr,
    //     timeout_height: Some(Height {
    //         revision_number: 5,
    //         revision_height: env.block.height + 5,
    //     }),
    //     timeout_timestamp,
    //     memo: serde_json::to_string(&autopilot_redeem_msg)
    //         .map_err(|err| LstAdapterError::Json(err.to_string()))?,
    // };
    // Ok(app.response("unbond").add_message(msg))
    Ok(response.add_messages(transfer_msgs))
}

fn adjust_total_balance(
    storage: &mut dyn Storage,
    amount: Uint128,
    final_amount: Uint128,
) -> StdResult<()> {
    TOTAL_BALANCE.update(storage, |balance| -> StdResult<Uint128> {
        balance
            .checked_add(final_amount)
            .map_err(|err| StdError::GenericErr {
                msg: err.to_string(),
            })?
            .checked_sub(amount)
            .map_err(|err| StdError::GenericErr {
                msg: err.to_string(),
            })
    })?;
    Ok(())
}

fn confirm_unbond(
    deps: DepsMut,
    info: MessageInfo,
    app: LstAdapter,
    amount: Uint128,
) -> LstAdapterResult {
    assert_observer(&info.sender, &OBSERVER.load(deps.storage)?)?;
    let mut unbonding = UNBONDING.load(deps.storage)?;
    let last = unbonding.last_mut();
    if let Some(last) = last {
        if last.status == UnbondStatus::Unconfirmed {
            last.status = UnbondStatus::Confirmed;
            adjust_total_balance(deps.storage, last.amount, amount)?;
            last.amount = amount;
            UNBONDING.save(deps.storage, &unbonding)?;

            return Ok(app.response("confirm unbond"));
        }
    }

    Err(LstAdapterError::NothingToConfirm {})
}

fn confirm_unbond_finished(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    app: LstAdapter,
    unbond_start_time: Timestamp,
) -> LstAdapterResult {
    assert_observer(&info.sender, &OBSERVER.load(deps.storage)?)?;
    let mut unbonding = UNBONDING.load(deps.storage)?;
    let pos = unbonding.iter().position(|info| {
        info.status == UnbondStatus::Confirmed && info.unbond_start == unbond_start_time
    });
    if let Some(pos) = pos {
        let unbond_info = unbonding.remove(pos);
        let unbond_period_secs = UNBOND_PERIOD_SECS.load(deps.storage)?;
        if unbond_info.unbond_start.seconds() + unbond_period_secs > env.block.time.seconds() {
            return Err(LstAdapterError::UnbondNotFinished {});
        }
        let mut redeemed_balance = REDEEMED_BALANCE.load(deps.storage)?;
        let denoms = DENOMS.load(deps.storage)?;
        let contract_balance = query_contract_balance(&deps.querier, &env, &denoms.underlying)?;
        redeemed_balance += unbond_info.amount;
        if redeemed_balance > contract_balance {
            return Err(LstAdapterError::StillWaitingForFunds {});
        }
        REDEEMED_BALANCE.save(deps.storage, &redeemed_balance)?;
        UNBONDING.save(deps.storage, &unbonding)?;
        return Ok(app.response("confirm unbond finished"));
    }

    Err(LstAdapterError::NoPendingUnbond {})
}

fn claim(deps: DepsMut, env: Env, info: MessageInfo, app: LstAdapter) -> LstAdapterResult {
    assert_vault(&info.sender, &VAULT.load(deps.storage)?)?;
    let claimable = get_claimable(&deps.as_ref(), &env)?;

    if claimable.amount.is_zero() {
        return Err(LstAdapterError::NothingToClaim {});
    }

    let redeemed_balance = REDEEMED_BALANCE.load(deps.storage)?;
    REDEEMED_BALANCE.save(deps.storage, &Uint128::zero())?;
    let msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![claimable],
    };

    TOTAL_BALANCE.update(deps.storage, |balance| -> StdResult<Uint128> {
        balance
            .checked_sub(redeemed_balance)
            .map_err(|err| StdError::GenericErr {
                msg: err.to_string(),
            })
    })?;
    Ok(app.response("claim").add_message(msg))
}

#[allow(clippy::too_many_arguments)]
fn update_ibc_config(
    deps: DepsMut,
    info: MessageInfo,
    app: LstAdapter,
    remote_chain: String,
    channel: String,
    revision: Option<u64>,
    block_offset: Option<u64>,
    timeout_secs: Option<u64>,
) -> LstAdapterResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    IBC_CONFIG.save(
        deps.storage,
        &IbcConfig {
            remote_chain,
            channel,
            revision,
            block_offset,
            timeout_secs,
        },
    )?;
    Ok(app.response("update ibc config"))
}

#[allow(clippy::too_many_arguments)]
fn update(
    deps: DepsMut,
    info: MessageInfo,
    app: LstAdapter,
    vault: Option<String>,
    observer: Option<String>,
    denoms: Option<Denoms>,
    stride_oracle: Option<String>,
    unbond_period: Option<u64>,
) -> LstAdapterResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    if let Some(vault) = vault {
        VAULT.save(deps.storage, &deps.api.addr_validate(&vault)?)?;
    }
    if let Some(observer) = observer {
        OBSERVER.save(deps.storage, &deps.api.addr_validate(&observer)?)?;
    }
    if let Some(denoms) = denoms {
        DENOMS.save(deps.storage, &denoms)?;
    }
    if let Some(stride_oracle) = stride_oracle {
        ORACLE.save(deps.storage, &deps.api.addr_validate(&stride_oracle)?)?;
    }
    if let Some(unbond_period) = unbond_period {
        UNBOND_PERIOD_SECS.save(deps.storage, &unbond_period)?;
    }
    Ok(app.response("update"))
}

fn get_balance(deps: Deps, env: Env) -> Result<Uint128, LstAdapterError> {
    let total_balance = TOTAL_BALANCE.load(deps.storage)?;
    let denoms = DENOMS.load(deps.storage)?;
    let lst_balance = query_contract_balance(&deps.querier, &env, &denoms.lst)?;
    let redemption_rate = query_redemption_rate(deps)?;
    Ok(total_balance.checked_add(lst_balance.checked_mul_floor(redemption_rate)?)?)
}

pub fn query_(
    deps: Deps,
    env: Env,
    _app: &LstAdapter,
    msg: LstAdapterQueryMsg,
) -> LstAdapterResult<Binary> {
    match msg {
        LstAdapterQueryMsg::IbcConfig {} => Ok(to_json_binary(
            &IBC_CONFIG.may_load(deps.storage)?.unwrap_or_default(),
        )?),
        LstAdapterQueryMsg::Owner {} => Ok(to_json_binary(
            &OWNER
                .current(deps.storage)?
                .map(String::from)
                .unwrap_or_default(),
        )?),
        LstAdapterQueryMsg::Vault {} => Ok(to_json_binary(&VAULT.load(deps.storage)?.to_string())?),
        LstAdapterQueryMsg::Oracle {} => {
            Ok(to_json_binary(&ORACLE.load(deps.storage)?.to_string())?)
        }
        LstAdapterQueryMsg::Denoms {} => Ok(to_json_binary(&DENOMS.load(deps.storage)?)?),
        LstAdapterQueryMsg::RedemptionRate {} => Ok(to_json_binary(&query_redemption_rate(deps)?)?),
        LstAdapterQueryMsg::PendingUnbonds {} => {
            Ok(to_json_binary(&UNBONDING.load(deps.storage)?)?)
        }
        LstAdapterQueryMsg::BalanceInUnderlying {} => Ok(to_json_binary(&get_balance(deps, env)?)?),
        LstAdapterQueryMsg::Claimable {} => Ok(to_json_binary(&get_claimable(&deps, &env)?)?),
    }
}

fn get_underlying_balance(deps: &Deps, env: &Env) -> StdResult<Coin> {
    let denoms = DENOMS.load(deps.storage)?;
    Ok(coin(
        query_contract_balance(&deps.querier, env, &denoms.underlying)?.into(),
        denoms.underlying,
    ))
}

fn get_active_unbonding(deps: &Deps, time: Timestamp) -> StdResult<Vec<UnbondInfo>> {
    let unbonding = UNBONDING.load(deps.storage)?;
    let unbond_period_secs = UNBOND_PERIOD_SECS.load(deps.storage)?;
    Ok(unbonding
        .into_iter()
        .filter(|info| info.unbond_start.plus_seconds(unbond_period_secs) > time)
        .collect())
}

fn get_info_amount(info: &UnbondInfo) -> Uint128 {
    info.amount
}

fn query_redemption_rate(deps: Deps) -> StdResult<Decimal> {
    let response: StrideRedemptionRateResponse = deps.querier.query_wasm_smart(
        ORACLE.load(deps.storage)?,
        &StrideQueryMsg::RedemptionRate {
            denom: DENOMS.load(deps.storage)?.lst,
            params: None,
        },
    )?;
    Ok(response.redemption_rate)
}

fn get_claimable(deps: &Deps, env: &Env) -> LstAdapterResult<Coin> {
    let mut underlying_balance = get_underlying_balance(deps, env)?;
    let unbonding = UNBONDING.load(deps.storage)?;
    let unbond_period_secs = UNBOND_PERIOD_SECS.load(deps.storage)?;
    let unbonding_expired: Vec<UnbondInfo> = unbonding
        .into_iter()
        .filter(|info| info.unbond_start.plus_seconds(unbond_period_secs) <= env.block.time)
        .collect();

    let claimable = if unbonding_expired.is_empty() {
        underlying_balance
    } else {
        let blocked = unbonding_expired.iter().map(get_info_amount).sum();
        if blocked < underlying_balance.amount {
            underlying_balance.amount = underlying_balance.amount.checked_sub(blocked)?;
            underlying_balance
        } else {
            coin(0u128, underlying_balance.denom)
        }
    };

    Ok(claimable)
}

pub fn migrate_(
    _deps: DepsMut,
    _env: Env,
    app: LstAdapter,
    _msg: LstAdapterMigrateMsg,
) -> LstAdapterResult {
    Ok(app.response("migrate"))
}
