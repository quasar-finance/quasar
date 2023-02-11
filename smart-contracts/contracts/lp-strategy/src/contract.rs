#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError,
};
use cw2::set_contract_version;
use cw_utils::must_pay;

use quasar_types::ibc::ChannelInfo;

use crate::bond::do_bond;
use crate::error::ContractError;
use crate::helpers::{get_ica_address, get_total_shares};
use crate::ibc_lock::Lock;
use crate::ibc_util::{do_ibc_join_pool_swap_extern_amount_in, do_transfer};
use crate::icq::try_icq;
use crate::msg::{
    ChannelsResponse, ConfigResponse, ExecuteMsg, IcaAddressResponse, IcaBalanceResponse,
    IcaChannelResponse, InstantiateMsg, LockResponse, PrimitiveSharesResponse, QueryMsg,
};
use crate::reply::{handle_ibc_reply, handle_execute_reply};
use crate::start_unbond::{do_start_unbond, StartUnbond};
use crate::state::{
    Config, OngoingDeposit, RawAmount, CHANNELS, CONFIG, IBC_LOCK, ICA_BALANCE, ICA_CHANNEL,
    LP_SHARES, PENDING_ACK, REPLIES, RETURNING,
};
use crate::unbond::{do_unbond, transfer_batch_unbond, PendingReturningUnbonds, ReturningUnbond};

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
            base_denom: msg.base_denom,
            local_denom: msg.local_denom,
            quote_denom: msg.quote_denom,
            return_source_channel: msg.return_source_channel,
            transfer_channel: msg.transfer_channel,
        },
    )?;

    IBC_LOCK.save(deps.storage, &Lock::new())?;

    LP_SHARES.save(deps.storage, &Uint128::zero())?;

    // this is a workaround so that the contract query does not fail for balance before deposits have been made successfully
    ICA_BALANCE.save(deps.storage, &Uint128::one())?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> StdResult<Response> {
    // Save the ibc message together with the sequence number, to be handled properly later at the ack, we can pass the ibc_kind one to one
    // TODO this needs and error check and error handling
    let kind = REPLIES.load(deps.storage, reply.id)?;
    match kind {
        crate::helpers::MsgKind::Ibc(ibc_kind) => handle_ibc_reply(deps, reply, ibc_kind)?,
        crate::helpers::MsgKind::WasmExecute(owner, amount) => {
            handle_execute_reply(deps, reply, owner, amount)?
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
        ExecuteMsg::Bond { id } => execute_bond(deps, env, info, id),
        ExecuteMsg::TransferJoinLock {
            channel,
            to_address,
        } => execute_transfer(deps, env, info, channel, to_address),
        ExecuteMsg::DepositAndLockTokens {
            pool_id,
            amount,
            denom,
            share_out_min_amount,
        } => execute_join_pool(
            deps,
            env,
            info,
            pool_id,
            denom,
            amount,
            share_out_min_amount,
        ),
        ExecuteMsg::StartUnbond { id, share_amount } => {
            execute_start_unbond(deps, env, info, id, share_amount)
        }
        ExecuteMsg::Unbond { id } => execute_unbond(deps, env, info, id),
        ExecuteMsg::AcceptReturningFunds { id } => {
            execute_accept_returning_funds(deps, &env, info, id)
        }
        ExecuteMsg::ReturnTransfer { amount } => execute_return_funds(deps, env, info, amount),
    }
}

pub fn execute_return_funds(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let msg = transfer_batch_unbond(
        deps.storage,
        &env,
        &PendingReturningUnbonds {
            unbonds: vec![ReturningUnbond {
                amount: RawAmount::LpShares(Uint128::new(100)),
                owner: info.sender,
                id: String::from("1"),
            }],
        },
        amount,
    )?;

    Ok(Response::new().add_submessage(msg))
}

pub fn execute_accept_returning_funds(
    deps: DepsMut,
    _env: &Env,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    let returning_amount = RETURNING
        .may_load(deps.storage, id)?
        .ok_or(ContractError::ReturningTransferNotFound)?;

    let amount = must_pay(&info, CONFIG.load(deps.storage)?.local_denom.as_str())?;
    if amount != returning_amount {
        return Err(ContractError::ReturningTransferIncorrectAmount);
    }

    // TODO cleanup the incoming transfer

    Ok(Response::new()
        .add_attribute("returning-transfer", id.to_string())
        .add_attribute("success", "true"))
}

pub fn execute_start_unbond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    share_amount: Uint128,
) -> Result<Response, ContractError> {
    do_start_unbond(
        deps.storage,
        StartUnbond {
            owner: info.sender.clone(),
            id,
            primitive_shares: share_amount,
        },
    )?;

    let msg = try_icq(deps.storage, env)?;

    match msg {
        Some(submsg) => Ok(Response::new()
            .add_submessage(submsg)
            .add_attribute("start-unbond", info.sender)
            .add_attribute("kind", "dispatch")),
        None => Ok(Response::new()
            .add_attribute("start-unbond", info.sender)
            .add_attribute("kind", "queue")),
    }
}

pub fn execute_unbond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    do_unbond(deps.storage, &env, info.sender.clone(), id)?;

    let msg = try_icq(deps.storage, env)?;

    match msg {
        Some(submsg) => Ok(Response::new()
            .add_submessage(submsg)
            .add_attribute("unbond", info.sender)
            .add_attribute("kind", "dispatch")),
        None => Ok(Response::new()
            .add_attribute("unbond", info.sender)
            .add_attribute("kind", "queue")),
    }
}

