use cosmwasm_std::{Addr, Coin, Deps, Order, StdResult, Uint128};
use cw20_base::state::BALANCES;
use lp_strategy::msg::{ConfigResponse, IcaAddressResponse, LpSharesResponse, QueryMsg};

use crate::{
    execute::may_pay_with_ratio,
    msg::{
        ActiveUsersResponse, DepositRatioResponse, InvestmentResponse, PendingBondsByIdResponse,
        PendingBondsResponse, PendingUnbondsByIdResponse, PendingUnbondsResponse, PrimitiveInfo,
        TvlInfoResponse,
    },
    state::{Unbond, BOND_STATE, INVESTMENT, PENDING_BOND_IDS, PENDING_UNBOND_IDS, UNBOND_STATE},
};

pub fn query_tvl_info(deps: Deps) -> StdResult<TvlInfoResponse> {
    let primitives = INVESTMENT.load(deps.storage)?.primitives;
    let mut prim_infos: Vec<PrimitiveInfo> = Vec::new();
    for prim in primitives {
        let addr = deps.api.addr_validate(prim.address.as_str())?;
        let ica = deps
            .querier
            .query_wasm_smart::<IcaAddressResponse>(addr.as_str(), &QueryMsg::IcaAddress {})?;
        let lp_shares = deps
            .querier
            .query_wasm_smart::<LpSharesResponse>(addr.as_str(), &QueryMsg::LpShares {})?
            .lp_shares;
        let config = deps
            .querier
            .query_wasm_smart::<ConfigResponse>(addr.as_str(), &QueryMsg::Config {})?
            .config;
        prim_infos.push(PrimitiveInfo {
            ica_address: ica.address,
            base_denom: config.base_denom,
            quote_denom: config.quote_denom,
            lp_denom: config.pool_denom,
            lp_shares,
        })
    }
    Ok(TvlInfoResponse {
        primitives: prim_infos,
    })
}

pub fn query_investment(deps: Deps) -> StdResult<InvestmentResponse> {
    let invest = INVESTMENT.load(deps.storage)?;

    let res = InvestmentResponse { info: invest };
    Ok(res)
}

pub fn query_deposit_ratio(deps: Deps, funds: Vec<Coin>) -> StdResult<DepositRatioResponse> {
    let invest = INVESTMENT.load(deps.storage)?;

    let (primitive_funding_amounts, remainder) = may_pay_with_ratio(&deps, &funds, invest).unwrap();

    let res = DepositRatioResponse {
        primitive_funding_amounts,
        remainder,
    };
    Ok(res)
}

pub fn query_pending_bonds(deps: Deps, address: String) -> StdResult<PendingBondsResponse> {
    let pending_bond_ids = PENDING_BOND_IDS.may_load(deps.storage, Addr::unchecked(address))?;
    let mut pending_bonds = vec![];

    pending_bond_ids.clone().unwrap().iter().for_each(|id| {
        let mut deposit_stubs = BOND_STATE.load(deps.storage, id.to_string()).unwrap();

        pending_bonds.append(deposit_stubs.as_mut());
    });

    Ok(PendingBondsResponse {
        pending_bonds,
        pending_bond_ids: pending_bond_ids.unwrap(),
    })
}

pub fn query_pending_unbonds(deps: Deps, address: String) -> StdResult<PendingUnbondsResponse> {
    let pending_unbond_ids = PENDING_UNBOND_IDS.may_load(deps.storage, Addr::unchecked(address))?;
    let mut pending_unbonds: Vec<Unbond> = vec![];

    if pending_unbond_ids.is_none() {
        return Ok(PendingUnbondsResponse {
            pending_unbonds,
            pending_unbond_ids: vec![],
        });
    }

    pending_unbond_ids
        .clone()
        .unwrap()
        .iter()
        .for_each(|id: &String| {
            let unbond_stubs: Unbond = UNBOND_STATE.load(deps.storage, id.to_string()).unwrap();
            pending_unbonds.push(unbond_stubs);
        });

    Ok(PendingUnbondsResponse {
        pending_unbonds,
        pending_unbond_ids: pending_unbond_ids.unwrap(),
    })
}

pub fn query_pending_bonds_by_id(deps: Deps, id: String) -> StdResult<PendingBondsByIdResponse> {
    let deposit_stubs = BOND_STATE.load(deps.storage, id).unwrap();

    Ok(PendingBondsByIdResponse {
        pending_bonds: deposit_stubs,
    })
}

