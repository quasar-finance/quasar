use std::collections::HashMap;

use cosmwasm_std::{Addr, Coin, Deps, Env, StdResult, Timestamp, Uint128};
use lp_strategy::msg::{ConfigResponse, IcaAddressResponse, LpSharesResponse, QueryMsg};

use crate::{
    execute::may_pay_with_ratio,
    msg::{
        DepositRatioResponse, InvestmentResponse, PendingBondsResponse, PrimitiveInfo,
        TvlInfoResponse, UnbondingClaimResponse,
    },
    state::{
        InvestmentInfo, BOND_STATE, INVESTMENT, PENDING_BOND_IDS, PENDING_UNBOND_IDS, UNBOND_STATE,
    },
};

pub fn query_unbonding_claims(
    deps: Deps,
    env: Env,
    addr: Addr,
) -> StdResult<UnbondingClaimResponse> {
    let ids = PENDING_UNBOND_IDS.load(deps.storage, addr)?;
    let mut resp = UnbondingClaimResponse {
        pending_unbonds: Uint128::zero(),
        unbonds: HashMap::new(),
        unbonded: Uint128::zero(),
    };
    for id in ids {
        let stub = UNBOND_STATE.load(deps.storage, id)?;
        let mut time = Timestamp::from_seconds(0);

        // find the largest time
        // TODO this should be done by an iter and remove clone
        for pending in stub.stub.clone() {
            if let Some(unlock) = pending.unlock_time {
                if unlock.seconds() >= time.seconds() {
                    time = unlock;
                }
            }
        }
        // if time is zero, all timestamps were None, so we don't have an unbonding time yet
        if time.seconds() == 0 {
            resp.pending_unbonds = resp.pending_unbonds.checked_add(stub.shares)?;
        }
        // if the current time is before the stub timestamp, funds are still unbondong
        else if env.block.time < time {
            // TODO make the addition save here
            resp.unbonds
                .entry(time.seconds())
                .and_modify(|old| *old += stub.shares)
                .or_insert(stub.shares);
        } else {
            resp.unbonded = resp.unbonded.checked_add(stub.shares)?;
        }
    }
    Ok(resp)
}

pub fn query_tvl_info(deps: Deps) -> StdResult<TvlInfoResponse> {
    let primitives = INVESTMENT.load(deps.storage)?.primitives;
    let mut prim_infos: Vec<PrimitiveInfo> = Vec::new();
    for prim in primitives {
        let addr = deps.api.addr_validate(prim.address.as_str())?;
        let ica = deps
            .querier
            .query_wasm_smart::<IcaAddressResponse>(addr.as_str(), &QueryMsg::IcaAddress {})?;
        let lp_shares = deps
            .querier
            .query_wasm_smart::<LpSharesResponse>(addr.as_str(), &QueryMsg::LpShares {})?
            .lp_shares;
        let config = deps
            .querier
            .query_wasm_smart::<ConfigResponse>(addr.as_str(), &QueryMsg::Config {})?
            .config;
        prim_infos.push(PrimitiveInfo {
            ica_address: ica.address,
            base_denom: config.base_denom,
            quote_denom: config.quote_denom,
            lp_denom: config.pool_denom,
            lp_shares,
        })
    }
    Ok(TvlInfoResponse {
        primitives: prim_infos,
    })
}

pub fn query_investment(deps: Deps) -> StdResult<InvestmentResponse> {
    let invest = INVESTMENT.load(deps.storage)?;

    let res = InvestmentResponse {
        info: InvestmentInfo {
            owner: invest.owner.clone(),
            min_withdrawal: invest.min_withdrawal,
            primitives: invest.primitives,
        },
    };
    Ok(res)
}

pub fn query_deposit_ratio(deps: Deps, funds: Vec<Coin>) -> StdResult<DepositRatioResponse> {
    let invest = INVESTMENT.load(deps.storage)?;

    let (primitive_funding_amounts, remainder) = may_pay_with_ratio(&deps, &funds, invest).unwrap();

    let res = DepositRatioResponse {
        primitive_funding_amounts,
        remainder,
    };
    Ok(res)
}

pub fn query_pending_bonds(deps: Deps, address: String) -> StdResult<PendingBondsResponse> {
    let pending_bond_ids = PENDING_BOND_IDS.load(deps.storage, Addr::unchecked(address))?;
    let mut pending_bonds = vec![];

    pending_bond_ids.iter().for_each(|id| {
        let mut deposit_stubs = BOND_STATE.load(deps.storage, id.to_string()).unwrap();

        pending_bonds.append(deposit_stubs.as_mut());
    });

    Ok(PendingBondsResponse {
        pending_bonds,
        pending_bond_ids,
    })
}
