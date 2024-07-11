use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockStorage, MockQuerier};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, OwnedDeps, StdError};
use cw_controllers::{Admin, AdminError};
use cw_storage_plus::Item;
use crate::ownership::ownership::{
    handle_claim_ownership, handle_ownership_proposal, handle_ownership_proposal_rejection,
    query_owner, query_ownership_proposal, OwnerProposal, MAX_DURATION,
};
use crate::error::ContractError;

const ADMIN: &str = "admin";
const NEW_OWNER: &str = "new_owner";
const INVALID_OWNER: &str = "invalid_owner";
const EXPIRED_DURATION: u64 = 1;
const VALID_DURATION: u64 = 600;

struct TestEnv<'a> {
    env: Env,
    info: MessageInfo,
    admin: Admin<'a>,
    proposal: Item<'a, OwnerProposal>,
}

fn setup_test_env<'a>(mut deps: DepsMut<'a>, sender: &str) -> (TestEnv<'a>, DepsMut<'a>) {
    let env = mock_env();
    let info = mock_info(sender, &[]);
    let admin = setup_contract(deps.branch());
    let proposal = Item::new("proposal");

    (
        TestEnv {
            env,
            info,
            admin,
            proposal,
        },
        deps,
    )
}

fn setup_contract(deps: DepsMut) -> Admin<'static> {
    let admin = Admin::new("admin");
    admin.set(deps, Some(Addr::unchecked(ADMIN))).unwrap();
    admin
}

#[test]
fn test_propose_new_owner_valid() {
    // Test proposing a new owner with a valid duration

    // Arrange
    let mut deps = mock_dependencies();
    let (mut test_env, mut deps) = setup_test_env(deps.as_mut(), ADMIN);

    // Act
    let res = handle_ownership_proposal(
        deps.branch(),
        test_env.info.clone(),
        test_env.env.clone(),
        NEW_OWNER.to_string(),
        VALID_DURATION,
        &test_env.admin,
        &test_env.proposal,
    );

    // Assert
    assert!(res.is_ok());

    // Verify event
    let event = &res.unwrap().events[0];
    assert_eq!(event.ty, "ownership_proposal");

    // Verify state using query
    let stored_proposal = query_ownership_proposal(deps.as_ref(), &test_env.proposal).unwrap();
    assert_eq!(stored_proposal.owner, Addr::unchecked(NEW_OWNER));
    assert_eq!(stored_proposal.expiry, test_env.env.block.time.seconds() + VALID_DURATION);
}

#[test]
fn test_propose_new_owner_invalid_duration() {
    // Test proposing a new owner with an invalid duration exceeding MAX_DURATION

    // Arrange
    let mut deps = mock_dependencies();
    let (mut test_env, mut deps) = setup_test_env(deps.as_mut(), ADMIN);

    // Act
    let res = handle_ownership_proposal(
        deps.branch(),
        test_env.info.clone(),
        test_env.env.clone(),
        NEW_OWNER.to_string(),
        MAX_DURATION + 1,
        &test_env.admin,
        &test_env.proposal,
    );

    // Assert
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), ContractError::InvalidDuration(MAX_DURATION));

    // Verify state using query - should return a not found error since the proposal was not stored
    let stored_proposal_err = query_ownership_proposal(deps.as_ref(), &test_env.proposal).unwrap_err();
    match stored_proposal_err {
        StdError::NotFound { .. } => (), // Expected error, test passes
        _ => panic!("Expected StdError::NotFound, but got a different error"),
    }
}

#[test]
fn test_propose_new_owner_unauthorized() {
    // Test proposing a new owner by an unauthorized sender

    // Arrange
    let mut deps = mock_dependencies();
    let (mut test_env, mut deps) = setup_test_env(deps.as_mut(), INVALID_OWNER);

    // Act
    let res = handle_ownership_proposal(
        deps.branch(),
        test_env.info.clone(),
        test_env.env.clone(),
        NEW_OWNER.to_string(),
        VALID_DURATION,
        &test_env.admin,
        &test_env.proposal,
    );

    // Assert
    assert!(res.is_err());
    // assert_eq!(res.unwrap_err(), ContractError::Unauthorized {});
    assert_eq!(res.unwrap_err(), ContractError::AdminError(AdminError::NotAdmin {}));

    // Verify state using query - should return a not found error since the proposal was not stored
    let stored_proposal_err = query_ownership_proposal(deps.as_ref(), &test_env.proposal).unwrap_err();
    match stored_proposal_err {
        StdError::NotFound { .. } => (), // Expected error, test passes
        _ => panic!("Expected StdError::NotFound, but got a different error"),
    }
}

