#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use std::collections::HashMap;

use cosmwasm_std::{to_binary, Addr, Binary, Coin, Deps, Env, Order, StdError, StdResult, Uint128};
use quasar_types::ibc::ChannelInfo;

use crate::{
    bond::Bond,
    error::Trap,
    helpers::{get_ica_address, get_total_primitive_shares, IbcMsgKind, SubMsgKind},
    msg::{
        ChannelsResponse, ConfigResponse, GetQueuesResponse, IcaAddressResponse,
        IcaBalanceResponse, IcaChannelResponse, ListBondingClaimsResponse, ListPendingAcksResponse,
        ListPrimitiveSharesResponse, ListRepliesResponse, ListUnbondingClaimsResponse,
        LockResponse, LpSharesResponse, OsmoLockResponse, PrimitiveSharesResponse, QueryMsg,
        SimulatedJoinResponse, TrappedErrorsResponse, UnbondingClaimResponse,
    },
    start_unbond::StartUnbond,
    state::{
        Unbond, BONDING_CLAIMS, BOND_QUEUE, CHANNELS, CONFIG, IBC_LOCK, ICA_CHANNEL, LP_SHARES,
        NEW_PENDING_ACK, OSMO_LOCK, PENDING_BOND_QUEUE, REPLIES, SHARES, SIMULATED_JOIN_AMOUNT_IN,
        SIMULATED_JOIN_RESULT, START_UNBOND_QUEUE, TOTAL_VAULT_BALANCE, TRAPS, UNBONDING_CLAIMS,
        UNBOND_QUEUE,
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
        QueryMsg::SimulatedJoin {} => to_binary(&handle_simulated_join(deps)?),
        QueryMsg::GetQueues {} => to_binary(&handle_get_queues(deps)?),
    }
}

pub fn handle_get_queues(deps: Deps) -> StdResult<GetQueuesResponse> {
    let pbq: Result<Vec<Bond>, StdError> = PENDING_BOND_QUEUE.iter(deps.storage)?.collect();
    let bq: Result<Vec<Bond>, StdError> = BOND_QUEUE.iter(deps.storage)?.collect();
    let suq: Result<Vec<StartUnbond>, StdError> = START_UNBOND_QUEUE.iter(deps.storage)?.collect();
    let uq: Result<Vec<Unbond>, StdError> = UNBOND_QUEUE.iter(deps.storage)?.collect();
    Ok(GetQueuesResponse {
        pending_bond_queue: pbq?,
        bond_queue: bq?,
        start_unbond_queue: suq?,
        unbond_queue: uq?,
    })
}

pub fn handle_simulated_join(deps: Deps) -> StdResult<SimulatedJoinResponse> {
    Ok(SimulatedJoinResponse {
        amount: SIMULATED_JOIN_AMOUNT_IN.may_load(deps.storage)?,
        result: SIMULATED_JOIN_RESULT.may_load(deps.storage)?,
    })
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
        unbond: UNBONDING_CLAIMS.may_load(deps.storage, (addr, id))?,
    })
}

pub fn handle_trapped_errors_query(deps: Deps) -> StdResult<TrappedErrorsResponse> {
    let trapped: StdResult<HashMap<String, Trap>> = TRAPS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|res| {
            let ((seq, chan), kind) = res?;
            Ok((format!("{}-{}", seq, chan), kind))
        })
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
    let pending: StdResult<HashMap<String, IbcMsgKind>> = NEW_PENDING_ACK
        .range(deps.storage, None, None, Order::Ascending)
        .map(|res| {
            let ((seq, chan), kind) = res?;
            Ok((format!("{}-{}", seq, chan), kind))
        })
        .collect();
    Ok(ListPendingAcksResponse { pending: pending? })
}

pub fn handle_list_replies(deps: Deps) -> StdResult<ListRepliesResponse> {
    let replies: StdResult<HashMap<u64, SubMsgKind>> = REPLIES
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(ListRepliesResponse { replies: replies? })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use super::*;

    #[test]
    fn get_trapped_errors_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let q = QueryMsg::TrappedErrors {};

        TRAPS
            .save(
                deps.as_mut().storage,
                (100, "channel-1".to_string()),
                &Trap {
                    error: "failed to do a thing".to_string(),
                    step: IbcMsgKind::Icq,
                    last_succesful: true,
                },
            )
            .unwrap();

        let res = query(deps.as_ref(), env, q).unwrap();
    }
}
