#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use std::collections::HashMap;

use cosmwasm_std::{to_binary, Addr, Binary, Coin, Deps, Env, Order, StdError, StdResult, Uint128};
use quasar_types::ibc::ChannelInfo;

use crate::{
    error::Trap,
    helpers::{get_ica_address, get_total_primitive_shares, IbcMsgKind, SubMsgKind},
    msg::{
        ChannelsResponse, ConfigResponse, IcaAddressResponse, IcaBalanceResponse,
        IcaChannelResponse, ListBondingClaimsResponse, ListPendingAcksResponse,
        ListPrimitiveSharesResponse, ListRepliesResponse, ListUnbondingClaimsResponse,
        LockResponse, LpSharesResponse, OsmoLockResponse, PrimitiveSharesResponse, QueryMsg,
        TrappedErrorsResponse, UnbondingClaimResponse,
    },
    state::{
        Unbond, BONDING_CLAIMS, CHANNELS, CONFIG, IBC_LOCK, ICA_CHANNEL, LP_SHARES, OSMO_LOCK,
        PENDING_ACK, REPLIES, SHARES, TOTAL_VAULT_BALANCE, TRAPS, UNBONDING_CLAIMS,
    },
};

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
        QueryMsg::LpShares {} => to_binary(&handle_lp_shares_query(deps)?),
        QueryMsg::TrappedErrors {} => to_binary(&handle_trapped_errors_query(deps)?),
        QueryMsg::ListUnbondingClaims {} => to_binary(&handle_list_unbonding_claims(deps)?),
        QueryMsg::UnbondingClaim { addr, id } => {
            to_binary(&handle_unbonding_claim_query(deps, addr, id)?)
        }
        QueryMsg::ListBondingClaims {} => to_binary(&handle_list_bonding_claims(deps)?),
        QueryMsg::ListPrimitiveShares {} => to_binary(&handle_list_primitive_shares(deps)?),
        QueryMsg::ListPendingAcks {} => to_binary(&handle_list_pending_acks(deps)?),
        QueryMsg::ListReplies {} => to_binary(&handle_list_replies(deps)?),
        QueryMsg::OsmoLock {} => to_binary(&handle_osmo_lock(deps)?),
    }
}

pub fn handle_osmo_lock(deps: Deps) -> StdResult<OsmoLockResponse> {
    Ok(OsmoLockResponse {
        lock_id: OSMO_LOCK.load(deps.storage)?,
    })
}

pub fn handle_list_unbonding_claims(deps: Deps) -> StdResult<ListUnbondingClaimsResponse> {
    let unbonds: StdResult<HashMap<Addr, (String, Unbond)>> = UNBONDING_CLAIMS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|res| {
            let val = res?;
            Ok((val.0 .0, (val.0 .1, val.1)))
        })
        .collect();
    Ok(ListUnbondingClaimsResponse { unbonds: unbonds? })
}

pub fn handle_unbonding_claim_query(
    deps: Deps,
    addr: Addr,
    id: String,
) -> StdResult<UnbondingClaimResponse> {
    Ok(UnbondingClaimResponse {
        unbond: UNBONDING_CLAIMS.load(deps.storage, (addr, id))?,
    })
}

pub fn handle_trapped_errors_query(deps: Deps) -> StdResult<TrappedErrorsResponse> {
    let trapped: StdResult<Vec<(u64, Trap)>> = TRAPS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(TrappedErrorsResponse { errors: trapped? })
}

pub fn handle_channels_query(deps: Deps) -> StdResult<ChannelsResponse> {
    let channels: Vec<ChannelInfo> = CHANNELS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|kv| kv.unwrap().1)
        .collect();
    Ok(ChannelsResponse { channels })
}

pub fn handle_lp_shares_query(deps: Deps) -> StdResult<LpSharesResponse> {
    Ok(LpSharesResponse {
        lp_shares: LP_SHARES.load(deps.storage)?,
    })
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
    let total = get_total_primitive_shares(deps.storage).map_err(|err| StdError::GenericErr {
        msg: err.to_string(),
    })?;
    Ok(PrimitiveSharesResponse { total })
}

pub fn handle_ica_balance(deps: Deps) -> StdResult<IcaBalanceResponse> {
    let amount = TOTAL_VAULT_BALANCE.load(deps.storage)?;

    Ok(IcaBalanceResponse {
        amount: Coin {
            denom: CONFIG.load(deps.storage)?.local_denom,
            amount,
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

pub fn handle_list_bonding_claims(deps: Deps) -> StdResult<ListBondingClaimsResponse> {
    let bonds: StdResult<HashMap<Addr, (String, Uint128)>> = BONDING_CLAIMS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|res| {
            let val = res?;
            Ok((val.0 .0, (val.0 .1, val.1)))
        })
        .collect();
    Ok(ListBondingClaimsResponse { bonds: bonds? })
}

pub fn handle_list_primitive_shares(deps: Deps) -> StdResult<ListPrimitiveSharesResponse> {
    let shares: StdResult<HashMap<Addr, Uint128>> = SHARES
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(ListPrimitiveSharesResponse { shares: shares? })
}

pub fn handle_list_pending_acks(deps: Deps) -> StdResult<ListPendingAcksResponse> {
    let pending: StdResult<HashMap<u64, IbcMsgKind>> = PENDING_ACK
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(ListPendingAcksResponse { pending: pending? })
}

pub fn handle_list_replies(deps: Deps) -> StdResult<ListRepliesResponse> {
    let replies: StdResult<HashMap<u64, SubMsgKind>> = REPLIES
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(ListRepliesResponse { replies: replies? })
}
