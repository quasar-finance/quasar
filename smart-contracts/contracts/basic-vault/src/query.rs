use cosmwasm_std::{Addr, Coin, Deps, StdResult};

use crate::{
    execute::may_pay_with_ratio,
    msg::{DepositRatioResponse, InvestmentResponse, PendingBondsResponse},
    state::{BOND_STATE, INVESTMENT, PENDING_BOND_IDS},
};

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
