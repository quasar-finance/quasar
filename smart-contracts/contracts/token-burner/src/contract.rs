use cosmwasm_std::{
    entry_point, to_json_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult,
};
use cw2::set_contract_version;
use cw20_base::msg::MigrateMsg;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::query_total_burn;
use crate::state::AMOUNT_BURNT;
use crate::BurnErrors;

// version info for migration info
const CONTRACT_NAME: &str = "token-burner";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, BurnErrors> {
    // Set the contract version in storage
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Return a response
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, BurnErrors> {
    match msg {
        ExecuteMsg::Burn {} => execute_burn(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TotalBurnt {} => to_json_binary(&query_total_burn(deps)?),
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, BurnErrors> {
    Ok(Response::new().add_attribute("migrate", "successful"))
}

pub fn execute_burn(deps: DepsMut, info: MessageInfo) -> Result<Response, BurnErrors> {
    if info.funds.is_empty() {
        return Err(BurnErrors::ZeroAmount {});
    }

    // Prepare the Burn message
    let burn_msg = CosmosMsg::Bank(BankMsg::Burn {
        amount: info.clone().funds,
    });

    for fund in &info.funds {
        let denom = &fund.denom;
        let amount = &fund.amount;
        let mut total_burn_amount = AMOUNT_BURNT
            .may_load(deps.storage, denom.clone())?
            .unwrap_or_default();
        total_burn_amount += amount;
        AMOUNT_BURNT.save(deps.storage, denom.clone(), &total_burn_amount)?;
    }

    // Return a response with the burn message
    Ok(Response::default().add_message(burn_msg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_info};
    use cosmwasm_std::{coin, BankMsg, CosmosMsg, SubMsg, Uint128};

    #[test]
    fn test_execute_coins_burn() {
        // Arrange
        let mut deps = mock_dependencies();
        let info = mock_info("sender", &[coin(100, "denom1"), coin(50, "denom2")]);

        // Act
        let res = execute_burn(deps.as_mut(), info.clone()).unwrap();

        // Assert
        // Ensure the response contains a BankMsg::Burn message
        assert_eq!(
            res.messages,
            vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Burn {
                amount: vec![coin(100, "denom1"), coin(50, "denom2")]
            }))]
        );

        // Ensure the total burn amount is updated correctly in storage
        let total_burn_amount = AMOUNT_BURNT
            .load(deps.as_ref().storage, "denom1".to_string())
            .unwrap_or_default();
        assert_eq!(total_burn_amount, Uint128::new(100),);
        let total_burn_amount = AMOUNT_BURNT
            .load(deps.as_ref().storage, "denom2".to_string())
            .unwrap_or_default();
        assert_eq!(total_burn_amount, Uint128::new(50),);

        // Additional test scenario: test when there's already existing burn amounts
        let info = mock_info("sender", &[coin(200, "denom1"), coin(30, "denom3")]);
        let res = execute_burn(deps.as_mut(), info.clone()).unwrap();
        assert_eq!(
            res.messages,
            vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Burn {
                amount: vec![coin(200, "denom1"), coin(30, "denom3")]
            }))]
        );

        // Ensure the total burn amount is updated correctly in storage
        let total_burn_amount = AMOUNT_BURNT
            .load(deps.as_ref().storage, "denom1".to_string())
            .unwrap_or_default();
        assert_eq!(total_burn_amount, Uint128::new(300),);

        let total_burn_amount = AMOUNT_BURNT
            .load(deps.as_ref().storage, "denom2".to_string())
            .unwrap_or_default();
        assert_eq!(total_burn_amount, Uint128::new(50),);

        let total_burn_amount = AMOUNT_BURNT
            .load(deps.as_ref().storage, "denom3".to_string())
            .unwrap_or_default();
        assert_eq!(total_burn_amount, Uint128::new(30),);
    }
}
