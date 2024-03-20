use base64::{engine::general_purpose::STANDARD, Engine};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{BankMsg, Deps, DepsMut, Env, MessageInfo, Response};

use crate::{
    state::{CONFIG, INCENTIVES_ADMIN, MERKLE_ROOT},
    ContractError,
};

use super::helpers::{is_contract_admin, is_incentives_admin};

#[cw_serde]
pub enum AdminExecuteMsg {
    /// Update the range executor admin.
    UpdateAdmin { new_admin: String },
    /// Update the range submitter admin.
    UpdateMerkleRoot { new_root: String },
    /// Clawback any remaining funds after expiration date
    Clawback {},
    /// Claim any already accumulates fees
    ClaimFees {},
}

pub fn handle_execute_admin(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: AdminExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        AdminExecuteMsg::UpdateAdmin { new_admin } => {
            execute_update_incentives_admin(deps, env, info, new_admin)
        }
        AdminExecuteMsg::UpdateMerkleRoot { new_root } => {
            execute_update_merkle_root(deps, env, info, new_root)
        }
        AdminExecuteMsg::Clawback {} => execute_clawback(deps.as_ref(), env),
        AdminExecuteMsg::ClaimFees {} => execute_claim_fees(deps.as_ref(), env),
    }
}

pub fn execute_update_incentives_admin(
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

pub fn execute_update_merkle_root(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_root: String,
) -> Result<Response, ContractError> {
    is_incentives_admin(deps.as_ref(), &info.sender)?;

    // Validate new merkle_root
    match STANDARD.decode(new_root.clone()) {
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

pub fn execute_clawback(deps: Deps, env: Env) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if config.expiration_block < env.block.height {
        return Err(ContractError::ExpirationHeightNotReached {});
    }

    let amount = deps.querier.query_all_balances(env.contract.address)?;

    let amount_attr = amount.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(",");
    let bank_msg = BankMsg::Send {
        to_address: config.clawback_address.to_string(),
        amount,
    };

    Ok(Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "clawback")
        .add_attribute("amount", amount_attr)
    )
}
#[cfg(test)]
mod tests {
    use crate::{
        admin::execute::execute_update_merkle_root,
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

        execute_update_merkle_root(deps.as_mut(), env, info, MERKLE_ROOT_STRING.to_string())
            .unwrap();

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

        let error = execute_update_merkle_root(
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
