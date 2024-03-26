use base64::{engine::general_purpose::STANDARD, Engine};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, BankMsg, Deps, DepsMut, Env, MessageInfo, Response, StdError, Uint128};

use crate::{
    state::{CLAIMED_INCENTIVES, CONFIG, INCENTIVES_ADMIN, MERKLE_ROOT},
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
    /// Cleanup all state items in the contract after the expiration date
    /// amount gives the max amount of state items to delete
    Cleanup { amount: Uint128 }
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
        AdminExecuteMsg::Cleanup { amount  } => todo!(),
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
    if env.block.height < config.expiration_block {
        return Err(ContractError::ExpirationHeightNotReached {});
    }

    let amount = deps.querier.query_all_balances(env.contract.address)?;

    let amount_attr = amount
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let bank_msg = BankMsg::Send {
        to_address: config.clawback_address.to_string(),
        amount,
    };

    Ok(Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "clawback")
        .add_attribute("amount", amount_attr))
}

fn execute_cleanup(deps: DepsMut, env: Env, amount: u64) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if env.block.height < config.expiration_block {
        return Err(ContractError::ExpirationHeightNotReached {});
    }

    // delete items from claimed incentives
    let to_delete: Result<Vec<(Addr, _)>, StdError> = CLAIMED_INCENTIVES.range(deps.storage, None, None, cosmwasm_std::Order::Descending).take(amount as usize).collect();
    for item  in to_delete? {
        CLAIMED_INCENTIVES.remove(deps.storage, item.0)
    }

    MERKLE_ROOT.remove(deps.storage);
    INCENTIVES_ADMIN.remove(deps.storage);

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{
        state::{Config, CONFIG, INCENTIVES_ADMIN, MERKLE_ROOT},
        ContractError,
    };
    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info},
        Addr, CosmosMsg,
    };

    use super::*;

    const MERKLE_ROOT_STRING: &str = "iGptCz22uFWoIxkwaqRzv5xV5DMnGz+hJntxP2YVsro=";
    const MERKLE_ROOT_STRING_INVALID: &str = "INVALIDiGptCz22uFWoIxkwaqRzv5xV5DMnGz+hJntxP2YVsro=";

    #[test]
    fn test_execute_clawback_works() {
        let mut deps = mock_dependencies_with_balance(&[coin(1000, "ugauge")]);
        let mut env = mock_env();

        // we mock a gauge of 100 blocks, where the creator can clawback after expiration of the gauge
        // in this example, 1000 out of the 10_000 tokens are unclaimed, and thus can be clawed back
        CONFIG
            .save(
                deps.as_mut().storage,
                &Config {
                    clawback_address: Addr::unchecked("bob"),
                    start_block: 1,
                    end_block: 100,
                    expiration_block: 1000,
                },
            )
            .unwrap();

        env.block.height = 1000;

        let resp = execute_clawback(deps.as_ref(), env).unwrap();

        assert_eq!(
            resp.messages[0].msg,
            CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
                to_address: "bob".to_string(),
                amount: vec![coin(1000, "ugauge")]
            })
        )
    }

    #[test]
    fn test_execute_clawback_block_height_too_low() {
        let mut deps = mock_dependencies_with_balance(&[coin(1000, "ugauge")]);
        let mut env = mock_env();

        // we mock a gauge of 100 blocks, where the creator can clawback after expiration of the gauge
        // in this example, 1000 out of the 10_000 tokens are unclaimed, and thus can be clawed back
        CONFIG
            .save(
                deps.as_mut().storage,
                &Config {
                    clawback_address: Addr::unchecked("bob"),
                    start_block: 1,
                    end_block: 100,
                    expiration_block: 1000,
                },
            )
            .unwrap();

        env.block.height = 999;

        let err = execute_clawback(deps.as_ref(), env).unwrap_err();

        assert_eq!(err, ContractError::ExpirationHeightNotReached {})
    }

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
