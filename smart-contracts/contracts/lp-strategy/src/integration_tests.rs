#[cfg(test)]
mod tests {
    use cosmwasm_std::{attr, Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::{
        contract::{execute, instantiate},
        msg::{ExecuteMsg, InstantiateMsg, LockOnly, LockResponse, QueryMsg},
        queries::query,
    };

    #[test]
    fn test_execute_lock() {
        const ADMIN: &str = "admin";
        const CONTRACT_OWNER: &str = "contract_owner";

        // returns an object that can be used with cw-multi-test
        fn contract() -> Box<dyn Contract<Empty>> {
            let contract = ContractWrapper::new(execute, instantiate, query);
            Box::new(contract)
        }

        // an app object is the blockchain simulator. we send initial balance here too if we need
        let mut app = App::new(|_router, _api, _storage| {});

        // upload the contracts to the blockchain and get back code_id to instantiate the contract later
        let contract_code_id = app.store_code(contract());

        // create the instantiate message
        let instantiate_msg = InstantiateMsg {
            lock_period: 100,
            pool_id: 1,
            pool_denom: "gamm/pool/1".to_string(),
            base_denom: "uosmo".to_string(),
            quote_denom: "uqsr".to_string(),
            local_denom: "ibc/local_osmo".to_string(),
            transfer_channel: "channel-0".to_string(),
            return_source_channel: "channel-0".to_string(),
            expected_connection: "connection-0".to_string(),
        };

        // instantiate the contract
        let contract_addr = app
            .instantiate_contract(
                contract_code_id,
                Addr::unchecked(CONTRACT_OWNER),
                &instantiate_msg,
                &[],
                "lp-strategy",
                Some(ADMIN.to_owned()),
            )
            .unwrap();

        // lock the contract manually using the migration lock
        let execute_msg = ExecuteMsg::Lock {
            lock_only: LockOnly::Migration,
        };
        let res = app
            .execute_contract(
                Addr::unchecked(ADMIN),
                contract_addr.clone(),
                &execute_msg,
                &[],
            )
            .unwrap();

        assert_eq!(res.events[1].attributes[1], attr("lock_only", "migration"));

        // check the lock
        let query_msg = QueryMsg::Lock {};
        let res: LockResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &query_msg)
            .unwrap();

        // migration lock should be the only one locked
        assert!(res.lock.migration.is_locked());
        assert!(res.lock.bond.is_unlocked());
        assert!(res.lock.start_unbond.is_unlocked());
        assert!(res.lock.unbond.is_unlocked());
        assert!(res.lock.recovery.is_unlocked());

        // lock the contract manually using the bond lock
        let execute_msg = ExecuteMsg::Lock {
            lock_only: LockOnly::Bond,
        };
        let res = app
            .execute_contract(
                Addr::unchecked(ADMIN),
                contract_addr.clone(),
                &execute_msg,
                &[],
            )
            .unwrap();

        assert_eq!(res.events[1].attributes[1], attr("lock_only", "bond"));

        // check the lock
        let query_msg = QueryMsg::Lock {};
        let res: LockResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &query_msg)
            .unwrap();

        // bond & migration lock should be the only ones locked
        assert!(res.lock.migration.is_locked());
        assert!(res.lock.bond.is_locked());
        assert!(res.lock.start_unbond.is_unlocked());
        assert!(res.lock.unbond.is_unlocked());
        assert!(res.lock.recovery.is_unlocked());

        // lock the contract manually using the start unbond lock
        let execute_msg = ExecuteMsg::Lock {
            lock_only: LockOnly::StartUnbond,
        };
        let res = app
            .execute_contract(
                Addr::unchecked(ADMIN),
                contract_addr.clone(),
                &execute_msg,
                &[],
            )
            .unwrap();

        assert_eq!(
            res.events[1].attributes[1],
            attr("lock_only", "start_unbond")
        );

        // check the lock
        let query_msg = QueryMsg::Lock {};
        let res: LockResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &query_msg)
            .unwrap();

        // bond, start unbond, & migration lock should be the only ones locked
        assert!(res.lock.migration.is_locked());
        assert!(res.lock.bond.is_locked());
        assert!(res.lock.start_unbond.is_locked());
        assert!(res.lock.unbond.is_unlocked());
        assert!(res.lock.recovery.is_unlocked());

        // lock the contract manually using the unbond lock
        let execute_msg = ExecuteMsg::Lock {
            lock_only: LockOnly::Unbond,
        };
        let res = app
            .execute_contract(
                Addr::unchecked(ADMIN),
                contract_addr.clone(),
                &execute_msg,
                &[],
            )
            .unwrap();

        assert_eq!(res.events[1].attributes[1], attr("lock_only", "unbond"));

        // check the lock
        let query_msg = QueryMsg::Lock {};
        let res: LockResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &query_msg)
            .unwrap();

        // bond, start unbond, unbond, & migration lock should be the only ones locked
        assert!(res.lock.migration.is_locked());
        assert!(res.lock.bond.is_locked());
        assert!(res.lock.start_unbond.is_locked());
        assert!(res.lock.unbond.is_locked());
        assert!(res.lock.recovery.is_unlocked());
    }
}