#[test]
fn test_propose_new_owner_as_current_owner() {
    // Test proposing a new owner who is already the current owner

    // Arrange
    let mut deps = mock_dependencies();
    let (mut test_env, mut deps) = setup_test_env(deps.as_mut(), ADMIN);

    // Act
    let res = handle_ownership_proposal(
        deps.branch(),
        test_env.info.clone(),
        test_env.env.clone(),
        ADMIN.to_string(),
        VALID_DURATION,
        &test_env.admin,
        &test_env.proposal,
    );

    // Assert
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), ContractError::InvalidOwnership {});

    // Verify state using query - should return a not found error since the proposal was not stored
    let stored_proposal_err = query_ownership_proposal(deps.as_ref(), &test_env.proposal).unwrap_err();
    match stored_proposal_err {
        StdError::NotFound { .. } => (), // Expected error, test passes
        _ => panic!("Expected StdError::NotFound, but got a different error"),
    }
}

#[test]
fn test_reject_ownership_proposal_valid() {
    // Test rejecting a valid ownership proposal

    // Arrange
    let mut deps = mock_dependencies();
    let (mut test_env, mut deps) = setup_test_env(deps.as_mut(), ADMIN);
    test_env.proposal
        .save(
            deps.storage,
            &OwnerProposal {
                owner: Addr::unchecked(NEW_OWNER),
                expiry: test_env.env.block.time.seconds() + VALID_DURATION,
            },
        )
        .unwrap();

    // Act
    let res = handle_ownership_proposal_rejection(deps.branch(), test_env.info.clone(), &test_env.admin, &test_env.proposal);

    // Assert
    assert!(res.is_ok());

    // Verify event
    let event = &res.unwrap().events[0];
    assert_eq!(event.ty, "ownership_proposal_rejection");

    // Verify state using query - should return a not found error since the proposal was removed
    let stored_proposal_err = query_ownership_proposal(deps.as_ref(), &test_env.proposal).unwrap_err();
    match stored_proposal_err {
        StdError::NotFound { .. } => (), // Expected error, test passes
        _ => panic!("Expected StdError::NotFound, but got a different error"),
    }
}

#[test]
fn test_reject_nonexistent_proposal() {
    // Test rejecting a proposal that does not exist

    // Arrange
    let mut deps = mock_dependencies();
    let (mut test_env, mut deps) = setup_test_env(deps.as_mut(), ADMIN);

    // Act
    let res = handle_ownership_proposal_rejection(deps.branch(), test_env.info.clone(), &test_env.admin, &test_env.proposal);

    // Assert
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), ContractError::ProposalNotFound {});

    // Verify state using query - should return a not found error since no proposal was ever created
    let stored_proposal_err = query_ownership_proposal(deps.as_ref(), &test_env.proposal).unwrap_err();
    match stored_proposal_err {
        StdError::NotFound { .. } => (), // Expected error, test passes
        _ => panic!("Expected StdError::NotFound, but got a different error"),
    }
}

#[test]
fn test_claim_ownership_valid() {
    // Test claiming ownership with a valid proposal

    // Arrange
    let mut deps = mock_dependencies();
    let (mut test_env, mut deps) = setup_test_env(deps.as_mut(), NEW_OWNER);
    test_env.proposal
        .save(
            deps.storage,
            &OwnerProposal {
                owner: Addr::unchecked(NEW_OWNER),
                expiry: test_env.env.block.time.seconds() + VALID_DURATION,
            },
        )
        .unwrap();

    // Act
    let res = handle_claim_ownership(deps.branch(), test_env.info.clone(), test_env.env.clone(), &test_env.admin, &test_env.proposal);

    // Assert
    assert!(res.is_ok());

    // Verify event
    let event = &res.unwrap().events[0];
    assert_eq!(event.ty, "update_owner");

    // Verify state using query
    let stored_owner = query_owner(deps.as_ref(), &test_env.admin).unwrap();
    assert_eq!(stored_owner, Some(Addr::unchecked(NEW_OWNER)));

    // Verify that the proposal is removed
    let stored_proposal_err = query_ownership_proposal(deps.as_ref(), &test_env.proposal).unwrap_err();
    match stored_proposal_err {
        StdError::NotFound { .. } => (), // Expected error, test passes
        _ => panic!("Expected StdError::NotFound, but got a different error"),
    }
}

