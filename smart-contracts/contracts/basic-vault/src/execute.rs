use std::ops::Add;

use cosmwasm_std::{
    to_binary, Addr, Coin, Decimal, DepsMut, Env, Fraction, MessageInfo, QuerierWrapper, Response,
    SubMsg, Uint128, WasmMsg,
};

use cw_utils::PaymentError;
use quasar_types::types::{CoinRatio, CoinWeight};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, PrimitiveInitMsg};

use crate::state::{
    BondingStub, Supply, BONDING_SEQ, DEPOSIT_STATE, INVESTMENT, PENDING_BOND_IDS, STRATEGY_BOND_ID,
};

// get_bonded returns the total amount of delegations from contract
// it ensures they are all the same denom
fn get_bonded(querier: &QuerierWrapper, contract: &Addr) -> Result<Uint128, ContractError> {
    let bonds = querier.query_all_delegations(contract)?;
    if bonds.is_empty() {
        return Ok(Uint128::zero());
    }
    let denom = bonds[0].amount.denom.as_str();
    bonds.iter().fold(Ok(Uint128::zero()), |racc, d| {
        let acc = racc?;
        if d.amount.denom.as_str() != denom {
            Err(ContractError::DifferentBondDenom {
                denom1: denom.into(),
                denom2: d.amount.denom.to_string(),
            })
        } else {
            Ok(acc + d.amount.amount)
        }
    })
}

fn assert_bonds(supply: &Supply, bonded: Uint128) -> Result<(), ContractError> {
    if supply.bonded != bonded {
        Err(ContractError::BondedMismatch {
            stored: supply.bonded,
            queried: bonded,
        })
    } else {
        Ok(())
    }
}

// returns amount if the coin is found and amount is non-zero
// errors otherwise
pub fn must_pay_multi(info: &MessageInfo, denom: &str) -> Result<Uint128, PaymentError> {
    match info.funds.iter().find(|c| c.denom == denom) {
        Some(coin) => {
            if coin.amount.is_zero() {
                Err(PaymentError::NoFunds {})
            } else {
                Ok(coin.amount)
            }
        }
        None => Err(PaymentError::MissingDenom(denom.to_string())),
    }
}

pub fn must_pay_with_ratio(
    info: &MessageInfo,
    ratio: CoinRatio,
) -> Result<Vec<Coin>, ContractError> {
    // verify that info.funds are passed in with the correct ratio
    let normed_ratio = ratio.get_normed_ratio();
    let mut last_expected_total = Uint128::zero();

    let coins: Result<Vec<Coin>, ContractError> = normed_ratio
        .iter()
        .map(|r| {
            let amount = must_pay_multi(info, &r.denom).unwrap();
            let expected_total = amount
                .checked_multiply_ratio(r.weight.denominator(), r.weight.numerator())
                .unwrap();

            if (last_expected_total.is_zero()) {
                last_expected_total = expected_total;
            }

            if expected_total.ne(&last_expected_total) {
                return Err(ContractError::IncorrectBondingRatio {});
            }
            Ok(Coin {
                denom: r.denom.clone(),
                amount: amount,
            })
        })
        .collect();

    coins
}

