use std::collections::HashMap;

use cosmwasm_std::{Deps, StdResult, Addr, Order, StdError, Uint128, Coin};
use quasar_types::ibc::ChannelInfo;

use crate::{msg::{ListUnbondingClaimsResponse, UnbondingClaimResponse, TrappedErrorsResponse, ChannelsResponse, ConfigResponse, LpSharesResponse, IcaAddressResponse, IcaChannelResponse, PrimitiveSharesResponse, IcaBalanceResponse, LockResponse}, state::{UNBONDING_CLAIMS, Unbond, TRAPS, CHANNELS, LP_SHARES, CONFIG, ICA_CHANNEL, ICA_BALANCE, IBC_LOCK}, error::Trap, helpers::{get_ica_address, get_total_shares}};

pub fn handle_list_unbonding_claims(deps: Deps) -> StdResult<ListUnbondingClaimsResponse> {
    let unbonds: StdResult<HashMap<(Addr, String), Unbond>> = UNBONDING_CLAIMS
        .range(deps.storage, None, None, Order::Ascending)
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
    let total = get_total_shares(deps.storage).map_err(|err| StdError::GenericErr {
        msg: err.to_string(),
    })?;

    if total.is_zero() {
        Ok(PrimitiveSharesResponse {
            total: Uint128::one(),
        })
    } else {
        Ok(PrimitiveSharesResponse { total })
    }
}

pub fn handle_ica_balance(deps: Deps) -> StdResult<IcaBalanceResponse> {
    let balance = ICA_BALANCE.load(deps.storage)?;
    let amount = if balance.is_zero() {
        Uint128::one()
    } else {
        balance
    };

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
