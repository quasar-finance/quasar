use cosmwasm_std::{Addr, Coin, Deps, StdResult};
use lp_strategy::msg::{ConfigResponse, IcaAddressResponse, LpSharesResponse, QueryMsg};

use crate::{
    execute::may_pay_with_ratio,
    msg::{
        DepositRatioResponse, InvestmentResponse, PendingBondsResponse, PrimitiveInfo,
        TvlInfoResponse,
    },
    state::{BOND_STATE, INVESTMENT, PENDING_BOND_IDS},
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

    let res = InvestmentResponse {
        owner: invest.owner.to_string(),
        min_withdrawal: invest.min_withdrawal,
        primitives: invest.primitives,
    };
    Ok(res)
}

pub fn query_deposit_ratio(deps: Deps, funds: Vec<Coin>) -> StdResult<DepositRatioResponse> {
    let invest = INVESTMENT.load(deps.storage)?;

    let (primitive_funding_amounts, remainder) =
        may_pay_with_ratio(&deps, &funds, &invest.primitives).unwrap();

    let res = DepositRatioResponse {
        primitive_funding_amounts,
        remainder,
    };
    Ok(res)
}

pub fn query_pending_bonds(deps: Deps, address: String) -> StdResult<PendingBondsResponse> {
    let pending_bond_ids = PENDING_BOND_IDS.load(deps.storage, Addr::unchecked(address.clone()))?;
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