pub fn bond(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // load vault info & sequence number
    let invest = INVESTMENT.load(deps.storage)?;
    let bond_seq = BONDING_SEQ.load(deps.storage)?;

    let mut deposit_stubs = vec![];

    let primitive_funding_amounts = must_pay_with_ratio(
        &info,
        // todo: This ratio should be constrained from share out calc, currently constrained by amount in.
        CoinRatio {
            ratio: invest
                .primitives
                .iter()
                .map(|i| match i.init {
                    PrimitiveInitMsg::LP(lp_init_msg) => CoinWeight {
                        denom: lp_init_msg.local_denom,
                        weight: i.weight,
                    },
                })
                .collect(),
        },
    )
    .unwrap();

    let bond_msgs: Result<Vec<SubMsg>, ContractError> = invest
        .primitives
        .iter()
        .zip(primitive_funding_amounts)
        .map(|(pc, funds)| match pc.init.clone() {
            crate::msg::PrimitiveInitMsg::LP(lp_init_msg) => {
                let deposit_stub = BondingStub {
                    address: pc.address.clone(),
                    bond_response: Option::None,
                };
                deposit_stubs.push(deposit_stub);

                // todo: do we need it to reply
                Ok(SubMsg::reply_always(
                    WasmMsg::Execute {
                        contract_addr: pc.address.clone(),
                        msg: to_binary(&lp_strategy::msg::ExecuteMsg::Bond {
                            id: bond_seq.to_string(),
                        })?,
                        funds: vec![funds],
                    },
                    STRATEGY_BOND_ID,
                ))
            }
        })
        .collect();

    // save bonding state for use during the callback
    PENDING_BOND_IDS.update(deps.storage, info.sender, |ids| match ids {
        Some(mut bond_ids) => {
            bond_ids.push(bond_seq.to_string());
            Ok::<Vec<String>, ContractError>(bond_ids)
        }
        None => Ok(vec![bond_seq.to_string()]),
    })?;
    DEPOSIT_STATE.save(deps.storage, bond_seq.to_string(), &deposit_stubs)?;
    BONDING_SEQ.save(deps.storage, &bond_seq.add(Uint128::from(1u128)))?;

    Ok(Response::new().add_submessages(bond_msgs?))
}

pub fn unbond(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    Ok(Response::new())
    // if info.funds.is_empty() {
    //     return Err(ContractError::NoFunds {});
    // }

    // let invest = INVESTMENT.load(deps.storage)?;
    // // ensure it is big enough to care
    // if amount < invest.min_withdrawal {
    //     return Err(ContractError::UnbondTooSmall {
    //         min_bonded: invest.min_withdrawal,
    //     });
    // }

    // need to convert amount to the set of amounts for each primitive

    // // // calculate tax and remainer to unbond
    // // let tax = amount * invest.exit_tax;

    // // burn from the original caller
    // execute_burn(deps.branch(), env.clone(), info.clone(), amount)?;
    // // if tax > Uint128::zero() {
    // //     let sub_info = MessageInfo {
    // //         sender: env.contract.address.clone(),
    // //         funds: vec![],
    // //     };
    // //     // call into cw20-base to mint tokens to owner, call as self as no one else is allowed
    // //     execute_mint(
    // //         deps.branch(),
    // //         env.clone(),
    // //         sub_info,
    // //         invest.owner.to_string(),
    // //         tax,
    // //     )?;
    // // }

    // // re-calculate bonded to ensure we have real values
    // // bonded is the total number of tokens we have delegated from this address
    // let bonded = get_bonded(&deps.querier, &env.contract.address)?;

    // // calculate how many native tokens this is worth and update supply
    // // let remainder = amount.checked_sub(tax).map_err(StdError::overflow)?;
    // let mut supply = TOTAL_SUPPLY.load(deps.storage)?;
    // // TODO: this is just a safety assertion - do we keep it, or remove caching?
    // // in the end supply is just there to cache the (expected) results of get_bonded() so we don't
    // // have expensive queries everywhere
    // assert_bonds(&supply, bonded)?;
    // let unbond = amount.multiply_ratio(bonded, supply.issued);
    // // let unbond = remainder.multiply_ratio(bonded, supply.issued);
    // supply.bonded = bonded.checked_sub(unbond).map_err(StdError::overflow)?;
    // supply.issued = supply
    //     .issued
    //     .checked_sub(amount)
    //     .map_err(StdError::overflow)?;
    // // supply.issued = supply
    // //     .issued
    // //     .checked_sub(remainder)
    // //     .map_err(StdError::overflow)?;
    // supply.claims += unbond;
    // TOTAL_SUPPLY.save(deps.storage, &supply)?;

    // // instead of creating a claim, we will be executing create claim on the vault primitive
    // // CLAIMS.create_claim(
    // //     deps.storage,
    // //     &info.sender,
    // //     unbond,
    // //     invest.unbonding_period.after(&env.block),
    // // )?;

    // let _undelegation_msg = todo!();

    // // unbond them
    // let res = Response::new()
    //     // .add_message(StakingMsg::Undelegate {
    //     //     validator: invest.validator,
    //     //     amount: coin(unbond.u128(), &invest.bond_denom),
    //     // })
    //     // .add_message(undelegation_msg)
    //     .add_attribute("action", "unbond")
    //     .add_attribute("to", info.sender)
    //     .add_attribute("unbonded", unbond)
    //     .add_attribute("burnt", amount);
    // Ok(res)
}

