use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response, Deps};
use cw_utils::nonpayable;
use crate::{msg::AdminExtensionExecuteMsg, ContractError};
use crate::state::{ADMIN_ADDRESS, VAULT_CONFIG, Config};

pub(crate) fn execute_admin(
    deps: DepsMut,
    info: MessageInfo,
    admin_msg: AdminExtensionExecuteMsg,
) -> Result<Response, ContractError> {
    match admin_msg {
        AdminExtensionExecuteMsg::UpdateAdmin { address } => execute_update_admin(deps, info, address),
        AdminExtensionExecuteMsg::UpdateConfig { updates } => execute_update_config(deps, info, updates),
    }
}

/// Updates the admin of the contract.
///
/// This function first checks if the message sender is nonpayable. If the sender sent funds, a `ContractError::NonPayable` error is returned.
/// Then, it checks if the message sender is the current admin. If not, a `ContractError::Unauthorized` error is returned.
/// If both checks pass, it saves the new admin address in the state.
///
/// # Parameters
///
/// - `deps`: A mutable reference to the contract's dependencies.
/// - `info`: The information about the calling message.
/// - `address`: The address of the new admin.
///
/// # Returns
///
/// - `Ok(Response)` - If the admin was successfully updated. The response contains the appropriate attributes.
/// - `Err(ContractError)` - If the function failed to update the admin due to an error.
pub fn execute_update_admin(
    deps: DepsMut,
    info: MessageInfo,
    address: String
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(|_| ContractError::NonPayable {})?;

    let previous_admin = assert_admin(deps.as_ref(), &info.sender)?;
    let new_admin = deps.api.addr_validate(&address)?;
    ADMIN_ADDRESS.save(deps.storage, &new_admin)?;

    Ok(Response::new()
        .add_attribute("action", "execute_update_admin")
        .add_attribute(
            "previous_admin",
            previous_admin,
        )
        .add_attribute("new_admin", &new_admin))
}

/// Updates the configuration of the contract.
///
/// This function first checks if the message sender is nonpayable. If the sender sent funds, a `ContractError::NonPayable` error is returned.
/// Then, it checks if the message sender is the current admin. If not, a `ContractError::Unauthorized` error is returned.
/// If both checks pass, it saves the new configuration in the state.
///
/// # Parameters
///
/// - `deps`: A mutable reference to the contract's dependencies.
/// - `info`: The information about the calling message.
/// - `updates`: The new configuration.
///
/// # Returns
///
/// - `Ok(Response)` - If the configuration was successfully updated. The response contains the appropriate attributes.
/// - `Err(ContractError)` - If the function failed to update the configuration due to an error.
pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    updates: Config
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(|_| ContractError::NonPayable {})?;

    assert_admin(deps.as_ref(), &info.sender)?;

    VAULT_CONFIG.save(deps.storage, &updates)?;

    Ok(Response::default()
        .add_attribute("action", "execute_update_config")
        .add_attribute("updates", &format!("{:?}", updates)))
}

