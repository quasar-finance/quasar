use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

use crate::state::MERKLE_ROOT;

use super::{helpers::is_valid_claim, CoinVec};

#[cw_serde]
#[derive(QueryResponses)]
pub enum IncentivesQueryMsg {
    // Get the pending ranges
    #[returns(Option<String>)]
    GetMerkleRoot {},
    // Get the pending ranges for a specific contract
    #[returns(bool)]
    IsValidClaim {
        address: String,
        coins: CoinVec,
        proof_hashes: Vec<[u8; 32]>,
        leaf_index: usize,
        total_leaves_count: usize,
    },
}

pub fn handle_query_incentives(
    deps: Deps,
    _env: Env,
    query_msg: IncentivesQueryMsg,
) -> StdResult<Binary> {
    match query_msg {
        IncentivesQueryMsg::GetMerkleRoot {} => query_merkle_root(deps),
        IncentivesQueryMsg::IsValidClaim {
            address,
            coins,
            proof_hashes,
            leaf_index,
            total_leaves_count,
        } => query_is_valid_claim(
            deps,
            address,
            coins,
            proof_hashes,
            leaf_index,
            total_leaves_count,
        ),
    }
}

pub fn query_merkle_root(deps: Deps) -> StdResult<Binary> {
    let merkle_root = MERKLE_ROOT.may_load(deps.storage)?;

    to_json_binary(&merkle_root)
}

pub fn query_is_valid_claim(
    deps: Deps,
    address: String,
    coins: CoinVec,
    proof_hashes: Vec<[u8; 32]>,
    leaf_index: usize,
    total_leaves_count: usize,
) -> StdResult<Binary> {
    let address_validated = deps.api.addr_validate(&address)?;
    match is_valid_claim(
        deps,
        &address_validated,
        &coins,
        proof_hashes,
        leaf_index,
        total_leaves_count,
    ) {
        Ok(_coins) => to_json_binary(&true),
        Err(_err) => to_json_binary(&false),
    }
}

#[cfg(test)]
mod tests {
    use crate::incentives::query::query_merkle_root;
    use crate::{
        admin::execute::execute_update_merkle_root,
        state::{INCENTIVES_ADMIN, MERKLE_ROOT},
    };
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr,
    };

    const MERKLE_ROOT_STRING: &str = "iGptCz22uFWoIxkwaqRzv5xV5DMnGz+hJntxP2YVsro=";

    #[test]
    fn test_query_merkle_root() {
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

        let merkle_root = query_merkle_root(deps.as_ref()).unwrap();
        println!("{:?}", merkle_root);
    }
}
