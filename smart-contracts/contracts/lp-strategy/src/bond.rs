use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Storage, SubMsg, Uint128};
use cw_utils::must_pay;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ContractError,
    helpers::{get_ica_address, get_total_shares, get_raw_total_shares},
    ibc_util::do_transfer,
    icq::try_icq,
    state::{OngoingDeposit, RawAmount, BONDING_CLAIMS, BOND_QUEUE, CONFIG, ICA_CHANNEL, SHARES},
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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    bond_id: String,
) -> Result<Option<SubMsg>, ContractError> {
    let amount = must_pay(&info, &CONFIG.load(deps.storage)?.local_denom)?;

    BOND_QUEUE.push_back(
        deps.storage,
        &Bond {
            amount,
            owner: info.sender,
            bond_id,
        },
    )?;

    try_icq(deps.storage, env)
}

// after the balance query, we can calculate the amount of the claim we need to create, we update the claims and transfer the funds
pub fn batch_bond(
    storage: &mut dyn Storage,
    env: &Env,
    query_balance: Uint128,
) -> Result<Option<SubMsg>, ContractError> {
    let transfer_chan = CONFIG.load(storage)?.transfer_channel;
    let to_address = get_ica_address(storage, ICA_CHANNEL.load(storage)?)?;

    if let Some((amount, deposits)) = fold_bonds(storage, query_balance)? {
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
        let claim_amount = create_claim(storage, item.amount, item.owner.clone(), total_balance)?;
        total = total.checked_add(item.amount)?;
        println!("{:?}", claim_amount);
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
    address: Addr,
    total_balance: Uint128,
) -> Result<Uint128, ContractError> {
    let total_shares = get_raw_total_shares(storage)?;

    // calculate the correct size of the claim
    let claim_amount = calculate_claim(user_balance, total_balance, total_shares)?;
    BONDING_CLAIMS.save(storage, address, &claim_amount)?;
    Ok(claim_amount)
}

// create a share and remove the amount from the claim
pub fn create_share(
    storage: &mut dyn Storage,
    owner: Addr,
    amount: Uint128,
) -> Result<Uint128, ContractError> {
    let claim = BONDING_CLAIMS.load(storage, owner.clone())?;
    if claim < amount {
        return Err(ContractError::InsufficientClaims);
    }

    if claim <= amount {
        BONDING_CLAIMS.remove(storage, owner.clone());
    } else {
        BONDING_CLAIMS.save(storage, owner.clone(), &claim.checked_sub(amount)?)?;
    }

    // TODO do we want to make shares fungible using cw20? if so, call into the minter and mint shares for the according to the claim
    SHARES.save(storage, owner, &amount)?;
    Ok(claim)
}

/// calculate the amount of for the claim of the user
/// user_shares = (user_balance / vault_balance) * vault_total_shares = (user_balance * vault_total_shares) / vault_balance
/// if the total_shares are zero, independant of the total_balance, the user shoudl get user_balance amount of shares
/// if the total_balance is zero, what do we do?, for now the same as if total_shares is zero
fn calculate_claim(
    user_balance: Uint128,
    total_balance: Uint128,
    total_shares: Uint128,
) -> Result<Uint128, ContractError> {
    if total_shares == Uint128::zero() || total_balance == Uint128::zero() {
        Ok(user_balance)
    } else {
        println!("hit me 2");
        println!("ub: {:?}, ts: {:?}, tb: {:?}", user_balance, total_shares, total_balance);
        Ok(user_balance
            .checked_mul(total_shares)?
            .checked_div(total_balance)?)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_dependencies;

    use super::*;

    // TODO rewrite this to a proptest
    #[test]
    fn calculate_claim_works() {
        let val = calculate_claim(Uint128::new(10), Uint128::new(100), Uint128::new(10)).unwrap();
        assert_eq!(val, Uint128::one())
    }

    #[test]
    fn fold_bonds_works() {
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
