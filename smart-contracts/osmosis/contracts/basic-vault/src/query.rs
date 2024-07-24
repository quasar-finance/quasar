use cosmwasm_std::{Addr, Coin, Deps, StdResult};
use lp_strategy::msg::{ConfigResponse, IcaAddressResponse, LpSharesResponse, QueryMsg};

use crate::{
    execute::may_pay_with_ratio,
    msg::{
        DepositRatioResponse, InvestmentResponse, PendingBondsByIdResponse, PendingBondsResponse,
        PendingUnbondsByIdResponse, PendingUnbondsResponse, PrimitiveInfo, TvlInfoResponse,
    },
    state::{Unbond, BOND_STATE, INVESTMENT, PENDING_BOND_IDS, PENDING_UNBOND_IDS, UNBOND_STATE},
};

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

    let res = InvestmentResponse { info: invest };
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
    let pending_bond_ids = PENDING_BOND_IDS.may_load(deps.storage, Addr::unchecked(address))?;
    let mut pending_bonds = vec![];

    pending_bond_ids.clone().unwrap().iter().for_each(|id| {
        let mut deposit_stubs = BOND_STATE.load(deps.storage, id.to_string()).unwrap();

        pending_bonds.append(deposit_stubs.as_mut());
    });

    Ok(PendingBondsResponse {
        pending_bonds,
        pending_bond_ids: pending_bond_ids.unwrap(),
    })
}

pub fn query_pending_unbonds(deps: Deps, address: String) -> StdResult<PendingUnbondsResponse> {
    let pending_unbond_ids = PENDING_UNBOND_IDS.may_load(deps.storage, Addr::unchecked(address))?;
    let mut pending_unbonds: Vec<Unbond> = vec![];

    if pending_unbond_ids.is_none() {
        return Ok(PendingUnbondsResponse {
            pending_unbonds,
            pending_unbond_ids: vec![],
        });
    }

    pending_unbond_ids
        .clone()
        .unwrap()
        .iter()
        .for_each(|id: &String| {
            let unbond_stubs: Unbond = UNBOND_STATE.load(deps.storage, id.to_string()).unwrap();
            pending_unbonds.push(unbond_stubs);
        });

    Ok(PendingUnbondsResponse {
        pending_unbonds,
        pending_unbond_ids: pending_unbond_ids.unwrap(),
    })
}

pub fn query_pending_bonds_by_id(deps: Deps, id: String) -> StdResult<PendingBondsByIdResponse> {
    let deposit_stubs = BOND_STATE.load(deps.storage, id).unwrap();

    Ok(PendingBondsByIdResponse {
        pending_bonds: deposit_stubs,
    })
}

pub fn query_pending_unbonds_by_id(
    deps: Deps,
    id: String,
) -> StdResult<PendingUnbondsByIdResponse> {
    let unbond_stubs = UNBOND_STATE.load(deps.storage, id).unwrap();

    Ok(PendingUnbondsByIdResponse {
        pending_unbonds: unbond_stubs,
    })
}
