use abstract_app::objects::ans_host::AnsHostError;
use abstract_app::sdk::AbstractSdkError;
use abstract_app::std::AbstractError;
use abstract_app::AppError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdError;
use cosmwasm_std::{to_json_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response};
use cw_orch::prelude::*;
use cw_storage_plus::Item;
use ica_oracle::msg::RedemptionRateResponse;
use thiserror::Error;

pub const REDEMPTION_RATE: Item<Decimal> = Item::new("redemption_rate");
pub const LAST_UPDATE: Item<u64> = Item::new("last_update");

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Abstract(#[from] AbstractError),

    #[error("{0}")]
    AbstractSdk(#[from] AbstractSdkError),

    #[error("{0}")]
    AnsHost(#[from] AnsHostError),

    #[error("{0}")]
    DappError(#[from] AppError),
}

#[cw_serde]
pub struct FakeStrideOracleInstantiateMsg {
    pub redemption_rate: Decimal,
}

#[cw_serde]
pub struct FakeStrideOracleMigrateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum FakeStrideOracleExecuteMsg {
    Update {
        redemption_rate: Decimal,
        last_update: u64,
    },
}

#[cw_serde]
pub enum FakeStrideOracleQueryMsg {
    RedemptionRate { denom: String, params: Option<u64> },
}

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: FakeStrideOracleInstantiateMsg,
) -> Result<Response, ContractError> {
    REDEMPTION_RATE.save(deps.storage, &msg.redemption_rate)?;
    LAST_UPDATE.save(deps.storage, &0u64)?;

    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: FakeStrideOracleExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        FakeStrideOracleExecuteMsg::Update {
            redemption_rate,
            last_update,
        } => {
            REDEMPTION_RATE.update(deps.storage, |_| -> Result<_, ContractError> {
                Ok(redemption_rate)
            })?;
            LAST_UPDATE.save(deps.storage, &last_update)?;
        }
    }
    Ok(Response::new())
}

pub fn query(
    deps: Deps,
    _env: Env,
    msg: FakeStrideOracleQueryMsg,
) -> Result<Binary, ContractError> {
    match msg {
        FakeStrideOracleQueryMsg::RedemptionRate { .. } => {
            Ok(to_json_binary(&RedemptionRateResponse {
                redemption_rate: REDEMPTION_RATE.load(deps.storage)?,
                update_time: LAST_UPDATE.load(deps.storage)?,
            })?)
        }
    }
}

pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: FakeStrideOracleMigrateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cw_orch::interface(
    FakeStrideOracleInstantiateMsg,
    FakeStrideOracleExecuteMsg,
    FakeStrideOracleQueryMsg,
    FakeStrideOracleMigrateMsg
)]
pub struct FakeStrideOracle;

impl<Chain> Uploadable for FakeStrideOracle<Chain> {
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(ContractWrapper::new_with_empty(execute, instantiate, query).with_migrate(migrate))
    }
}