pub fn claim(_deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::new())

    // // find how many tokens the contract has
    // let invest = INVESTMENT.load(deps.storage)?;
    // let mut balance = deps
    //     .querier
    //     .query_balance(&env.contract.address, &invest.bond_denom)?;
    // if balance.amount < invest.min_withdrawal {
    //     return Err(ContractError::BalanceTooSmall {});
    // }

    // // check how much to send - min(balance, claims[sender]), and reduce the claim
    // // Ensure we have enough balance to cover this and only send some claims if that is all we can cover
    // let to_send =
    //     CLAIMS.claim_tokens(deps.storage, &info.sender, &env.block, Some(balance.amount))?;
    // if to_send == Uint128::zero() {
    //     return Err(ContractError::NothingToClaim {});
    // }

    // // update total supply (lower claim)
    // TOTAL_SUPPLY.update(deps.storage, |mut supply| -> StdResult<_> {
    //     supply.claims = supply.claims.checked_sub(to_send)?;
    //     Ok(supply)
    // })?;

    // // transfer tokens to the sender
    // balance.amount = to_send;
    // let res = Response::new()
    //     .add_message(BankMsg::Send {
    //         to_address: info.sender.to_string(),
    //         amount: vec![balance],
    //     })
    //     .add_attribute("action", "claim")
    //     .add_attribute("from", info.sender)
    //     .add_attribute("amount", to_send);
    // Ok(res)
}

/// reinvest will withdraw all pending rewards,
/// then issue a callback to itself via _bond_all_tokens
/// to reinvest the new earnings (and anything else that accumulated)
pub fn reinvest(deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let _contract_addr = env.contract.address;
    let _invest = INVESTMENT.load(deps.storage)?;
    let _msg = to_binary(&ExecuteMsg::_BondAllTokens {})?;

    // and bond them to the validator
    let res = Response::new();
    // TODO: Replace below with a WithdrawRewards message. if primitive::WithdrawRewards ends up being async, we will have to (a: pass in a callback msg, or b: implement the callback msg standard that we come up with)
    // .add_message(DistributionMsg::WithdrawDelegatorReward {
    //     validator: invest.validator,
    // })
    // .add_message(WasmMsg::Execute {
    //     contract_addr: contract_addr.to_string(),
    //     msg,
    //     funds: vec![],
    // });
    Ok(res)
}

pub fn _bond_all_tokens(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new())

    // // this is just meant as a call-back to ourself
    // if info.sender != env.contract.address {
    //     return Err(ContractError::Unauthorized {});
    // }

    // // find how many tokens we have to bond
    // let invest = INVESTMENT.load(deps.storage)?;
    // let mut balance = deps
    //     .querier
    //     .query_balance(&env.contract.address, &invest.bond_denom)?;

    // // we deduct pending claims from our account balance before reinvesting.
    // // if there is not enough funds, we just return a no-op
    // match TOTAL_SUPPLY.update(deps.storage, |mut supply| -> StdResult<_> {
    //     balance.amount = balance.amount.checked_sub(supply.claims)?;
    //     // this just triggers the "no op" case if we don't have min_withdrawal left to reinvest
    //     balance.amount.checked_sub(invest.min_withdrawal)?;
    //     supply.bonded += balance.amount;
    //     Ok(supply)
    // }) {
    //     Ok(_) => {}
    //     // if it is below the minimum, we do a no-op (do not revert other state from withdrawal)
    //     Err(StdError::Overflow { .. }) => return Ok(Response::default()),
    //     Err(e) => return Err(ContractError::Std(e)),
    // }

    // // and bond them to the validator
    // let res = Response::new()
    //     // TODO: replace this with the entryMsg on the primitive, the response to this can of course be handled in the same way as the standard bond
    //     // .add_message(StakingMsg::Delegate {
    //     //     validator: invest.validator,
    //     //     amount: balance.clone(),
    //     // })
    //     .add_attribute("action", "reinvest")
    //     .add_attribute("bonded", balance.amount);
    // Ok(res)
}
