use std::cmp::Ordering;

use cosmwasm_std::{Addr, Env, MessageInfo, QuerierWrapper, Storage, SubMsg, Uint128};
use cw_utils::must_pay;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ContractError,
    helpers::{get_ica_address, get_total_primitive_shares},
    ibc_util::do_transfer,
    icq::try_icq,
    state::{
        OngoingDeposit, RawAmount, BONDING_CLAIMS, BOND_QUEUE, CONFIG, ICA_CHANNEL,
        PENDING_BOND_QUEUE, SHARES,
    },
};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Bond {
    pub amount: Uint128,
    pub owner: Addr,
    pub bond_id: String,
}

// A deposit starts of by querying the state of the ica counterparty contract
pub fn do_bond(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    env: Env,
    info: MessageInfo,
    bond_id: String,
) -> Result<Option<SubMsg>, ContractError> {
    let amount = must_pay(&info, &CONFIG.load(storage)?.local_denom)?;

    PENDING_BOND_QUEUE.push_back(
        storage,
        &Bond {
            amount,
            owner: info.sender,
            bond_id,
        },
    )?;

    // TODO: move this to the execute_bond function
    try_icq(storage, querier, env)
}

// after the balance query, we can calculate the amount of the claim we need to create, we update the claims and transfer the funds
pub fn batch_bond(
    storage: &mut dyn Storage,
    env: &Env,
    total_vault_value: Uint128,
) -> Result<Option<SubMsg>, ContractError> {
    let transfer_chan = CONFIG.load(storage)?.transfer_channel;
    let to_address = get_ica_address(storage, ICA_CHANNEL.load(storage)?)?;

    if let Some((amount, deposits)) = fold_bonds(storage, total_vault_value)? {
        Ok(Some(do_transfer(
            storage,
            env,
            amount,
            transfer_chan,
            to_address,
            deposits,
        )?))
    } else {
        Ok(None)
    }
}

/// fold_bonds folds the queue and attributes shares to the depositors according to the given total value
pub fn fold_bonds(
    storage: &mut dyn Storage,
    total_balance: Uint128,
) -> Result<Option<(Uint128, Vec<OngoingDeposit>)>, ContractError> {
    let mut total = Uint128::zero();
    let mut deposits: Vec<OngoingDeposit> = vec![];

    if BOND_QUEUE.is_empty(storage)? {
        return Ok(None);
    }

    while !BOND_QUEUE.is_empty(storage)? {
        let item: Bond =
            BOND_QUEUE
                .pop_front(storage)?
                .ok_or(ContractError::QueueItemNotFound {
                    queue: "bond".to_string(),
                })?;
        let claim_amount = create_claim(
            storage,
            item.amount,
            &item.owner,
            &item.bond_id,
            total_balance,
        )?;
        total = total
            .checked_add(item.amount)
            .map_err(|err| ContractError::TracedOverflowError(err, "fold_bonds".to_string()))?;
        deposits.push(OngoingDeposit {
            claim_amount,
            owner: item.owner,
            raw_amount: RawAmount::LocalDenom(item.amount),
            bond_id: item.bond_id,
        });
    }

    Ok(Some((total, deposits)))
}

// create_claim
fn create_claim(
    storage: &mut dyn Storage,
    user_balance: Uint128,
    address: &Addr,
    bond_id: &str,
    total_balance: Uint128,
) -> Result<Uint128, ContractError> {
    let total_shares = get_total_primitive_shares(storage)?;

    // calculate the correct size of the claim
    let claim_amount = calculate_claim(user_balance, total_balance, total_shares)?;
    BONDING_CLAIMS.save(storage, (address, bond_id), &claim_amount)?;
    Ok(claim_amount)
}

// create a share and remove the amount from the claim
pub fn create_share(
    storage: &mut dyn Storage,
    owner: &Addr,
    bond_id: &str,
    amount: Uint128,
) -> Result<Uint128, ContractError> {
    let claim = BONDING_CLAIMS.load(storage, (owner, bond_id))?;

    match claim.cmp(&amount) {
        Ordering::Less => return Err(ContractError::InsufficientClaims),
        Ordering::Equal => BONDING_CLAIMS.remove(storage, (owner, bond_id)),
        Ordering::Greater => BONDING_CLAIMS.save(
            storage,
            (owner, bond_id),
            &claim.checked_sub(amount).map_err(|err| {
                ContractError::TracedOverflowError(err, "create_share".to_string())
            })?,
        )?,
    }

    // TODO do we want to make shares fungible using cw20? if so, call into the minter and mint shares for the according to the claim
    SHARES.update(
        storage,
        owner.clone(),
        |old| -> Result<Uint128, ContractError> {
            if let Some(existing) = old {
                Ok(existing.checked_add(amount).map_err(|err| {
                    ContractError::TracedOverflowError(
                        err,
                        "create_share_update_shares".to_string(),
                    )
                })?)
            } else {
                Ok(amount)
            }
        },
    )?;
    Ok(claim)
}