/// Helper function for a streamlined admin authentication check.
///
/// This function compares the address of the message sender (caller) with the current admin 
/// address stored in the state. This provides a convenient way to verify if the caller 
/// is the admin in a single line.
///
/// # Returns
///
/// - `Ok(Addr)` - If the caller is the admin. The returned `Addr` is the address of the admin.
/// - `Err(ContractError)` - If the caller is not the admin. The error variant will be `ContractError::Unauthorized`.
pub fn assert_admin(
    deps: Deps,
    caller: &Addr,
) -> Result<Addr, ContractError> {
    if !(ADMIN_ADDRESS.load(deps.storage)? == caller) {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(caller.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_info},
        Decimal, Uint128, coin,
    };

    #[test]
    fn test_execute_update_admin_success() {
        let old_admin = Addr::unchecked("old_admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &old_admin).unwrap();

        let new_admin = Addr::unchecked("new_admin");
        let info_admin: MessageInfo = mock_info("old_admin", &vec![]);

        execute_update_admin(deps.as_mut(), info_admin, new_admin.to_string()).unwrap();
        assert_eq!(ADMIN_ADDRESS.load(&deps.storage).unwrap(), new_admin);
    }

    #[test]
    fn test_execute_update_admin_not_admin() {
        let old_admin = Addr::unchecked("old_admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &old_admin).unwrap();

        let new_admin = Addr::unchecked("new_admin");
        let info_not_admin = mock_info("not_admin", &vec![]);

        execute_update_admin(deps.as_mut(), info_not_admin, new_admin.to_string()).unwrap_err();
        assert_eq!(ADMIN_ADDRESS.load(&deps.storage).unwrap(), old_admin);
    }

    #[test]
    fn test_execute_update_admin_with_funds() {
        let old_admin = Addr::unchecked("old_admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &old_admin).unwrap();

        let new_admin = Addr::unchecked("new_admin");

        let info_admin_with_funds = mock_info("old_admin", &vec![coin(1, "token")]);

        let result = execute_update_admin(deps.as_mut(), info_admin_with_funds, new_admin.to_string());
        assert!(result.is_err(), "Expected Err, but got: {:?}", result);
    }

    #[test]
    fn test_execute_update_admin_same_admin() {
        let old_admin = Addr::unchecked("old_admin");
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &old_admin).unwrap();

        let info_admin: MessageInfo = mock_info("old_admin", &vec![]);

        let res = execute_update_admin(deps.as_mut(), info_admin, old_admin.to_string());
        assert!(res.is_ok());
        assert_eq!(ADMIN_ADDRESS.load(&deps.storage).unwrap(), old_admin);
    }

    #[test]
    fn test_execute_update_config_success() {
        let admin = Addr::unchecked("admin");
        let old_config = Config{
            treasury: Addr::unchecked("old_treasury"),
            performance_fee: Decimal::new(Uint128::from(100u128))
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        VAULT_CONFIG.save(deps.as_mut().storage, &old_config).unwrap();

        let new_config = Config{
            treasury: Addr::unchecked("new_treasury"),
            performance_fee: Decimal::new(Uint128::from(200u128))
        };
        let info_admin: MessageInfo = mock_info("admin", &vec![]);

        assert!(execute_update_config(deps.as_mut(), info_admin, new_config.clone()).is_ok());
        assert_eq!(VAULT_CONFIG.load(deps.as_mut().storage).unwrap(), new_config);
    }

    #[test]
    fn test_execute_update_config_not_admin() {
        let admin = Addr::unchecked("admin");
        let old_config = Config{
            treasury: Addr::unchecked("old_treasury"),
            performance_fee: Decimal::new(Uint128::from(100u128))
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        VAULT_CONFIG.save(deps.as_mut().storage, &old_config).unwrap();

        let new_config = Config{
            treasury: Addr::unchecked("new_treasury"),
            performance_fee: Decimal::new(Uint128::from(200u128))
        };
        let info_not_admin = mock_info("not_admin", &vec![]);

        assert!(execute_update_config(deps.as_mut(), info_not_admin, new_config.clone()).is_err());
        assert_eq!(VAULT_CONFIG.load(deps.as_mut().storage).unwrap(), old_config);
    }

    #[test]
    fn test_execute_update_config_with_funds() {
        let admin = Addr::unchecked("admin");
        let old_config = Config{
            treasury: Addr::unchecked("old_treasury"),
            performance_fee: Decimal::new(Uint128::from(100u128))
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        VAULT_CONFIG.save(deps.as_mut().storage, &old_config).unwrap();

        let new_config = Config{
            treasury: Addr::unchecked("new_treasury"),
            performance_fee: Decimal::new(Uint128::from(200u128))
        };

        let info_admin_with_funds = mock_info("admin", &vec![coin(1, "token")]);

        let result = execute_update_config(deps.as_mut(), info_admin_with_funds, new_config);
        assert!(result.is_err(), "Expected Err, but got: {:?}", result);
    }


    #[test]
    fn test_execute_update_config_same_config() {
        let admin = Addr::unchecked("admin");
        let old_config = Config{
            treasury: Addr::unchecked("old_treasury"),
            performance_fee: Decimal::new(Uint128::from(100u128))
        };
        let mut deps = mock_dependencies();
        ADMIN_ADDRESS.save(deps.as_mut().storage, &admin).unwrap();
        VAULT_CONFIG.save(deps.as_mut().storage, &old_config).unwrap();

        let info_admin: MessageInfo = mock_info("admin", &vec![]);

        let res = execute_update_config(deps.as_mut(), info_admin, old_config.clone());
        assert!(res.is_ok());
        assert_eq!(VAULT_CONFIG.load(deps.as_mut().storage).unwrap(), old_config);
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