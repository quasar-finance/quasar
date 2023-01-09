#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Reply, Response, StdError,
    StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_utils::must_pay;
use quasar_types::ibc::ChannelInfo;

use crate::error::ContractError;
use crate::helpers::parse_seq;
use crate::msg::{ChannelsResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CHANNELS, CONFIG, DEPOSIT_SEQ, PENDING_ACK, REPLIES};
use crate::strategy::{do_ibc_join_pool_swap_extern_amount_in, do_transfer};
use crate::vault::do_deposit;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:lp-strategy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;
    CONFIG.save(
        deps.storage,
        &Config {
            lock_period: msg.lock_period,
            pool_id: msg.pool_id,
            pool_denom: msg.pool_denom,
            denom: msg.denom,
            local_denom: msg.local_denom,
        },
    )?;

    // set the deposit sequence number to zero
    DEPOSIT_SEQ.save(deps.storage, &Uint128::zero())?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    // Save the ibc message together with the sequence number, to be handled properly later at the ack, we can pass the ibc_kind one to one
    // TODO this needs and error check and error handling
    let pending = REPLIES.load(deps.storage, msg.id)?;

    let seq = parse_seq(
        msg.clone()
            .result
            .into_result()
            .map_err(|err| StdError::GenericErr { msg: err })?
            .events,
    )
    .map_err(|_| StdError::GenericErr {
        msg: format!("{:?}", msg),
    })?;
    PENDING_ACK.save(deps.storage, seq, &pending)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => execute_deposit(deps, env, info),
        ExecuteMsg::TransferJoinLock {
            channel,
            to_address,
        } => execute_transfer(deps, env, info, channel, to_address),
        ExecuteMsg::DepositAndLockTokens {
            channel,
            pool_id,
            amount,
            denom,
            share_out_min_amount,
        } => execute_join_pool(
            deps,
            env,
            info,
            channel,
            pool_id,
            denom,
            amount,
            share_out_min_amount,
        ),
    }
}

pub fn execute_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let deposit = do_deposit(deps, env, info.clone())?;

    Ok(Response::new()
        .add_submessage(deposit)
        .add_attribute("deposit", info.sender))
}

// transfer funds sent to the contract to an address on osmosis, this needs an extra change to always
// always send funds to the contracts ICA address
pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    channel: String, // TODO see if we can move channel mapping to a more zone like approach
    to_address: String,
) -> Result<Response, ContractError> {
    let dep_seq = DEPOSIT_SEQ.load(deps.storage)?;
    DEPOSIT_SEQ.save(deps.storage, &dep_seq.checked_add(Uint128::one())?)?;

    let amount = must_pay(&info, &CONFIG.load(deps.storage)?.local_denom)?;

    let transfer = do_transfer(
        deps.storage,
        env,
        info.sender,
        amount,
        channel.clone(),
        to_address.clone(),
        dep_seq,
    )?;

    Ok(Response::new()
        .add_submessage(transfer)
        .add_attribute("ibc-tranfer-channel", channel)
        .add_attribute("ibc-transfer-receiver", to_address))
}

pub fn execute_join_pool(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    channel_id: String,
    pool_id: u64,
    denom: String,
    amount: Uint128,
    share_out_min_amount: Uint128,
) -> Result<Response, ContractError> {
    let dep_seq = DEPOSIT_SEQ.load(deps.storage)?;
    DEPOSIT_SEQ.save(deps.storage, &dep_seq.checked_add(Uint128::one())?)?;

    let join = do_ibc_join_pool_swap_extern_amount_in(
        deps.storage,
        env,
        channel_id.clone(),
        pool_id,
        denom.clone(),
        amount,
        share_out_min_amount,
        info.sender,
        dep_seq,
    )?;

    Ok(Response::new()
        .add_submessage(join)
        .add_attribute("ibc-join-pool-channel", channel_id)
        .add_attribute("denom", denom))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Channels {} => to_binary(&handle_channels_query(deps)?),
    }
}

pub fn handle_channels_query(deps: Deps) -> StdResult<ChannelsResponse> {
    let channels: Vec<ChannelInfo> = CHANNELS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|kv| kv.unwrap().1)
        .collect();
    Ok(ChannelsResponse { channels })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        IbcTimeoutBlock,
    };

    const DENOM: &str = "satoshi";
    const CREATOR: &str = "creator";
    const INVESTOR: &str = "investor";
    const BUYER: &str = "buyer";

    fn default_instantiate(
        supply_decimals: u8,
        reserve_decimals: u8,
        reserve_supply: Uint128,
    ) -> InstantiateMsg {
        InstantiateMsg {
            lock_period: todo!(),
            pool_id: todo!(),
            pool_denom: todo!(),
            denom: todo!(),
            local_denom: todo!(),
        }
    }

    fn setup_test(
        deps: DepsMut,
        supply_decimals: u8,
        reserve_decimals: u8,
        reserve_supply: Uint128,
    ) {
    }
}
