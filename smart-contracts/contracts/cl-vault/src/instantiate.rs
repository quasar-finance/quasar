use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, Uint128};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::Pool;
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

use crate::ContractError;
use crate::msg::InstantiateMsg;
use crate::vault::concentrated_liquidity::create_position;
use crate::helpers::must_pay_two;
use crate::reply::Replies;
use crate::state::{ADMIN_ADDRESS, RANGE_ADMIN, LOCKUP_DURATION, PoolConfig, POOL_CONFIG, VAULT_CONFIG};

pub fn handle_instantiate(
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

    let admin = deps.api.addr_validate(&msg.admin)?;

    ADMIN_ADDRESS.save(deps.storage, &admin)?;
    RANGE_ADMIN.save(deps.storage, &deps.api.addr_validate(&msg.range_admin)?)?;

    LOCKUP_DURATION.save(deps.storage, &cw_utils::Duration::Time(msg.lockup_duration))?;

    let create_denom_msg: CosmosMsg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: msg.vault_token_subdenom,
    }
        .into();

    // in order to create the initial position, we need some funds to throw in there, these funds should be seen as burned
    let (initial0, initial1) = must_pay_two(&info, (pool.token0, pool.token1))?;

    // DOUBTS: Are we creating a position with the funds.amount for token0 and token1, expecting a min amount out of 0, right?
    let create_position_msg = create_position(
        deps.storage,
        &env,
        msg.initial_lower_tick,
        msg.initial_upper_tick,
        vec![initial0, initial1],
        Uint128::zero(),
        Uint128::zero(),
    )?;

    // DOUBTS:
    // - Why only on_success?
    // - General Wasm question: Is the failure of one of the submessages atomically reverting all the submessages execution, as well as the past workflow of this same function?
    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            create_denom_msg,
            Replies::CreateDenom as u64,
        ))
        .add_submessage(SubMsg::reply_on_success(
            create_position_msg,
            Replies::InstantiateCreatePosition as u64,
        ))
    )
}
