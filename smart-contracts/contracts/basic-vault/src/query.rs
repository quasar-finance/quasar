use cosmwasm_std::{StdResult, Deps};

use crate::{msg::InvestmentResponse};

pub fn query_investment(_deps: Deps) -> StdResult<InvestmentResponse> {
    todo!()
    // let invest = INVESTMENT.load(deps.storage)?;
    // let supply = TOTAL_SUPPLY.load(deps.storage)?;

    // let res = InvestmentResponse {
    //     owner: invest.owner.to_string(),
    //     min_withdrawal: invest.min_withdrawal,
    //     token_supply: supply.issued,
    //     bonded_tokens: coin(supply.bonded.u128(), &invest.bond_denom),
    //     nominal_value: if supply.issued.is_zero() {
    //         FALLBACK_RATIO
    //     } else {
    //         Decimal::from_ratio(supply.bonded, supply.issued)
    //     },
    //     primitives: invest.primitives,
    // };
    // Ok(res)
}
