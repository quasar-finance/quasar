#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Empty, Env, IbcMsg, IbcTimeout,
    MessageInfo, Order, Reply, Response, StdError, StdResult, Storage, SubMsg, Timestamp, Uint128,
};
use cw2::set_contract_version;
use quasar_types::ibc::{ChannelInfo, ChannelType};
use quasar_types::ica::packet::{InterchainAccountPacketData, Type};
use quasar_types::ica::traits::Pack;

use crate::error::ContractError;
use crate::helpers::{create_reply, create_submsg, parse_seq, IbcMsgKind, IcaMessages, MsgKind};
use crate::msg::{ChannelsResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CHANNELS, CONFIG, PENDING_ACK, REPLIES};
use crate::strategy::do_ibc_join_pool_swap_extern_amount_in;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:lp-strategy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
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
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    // Save the ibc message together with the sequence number, to be handled properly later at the ack, we can pass the ibc_kind one to one
    // TODO this needs and error check and error handling
    let kind = REPLIES.load(deps.storage, msg.id)?;
    match kind {
        MsgKind::Ibc(ibc_kind) => {
            let seq = parse_seq(msg)?;
            PENDING_ACK.save(deps.storage, seq, &ibc_kind)?;
        }
    }
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
        ExecuteMsg::TransferJoinLock {
            channel,
            to_address,
        } => execute_transfer(deps, env, info, channel, to_address),
        ExecuteMsg::DepositAndLockTokens {
            channel,
            pool_id,
            amount,
            lock_period,
            denom,
            share_out_min_amount,
        } => execute_join_pool(
            deps,
            env,
            channel,
            pool_id,
            denom,
            amount,
            share_out_min_amount,
        ),
    }
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
    let transfer = do_transfer(deps.storage, env, info.funds, channel.clone(), to_address.clone())?;

    Ok(Response::new()
        .add_submessage(transfer)
        .add_attribute("ibc-tranfer-channel", channel)
        .add_attribute("ibc-transfer-receiver", to_address))
}

pub fn execute_join_pool(
    deps: DepsMut,
    env: Env,
    channel_id: String,
    pool_id: u64,
    denom: String,
    amount: Uint128,
    share_out_min_amount: Uint128,
) -> Result<Response, ContractError> {
    let join = do_ibc_join_pool_swap_extern_amount_in(
        deps.storage,
        env,
        channel_id.clone(),
        pool_id,
        denom.clone(),
        amount,
        share_out_min_amount,
    )?;

    Ok(Response::new()
        .add_submessage(join)
        .add_attribute("ibc-join-pool-channel", channel_id)
        .add_attribute("denom", denom))
}

fn do_transfer(
    storage: &mut dyn Storage,
    env: Env,
    funds: Vec<Coin>,
    channel_id: String,
    to_address: String,
) -> Result<SubMsg, ContractError> {
    if funds.len() != 1 {
        return Err(ContractError::PaymentError(
            cw_utils::PaymentError::MultipleDenoms {},
        ));
    }
    // todo check denom of funds once we have denom mapping done

    let timeout = IbcTimeout::with_timestamp(env.block.time.plus_seconds(300));
    let transfer = IbcMsg::Transfer {
        channel_id,
        to_address,
        amount: funds[0].clone(),
        timeout,
    };

    Ok(create_submsg(
        storage,
        MsgKind::Ibc(IbcMsgKind::Transfer),
        transfer,
    )?)
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
