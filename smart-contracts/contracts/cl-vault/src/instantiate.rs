use cosmwasm_std::{
    coin, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, Response, StdError, SubMsg, SubMsgResult,
    Uint128,
};
use osmosis_std::try_proto_to_cosmwasm_coins;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    MsgCreatePositionResponse, Pool,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    MsgCreateDenom, MsgCreateDenomResponse, MsgMint,
};

use crate::helpers::{get_asset0_value, must_pay_one_or_two};
use crate::math::tick::{build_tick_exp_cache, verify_tick_exp_cache};
use crate::msg::InstantiateMsg;
use crate::reply::Replies;
use crate::state::{
    Metadata, MigrationStatus, PoolConfig, Position, ADMIN_ADDRESS, METADATA, MIGRATION_STATUS,
    POOL_CONFIG, POSITION, RANGE_ADMIN, VAULT_CONFIG, VAULT_DENOM,
};
use crate::vault::concentrated_liquidity::{create_position, get_position};
use crate::ContractError;

pub fn handle_instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // a performance fee of more than 1 means that the performance fee is more than 100%
    if msg.config.performance_fee > Decimal::one() {
        return Err(ContractError::Std(StdError::generic_err(
            "performance fee cannot be more than 1.0",
        )));
    }

    build_tick_exp_cache(deps.storage)?;
    verify_tick_exp_cache(deps.storage)?;

    deps.api.addr_validate(msg.config.dex_router.as_str())?;

    VAULT_CONFIG.save(deps.storage, &msg.config)?;

    let pool: Pool = PoolmanagerQuerier::new(&deps.querier)
        .pool(msg.pool_id)?
        .pool
        .ok_or(ContractError::PoolNotFound {
            pool_id: msg.pool_id,
        })?
        .try_into()?;

    POOL_CONFIG.save(
        deps.storage,
        &PoolConfig {
            pool_id: pool.id,
            token0: pool.token0.clone(),
            token1: pool.token1.clone(),
        },
    )?;

    METADATA.save(
        deps.storage,
        &Metadata {
            thesis: msg.thesis,
            name: msg.name,
        },
    )?;

    let admin = deps.api.addr_validate(&msg.admin)?;

    ADMIN_ADDRESS.save(deps.storage, &admin)?;
    RANGE_ADMIN.save(deps.storage, &deps.api.addr_validate(&msg.range_admin)?)?;
    MIGRATION_STATUS.save(deps.storage, &MigrationStatus::Open)?;

    let create_denom_msg: CosmosMsg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: msg.vault_token_subdenom,
    }
    .into();

    // in order to create the initial position, we need some funds to throw in there, these funds should be seen as burned
    let (initial0, initial1) = must_pay_one_or_two(&info, (pool.token0, pool.token1))?;

    let create_position_msg = create_position(
        deps,
        &env,
        msg.initial_lower_tick,
        msg.initial_upper_tick,
        vec![initial0, initial1],
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            create_denom_msg,
            Replies::CreateDenom as u64,
        ))
        .add_submessage(SubMsg::reply_on_success(
            create_position_msg,
            Replies::InstantiateCreatePosition as u64,
        )))
}

pub fn handle_create_denom_reply(
    deps: DepsMut,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let response: MsgCreateDenomResponse = data.try_into()?;
    VAULT_DENOM.save(deps.storage, &response.new_token_denom)?;

    Ok(Response::new().add_attribute("vault_denom", response.new_token_denom))
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
            join_time: env.block.time.seconds(),
            claim_after: None,
        },
    )?;

    let position_info = get_position(deps.storage, &deps.querier)?;
    // Check if asset0 and asset1 are present, and handle the case where they are not.
    let asset0 = position_info
        .asset0
        .ok_or_else(|| ContractError::MissingAssetInfo {
            asset: "asset0".to_string(),
        })?;
    let asset1 = position_info
        .asset1
        .ok_or_else(|| ContractError::MissingAssetInfo {
            asset: "asset1".to_string(),
        })?;

    let assets = try_proto_to_cosmwasm_coins(vec![asset0, asset1])?;
    let free_asset0 = deps
        .querier
        .query_balance(&env.contract.address, assets[0].denom.clone())?;
    let free_asset1 = deps
        .querier
        .query_balance(&env.contract.address, assets[1].denom.clone())?;

    let asset_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        assets[0].amount + free_asset0.amount,
        assets[1].amount + free_asset1.amount,
    )?;

    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    // todo do we want to mint the initial mint to the instantiater, or just not care?
    let mint_msg = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(coin(asset_value.u128(), vault_denom).into()),
        mint_to_address: env.contract.address.to_string(),
    };

    Ok(Response::new()
        .add_message(mint_msg)
        .add_attribute("initial_position", response.position_id.to_string())
        .add_attribute("initial_liquidity", response.liquidity_created))
}