#[test]
fn test_claim_ownership_expired_proposal() {
    // Test claiming ownership with an expired proposal

    // Arrange
    let mut deps = mock_dependencies();
    let (mut test_env, mut deps) = setup_test_env(deps.as_mut(), NEW_OWNER);
    test_env.proposal
        .save(
            deps.storage,
            &OwnerProposal {
                owner: Addr::unchecked(NEW_OWNER),
                expiry: test_env.env.block.time.seconds() - EXPIRED_DURATION,
            },
        )
        .unwrap();

    // Act
    let res = handle_claim_ownership(deps.branch(), test_env.info.clone(), test_env.env.clone(), &test_env.admin, &test_env.proposal);

    // Assert
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), ContractError::Expired {});

    // Verify state using query - proposal should still exist
    let stored_proposal = query_ownership_proposal(deps.as_ref(), &test_env.proposal).unwrap();
    assert_eq!(stored_proposal.owner, Addr::unchecked(NEW_OWNER));
    assert_eq!(stored_proposal.expiry, test_env.env.block.time.seconds() - EXPIRED_DURATION);

    // Verify owner remains unchanged
    let stored_owner = query_owner(deps.as_ref(), &test_env.admin).unwrap();
    assert_eq!(stored_owner, Some(Addr::unchecked(ADMIN)));
}

#[test]
fn test_claim_ownership_unauthorized() {
    // Test claiming ownership by an unauthorized sender

    // Arrange
    let mut deps = mock_dependencies();
    let (mut test_env, mut deps) = setup_test_env(deps.as_mut(), INVALID_OWNER);
    test_env.proposal
        .save(
            deps.storage,
            &OwnerProposal {
                owner: Addr::unchecked(NEW_OWNER),
                expiry: test_env.env.block.time.seconds() + VALID_DURATION,
            },
        )
        .unwrap();

    // Act
    let res = handle_claim_ownership(deps.branch(), test_env.info.clone(), test_env.env.clone(), &test_env.admin, &test_env.proposal);

    // Assert
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), ContractError::Unauthorized {});

    // Verify state using query - proposal should still exist
    let stored_proposal = query_ownership_proposal(deps.as_ref(), &test_env.proposal).unwrap();
    assert_eq!(stored_proposal.owner, Addr::unchecked(NEW_OWNER));
    assert_eq!(stored_proposal.expiry, test_env.env.block.time.seconds() + VALID_DURATION);

    // Verify owner remains unchanged
    let stored_owner = query_owner(deps.as_ref(), &test_env.admin).unwrap();
    assert_eq!(stored_owner, Some(Addr::unchecked(ADMIN)));
}

#[test]
fn test_query_ownership_proposal() {
    // Test querying an existing ownership proposal

    // Arrange
    let mut deps = mock_dependencies();
    let (mut test_env, mut deps) = setup_test_env(deps.as_mut(), ADMIN);
    test_env.proposal
        .save(
            deps.storage,
            &OwnerProposal {
                owner: Addr::unchecked(NEW_OWNER),
                expiry: test_env.env.block.time.seconds() + VALID_DURATION,
            },
        )
        .unwrap();

    // Act
    let res = query_ownership_proposal(deps.as_ref(), &test_env.proposal);

    // Assert
    assert!(res.is_ok());
    let stored_proposal = res.unwrap();
    assert_eq!(stored_proposal.owner, Addr::unchecked(NEW_OWNER));
    assert_eq!(stored_proposal.expiry, test_env.env.block.time.seconds() + VALID_DURATION);
}

#[test]
fn test_query_owner() {
    // Test querying the current owner

    // Arrange
    let mut deps = mock_dependencies();
    let (mut test_env, mut deps) = setup_test_env(deps.as_mut(), ADMIN);

    // Act
    let res = query_owner(deps.as_ref(), &test_env.admin);

    // Assert
    assert!(res.is_ok());
    let owner = res.unwrap();
    assert_eq!(owner, Some(Addr::unchecked(ADMIN)));
}
