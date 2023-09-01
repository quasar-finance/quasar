use cosmwasm_std::coin;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::to_binary;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::Decimal;

use cosmwasm_std::Reply;
use cosmwasm_std::SubMsg;
use cosmwasm_std::SubMsgResult;
use cosmwasm_std::Uint128;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePositionResponse;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::Pool;
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenomResponse;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMint;

use crate::concentrated_liquidity::create_position;

use crate::error::ContractError;
use crate::error::ContractResult;
use crate::helpers::must_pay_two;
use crate::merge::execute_merge;
use crate::msg::ModifyRangeMsg;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::query_info;
use crate::query::query_metadata;
use crate::query::query_pool;
use crate::query::query_position;
use crate::query::query_total_assets;
use crate::query::query_total_vault_token_supply;
use crate::query::query_user_balance;
use crate::query::query_user_rewards;
use crate::reply::handle_reply;
use crate::reply::Replies;

use crate::rewards::execute_distribute_rewards;
use crate::state::Position;
use crate::state::POSITION;
use crate::state::VAULT_DENOM;
use crate::state::{PoolConfig, POOL_CONFIG, VAULT_CONFIG};
use crate::state::{ADMIN_ADDRESS, RANGE_ADMIN};
use crate::vault::admin::execute_admin;

use crate::vault::claim::execute_claim_user_rewards;
use crate::vault::deposit::execute_any_deposit;
use crate::vault::deposit::execute_exact_deposit;
use crate::vault::range::execute_modify_range;
use crate::vault::withdraw::execute_withdraw;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cl-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    VAULT_CONFIG.save(deps.storage, &msg.config)?;

    let pool: Pool = PoolmanagerQuerier::new(&deps.querier)
        .pool(msg.pool_id)?
        .pool
        .ok_or(ContractError::PoolNotFound {
            pool_id: msg.pool_id,
        })?
        .try_into()
        .unwrap();

    POOL_CONFIG.save(
        deps.storage,
        &PoolConfig {
            pool_id: pool.id,
            token0: pool.token0.clone(),
            token1: pool.token1.clone(),
        },
    )?;

    ADMIN_ADDRESS.save(deps.storage, &deps.api.addr_validate(&msg.admin)?)?;
    RANGE_ADMIN.save(deps.storage, &deps.api.addr_validate(&msg.range_admin)?)?;

    let create_denom: CosmosMsg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: msg.vault_token_subdenom,
    }
    .into();

    // in order to create the initial position, we need some funds to throw in there, these funds should be seen as burned
    let (initial0, initial1) = must_pay_two(&info, (pool.token0, pool.token1))?;

    let create_pos = create_position(
        deps.storage,
        &env,
        msg.initial_lower_tick,
        msg.initial_upper_tick,
        vec![initial0, initial1],
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            create_denom,
            Replies::CreateDenom as u64,
        ))
        .add_submessage(SubMsg::reply_on_success(
            create_pos,
            Replies::InstantiateCreatePosition as u64,
        )))
}

pub fn handle_instantiate_create_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let response: MsgCreatePositionResponse = data.try_into()?;
    POSITION.save(
        deps.storage,
        &Position {
            position_id: response.position_id,
        },
    )?;

    let liquidity = Decimal::raw(response.liquidity_created.parse()?);
    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    // todo do we want to mint the initial mint to the instantiater, or just not care?
    let mint = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(coin(liquidity.atomics().u128(), vault_denom).into()),
        mint_to_address: env.contract.address.to_string(),
    };

    Ok(Response::new()
        .add_message(mint)
        .add_attribute("initial-position", response.position_id.to_string())
        .add_attribute("initial-liquidity", response.liquidity_created))
}

pub fn handle_create_denom_reply(
    deps: DepsMut,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let response: MsgCreateDenomResponse = data.try_into()?;
    VAULT_DENOM.save(deps.storage, &response.new_token_denom)?;

    Ok(Response::new().add_attribute("vault_denom", response.new_token_denom))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        cw_vault_multi_standard::VaultStandardExecuteMsg::AnyDeposit {
            amount: _,
            asset: _,
            recipient: _,
        } => unimplemented!(),
        cw_vault_multi_standard::VaultStandardExecuteMsg::ExactDeposit { recipient } => {
            execute_exact_deposit(deps, env, &info, recipient)
        }
        cw_vault_multi_standard::VaultStandardExecuteMsg::Redeem { recipient, amount } => {
            execute_withdraw(deps, env, info, recipient, amount)
        }
        cw_vault_multi_standard::VaultStandardExecuteMsg::VaultExtension(vault_msg) => {
            match vault_msg {
                crate::msg::ExtensionExecuteMsg::Admin(admin_msg) => {
                    execute_admin(deps, info, admin_msg)
                }
                crate::msg::ExtensionExecuteMsg::Merge(msg) => execute_merge(deps, env, info, msg),
                crate::msg::ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                    lower_price,
                    upper_price,
                    max_slippage,
                }) => execute_modify_range(deps, env, info, lower_price, upper_price, max_slippage),
                crate::msg::ExtensionExecuteMsg::DistributeRewards {} => {
                    execute_distribute_rewards(deps, env)
                }
                crate::msg::ExtensionExecuteMsg::ClaimRewards {} => {
                    execute_claim_user_rewards(deps, info.sender.as_str())
                }
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    match msg {
        cw_vault_multi_standard::VaultStandardQueryMsg::VaultStandardInfo {} => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::Info {} => {
            Ok(to_binary(&query_info(deps)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::PreviewDeposit { assets: _ } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::DepositRatio => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::PreviewRedeem { amount: _ } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::TotalAssets {} => {
            Ok(to_binary(&query_total_assets(deps, env)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::TotalVaultTokenSupply {} => {
            Ok(to_binary(&query_total_vault_token_supply(deps)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::ConvertToShares { amount: _ } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::ConvertToAssets { amount: _ } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension(msg) => match msg {
            crate::msg::ExtensionQueryMsg::Metadata =>  Ok(to_binary(&query_metadata(deps)?)?),
            crate::msg::ExtensionQueryMsg::Balances(msg) => match msg {
                crate::msg::UserBalanceQueryMsg::UserLockedBalance { user } => {
                    Ok(to_binary(&query_user_balance(deps, user)?)?)
                }
                crate::msg::UserBalanceQueryMsg::UserRewards { user } => {
                    Ok(to_binary(&query_user_rewards(deps, user)?)?)
                }
            },
            crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(msg) => match msg {
                crate::msg::ClQueryMsg::Pool {} => Ok(to_binary(&query_pool(deps)?)?),
                crate::msg::ClQueryMsg::Position {} => Ok(to_binary(&query_position(deps)?)?),
                crate::msg::ClQueryMsg::RangeAdmin {} => todo!(),
            },
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    handle_reply(deps, env, msg)
}

#[cfg(test)]
mod tests {}
