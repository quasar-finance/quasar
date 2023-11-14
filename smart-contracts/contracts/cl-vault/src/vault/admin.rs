use crate::error::ContractResult;
use crate::helpers::{assert_admin, sort_tokens};
use crate::rewards::CoinList;
use crate::state::{VaultConfig, ADMIN_ADDRESS, RANGE_ADMIN, STRATEGIST_REWARDS, VAULT_CONFIG, Metadata, METADATA};
use crate::{msg::AdminExtensionExecuteMsg, ContractError};
use cosmwasm_std::{BankMsg, DepsMut, MessageInfo, Response};
use cw_utils::nonpayable;

pub(crate) fn execute_admin(
    deps: DepsMut,
    info: MessageInfo,
    admin_msg: AdminExtensionExecuteMsg,
) -> Result<Response, ContractError> {
    match admin_msg {
        AdminExtensionExecuteMsg::UpdateAdmin { address } => {
            execute_update_admin(deps, info, address)
        }
        AdminExtensionExecuteMsg::UpdateConfig { updates } => {
            execute_update_config(deps, info, updates)
        }
        AdminExtensionExecuteMsg::UpdateMetadata { updates } => {
            execute_update_metadata(deps, info, updates)
        }
        AdminExtensionExecuteMsg::UpdateRangeAdmin { address } => {
            execute_update_range_admin(deps, info, address)
        }
        AdminExtensionExecuteMsg::ClaimStrategistRewards {} => {
            execute_claim_strategist_rewards(deps, info)
        }
    }
}

pub fn execute_claim_strategist_rewards(
    deps: DepsMut,
    info: MessageInfo,
) -> ContractResult<Response> {
    let range_admin = RANGE_ADMIN.load(deps.storage)?;
    if info.sender != range_admin {
        return Err(ContractError::Unauthorized {});
    }

    // get the currently attained rewards
    let rewards = STRATEGIST_REWARDS.load(deps.storage)?;
    // empty the saved rewards
    STRATEGIST_REWARDS.save(deps.storage, &CoinList::new())?;

    Ok(Response::new()
        .add_attribute("rewards", format!("{:?}", rewards.coins()))
        .add_message(BankMsg::Send {
            to_address: range_admin.to_string(),
            amount: sort_tokens(rewards.coins()),
        }))
}

/// Updates the admin of the contract.
///
/// This function first checks if the message sender is nonpayable. If the sender sent funds, a `ContractError::NonPayable` error is returned.
/// Then, it checks if the message sender is the current admin. If not, a `ContractError::Unauthorized` error is returned.
/// If both checks pass, it saves the new admin address in the state.
pub fn execute_update_admin(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(|_| ContractError::NonPayable {})?;

    let previous_admin = assert_admin(deps.as_ref(), &info.sender)?;
    let new_admin = deps.api.addr_validate(&address)?;
    ADMIN_ADDRESS.save(deps.storage, &new_admin)?;

    Ok(Response::new()
        .add_attribute("action", "execute_update_admin")
        .add_attribute("previous_admin", previous_admin)
        .add_attribute("new_admin", &new_admin))
}

/// Updates the range admin of the contract.
///
/// This function first checks if the message sender is nonpayable. If the sender sent funds, a `ContractError::NonPayable` error is returned.
/// Then, it checks if the message sender is the current admin. If not, a `ContractError::Unauthorized` error is returned.
/// If both checks pass, it saves the new range admin address in the state.
pub fn execute_update_range_admin(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(|_| ContractError::NonPayable {})?;
    assert_admin(deps.as_ref(), &info.sender)?;

    let previous_admin = RANGE_ADMIN.load(deps.storage)?;
    let new_admin = deps.api.addr_validate(&address)?;
    RANGE_ADMIN.save(deps.storage, &new_admin)?;

    Ok(Response::new()
        .add_attribute("action", "execute_update_admin")
        .add_attribute("previous_admin", previous_admin)
        .add_attribute("new_admin", &new_admin))
}

/// Updates the configuration of the contract.
///
/// This function first checks if the message sender is nonpayable. If the sender sent funds, a `ContractError::NonPayable` error is returned.
/// Then, it checks if the message sender is the current admin. If not, a `ContractError::Unauthorized` error is returned.
/// If both checks pass, it saves the new configuration in the state.
pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    updates: VaultConfig,
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(|_| ContractError::NonPayable {})?;
    assert_admin(deps.as_ref(), &info.sender)?;

    VAULT_CONFIG.save(deps.storage, &updates)?;

    Ok(Response::default()
        .add_attribute("action", "execute_update_config")
        .add_attribute("updates", format!("{:?}", updates)))
}

