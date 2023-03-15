use cosmwasm_std::{
    to_binary, Addr, Attribute, BankMsg, Coin, Decimal, Deps, DepsMut, Env, Fraction, MessageInfo,
    OverflowError, QuerierWrapper, Response, StdError, Uint128, WasmMsg,
};

use cw20_base::contract::{execute_burn, execute_mint};
use cw_utils::PaymentError;
use lp_strategy::msg::{IcaBalanceResponse, PrimitiveSharesResponse};
use quasar_types::types::{CoinRatio, CoinWeight};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, PrimitiveConfig};

use crate::state::{
    BondingStub, InvestmentInfo, Supply, Unbond, UnbondingStub, BONDING_SEQ, BONDING_SEQ_TO_ADDR,
    BOND_STATE, INVESTMENT, PENDING_BOND_IDS, PENDING_UNBOND_IDS, TOTAL_SUPPLY, UNBOND_STATE,
};

// get_bonded returns the total amount of delegations from contract
// it ensures they are all the same denom
fn _get_bonded(querier: &QuerierWrapper, contract: &Addr) -> Result<Uint128, ContractError> {
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

fn _assert_bonds(supply: &Supply, bonded: Uint128) -> Result<(), ContractError> {
    if supply.bonded != bonded {
        Err(ContractError::BondedMismatch {
            stored: supply.bonded,
            queried: bonded,
        })
    } else {
        Ok(())
    }
}

// todo test
// returns amount if the coin is found and amount is non-zero
// errors otherwise
pub fn must_pay_multi(funds: &[Coin], denom: &str) -> Result<Uint128, PaymentError> {
    match funds.iter().find(|c| c.denom == denom) {
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

// todo test
pub fn may_pay_with_ratio(
    deps: &Deps,
    funds: &[Coin],
    mut invest: InvestmentInfo,
) -> Result<(Vec<Coin>, Vec<Coin>), ContractError> {
    // normalize primitives
    invest.normalize_primitive_weights();

    // load cached balance of primitive contracts
    let deposit_amount_weights: Vec<CoinWeight> = invest
        .primitives
        .iter()
        .map(|pc| -> Result<CoinWeight, ContractError> {
            let supply: PrimitiveSharesResponse = deps.querier.query_wasm_smart(
                pc.address.clone(),
                &lp_strategy::msg::QueryMsg::PrimitiveShares {},
            )?;
            let balance: IcaBalanceResponse = deps.querier.query_wasm_smart(
                pc.address.clone(),
                &lp_strategy::msg::QueryMsg::IcaBalance {},
            )?;

            Ok(CoinWeight {
                weight: Decimal::from_ratio(
                    balance.amount.amount.checked_mul(pc.weight.numerator())?,
                    supply.total.checked_mul(pc.weight.denominator())?,
                ),
                denom: balance.amount.denom,
            })
        })
        .collect::<Result<Vec<CoinWeight>, ContractError>>()?;

    // TODO: change the error
    if deposit_amount_weights
        .first()
        .ok_or(ContractError::CoinsWeightVectorIsEmpty {})?
        .weight
        == Decimal::zero()
    {
        return Err(ContractError::Std(StdError::GenericErr {
            msg: "Deposit amount weight for primitive is zero".to_string(),
        }));
    }

    let token_weights: Vec<CoinWeight> = deposit_amount_weights.clone().iter().try_fold(
        vec![],
        |mut acc: Vec<CoinWeight>,
         coin_weight: &CoinWeight|
         -> Result<Vec<CoinWeight>, ContractError> {
            // look through acc for existing denom and add weight, or else push it to the back of the vec
            let existing_weight = acc.iter_mut().find(|cw| cw.denom == coin_weight.denom);
            match existing_weight {
                Some(weight) => weight.weight = weight.weight.checked_add(coin_weight.weight)?,
                None => acc.push(coin_weight.clone()),
            };
            Ok(acc)
        },
    )?;

    if token_weights
        .first()
        .ok_or(ContractError::TokenWeightsIsEMpty {})?
        .weight
        == Decimal::zero()
    {
        return Err(ContractError::Std(StdError::GenericErr {
            msg: format!(
                "token weight is zero for {}",
                token_weights.first().unwrap().denom
            ),
        }));
    }

    let mut max_bond = Uint128::MAX;
    for coin_weight in token_weights {
        let amount = must_pay_multi(funds, &coin_weight.denom)?;
        let bond_for_token = amount.multiply_ratio(
            coin_weight.weight.denominator(),
            coin_weight.weight.numerator(),
        );
        if bond_for_token < max_bond {
            max_bond = bond_for_token;
        }
    }

    let ratio = CoinRatio {
        ratio: deposit_amount_weights,
    };

    if (max_bond == Uint128::zero() || max_bond == Uint128::MAX) {
        return Err(ContractError::Std(StdError::GenericErr {
            msg: format!(
                "Unable to correctly determine max_bond, value: {}",
                max_bond
            ),
        }));
    }

    // verify that >0 of each token in ratio is passed in, return (funds, remainder))
    // where funds is the max amount we can use in compliance with the ratio
    // and remainder is the change to return to user
    let normed_ratio = ratio.get_normed_ratio();
    let mut remainder = funds.to_owned();

    let coins: Result<Vec<Coin>, ContractError> = normed_ratio?
        .iter()
        .map(|r| {
            let amount = must_pay_multi(funds, &r.denom)?;
            let expected_amount =
                max_bond.checked_multiply_ratio(r.weight.numerator(), r.weight.denominator())?;

            if expected_amount > amount {
                return Err(ContractError::IncorrectBondingRatio {});
            }

            remainder = remainder
                .iter()
                .map(|c| -> Result<Coin, ContractError> {
                    if c.denom == r.denom {
                        Ok(Coin {
                            amount: c.amount.checked_sub(expected_amount)?,
                            denom: c.denom.clone(),
                        })
                    } else {
                        Ok(c.clone())
                    }
                })
                .collect::<Result<Vec<Coin>, ContractError>>()?;

            Ok(Coin {
                denom: r.denom.clone(),
                amount: expected_amount,
            })
        })
        .collect();

    let c = coins?;

    if c.first()
        .ok_or(ContractError::CoinsVectorIsEmpty {})?
        .amount
        == Uint128::zero()
    {
        return Err(ContractError::Std(StdError::GenericErr {
            msg: "we failed here".to_string(),
        }));
    }

    Ok((c, remainder))
}

// todo test
pub fn bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    if info.funds.is_empty() || info.funds.iter().all(|c| c.amount.is_zero()) {
        return Err(ContractError::NoFunds {});
    }

    // load vault info & sequence number
    let invest = INVESTMENT.load(deps.storage)?;
    let bond_seq = BONDING_SEQ.load(deps.storage)?;

    // find recipient
    let recipient_addr = match recipient {
        Some(r) => deps.api.addr_validate(&r)?,
        None => info.sender,
    };

    let mut deposit_stubs = vec![];

    let (primitive_funding_amounts, remainder) =
        may_pay_with_ratio(&deps.as_ref(), &info.funds, invest.clone())?;

    let bond_msgs: Result<Vec<WasmMsg>, ContractError> = invest
        .primitives
        .iter()
        .zip(primitive_funding_amounts.clone())
        .map(|(pc, funds)| {
            let deposit_stub = BondingStub {
                address: pc.address.clone(),
                bond_response: Option::None,
            };
            deposit_stubs.push(deposit_stub);

            // todo: do we need it to reply
            Ok(WasmMsg::Execute {
                contract_addr: pc.address.clone(),
                msg: to_binary(&lp_strategy::msg::ExecuteMsg::Bond {
                    id: bond_seq.to_string(),
                })?,
                funds: vec![funds],
            })
        })
        .collect();

    // save bonding state for use during the callback
    PENDING_BOND_IDS.update(deps.storage, recipient_addr.clone(), |ids| match ids {
        Some(mut bond_ids) => {
            bond_ids.push(bond_seq.to_string());
            Ok::<Vec<String>, ContractError>(bond_ids)
        }
        None => Ok(vec![bond_seq.to_string()]),
    })?;
    BOND_STATE.save(deps.storage, bond_seq.to_string(), &deposit_stubs)?;
    BONDING_SEQ_TO_ADDR.save(
        deps.storage,
        bond_seq.to_string(),
        &recipient_addr.to_string(),
    )?;
    BONDING_SEQ.save(deps.storage, &bond_seq.checked_add(Uint128::new(1))?)?;

    let mut remainder_msgs = vec![];

    remainder.iter().for_each(|r| {
        if (r.amount > Uint128::zero()) {
            remainder_msgs.push(BankMsg::Send {
                to_address: recipient_addr.to_string(),
                amount: vec![Coin {
                    denom: r.denom.clone(),
                    amount: r.amount,
                }],
            });
        }
    });

    Ok(Response::new()
        .add_attribute("bond_id", bond_seq.to_string())
        .add_messages(bond_msgs?)
        .add_messages(remainder_msgs))
}

pub fn unbond(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    if let Some(unbond_amount) = amount {
        let (start_unbond_msgs, start_unbond_attrs) =
            do_start_unbond(deps.branch(), &env, &info, unbond_amount)?;

        let (unbond_msgs, unbond_attrs) = do_unbond(deps, &env, &info)?;

        Ok(Response::new()
            .add_messages(start_unbond_msgs)
            .add_messages(unbond_msgs)
            .add_attributes(start_unbond_attrs)
            .add_attributes(unbond_attrs))
    } else {
        let (unbond_msgs, unbond_attrs) = do_unbond(deps, &env, &info)?;
        Ok(Response::new()
            .add_messages(unbond_msgs)
            .add_attributes(unbond_attrs))
    }
}

pub fn do_start_unbond(
    mut deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    amount: Uint128,
) -> Result<(Vec<WasmMsg>, Vec<Attribute>), ContractError> {
    if amount.is_zero() {
        // skip start unbond
        return Ok((vec![], vec![]));
    }
    // check that user has vault tokens and the amount is > min_withdrawal

    let invest = INVESTMENT.load(deps.storage)?;
    let bond_seq = BONDING_SEQ.load(deps.storage)?;

    //TODO: Normalize primitive weights

    // // ensure it is big enough to care
    if amount < invest.min_withdrawal {
        return Err(ContractError::UnbondTooSmall {
            min_bonded: invest.min_withdrawal,
        });
    }

    // this should error if amount larger than sender balance
    // todo: verify above statement
    execute_burn(deps.branch(), env.clone(), info.clone(), amount)?;

    let mut unbonding_stubs = vec![];

    let start_unbond_msgs: Vec<WasmMsg> = invest
        .primitives
        .iter()
        .map(|pc| -> Result<WasmMsg, ContractError> {
            // lets get the amount of tokens to unbond for this primitive
            // todo make sure weights are normalized!!
            let primitive_share_amount =
                amount.multiply_ratio(pc.weight.numerator(), pc.weight.denominator());

            // todo: safety asertion - make sure we have enough shares to unbond for this user (else we have major code error)
            // let our_shares = deps.querier.query_wasm_smart(pc.address, )

            unbonding_stubs.push(UnbondingStub {
                address: pc.address.clone(),
                unlock_time: None,
                unbond_response: None,
                unbond_funds: vec![],
            });

            Ok(WasmMsg::Execute {
                contract_addr: pc.address.clone(),
                msg: to_binary(&lp_strategy::msg::ExecuteMsg::StartUnbond {
                    id: bond_seq.to_string(),
                    share_amount: primitive_share_amount,
                })?,
                funds: vec![],
            })
        })
        .collect::<Result<Vec<WasmMsg>, ContractError>>()?;

    // jimeny cricket, we need to save the unbonding state for use during the callback
    PENDING_UNBOND_IDS.update(deps.storage, info.sender.clone(), |ids| match ids {
        Some(mut bond_ids) => {
            bond_ids.push(bond_seq.to_string());
            Ok::<Vec<String>, ContractError>(bond_ids)
        }
        None => Ok(vec![bond_seq.to_string()]),
    })?;
    UNBOND_STATE.save(
        deps.storage,
        bond_seq.to_string(),
        &Unbond {
            stub: unbonding_stubs,
            shares: amount,
        },
    )?;
    BONDING_SEQ_TO_ADDR.save(deps.storage, bond_seq.to_string(), &info.sender.to_string())?;
    BONDING_SEQ.save(deps.storage, &bond_seq.checked_add(Uint128::from(1u128))?)?;

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

    // re-calculate bonded to ensure we have real values
    // bonded is the total number of tokens we have delegated from this address
    // let bonded = get_bonded(&deps.querier, &env.contract.address)?;

    // // calculate how many native tokens this is worth and update supply
    // // let remainder = amount.checked_sub(tax).map_err(StdError::overflow)?;
    let mut supply = TOTAL_SUPPLY.load(deps.storage)?;
    // // TODO: this is just a safety assertion - do we keep it, or remove caching?
    // // in the end supply is just there to cache the (expected) results of get_bonded() so we don't
    // // have expensive queries everywhere
    // assert_bonds(&supply, bonded)?;
    // let unbond = amount.multiply_ratio(bonded, supply.issued);
    // // let unbond = remainder.multiply_ratio(bonded, supply.issued);
    // supply.bonded = bonded.checked_sub(unbond).map_err(StdError::overflow)?;
    supply.issued = supply
        .issued
        .checked_sub(amount)
        .map_err(StdError::overflow)?;
    // supply.issued = supply
    //     .issued
    //     .checked_sub(remainder)
    //     .map_err(StdError::overflow)?;
    // supply.claims += unbond;
    TOTAL_SUPPLY.save(deps.storage, &supply)?;

    // instead of creating a claim, we will be executing create claim on the vault primitive
    // CLAIMS.create_claim(
    //     deps.storage,
    //     &info.sender,
    //     unbond,
    //     invest.unbonding_period.after(&env.block),
    // )?;

    Ok((
        start_unbond_msgs,
        vec![
            Attribute {
                key: "action".to_string(),
                value: "start_unbond".to_string(),
            },
            Attribute {
                key: "from".to_string(),
                value: info.sender.to_string(),
            },
            Attribute {
                key: "burnt".to_string(),
                value: amount.to_string(),
            },
            Attribute {
                key: "bond_id".to_string(),
                value: bond_seq.to_string(),
            },
        ],
    ))
}

// find all unbondable pending unbonds where unlock_time < env.block.time
// then trigger unbonds
pub fn do_unbond(
    mut deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
) -> Result<(Vec<WasmMsg>, Vec<Attribute>), ContractError> {
    let pending_unbond_ids = PENDING_UNBOND_IDS.load(deps.storage, info.sender.clone())?;

    let mut unbond_msgs: Vec<WasmMsg> = vec![];
    for unbond_id in pending_unbond_ids.iter() {
        let unbond_stubs = UNBOND_STATE.load(deps.storage, unbond_id.clone())?;
        let mut current_unbond_msgs = find_and_return_unbondable_msgs(
            deps.branch(),
            env,
            info,
            unbond_id,
            unbond_stubs.stub,
        )?;
        unbond_msgs.append(current_unbond_msgs.as_mut());
    }

    Ok((
        unbond_msgs.clone(),
        vec![
            Attribute {
                key: "action".to_string(),
                value: "unbond".to_string(),
            },
            Attribute {
                key: "from".to_string(),
                value: info.sender.to_string(),
            },
            Attribute {
                key: "num_unbondable_ids".to_string(),
                value: unbond_msgs.len().to_string(),
            },
        ],
    ))
}

pub fn find_and_return_unbondable_msgs(
    _deps: DepsMut,
    env: &Env,
    _info: &MessageInfo,
    unbond_id: &String,
    unbond_stubs: Vec<UnbondingStub>,
) -> Result<Vec<WasmMsg>, ContractError> {
    // go through unbond_stubs and find ones where unlock_time < env.block.time and execute
    Ok(unbond_stubs
        .iter()
        .filter(|stub| {
            stub.unlock_time
                .map_or(false, |unlock_time| unlock_time < env.block.time)
        })
        .map(|stub| -> Result<WasmMsg, ContractError> {
            Ok(WasmMsg::Execute {
                contract_addr: stub.address.clone(),
                msg: to_binary(&lp_strategy::msg::ExecuteMsg::Unbond {
                    id: unbond_id.clone(),
                })?,
                funds: vec![],
            })
        })
        .collect::<Result<Vec<WasmMsg>, ContractError>>()?)
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
