use cosmwasm_std::{
    from_binary, to_binary, Attribute, BankMsg, Coin, Decimal, Deps, DepsMut, Env, Fraction,
    MessageInfo, Response, StdError, Uint128, WasmMsg,
};

use cw20_base::contract::execute_burn;
use cw_utils::PaymentError;
use lp_strategy::msg::{IcaBalanceResponse, PrimitiveSharesResponse, UnbondingClaimResponse};
use quasar_types::types::{CoinRatio, CoinWeight};

use crate::error::ContractError;

use crate::helpers::can_unbond_from_primitive;
use crate::state::{
    BondingStub, InvestmentInfo, Unbond, UnbondingStub, BONDING_SEQ, BONDING_SEQ_TO_ADDR,
    BOND_STATE, INVESTMENT, PENDING_BOND_IDS, PENDING_UNBOND_IDS, TOTAL_SUPPLY, UNBOND_STATE,
};

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
            let balance: IcaBalanceResponse = deps.querier.query_wasm_smart(
                pc.address.clone(),
                &lp_strategy::msg::QueryMsg::IcaBalance {},
            )?;
            let supply: PrimitiveSharesResponse = deps.querier.query_wasm_smart(
                pc.address.clone(),
                &lp_strategy::msg::QueryMsg::PrimitiveShares {},
            )?;

            // if only one of the two is zero, we should error
            if (supply.total.is_zero() && !balance.amount.amount.is_zero()) || (!supply.total.is_zero() && balance.amount.amount.is_zero()) {
                return Err(ContractError::Std(StdError::GenericErr {
                    msg: "Unexpected primitive state, either both supply and balance should be zero, or neither.".to_string(),
                }));
            }

            let ratio = match supply.total.is_zero() {
                true => Decimal::one(),
                false => Decimal::from_ratio(balance.amount.amount, supply.total),
            };

            Ok(CoinWeight {
                weight: Decimal::from_ratio(
                    ratio.numerator().checked_mul(pc.weight.numerator())?,
                    ratio.denominator().checked_mul(pc.weight.denominator())?,
                ),
                denom: balance.amount.denom,
            })
        })
        .collect::<Result<Vec<CoinWeight>, ContractError>>()?;

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

    let token_weights: Vec<CoinWeight> = deposit_amount_weights.iter().try_fold(
        vec![],
        |mut acc: Vec<CoinWeight>,
         coin_weight: &CoinWeight|
         -> Result<Vec<CoinWeight>, ContractError> {
            // look through acc for existing denom and add weight, or else push it to the back of the vec
            // todo: verify this works for multiple tokens, this might not overwrite when two primitives have the same token
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

    if max_bond == Uint128::zero() || max_bond == Uint128::MAX {
        return Err(ContractError::Std(StdError::GenericErr {
            msg: format!("Unable to correctly determine max_bond, value: {max_bond}"),
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

pub fn bond(
    deps: DepsMut,
    _env: Env,
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
        .zip(primitive_funding_amounts)
        .map(|(pc, funds)| {
            let deposit_stub = BondingStub {
                address: pc.address.clone(),
                bond_response: Option::None,
            };
            deposit_stubs.push(deposit_stub);

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

    let remainder_msg = BankMsg::Send {
        to_address: recipient_addr.to_string(),
        amount: remainder
            .iter()
            .map(|r| Coin {
                denom: r.denom.clone(),
                amount: r.amount,
            })
            .collect(),
    };

    Ok(Response::new()
        .add_attribute("bond_id", bond_seq.to_string())
        .add_messages(bond_msgs?)
        .add_message(remainder_msg))
}

pub fn unbond(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    let start_unbond_response =
        do_start_unbond(deps.branch(), &env, &info, amount)?.unwrap_or(Response::new());

    let unbond_response = do_unbond(deps, &env, &info)?.unwrap_or(Response::new());

    let start_unbond_msgs = start_unbond_response
        .messages
        .iter()
        .map(|sm| sm.msg.clone());
    let unbond_msgs = unbond_response.messages.iter().map(|sm| sm.msg.clone());

    Ok(Response::new()
        .add_messages(start_unbond_msgs)
        .add_messages(unbond_msgs)
        .add_attributes(start_unbond_response.attributes)
        .add_attributes(unbond_response.attributes))
}

pub fn do_start_unbond(
    mut deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    amount: Option<Uint128>,
) -> Result<Option<Response>, ContractError> {
    let unbond_amount = amount.unwrap_or(Uint128::zero());
    if unbond_amount.is_zero() {
        // skip start unbond
        return Ok(None);
    }

    let invest = INVESTMENT.load(deps.storage)?;
    let bond_seq = BONDING_SEQ.load(deps.storage)?;

    // check that user has vault tokens and the amount is > min_withdrawal
    if unbond_amount < invest.min_withdrawal {
        return Err(ContractError::UnbondTooSmall {
            min_bonded: invest.min_withdrawal,
        });
    }

    // burn if balance is more than or equal to amount (handled in execute_burn)
    execute_burn(deps.branch(), env.clone(), info.clone(), unbond_amount)?;

    let mut unbonding_stubs = vec![];

    let num_primitives = Uint128::from(invest.primitives.len() as u128);
    let start_unbond_msgs: Vec<WasmMsg> = invest
        .primitives
        .iter()
        .map(|pc| -> Result<WasmMsg, ContractError> {
            // lets get the amount of tokens to unbond for this primitive
            let primitive_share_amount = unbond_amount.multiply_ratio(
                pc.weight.numerator().checked_mul(num_primitives)?,
                pc.weight.denominator(),
            );

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

    // We need to save the unbonding state for use during the callback
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
            shares: unbond_amount,
        },
    )?;
    BONDING_SEQ_TO_ADDR.save(deps.storage, bond_seq.to_string(), &info.sender.to_string())?;
    BONDING_SEQ.save(deps.storage, &bond_seq.checked_add(Uint128::from(1u128))?)?;

    let mut supply = TOTAL_SUPPLY.load(deps.storage)?;
    supply.issued = supply
        .issued
        .checked_sub(unbond_amount)
        .map_err(StdError::overflow)?;

    TOTAL_SUPPLY.save(deps.storage, &supply)?;

    Ok(Some(
        Response::new()
            .add_messages(start_unbond_msgs)
            .add_attributes(vec![
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
                    value: unbond_amount.to_string(),
                },
                Attribute {
                    key: "bond_id".to_string(),
                    value: bond_seq.to_string(),
                },
            ]),
    ))
}

// find all unbondable pending unbonds where unlock_time < env.block.time
// then trigger unbonds
pub fn do_unbond(
    mut deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
) -> Result<Option<Response>, ContractError> {
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

    Ok(Some(
        Response::new()
            .add_messages(unbond_msgs.clone())
            .add_attributes(vec![
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
            ]),
    ))
}

pub fn find_and_return_unbondable_msgs(
    deps: DepsMut,
    env: &Env,
    _info: &MessageInfo,
    unbond_id: &str,
    unbond_stubs: Vec<UnbondingStub>,
) -> Result<Vec<WasmMsg>, ContractError> {
    // go through unbond_stubs and find ones where unlock_time < env.block.time and execute
    let mut unbond_msgs = vec![];

    for stub in unbond_stubs.iter() {
        let can_unbond = can_unbond_from_primitive(deps.as_ref(), env, unbond_id, &stub)?;

        if (can_unbond) {
            unbond_msgs.push(WasmMsg::Execute {
                contract_addr: stub.address.clone(),
                msg: to_binary(&lp_strategy::msg::ExecuteMsg::Unbond {
                    id: unbond_id.to_string(),
                })?,
                funds: vec![],
            })
        }
    }

    Ok(unbond_msgs)
}

// claim is equivalent to calling unbond with amount: 0
pub fn claim(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    Ok(do_unbond(deps, &env, &info)?.unwrap_or(Response::new()))
}
