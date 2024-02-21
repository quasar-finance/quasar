use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{
    state::{INCENTIVES_ADMIN, MERKLE_ROOT},
    ContractError,
};

use super::helpers::{is_contract_admin, is_incentives_admin};

#[cw_serde]
pub enum AdminExecuteMsg {
    /// Update the range executor admin.
    UpdateAdmin { new_admin: String },
    /// Update the range submitter admin.
    UpdateMerkleRoot { new_root: String },
}

pub fn execute_admin_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: AdminExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        AdminExecuteMsg::UpdateAdmin { new_admin } => {
            update_incentives_admin(deps, env, info, new_admin)
        }
        AdminExecuteMsg::UpdateMerkleRoot { new_root } => {
            update_merkle_root(deps, env, info, new_root)
        }
    }
}

pub fn update_incentives_admin(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response, ContractError> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;

    let address_validated = deps.api.addr_validate(&new_admin)?;
    INCENTIVES_ADMIN.save(deps.storage, &address_validated)?;

    Ok(Response::default())
}

pub fn update_merkle_root(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_root: String,
) -> Result<Response, ContractError> {
    is_incentives_admin(deps.as_ref(), &info.sender)?;

    // Validate new merkle_root
    match base64::decode(new_root.clone()) {
        Ok(f) => f,
        Err(e) => {
            return Err(ContractError::FailedToDecodeRoot {
                root: e.to_string(),
            })
        }
    };

    MERKLE_ROOT.save(deps.storage, &new_root)?;

    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::update_merkle_root;
    use crate::{
        state::{INCENTIVES_ADMIN, MERKLE_ROOT},
        ContractError,
    };
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr,
    };

    const MERKLE_ROOT_STRING: &str = "iGptCz22uFWoIxkwaqRzv5xV5DMnGz+hJntxP2YVsro=";
    const MERKLE_ROOT_STRING_INVALID: &str = "INVALIDiGptCz22uFWoIxkwaqRzv5xV5DMnGz+hJntxP2YVsro=";

    #[test]
    fn test_update_merkle_root_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("admin", &vec![]);

        // Set incentives admin
        INCENTIVES_ADMIN
            .save(&mut deps.storage, &Addr::unchecked("admin"))
            .unwrap();

        // Assert before
        let merkle_root = MERKLE_ROOT.may_load(&deps.storage).unwrap();
        assert_eq!(merkle_root, None);

        update_merkle_root(deps.as_mut(), env, info, MERKLE_ROOT_STRING.to_string()).unwrap();

        // Assert after
        let merkle_root = MERKLE_ROOT.load(&deps.storage).unwrap();
        assert_eq!(merkle_root, MERKLE_ROOT_STRING.to_string());
    }

    #[test]
    fn test_update_merkle_root_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("admin", &vec![]);

        // Set incentives admin
        INCENTIVES_ADMIN
            .save(&mut deps.storage, &Addr::unchecked("admin"))
            .unwrap();

        // Assert before
        let merkle_root = MERKLE_ROOT.may_load(&deps.storage).unwrap();
        assert_eq!(merkle_root, None);

        let error = update_merkle_root(
            deps.as_mut(),
            env,
            info,
            MERKLE_ROOT_STRING_INVALID.to_string(),
        )
        .unwrap_err();

        // Assert error
        assert!(
            matches!(error, ContractError::FailedToDecodeRoot { root } if root.contains("Invalid padding"))
        );
    }
}