pub fn execute_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    let msg = do_bond(deps, env, info.clone(), id)?;

    // if msg is some, we are dispatching an icq
    match msg {
        Some(submsg) => Ok(Response::new()
            .add_submessage(submsg)
            .add_attribute("deposit", info.sender)
            .add_attribute("kind", "dispatch")),
        None => Ok(Response::new()
            .add_attribute("deposit", info.sender)
            .add_attribute("kind", "queue")),
    }
}

// transfer funds sent to the contract to an address on osmosis, this call ignores the lock system
pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    channel: String,
    to_address: String,
) -> Result<Response, ContractError> {
    let amount = must_pay(&info, &CONFIG.load(deps.storage)?.local_denom)?;

    let transfer = do_transfer(
        deps.storage,
        &env,
        amount,
        channel.clone(),
        to_address.clone(),
        // add a dummy ongoing deposit, actual ongoing deposit should calculate the claim using the total balance
        vec![OngoingDeposit {
            claim_amount: amount,
            owner: info.sender,
            raw_amount: RawAmount::LocalDenom(amount),
            bond_id: "id".to_string(),
        }],
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
    pool_id: u64,
    denom: String,
    amount: Uint128,
    share_out_min_amount: Uint128,
) -> Result<Response, ContractError> {
    let join = do_ibc_join_pool_swap_extern_amount_in(
        deps.storage,
        env,
        pool_id,
        denom.clone(),
        amount,
        share_out_min_amount,
        // add a dummy ongoing deposit, actual ongoing deposit should calculate the claim using the total balance
        vec![OngoingDeposit {
            claim_amount: amount,
            owner: info.sender,
            raw_amount: RawAmount::LocalDenom(amount),
            bond_id: "id".to_string(),
        }],
    )?;

    Ok(Response::new()
        .add_submessage(join)
        .add_attribute("denom", denom))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Channels {} => to_binary(&handle_channels_query(deps)?),
        QueryMsg::Config {} => to_binary(&handle_config_query(deps)?),
        QueryMsg::IcaAddress {} => to_binary(&handle_ica_address_query(deps)?),
        QueryMsg::PrimitiveShares {} => to_binary(&handle_primitive_shares(deps)?),
        QueryMsg::IcaBalance {} => to_binary(&handle_ica_balance(deps)?),
        QueryMsg::IcaChannel {} => to_binary(&handle_ica_channel(deps)?),
        QueryMsg::Lock {} => to_binary(&handle_lock(deps)?),
    }
}

pub fn handle_channels_query(deps: Deps) -> StdResult<ChannelsResponse> {
    let channels: Vec<ChannelInfo> = CHANNELS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|kv| kv.unwrap().1)
        .collect();
    Ok(ChannelsResponse { channels })
}

pub fn handle_config_query(deps: Deps) -> StdResult<ConfigResponse> {
    Ok(ConfigResponse {
        config: CONFIG.load(deps.storage)?,
    })
}

pub fn handle_ica_address_query(deps: Deps) -> StdResult<IcaAddressResponse> {
    Ok(IcaAddressResponse {
        address: get_ica_address(deps.storage, ICA_CHANNEL.load(deps.storage)?)
            .expect("ica address setup correctly"),
    })
}

pub fn handle_ica_channel(deps: Deps) -> StdResult<IcaChannelResponse> {
    Ok(IcaChannelResponse {
        channel: ICA_CHANNEL.load(deps.storage)?,
    })
}

pub fn handle_primitive_shares(deps: Deps) -> StdResult<PrimitiveSharesResponse> {
    Ok(PrimitiveSharesResponse {
        total: get_total_shares(deps.storage).map_err(|err| StdError::GenericErr {
            msg: err.to_string(),
        })?,
    })
}

pub fn handle_ica_balance(deps: Deps) -> StdResult<IcaBalanceResponse> {
    Ok(IcaBalanceResponse {
        amount: Coin {
            denom: CONFIG
                .load(deps.storage)
                .map_err(|err| StdError::GenericErr {
                    msg: err.to_string(),
                })?
                .local_denom,
            amount: ICA_BALANCE
                .load(deps.storage)
                .map_err(|err| StdError::GenericErr {
                    msg: "could not load balance: <".to_string() + &err.to_string() + ">",
                })?,
        },
    })
}

pub fn handle_lock(deps: Deps) -> StdResult<LockResponse> {
    Ok(LockResponse {
        lock: IBC_LOCK
            .load(deps.storage)
            .map_err(|err| StdError::GenericErr {
                msg: err.to_string(),
            })?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DENOM: &str = "satoshi";
    const CREATOR: &str = "creator";
    const INVESTOR: &str = "investor";
    const BUYER: &str = "buyer";

    // fn default_instantiate() -> InstantiateMsg {
    //     InstantiateMsg {
    //         lock_period: todo!(),
    //         pool_id: todo!(),
    //         pool_denom: todo!(),
    //         denom: todo!(),
    //         local_denom: todo!(),
    //     }
    // }

    fn setup_test(
        _deps: DepsMut,
        _supply_decimals: u8,
        _reserve_decimals: u8,
        _reserve_supply: Uint128,
    ) {
    }
}
