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
const CONTRACT_NAME: &str = "quasar_airdrop";
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

    // Initialize the AMOUNT_BURNT storage with an empty vector
    AMOUNT_BURNT.save(deps.storage, &vec![])?;

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
        ExecuteMsg::Burn {} => execute_coins_burn(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TotalBurntQuery {} => to_json_binary(&query_total_burn(deps)?),
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, BurnErrors> {
    Ok(Response::new().add_attribute("migrate", "successful"))
}

pub fn execute_coins_burn(deps: DepsMut, info: MessageInfo) -> Result<Response, BurnErrors> {
    // Prepare the Burn message
    let burn_msg = CosmosMsg::Bank(BankMsg::Burn {
        amount: info.clone().funds,
    });

    // Load the total burn amount from storage
    let mut total_burn_amount = AMOUNT_BURNT.load(deps.storage)?;

    // Iterate over the coins to update the total burn amount
    for c in info.clone().funds {
        if let Some(c2) = total_burn_amount.iter_mut().find(|c2| c.denom == c2.denom) {
            c2.amount += c.amount;
        } else {
            // If the coin denom doesn't exist in total_burn_amount, add it
            total_burn_amount.push(c.clone()); // Clone the Coin to avoid borrowing issues
        }
    }

    // Save the updated total burn amount to storage
    AMOUNT_BURNT.save(deps.storage, &total_burn_amount)?;

    // Return a response with the burn message
    Ok(Response::default().add_message(burn_msg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_info};
    use cosmwasm_std::{coin, BankMsg, Coin, CosmosMsg, SubMsg};

    #[test]
    fn test_execute_coins_burn() {
        // Arrange
        let mut deps = mock_dependencies();
        let info = mock_info("sender", &[coin(100, "denom1"), coin(50, "denom2")]);

        AMOUNT_BURNT.save(deps.as_mut().storage, &vec![]).unwrap();

        // Act
        let res = execute_coins_burn(deps.as_mut(), info.clone()).unwrap();

        // Assert
        // Ensure the response contains a BankMsg::Burn message
        assert_eq!(
            res.messages,
            vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Burn {
                amount: vec![coin(100, "denom1"), coin(50, "denom2")]
            }))]
        );

        // Ensure the total burn amount is updated correctly in storage
        let total_burn_amount: Vec<Coin> =
            AMOUNT_BURNT.load(deps.as_ref().storage).unwrap_or_default();
        assert_eq!(
            total_burn_amount,
            vec![coin(100, "denom1"), coin(50, "denom2")]
        );

        // Additional test scenario: test when there's already existing burn amounts
        let info = mock_info("sender", &[coin(200, "denom1"), coin(30, "denom3")]);
        let res = execute_coins_burn(deps.as_mut(), info.clone()).unwrap();
        assert_eq!(
            res.messages,
            vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Burn {
                amount: vec![coin(200, "denom1"), coin(30, "denom3")]
            }))]
        );

        // Ensure the total burn amount is updated correctly in storage
        let total_burn_amount: Vec<Coin> =
            AMOUNT_BURNT.load(deps.as_ref().storage).unwrap_or_default();
        assert_eq!(
            total_burn_amount,
            vec![
                coin(300, "denom1"), // 100 + 200
                coin(50, "denom2"),
                coin(30, "denom3"),
            ]
        );
    }
}
