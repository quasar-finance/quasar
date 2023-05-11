use cosmwasm_std::{Addr, Empty, Env, QuerierWrapper, Storage};

use crate::{
    error::ContractError,
    helpers::is_contract_admin,
    state::{DEPOSITOR, LOCK_ADMIN},
};

// check if sender is the admin
pub fn check_depositor(storage: &mut dyn Storage, sender: &Addr) -> Result<bool, ContractError> {
    let depositor = DEPOSITOR.load(storage)?;
    Ok(&depositor == sender)
}

pub fn add_lock_admin(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: &Env,
    sender: Addr,
    to_add: Addr,
) -> Result<(), ContractError> {
    is_contract_admin(querier, env, &sender)?;
    LOCK_ADMIN.save(storage, &to_add, &Empty::default())?;
    Ok(())
}

pub fn is_lock_admin(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: &Env,
    sender: &Addr,
) -> Result<(), ContractError> {
    // if the may load is none, we check the contract admin status, if that errors, then the sender is neither contract
    // admin or a lock admin
    let lock = LOCK_ADMIN.may_load(storage, &sender)?;
    match lock {
        Some(_) => Ok(()),
        None => is_contract_admin(querier, env, &sender),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, MockQuerier},
        to_binary, ContractInfo, ContractInfoResponse, ContractResult, QuerierResult, WasmQuery,
    };

    #[test]
    fn test_lock_admin() {
        let mut deps = mock_dependencies();
        let admin = "bob";
        let admin1 = "alice";
        let admin2 = "eve";
        let env = mock_env();

        let mut info = ContractInfoResponse::default();
        info.admin = Some(admin.to_string());
        let mut q = MockQuerier::default();
        q.update_wasm(move |q: &WasmQuery| -> QuerierResult {
            match q {
                WasmQuery::ContractInfo { contract_addr: _ } => {
                    QuerierResult::Ok(ContractResult::Ok(to_binary(&info).unwrap()))
                }
                _ => unreachable!(),
            }
        });

        let querier: QuerierWrapper<Empty> = QuerierWrapper::new(&q);

        // add the extra admin
        add_lock_admin(
            deps.as_mut().storage,
            &querier,
            &env,
            Addr::unchecked(admin),
            Addr::unchecked(admin1),
        )
        .unwrap();
        is_lock_admin(
            deps.as_mut().storage,
            &querier,
            &env,
            &Addr::unchecked(admin),
        )
        .unwrap();
        is_lock_admin(
            deps.as_mut().storage,
            &querier,
            &env,
            &Addr::unchecked(admin1),
        )
        .unwrap();
        is_lock_admin(
            deps.as_mut().storage,
            &querier,
            &env,
            &Addr::unchecked(admin2),
        )
        .unwrap_err();
        add_lock_admin(
            deps.as_mut().storage,
            &querier,
            &env,
            Addr::unchecked(admin),
            Addr::unchecked(admin2),
        )
        .unwrap();
        is_lock_admin(
            deps.as_mut().storage,
            &querier,
            &env,
            &Addr::unchecked(admin2),
        )
        .unwrap();
    }

    #[test]
    fn test_admin() {
        let mut deps = mock_dependencies();
        let sender1 = Addr::unchecked("alice");
        let sender2 = Addr::unchecked("eve");

        DEPOSITOR.save(deps.as_mut().storage, &sender1).unwrap();
        assert!(check_depositor(deps.as_mut().storage, &sender1).unwrap());
        assert_eq!(check_depositor(deps.as_mut().storage, &sender1), Ok(true));
        assert_eq!(check_depositor(deps.as_mut().storage, &sender2), Ok(false));
    }
}
