use cosmwasm_std::{StdResult, Deps, Coin};

use crate::{msg::{InvestmentResponse, DepositRatioResponse}, state::INVESTMENT, execute::may_pay_with_ratio};

pub fn query_investment(deps: Deps) -> StdResult<InvestmentResponse> {
    let invest = INVESTMENT.load(deps.storage)?;

    let res = InvestmentResponse {
        owner: invest.owner.to_string(),
        min_withdrawal: invest.min_withdrawal,
        primitives: invest.primitives,
    };
    Ok(res)
}


pub fn query_deposit_ratio(deps: Deps, funds:Vec<Coin>) -> StdResult<DepositRatioResponse> {
    let invest = INVESTMENT.load(deps.storage)?;

    let (primitive_funding_amounts, remainder) =
        may_pay_with_ratio(&deps, &funds, &invest.primitives).unwrap();

    let res = DepositRatioResponse {
        primitive_funding_amounts,
        remainder
    };
    Ok(res)
}