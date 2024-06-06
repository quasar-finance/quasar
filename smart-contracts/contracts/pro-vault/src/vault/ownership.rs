
use cosmwasm_std::{
    ensure, Addr, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, StdError,
};
use cw_controllers::Admin;
use cw_storage_plus::Item;
use cosmwasm_schema::cw_serde;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::vault::provault::Vault;
use crate::error::ContractError;
use crate::ownership;
use crate::vault::error::VaultError;
use crate::strategy::strategy::{Strategy, STRATEGY, StrategyKey};
use crate::ownership::ownership::{
    OwnerProposal, Ownership, OwnershipActions, query_owner, query_ownership_proposal,
    handle_claim_ownership, handle_ownership_proposal, handle_ownership_proposal_rejection
};

// Implement the ownership trait in the Vault struct so Vault struct operations 
// can be protected by choosen owner/admin.
impl Ownership for Vault {
    fn handle_ownership_proposal(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        proposed_owner: String,
        duration: u64,
        owner: &Admin,
        proposal: &Item<OwnerProposal>,
    ) -> Result<Response, ContractError> {
        handle_ownership_proposal(deps, info, env, proposed_owner, duration, owner, proposal)
    }

    fn handle_ownership_proposal_rejection(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        owner: &Admin,
        proposal: &Item<OwnerProposal>,
    ) -> Result<Response, ContractError> {
        handle_ownership_proposal_rejection(deps, info, owner, proposal)
    }

    fn handle_claim_ownership(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        owner: &Admin,
        proposal: &Item<OwnerProposal>,
    ) -> Result<Response, ContractError> {
        handle_claim_ownership(deps, info, env, owner, proposal)
    }

    fn query_ownership_proposal(
        &self,
        deps: Deps,
        proposal: &Item<OwnerProposal>,
    ) -> StdResult<OwnerProposal> {
        query_ownership_proposal(deps, proposal)
    }

    fn query_owner(&self, deps: Deps, owner: &Admin) -> StdResult<Option<Addr>> {
        query_owner(deps, owner)
    }
}