pub fn execute_update_metadata(
    deps: DepsMut,
    info: MessageInfo,
    updates: Metadata,
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(|_| ContractError::NonPayable {})?;
    assert_admin(deps.as_ref(), &info.sender)?;

    METADATA.save(deps.storage, &updates)?;

    Ok(Response::default()
        .add_attribute("action", "execute_update_metadata")
        .add_attribute("updates", format!("{:?}", updates)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_info},
        Addr, CosmosMsg, Decimal, Uint128,
    };

    #[test]
    fn test_execute_claim_strategist_rewards_success() {
        let range_admin = Addr::unchecked("bob");
        let mut deps = mock_dependencies();
        let rewards = vec![coin(12304151, "uosmo"), coin(5415123, "uatom")];
        STRATEGIST_REWARDS
            .save(
                deps.as_mut().storage,
                &CoinList::from_coins(rewards.clone()),
            )
            .unwrap();

        RANGE_ADMIN
            .save(deps.as_mut().storage, &range_admin)
            .unwrap();

        let response =
            execute_claim_strategist_rewards(deps.as_mut(), mock_info(range_admin.as_str(), &[]))
                .unwrap();
        assert_eq!(
            CosmosMsg::Bank(BankMsg::Send {
                to_address: range_admin.to_string(),
                amount: sort_tokens(rewards)
            }),
            response.messages[0].msg
        )
    }

    #[test]
    fn test_execute_claim_strategist_rewards_not_admin() {
        let range_admin = Addr::unchecked("bob");
        let mut deps = mock_dependencies();
        let rewards = vec![coin(12304151, "uosmo"), coin(5415123, "uatom")];
        STRATEGIST_REWARDS
            .save(deps.as_mut().storage, &CoinList::from_coins(rewards))
            .unwrap();

        RANGE_ADMIN
            .save(deps.as_mut().storage, &range_admin)
            .unwrap();

        let err =
            execute_claim_strategist_rewards(deps.as_mut(), mock_info("alice", &[])).unwrap_err();
        assert_eq!(ContractError::Unauthorized {}, err)
    }

    #[test]
    fn test_execute_update_admin_success() {
        let old_admin = Addr::unchecked("old_admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS
            .save(deps.as_mut().storage, &old_admin)
            .unwrap();

        let new_admin = Addr::unchecked("new_admin");
        let info_admin: MessageInfo = mock_info("old_admin", &[]);

        execute_update_admin(deps.as_mut(), info_admin, new_admin.to_string()).unwrap();
        assert_eq!(ADMIN_ADDRESS.load(&deps.storage).unwrap(), new_admin);
    }

    #[test]
    fn test_execute_update_admin_not_admin() {
        let old_admin = Addr::unchecked("old_admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS
            .save(deps.as_mut().storage, &old_admin)
            .unwrap();

        let new_admin = Addr::unchecked("new_admin");
        let info_not_admin = mock_info("not_admin", &[]);

        execute_update_admin(deps.as_mut(), info_not_admin, new_admin.to_string()).unwrap_err();
        assert_eq!(ADMIN_ADDRESS.load(&deps.storage).unwrap(), old_admin);
    }

    #[test]
    fn test_execute_update_admin_with_funds() {
        let old_admin = Addr::unchecked("old_admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS
            .save(deps.as_mut().storage, &old_admin)
            .unwrap();

        let new_admin = Addr::unchecked("new_admin");

        let info_admin_with_funds = mock_info("old_admin", &[coin(1, "token")]);

        let result =
            execute_update_admin(deps.as_mut(), info_admin_with_funds, new_admin.to_string());
        assert!(result.is_err(), "Expected Err, but got: {:?}", result);
    }

    #[test]
    fn test_execute_update_admin_same_admin() {
        let old_admin = Addr::unchecked("old_admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS
            .save(deps.as_mut().storage, &old_admin)
            .unwrap();

        let info_admin: MessageInfo = mock_info("old_admin", &[]);

        let res = execute_update_admin(deps.as_mut(), info_admin, old_admin.to_string());
        assert!(res.is_ok());
        assert_eq!(ADMIN_ADDRESS.load(&deps.storage).unwrap(), old_admin);
    }

    #[test]
    fn test_execute_update_range_admin_success() {
        let admin = Addr::unchecked("admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();

        let old_range_admin = Addr::unchecked("rang_admin1");
        RANGE_ADMIN
            .save(deps.as_mut().storage, &old_range_admin)
            .unwrap();
        let new_range_admin = Addr::unchecked("rang_admin2");
        let info_admin: MessageInfo = mock_info("admin", &[]);

        execute_update_range_admin(deps.as_mut(), info_admin, new_range_admin.to_string()).unwrap();
        assert_eq!(RANGE_ADMIN.load(&deps.storage).unwrap(), new_range_admin);
    }

    #[test]
    fn test_execute_update_range_admin_not_admin() {
        let admin = Addr::unchecked("admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();

        let old_range_admin = Addr::unchecked("rang_admin1");
        RANGE_ADMIN
            .save(deps.as_mut().storage, &old_range_admin)
            .unwrap();
        let new_range_admin = Addr::unchecked("rang_admin2");
        let info_not_admin = mock_info("not_admin", &[]);

        execute_update_range_admin(deps.as_mut(), info_not_admin, new_range_admin.to_string())
            .unwrap_err();
        assert_eq!(RANGE_ADMIN.load(&deps.storage).unwrap(), old_range_admin);
    }

    #[test]
    fn test_execute_update_range_admin_with_funds() {
        let admin = Addr::unchecked("admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();

        let old_range_admin = Addr::unchecked("rang_admin1");
        RANGE_ADMIN
            .save(deps.as_mut().storage, &old_range_admin)
            .unwrap();
        let new_range_admin = Addr::unchecked("rang_admin2");

        let info_admin_with_funds = mock_info(admin.as_str(), &[coin(1, "token")]);

        let result = execute_update_range_admin(
            deps.as_mut(),
            info_admin_with_funds,
            new_range_admin.to_string(),
        );
        assert!(result.is_err(), "Expected Err, but got: {:?}", result);
    }

    #[test]
    fn test_execute_update_range_admin_same_admin() {
        let admin = Addr::unchecked("admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();

        let old_range_admin = Addr::unchecked("rang_admin1");
        RANGE_ADMIN
            .save(deps.as_mut().storage, &old_range_admin)
            .unwrap();
        let new_range_admin = Addr::unchecked("rang_admin1");

        let info_admin = mock_info(admin.as_str(), &[]);

        let res =
            execute_update_range_admin(deps.as_mut(), info_admin, new_range_admin.to_string());
        assert!(res.is_ok());
        assert_eq!(RANGE_ADMIN.load(&deps.storage).unwrap(), old_range_admin);
    }

    #[test]
    fn test_execute_update_config_success() {
        let admin = Addr::unchecked("admin");
        let old_config = VaultConfig {
            treasury: Addr::unchecked("old_treasury"),
            performance_fee: Decimal::new(Uint128::from(100u128)),
            swap_max_slippage: Decimal::from_ratio(1u128, 100u128),
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        VAULT_CONFIG
            .save(deps.as_mut().storage, &old_config)
            .unwrap();

        let new_config = VaultConfig {
            treasury: Addr::unchecked("new_treasury"),
            performance_fee: Decimal::new(Uint128::from(200u128)),
            swap_max_slippage: Decimal::from_ratio(1u128, 100u128),
        };
        let info_admin: MessageInfo = mock_info("admin", &[]);

        assert!(execute_update_config(deps.as_mut(), info_admin, new_config.clone()).is_ok());
        assert_eq!(
            VAULT_CONFIG.load(deps.as_mut().storage).unwrap(),
            new_config
        );
    }

    #[test]
    fn test_execute_update_config_not_admin() {
        let admin = Addr::unchecked("admin");
        let old_config = VaultConfig {
            treasury: Addr::unchecked("old_treasury"),
            performance_fee: Decimal::new(Uint128::from(100u128)),
            swap_max_slippage: Decimal::from_ratio(1u128, 100u128),
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        VAULT_CONFIG
            .save(deps.as_mut().storage, &old_config)
            .unwrap();

        let new_config = VaultConfig {
            treasury: Addr::unchecked("new_treasury"),
            performance_fee: Decimal::new(Uint128::from(200u128)),
            swap_max_slippage: Decimal::from_ratio(1u128, 100u128),
        };
        let info_not_admin = mock_info("not_admin", &[]);

        assert!(execute_update_config(deps.as_mut(), info_not_admin, new_config).is_err());
        assert_eq!(
            VAULT_CONFIG.load(deps.as_mut().storage).unwrap(),
            old_config
        );
    }

    #[test]
    fn test_execute_update_config_with_funds() {
        let admin = Addr::unchecked("admin");
        let old_config = VaultConfig {
            treasury: Addr::unchecked("old_treasury"),
            performance_fee: Decimal::new(Uint128::from(100u128)),
            swap_max_slippage: Decimal::from_ratio(1u128, 100u128),
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        VAULT_CONFIG
            .save(deps.as_mut().storage, &old_config)
            .unwrap();

        let new_config = VaultConfig {
            treasury: Addr::unchecked("new_treasury"),
            performance_fee: Decimal::new(Uint128::from(200u128)),
            swap_max_slippage: Decimal::from_ratio(1u128, 100u128),
        };

        let info_admin_with_funds = mock_info("admin", &[coin(1, "token")]);

        let result = execute_update_config(deps.as_mut(), info_admin_with_funds, new_config);
        assert!(result.is_err(), "Expected Err, but got: {:?}", result);
    }

    #[test]
    fn test_execute_update_config_same_config() {
        let admin = Addr::unchecked("admin");
        let old_config = VaultConfig {
            treasury: Addr::unchecked("old_treasury"),
            performance_fee: Decimal::new(Uint128::from(100u128)),
            swap_max_slippage: Decimal::from_ratio(1u128, 100u128),
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        VAULT_CONFIG
            .save(deps.as_mut().storage, &old_config)
            .unwrap();

        let info_admin: MessageInfo = mock_info("admin", &[]);

        let res = execute_update_config(deps.as_mut(), info_admin, old_config.clone());
        assert!(res.is_ok());
        assert_eq!(
            VAULT_CONFIG.load(deps.as_mut().storage).unwrap(),
            old_config
        );
    }

    #[test]
    fn test_execute_update_metadata_success() {
        let admin = Addr::unchecked("admin");
        let old_metadata = Metadata {
            name: "old_name".to_string(),
            thesis: "old_thesis".to_string(),
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        METADATA
            .save(deps.as_mut().storage, &old_metadata)
            .unwrap();

        let new_metadata = Metadata {
            name: "new_name".to_string(),
            thesis: "new_thesis".to_string(),
        };
        let info_admin: MessageInfo = mock_info("admin", &[]);

        assert!(execute_update_metadata(deps.as_mut(), info_admin, new_metadata.clone()).is_ok());
        assert_eq!(
            METADATA.load(deps.as_mut().storage).unwrap(),
            new_metadata
        );
    }

    #[test]
    fn test_execute_update_metadata_not_admin() {
        let admin = Addr::unchecked("admin");
        let old_metadata = Metadata {
            name: "old_name".to_string(),
            thesis: "old_thesis".to_string(),
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        METADATA
            .save(deps.as_mut().storage, &old_metadata)
            .unwrap();

        let new_metadata = Metadata {
            name: "new_name".to_string(),
            thesis: "new_thesis".to_string(),
        };
        let info_not_admin = mock_info("not_admin", &[]);

        assert!(execute_update_metadata(deps.as_mut(), info_not_admin, new_metadata).is_err());
        assert_eq!(
            METADATA.load(deps.as_mut().storage).unwrap(),
            old_metadata
        );
    }

    #[test]
    fn test_execute_update_metadata_with_funds() {
        let admin = Addr::unchecked("admin");
        let old_metadata = Metadata {
            name: "old_name".to_string(),
            thesis: "old_thesis".to_string(),
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        METADATA
            .save(deps.as_mut().storage, &old_metadata)
            .unwrap();

        let new_metadata = Metadata {
            name: "new_name".to_string(),
            thesis: "new_thesis".to_string(),
        };

        let info_admin_with_funds = mock_info("admin", &[coin(1, "token")]);

        let result = execute_update_metadata(deps.as_mut(), info_admin_with_funds, new_metadata);
        assert!(result.is_err(), "Expected Err, but got: {:?}", result);
    }

    #[test]
    fn test_execute_update_metadata_same_metadata() {
        let admin = Addr::unchecked("admin");
        let old_metadata = Metadata {
            name: "old_name".to_string(),
            thesis: "old_thesis".to_string(),
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        METADATA
            .save(deps.as_mut().storage, &old_metadata)
            .unwrap();

        let info_admin: MessageInfo = mock_info("admin", &[]);

        let res = execute_update_metadata(deps.as_mut(), info_admin, old_metadata.clone());
        assert!(res.is_ok());
        assert_eq!(
            METADATA.load(deps.as_mut().storage).unwrap(),
            old_metadata
        );
    }

    #[test]
    fn test_assert_admin() {
        let mut deps = mock_dependencies();
        let admin = Addr::unchecked("admin");
        let not_admin = Addr::unchecked("not_admin");

        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        assert!(assert_admin(deps.as_ref(), &admin).is_ok());
        assert!(assert_admin(deps.as_ref(), &not_admin).is_err());
    }
}
