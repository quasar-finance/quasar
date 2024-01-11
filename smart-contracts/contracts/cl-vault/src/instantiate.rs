use cosmwasm_std::{
    coin, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, Response, StdError, SubMsg, SubMsgResult,
    Uint128,
};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    MsgCreatePositionResponse, Pool,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    MsgCreateDenom, MsgCreateDenomResponse, MsgMint,
};

use crate::helpers::must_pay_one_or_two;
use crate::math::tick::{build_tick_exp_cache, verify_tick_exp_cache};
use crate::msg::InstantiateMsg;
use crate::reply::Replies;
use crate::rewards::CoinList;
use crate::state::{
    Metadata, PoolConfig, Position, ADMIN_ADDRESS, IS_DISTRIBUTING, METADATA,
    POOL_CONFIG, POSITION, RANGE_ADMIN, STRATEGIST_REWARDS, VAULT_CONFIG,
    VAULT_DENOM,
};
use crate::vault::concentrated_liquidity::create_position;
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

    STRATEGIST_REWARDS.save(deps.storage, &CoinList::new())?;
    IS_DISTRIBUTING.save(deps.storage, &false)?;

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
        },
    )?;

    let liquidity_amount = Decimal::raw(response.liquidity_created.parse()?);
    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    // todo do we want to mint the initial mint to the instantiater, or just not care?
    let mint_msg = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(coin(liquidity_amount.to_uint_floor().into(), vault_denom).into()),
        mint_to_address: env.contract.address.to_string(),
    };

    Ok(Response::new()
        .add_message(mint_msg)
        .add_attribute("initial_position", response.position_id.to_string())
        .add_attribute("initial_liquidity", response.liquidity_created))
}
