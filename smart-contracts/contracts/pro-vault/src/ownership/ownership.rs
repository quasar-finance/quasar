// ownership.rs
// Taken from the vaultenator 
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult, Addr, StdError, ensure, ensure_eq, attr};
use cw_controllers::Admin;
use cw_storage_plus::Item;
use crate::error::ContractError;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::ownership::error::OwnershipError::{Std, 
    Unauthorized, ProposalAlreadyExists, 
    NoProposalExists, InvalidProposal};

pub const MAX_DURATION: u64 = 604800u64; // One week in seconds

#[cw_serde]
pub struct OwnerProposal {
    pub owner: Addr,
    pub expiry: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum OwnershipActions {
    ClaimOwnership {},
    ProposeNewOwner { new_owner: String, duration: u64 },
    RejectOwnershipProposal {},
}

pub trait Ownership {
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

pub fn handle_ownership_proposal(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    proposed_owner: String,
    duration: u64,
    owner: &Admin,
    proposal: &Item<OwnerProposal>,
) -> Result<Response, ContractError> {
    owner.assert_admin(deps.as_ref(), &info.sender)?;
    let proposed_owner = deps.api.addr_validate(&proposed_owner)?;

    ensure!(
        !owner.is_admin(deps.as_ref(), &proposed_owner)?,
        ContractError::InvalidOwnership {}
    );

    if MAX_DURATION < duration {
        return Err(ContractError::InvalidDuration(MAX_DURATION));
    }

    let expiry = env.block.time.seconds() + duration;

    proposal.save(
        deps.storage,
        &OwnerProposal {
            owner: proposed_owner.clone(),
            expiry,
        },
    )?;

    let proposal_event = Event::new("ownership_proposal")
    .add_attributes(vec![
        attr("action", "propose_new_owner"),
        attr("proposed_owner", proposed_owner.to_string()),
        attr("expiry", expiry.to_string()),
    ]);

    Ok(Response::new().add_event(proposal_event))
}

pub fn handle_ownership_proposal_rejection(
    deps: DepsMut,
    info: MessageInfo,
    owner: &Admin,
    proposal: &Item<OwnerProposal>,
) -> Result<Response, ContractError> {
    owner.assert_admin(deps.as_ref(), &info.sender)?;
    if proposal.may_load(deps.storage)?.is_none() {
        return Err(ContractError::ProposalNotFound {});
    }
    proposal.remove(deps.storage);
    let reject_proposal_event = Event::new("ownership_proposal_rejection")
        .add_attributes(vec![
            attr("action", "reject_ownership_proposal"),
            attr("sender", info.sender.to_string()),
    ]);
    Ok(Response::new().add_event(reject_proposal_event))
}

pub fn handle_claim_ownership(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    owner: &Admin,
    proposal: &Item<OwnerProposal>,
) -> Result<Response, ContractError> {
    let p = proposal
        .load(deps.storage)
        .map_err(|_| ContractError::ProposalNotFound {})?;

    ensure_eq!(p.owner, info.sender, ContractError::Unauthorized {});

    if env.block.time.seconds() > p.expiry {
        return Err(ContractError::Expired {});
    }

    let new_owner = p.owner;

    proposal.remove(deps.storage);

    owner.set(deps, Some(new_owner.clone()))?;

    let accept_proposal_event =
        Event::new("update_owner").add_attribute("new_owner", new_owner.to_string());

    Ok(Response::new().add_event(accept_proposal_event))
}

pub fn query_ownership_proposal(
    deps: Deps,
    proposal: &Item<OwnerProposal>,
) -> StdResult<OwnerProposal> {
    let res = proposal.load(deps.storage)?;
    Ok(res)
}

pub fn query_owner(deps: Deps, owner: &Admin) -> StdResult<Option<Addr>> {
    owner.get(deps).map_err(|e| StdError::generic_err(e.to_string()))
}