/// calculate the amount of for the claim of the user
/// user_shares = (user_balance / vault_balance) * vault_total_shares = (user_balance * vault_total_shares) / vault_balance
/// if the total_shares are zero, independant of the total_balance, the user shoudl get user_balance amount of shares
/// if the total_balance is zero, what do we do?, for now the same as if total_shares is zero
/// ```rust
/// #  use cosmwasm_std::Uint128;
/// # use lp_strategy::bond::calculate_claim;
///
/// # fn calculate_claim_works() {
/// let val = calculate_claim(Uint128::new(10), Uint128::new(100), Uint128::new(10)).unwrap();
/// assert_eq!(val, Uint128::one())
/// # }
/// ```
pub fn calculate_claim(
    user_balance: Uint128,
    total_balance: Uint128,
    total_shares: Uint128,
) -> Result<Uint128, ContractError> {
    if total_shares == Uint128::zero() || total_balance == Uint128::zero() {
        Ok(user_balance)
    } else {
        Ok(user_balance
            .checked_mul(total_shares)
            .map_err(|err| ContractError::TracedOverflowError(err, "calculate_claim".to_string()))?
            .checked_div(total_balance)?)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_env, MockQuerier},
        to_binary, CosmosMsg, Empty, IbcMsg, IbcTimeout,
    };

    use crate::{
        ibc_lock::Lock,
        icq::prepare_full_query,
        state::{LpCache, IBC_LOCK, ICQ_CHANNEL, LP_SHARES},
        test_helpers::default_setup,
    };

    use super::*;

    #[test]
    fn calculate_claim_works() {
        let val = calculate_claim(Uint128::new(10), Uint128::new(100), Uint128::new(10)).unwrap();
        assert_eq!(val, Uint128::one())
    }

    #[test]
    fn do_bond_locked_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();
        let config = CONFIG.load(deps.as_ref().storage).unwrap();
        let owner = Addr::unchecked("bob");

        let info = MessageInfo {
            sender: owner,
            funds: vec![coin(1000, config.local_denom)],
        };

        IBC_LOCK
            .save(deps.as_mut().storage, &Lock::new().lock_bond())
            .unwrap();
        let id = "my-id";

        let qx: MockQuerier<Empty> = MockQuerier::new(&[]);
        let q = QuerierWrapper::new(&qx);

        let res = do_bond(deps.as_mut().storage, q, env, info, id.to_string()).unwrap();
        assert_eq!(res, None)
    }

    #[test]
    fn do_bond_unlocked_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();
        let config = CONFIG.load(deps.as_ref().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let id = "my-id";

        LP_SHARES
            .save(
                deps.as_mut().storage,
                &LpCache {
                    locked_shares: Uint128::new(100),
                    w_unlocked_shares: Uint128::zero(),
                    d_unlocked_shares: Uint128::zero(),
                },
            )
            .unwrap();

        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();

        let info = MessageInfo {
            sender: owner,
            funds: vec![coin(1000, config.local_denom)],
        };
        let qx: MockQuerier<Empty> = MockQuerier::new(&[]);
        let q = QuerierWrapper::new(&qx);

        let res = do_bond(deps.as_mut().storage, q, env.clone(), info, id.to_string()).unwrap();
        assert!(res.is_some());

        // mocking the pending bonds is real ugly here
        let packet =
            prepare_full_query(deps.as_mut().storage, env.clone(), Uint128::new(1000)).unwrap();

        let icq_msg = CosmosMsg::Ibc(IbcMsg::SendPacket {
            channel_id: ICQ_CHANNEL.load(deps.as_mut().storage).unwrap(),
            data: to_binary(&packet).unwrap(),
            timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(7200)),
        });

        assert_eq!(res.unwrap().msg, icq_msg)
    }

    #[test]
    fn batch_bond_works() {}

    #[test]
    fn create_claim_works() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked("bob");
        let id = "my-id";
        let user_balance = Uint128::new(10);
        let total_balance = Uint128::new(100);
        SHARES
            .save(deps.as_mut().storage, owner.clone(), &Uint128::new(100))
            .unwrap();

        let claim_amount = create_claim(
            deps.as_mut().storage,
            user_balance,
            &owner,
            id,
            total_balance,
        )
        .unwrap();
        assert_eq!(claim_amount, Uint128::new(10));
        assert_eq!(
            BONDING_CLAIMS
                .load(deps.as_ref().storage, (&owner, id))
                .unwrap(),
            claim_amount
        );
    }

    #[test]
    fn create_exact_share_works() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked("bob");
        let id = "my-id";
        let amount = Uint128::new(100);

        BONDING_CLAIMS
            .save(deps.as_mut().storage, (&owner, id), &amount)
            .unwrap();

        create_share(deps.as_mut().storage, &owner, id, amount).unwrap();
        // the claim in BONDING_CLAIMS should have been deleted
        assert_eq!(
            BONDING_CLAIMS
                .may_load(deps.as_ref().storage, (&owner, id))
                .unwrap(),
            None
        );

        // we should have minted exactly 100 shares by now,
        // we should have minted exactly 100 shares by now,
        assert_eq!(SHARES.load(deps.as_ref().storage, owner).unwrap(), amount);
    }

    #[test]
    fn create_less_shares_works() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked("bob");
        let id = "my-id";
        let amount = Uint128::new(100);
        let smaller_amount = Uint128::new(99);

        BONDING_CLAIMS
            .save(deps.as_mut().storage, (&owner, id), &amount)
            .unwrap();

        create_share(deps.as_mut().storage, &owner, id, smaller_amount).unwrap();
        // the claim in BONDING_CLAIMS should have been deleted
        assert_eq!(
            BONDING_CLAIMS
                .may_load(deps.as_ref().storage, (&owner, id))
                .unwrap(),
            Some(Uint128::one())
        );
        // we should have amount shares by now
        assert_eq!(
            SHARES.load(deps.as_ref().storage, owner).unwrap(),
            smaller_amount
        );
    }

    #[test]
    fn create_too_many_shares_fails() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked("bob");
        let id = "my-id";
        let amount = Uint128::new(100);
        let incorrect_amount = Uint128::new(101);

        BONDING_CLAIMS
            .save(deps.as_mut().storage, (&owner, id), &amount)
            .unwrap();

        let err = create_share(deps.as_mut().storage, &owner, id, incorrect_amount).unwrap_err();
        // we should not have created shares
        assert_eq!(err, ContractError::InsufficientClaims);
        // our bonding claim should still exist
        assert_eq!(
            BONDING_CLAIMS
                .load(deps.as_ref().storage, (&owner, id))
                .unwrap(),
            amount
        )
    }

    #[test]
    fn empty_fold_bonds_works() {
        let mut deps = mock_dependencies();
        let total_balance = Uint128::new(150);
        // we don't have any bonds to setup

        let res = fold_bonds(deps.as_mut().storage, total_balance).unwrap();
        assert_eq!(res, None)
    }

    #[test]
    fn filled_fold_bonds_works() {
        let mut deps = mock_dependencies();
        let total_balance = Uint128::new(150);
        let bonds = vec![
            Bond {
                amount: Uint128::new(34),
                owner: Addr::unchecked("person1"),
                bond_id: "id1".to_string(),
            },
            Bond {
                amount: Uint128::new(50),
                owner: Addr::unchecked("person2"),
                bond_id: "id2".to_string(),
            },
            Bond {
                amount: Uint128::new(2),
                owner: Addr::unchecked("person3"),
                bond_id: "id3".to_string(),
            },
            Bond {
                amount: Uint128::new(7),
                owner: Addr::unchecked("person4"),
                bond_id: "id4".to_string(),
            },
            Bond {
                amount: Uint128::new(57),
                owner: Addr::unchecked("person5"),
                bond_id: "id5".to_string(),
            },
        ];
        for b in &bonds {
            BOND_QUEUE.push_back(deps.as_mut().storage, b).unwrap();
        }

        let (total, ongoing) = fold_bonds(deps.as_mut().storage, total_balance)
            .unwrap()
            .unwrap();
        assert_eq!(total_balance, total);
        for (i, b) in bonds.iter().enumerate() {
            assert_eq!(ongoing[i].bond_id, b.bond_id);
            assert_eq!(ongoing[i].owner, b.owner);

            assert_eq!(ongoing[i].claim_amount, b.amount)
        }
    }
}
