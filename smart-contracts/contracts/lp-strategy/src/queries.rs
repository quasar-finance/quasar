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
        OSMO_LOCK, PENDING_ACK, PENDING_BOND_QUEUE, PENDING_UNBONDING_CLAIMS, REPLIES, SHARES,
        SIMULATED_JOIN_AMOUNT_IN, SIMULATED_JOIN_RESULT, START_UNBOND_QUEUE, TOTAL_VAULT_BALANCE,
        TRAPS, UNBONDING_CLAIMS, UNBOND_QUEUE,
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
    let pending_unbonds: StdResult<HashMap<Addr, (String, Unbond)>> = PENDING_UNBONDING_CLAIMS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|res| {
            let val = res?;
            Ok((val.0 .0, (val.0 .1, val.1)))
        })
        .collect();
    Ok(ListUnbondingClaimsResponse {
        unbonds: unbonds?,
        pending_unbonds: pending_unbonds?,
    })
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
            Ok((format!("{seq}-{chan}"), kind))
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
    let pending: StdResult<HashMap<String, IbcMsgKind>> = PENDING_ACK
        .range(deps.storage, None, None, Order::Ascending)
        .map(|res| {
            let ((seq, chan), kind) = res?;
            Ok((format!("{seq}-{chan}"), kind))
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
    use cosmwasm_std::{testing::{mock_dependencies, mock_env}, Timestamp};

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

        let _res = query(deps.as_ref(), env, q).unwrap();
    }
    #[test]
    fn test_handle_list_unbonding_claims() -> StdResult<()> {
        let mut deps = mock_dependencies();

        // Create some unbonding claims
        let user1 = Addr::unchecked("user1");
        let id1 = "1".to_string();
        let user2 = Addr::unchecked("user2");
        let id2 = "2".to_string();
        let unbonds = vec![
            (user1.clone(),(id1.clone(), Unbond {lp_shares: Uint128::new(100), unlock_time: Timestamp::from_seconds(101), attempted: false, owner: user1.clone(), id: id1 })),
            (user2.clone(), (id2.clone(), Unbond {lp_shares: Uint128::new(200), unlock_time: Timestamp::from_seconds(102), attempted: true, owner: user2.clone(), id: id2 })),
        ];
        for (addr, (id, unbond)) in unbonds.clone() {
            let key = (addr, id.clone());
            UNBONDING_CLAIMS.save(&mut deps.storage, key, &unbond)?;
        }

        let id3 = "3".to_string();
        let id4 = "4".to_string();
        // Create some pending unbonding claims
        let pending_unbonds = vec![
            (user1.clone(), (id3.clone(), Unbond {lp_shares:  Uint128::new(50), unlock_time:  Timestamp::from_seconds(103), attempted: false, owner: user1.clone(), id: id3 })),
            (user2.clone(), (id4.clone(), Unbond {lp_shares:  Uint128::new(150) , unlock_time:  Timestamp::from_seconds(104), attempted: true, owner: user2.clone(), id: id4 })),
        ];
        for (addr, (id, unbond)) in pending_unbonds.clone() {
            let key = (addr, id.clone());
            let value = unbond.clone();
            PENDING_UNBONDING_CLAIMS.save(&mut deps.storage, key, &value)?;
        }

        // Call the function and check the response
        let res = handle_list_unbonding_claims(deps.as_ref())?;

        // Check the unbonds
        let mut expected_unbonds = HashMap::new();
        for (addr, (denom, unbond)) in unbonds {
            expected_unbonds.insert(addr, (denom, unbond));
        }
        assert_eq!(res.unbonds, expected_unbonds);

        // Check the pending unbonds
        let mut expected_pending_unbonds = HashMap::new();
        for (addr, (id, unbond)) in pending_unbonds {
            expected_pending_unbonds.insert(addr, (id, unbond));
        }
        assert_eq!(res.pending_unbonds, expected_pending_unbonds);

        Ok(())
    }
}
