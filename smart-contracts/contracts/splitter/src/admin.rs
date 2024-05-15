use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response};

use crate::{
    msg::AdminMsg,
    state::{Receiver, Receivers, ADMIN, RECEIVERS},
    ContractError,
};

pub fn execute_admin(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: AdminMsg,
) -> Result<Response, ContractError> {
    assert_admin(deps.as_ref(), env, info)?;
    match msg {
        AdminMsg::UpdateReceivers { new } => update_receivers(deps, new),
        AdminMsg::UpdateAdmin { new } => update_admin(deps, &new),
    }
}

/// Overwrite the old receivers with a complete new set of receivers
pub fn update_receivers(deps: DepsMut, new: Vec<Receiver>) -> Result<Response, ContractError> {
    let recv: Receivers = new.try_into()?;
    RECEIVERS.save(deps.storage, &recv)?;

    Ok(Response::new()
        .add_attribute("action", "update_receivers")
        .add_attribute("new", recv.to_string()))
}

/// check whether the given admin address is either the admin inside the contract state or the cosmwasm admin of the contract
fn assert_admin(deps: Deps, env: Env, info: MessageInfo) -> Result<(), ContractError> {
    if ADMIN.load(deps.storage)? == info.sender {
        Ok(())
    } else if let Some(contract_admin) = deps
        .querier
        .query_wasm_contract_info(env.contract.address)?
        .admin
    {
        if contract_admin == info.sender {
            Ok(())
        } else {
            Err(ContractError::Unauthorized {})
        }
    } else {
        Err(ContractError::Unauthorized {})
    }
}

/// Overwrite the old admin with a new admin address
pub fn update_admin(deps: DepsMut, new: &str) -> Result<Response, ContractError> {
    let new_admin = deps.api.addr_validate(new)?;

    ADMIN.save(deps.storage, &new_admin)?;

    Ok(Response::new().add_attribute("new_admin", new_admin))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{testing::mock_dependencies, Addr, Decimal};

    use crate::{state::{Receiver, ADMIN, RECEIVERS}, ContractError};

    use super::{update_admin, update_receivers};

    #[test]
    fn update_admin_works() {
        let mut deps = mock_dependencies();
        ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked("old_admin"))
            .unwrap();

        let new = "new_admin";
        update_admin(deps.as_mut(), new).unwrap();

        assert_eq!(ADMIN.load(deps.as_mut().storage).unwrap(), new)
    }

    #[test]
    fn update_receivers_works() {
        let mut deps = mock_dependencies();
        ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked("old_admin"))
            .unwrap();
        RECEIVERS
            .save(
                deps.as_mut().storage,
                &vec![
                    Receiver {
                        address: Addr::unchecked("alice"),
                        share: Decimal::from_str("0.6").unwrap(),
                    },
                    Receiver {
                        address: Addr::unchecked("bob"),
                        share: Decimal::from_str("0.4").unwrap(),
                    },
                ]
                .try_into()
                .unwrap(),
            )
            .unwrap();

            let new_receivers = vec![
                Receiver {
                    address: Addr::unchecked("alice"),
                    share: Decimal::from_str("0.5").unwrap(),
                },
                Receiver {
                    address: Addr::unchecked("bob"),
                    share: Decimal::from_str("0.5").unwrap(),
                },
            ];

        update_receivers(deps.as_mut(), new_receivers.clone()).unwrap();
        assert_eq!(RECEIVERS.load(deps.as_mut().storage).unwrap(), new_receivers.try_into().unwrap())
    }

    #[test]
    fn update_receivers_fails_on_bad_total() {
        let mut deps = mock_dependencies();
        ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked("old_admin"))
            .unwrap();
        RECEIVERS
            .save(
                deps.as_mut().storage,
                &vec![
                    Receiver {
                        address: Addr::unchecked("alice"),
                        share: Decimal::from_str("0.6").unwrap(),
                    },
                    Receiver {
                        address: Addr::unchecked("bob"),
                        share: Decimal::from_str("0.4").unwrap(),
                    },
                ]
                .try_into()
                .unwrap(),
            )
            .unwrap();

            let new_receivers = vec![
                Receiver {
                    address: Addr::unchecked("alice"),
                    share: Decimal::from_str("0.5").unwrap(),
                },
                Receiver {
                    address: Addr::unchecked("bob"),
                    share: Decimal::from_str("0.4").unwrap(),
                },
            ];

        let err = update_receivers(deps.as_mut(), new_receivers.clone()).unwrap_err();
        assert_eq!(err, ContractError::IncorrectReceivers)
    }
}