pub fn query_pending_unbonds_by_id(
    deps: Deps,
    id: String,
) -> StdResult<PendingUnbondsByIdResponse> {
    let unbond_stubs = UNBOND_STATE.load(deps.storage, id).unwrap();

    Ok(PendingUnbondsByIdResponse {
        pending_unbonds: unbond_stubs,
    })
}

pub fn query_active_users(deps: Deps) -> StdResult<ActiveUsersResponse> {
    let mut addresses: Vec<Addr> = vec![];
    let mut balances: Vec<Uint128> = vec![];

    for res in BALANCES.range(deps.storage, None, None, Order::Ascending) {
        addresses.push(res.as_ref().unwrap().0.clone());
        balances.push(res.as_ref().unwrap().1);
    }

    Ok(ActiveUsersResponse {
        addresses,
        balances,
    })
}

#[cfg(test)]
mod tests {
    use crate::msg::ActiveUsersResponse;
    use crate::query::query_active_users;
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::{Addr, StdResult, Uint128};
    use cw20_base::state::BALANCES;

    #[test]
    fn test_query_active_users_no_data() {
        // Create a mock dependencies object
        let deps = mock_dependencies();

        // No data in the BALANCES map
        let result: StdResult<ActiveUsersResponse> = query_active_users(deps.as_ref());

        // Ensure the query executed successfully and the response is empty
        assert!(result.is_ok(), "Query should return a successful result");

        // Get the response
        let response = result.unwrap();

        // Check that the response is empty
        assert_eq!(response.addresses.len(), 0, "Expected no addresses");
        assert_eq!(response.balances.len(), 0, "Expected no balances");
    }

    #[test]
    fn test_query_active_users_with_zero_balance() {
        let mut deps = mock_dependencies();

        // Add addresses with zero balance and non-zero balances
        let addr1 = Addr::unchecked("address1");
        let addr2 = Addr::unchecked("address2");
        let addr3 = Addr::unchecked("address3");

        BALANCES
            .save(&mut deps.storage, &addr1, &Uint128::from(0u128))
            .unwrap(); // Zero balance
        BALANCES
            .save(&mut deps.storage, &addr2, &Uint128::from(200u128))
            .unwrap(); // Non-zero balance
        BALANCES
            .save(&mut deps.storage, &addr3, &Uint128::from(300u128))
            .unwrap(); // Non-zero balance

        // Query the active users
        let result: StdResult<ActiveUsersResponse> = query_active_users(deps.as_ref());

        assert!(result.is_ok(), "Query should return a successful result");

        let response = result.unwrap();

        // Ensure that zero balance is returned properly
        assert_eq!(
            response.addresses,
            vec![addr1, addr2, addr3],
            "Expected addresses did not match"
        );
        assert_eq!(
            response.balances,
            vec![
                Uint128::from(0u128),
                Uint128::from(200u128),
                Uint128::from(300u128)
            ],
            "Expected balances did not match"
        );
    }

    #[test]
    fn test_query_active_users_disordered() {
        let mut deps = mock_dependencies();

        // Add addresses with disordered balances
        let addr1 = Addr::unchecked("address3");
        let addr2 = Addr::unchecked("address2");
        let addr3 = Addr::unchecked("address1");

        BALANCES
            .save(&mut deps.storage, &addr1, &Uint128::from(300u128))
            .unwrap(); // 300
        BALANCES
            .save(&mut deps.storage, &addr2, &Uint128::from(200u128))
            .unwrap(); // 200
        BALANCES
            .save(&mut deps.storage, &addr3, &Uint128::from(100u128))
            .unwrap(); // 100

        // Query the active users
        let result: StdResult<ActiveUsersResponse> = query_active_users(deps.as_ref());

        assert!(result.is_ok(), "Query should return a successful result");

        let response = result.unwrap();

        // Even if the balances were inserted in disordered form, the query output should be sorted
        assert_eq!(
            response.addresses,
            vec![addr3, addr2, addr1],
            "Expected addresses to be in ascending order"
        );
        assert_eq!(
            response.balances,
            vec![
                Uint128::from(100u128),
                Uint128::from(200u128),
                Uint128::from(300u128)
            ],
            "Expected balances to be in ascending order"
        );
    }
}
