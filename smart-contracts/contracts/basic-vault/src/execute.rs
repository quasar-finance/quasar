use cosmwasm_std::{attr, coin, to_json_binary, Attribute, Coin, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdError, Uint128, WasmMsg, Addr, CosmosMsg, BankMsg, Event};

use cw20::BalanceResponse;
use cw20_base::contract::execute_burn;
use cw_utils::{must_pay, nonpayable, PaymentError};

use lp_strategy::msg::{IcaBalanceResponse, PrimitiveSharesResponse};
use quasar_types::types::{CoinRatio, CoinWeight};

use crate::error::ContractError;
use crate::helpers::{can_unbond_from_primitive, is_contract_admin, update_user_reward_index};
use crate::msg::PrimitiveConfig;
use crate::state::{
    BondingStub, Cap, InvestmentInfo, Unbond, UnbondingStub, BONDING_SEQ, BONDING_SEQ_TO_ADDR,
    BOND_STATE, CAP, INVESTMENT, PENDING_BOND_IDS, PENDING_UNBOND_IDS, UNBOND_STATE,
};
use crate::types::FromUint128;

// returns amount if the coin is found and amount is non-zero
// errors otherwise
pub fn must_pay_multi(funds: &[Coin], denom: &str) -> Result<Uint128, PaymentError> {
    match funds.iter().find(|c| c.denom == denom) {
        Some(coin) => {
            if coin.amount.is_zero() {
                Err(PaymentError::MissingDenom(denom.to_string()))
            } else {
                Ok(coin.amount)
            }
        }
        None => Err(PaymentError::MissingDenom(denom.to_string())),
    }
}

pub fn get_deposit_amount_weights(
    deps: &Deps,
    primitives: &[PrimitiveConfig],
) -> Result<CoinRatio, ContractError> {
    let weights = primitives
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
            weight: ratio.checked_mul(pc.weight)?,
            denom: balance.amount.denom,
        })
    })
    .collect::<Result<Vec<CoinWeight>, ContractError>>()?;

    let mut ratio = CoinRatio { ratio: weights };
    ratio.normalize()?;

    Ok(ratio)
}

pub fn get_token_amount_weights(
    deposit_amount_weights: &[CoinWeight],
) -> Result<Vec<CoinWeight>, ContractError> {
    deposit_amount_weights.iter().try_fold(
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
    )
}

pub fn get_max_bond(
    funds: &[Coin],
    token_weights: &Vec<CoinWeight>,
) -> Result<Decimal, ContractError> {
    let mut max_bond = Decimal::MAX;
    for coin_weight in token_weights {
        let amount = must_pay_multi(funds, &coin_weight.denom)?;
        let bond_for_token = Decimal::from_uint128(amount).checked_div(coin_weight.weight)?;

        if bond_for_token < max_bond {
            max_bond = bond_for_token;
        }
    }
    Ok(max_bond)
}

pub fn get_deposit_and_remainder_for_ratio(
    funds: &[Coin],
    max_bond: Decimal,
    ratio: &CoinRatio,
) -> Result<(Vec<Coin>, Vec<Coin>), ContractError> {
    // verify that >0 of each token in ratio is passed in, return (funds, remainder))
    // where funds is the max amount we can use in compliance with the ratio
    // and remainder is the change to return to user
    let mut remainder = funds.to_owned();

    let coins: Result<Vec<Coin>, ContractError> = ratio
        .ratio
        .iter()
        .filter(|r| r.weight > Decimal::zero())
        .map(|r| {
            let amount = Decimal::from_uint128(must_pay_multi(funds, &r.denom)?);
            let expected_amount = max_bond.checked_mul(r.weight)?;

            if expected_amount > amount {
                return Err(ContractError::IncorrectBondingRatio {});
            }

            remainder = remainder
                .iter()
                .map(|c| -> Result<Coin, ContractError> {
                    if c.denom == r.denom {
                        Ok(Coin {
                            amount: c.amount.checked_sub(expected_amount.to_uint_floor())?,
                            denom: c.denom.clone(),
                        })
                    } else {
                        Ok(c.clone())
                    }
                })
                .collect::<Result<Vec<Coin>, ContractError>>()?;

            Ok(Coin {
                denom: r.denom.clone(),
                amount: expected_amount.to_uint_floor(),
            })
        })
        .collect();

    Ok((coins?, remainder))
}

pub fn divide_by_ratio(
    funds: Coin,
    invest: InvestmentInfo,
) -> Result<Vec<(Coin, String)>, ContractError> {
    let coins: Result<Vec<(Coin, String)>, cosmwasm_std::OverflowError> = invest
        .primitives
        .iter()
        .map(
            |config| -> Result<(Coin, String), cosmwasm_std::OverflowError> {
                config
                    .weight
                    .checked_mul(Decimal::from_uint128(funds.amount))
                    .map(|dec| {
                        (
                            coin(dec.to_uint_floor().u128(), funds.denom.as_str()),
                            config.address.clone(),
                        )
                    })
            },
        )
        .collect();
    Ok(coins?)
}

pub fn may_pay_with_ratio(
    deps: &Deps,
    funds: &[Coin],
    mut invest: InvestmentInfo,
) -> Result<(Vec<Coin>, Vec<Coin>), ContractError> {
    // normalize primitives
    invest.normalize_primitive_weights();

    // load cached balance of primitive contracts
    let deposit_amount_ratio = get_deposit_amount_weights(deps, &invest.primitives)?;

    if deposit_amount_ratio
        .ratio
        .first()
        .ok_or(ContractError::CoinsWeightVectorIsEmpty {})?
        .weight
        == Decimal::zero()
    {
        return Err(ContractError::Std(StdError::GenericErr {
            msg: "Deposit amount weight for primitive is zero".to_string(),
        }));
    }

    let token_weights: Vec<CoinWeight> = get_token_amount_weights(&deposit_amount_ratio.ratio)?;

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

    let max_bond = get_max_bond(funds, &token_weights)?;

    if max_bond == Decimal::zero() || max_bond == Decimal::MAX {
        return Err(ContractError::Std(StdError::GenericErr {
            msg: format!("Unable to correctly determine max_bond, value: {max_bond}"),
        }));
    }

    let (coins, remainder) =
        get_deposit_and_remainder_for_ratio(funds, max_bond, &deposit_amount_ratio)?;

    if coins
        .first()
        .ok_or(ContractError::CoinsVectorIsEmpty {})?
        .amount
        == Uint128::zero()
    {
        return Err(ContractError::Std(StdError::GenericErr {
            msg: "we failed here".to_string(),
        }));
    }

    Ok((coins, remainder))
}

pub fn bond(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    let invest = INVESTMENT.load(deps.storage)?;

    // load vault info & sequence number
    let bond_seq = BONDING_SEQ.load(deps.storage)?;

    // get the deposited funds
    let amount = must_pay(&info, invest.deposit_denom.as_str())?;

    // find recipient
    let recipient_addr = match recipient {
        Some(r) => deps.api.addr_validate(&r)?,
        None => info.sender,
    };

    let mut deposit_stubs = vec![];
    let divided = divide_by_ratio(coin(amount.u128(), invest.deposit_denom.as_str()), invest)?;

    CAP.update(
        deps.storage,
        |cap| -> Result<crate::state::Cap, ContractError> { cap.update_current(amount) },
    )?;

    let bond_msgs: Result<Vec<WasmMsg>, ContractError> = divided
        .into_iter()
        .map(|(coin, prim_addr)| {
            let deposit_stub = BondingStub {
                address: prim_addr.clone(),
                bond_response: None,
                primitive_value: None,
                amount: coin.amount,
            };
            deposit_stubs.push(deposit_stub);

            Ok(WasmMsg::Execute {
                contract_addr: prim_addr,
                msg: to_json_binary(&lp_strategy::msg::ExecuteMsg::Bond {
                    id: bond_seq.to_string(),
                })?,
                funds: vec![coin],
            })
        })
        .collect();

    // let (primitive_funding_amounts, remainder) =
    //     may_pay_with_ratio(&deps.as_ref(), &info.funds, invest.clone())?;

    // let bond_msgs: Result<Vec<WasmMsg>, ContractError> = invest
    //     .primitives
    //     .iter()
    //     .zip(primitive_funding_amounts)
    //     .map(|(pc, funds)| {
    //         let deposit_stub = BondingStub {
    //             address: pc.address.clone(),
    //             bond_response: None,
    //             primitive_value: None,
    //             amount: funds.amount,
    //         };
    //         deposit_stubs.push(deposit_stub);

    //         Ok(WasmMsg::Execute {
    //             contract_addr: pc.address.clone(),
    //             msg: to_json_binary(&lp_strategy::msg::ExecuteMsg::Bond {
    //                 id: bond_seq.to_string(),
    //             })?,
    //             funds: vec![funds],
    //         })
    //     })
    //     .collect();

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

    // let remainder_msg = BankMsg::Send {
    //     to_address: recipient_addr.to_string(),
    //     amount: remainder
    //         .iter()
    //         .filter(|c| !c.amount.is_zero())
    //         .map(|r| Coin {
    //             denom: r.denom.clone(),
    //             amount: r.amount,
    //         })
    //         .collect(),
    // };

    Ok(Response::new()
        .add_attribute("bond_id", bond_seq.to_string())
        .add_messages(bond_msgs?))
    // .add_message(remainder_msg))
}

pub fn unbond(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let start_unbond_response =
        do_start_unbond(deps.branch(), &env, &info, amount)?.unwrap_or(Response::new());

    let start_unbond_msgs = start_unbond_response
        .messages
        .iter()
        .map(|sm| sm.msg.clone());

    Ok(Response::new()
        .add_messages(start_unbond_msgs)
        .add_attributes(start_unbond_response.attributes))
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
    let update_user_rewards_idx_msg =
        update_user_reward_index(deps.as_ref().storage, &info.sender)?;

    let mut unbonding_stubs = vec![];
    let supply = cw20_base::contract::query_token_info(deps.as_ref())?.total_supply;

    let start_unbond_msgs: Vec<WasmMsg> = invest
        .primitives
        .iter()
        .map(|pc| -> Result<WasmMsg, ContractError> {
            // get this vaults primitive share balance
            let our_balance: BalanceResponse = deps.querier.query_wasm_smart(
                pc.address.clone(),
                &lp_strategy::msg::QueryMsg::Balance {
                    address: env.contract.address.to_string(),
                },
            )?;

            // lets get the amount of tokens to unbond for this primitive: p_unbond_amount = (v_unbond_amount / v_total_supply) * our_p_balance
            let primitive_share_amount = Decimal::from_ratio(unbond_amount, supply)
                .checked_mul(Decimal::from_uint128(our_balance.balance))?
                .to_uint_floor();

            unbonding_stubs.push(UnbondingStub {
                address: pc.address.clone(),
                unlock_time: None,
                unbond_response: None,
                unbond_funds: vec![],
            });

            Ok(WasmMsg::Execute {
                contract_addr: pc.address.clone(),
                msg: to_json_binary(&lp_strategy::msg::ExecuteMsg::StartUnbond {
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

    execute_burn(deps.branch(), env.clone(), info.clone(), unbond_amount)?;
    Ok(Some(
        Response::new()
            .add_messages(start_unbond_msgs)
            .add_message(update_user_rewards_idx_msg)
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
    let pending_unbond_ids_opt = PENDING_UNBOND_IDS.may_load(deps.storage, info.sender.clone())?;

    match pending_unbond_ids_opt {
        Some(pending_unbond_ids) => {
            let mut unbond_msgs: Vec<WasmMsg> = vec![];
            for unbond_id in pending_unbond_ids.iter() {
                let unbond_stubs_opt = UNBOND_STATE.may_load(deps.storage, unbond_id.clone())?;
                if let Some(unbond_stubs) = unbond_stubs_opt {
                    let mut current_unbond_msgs = find_and_return_unbondable_msgs(
                        deps.branch(),
                        env,
                        info,
                        unbond_id,
                        unbond_stubs.stub,
                    )?;
                    unbond_msgs.append(current_unbond_msgs.as_mut());
                }
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
        None => Ok(None),
    }
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
        let can_unbond = can_unbond_from_primitive(deps.as_ref(), env, unbond_id, stub)?;

        if can_unbond {
            unbond_msgs.push(WasmMsg::Execute {
                contract_addr: stub.address.clone(),
                msg: to_json_binary(&lp_strategy::msg::ExecuteMsg::Unbond {
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
    nonpayable(&info)?;

    Ok(do_unbond(deps, &env, &info)?.unwrap_or(Response::new()))
}

pub fn update_cap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_total: Option<Uint128>,
    new_cap_admin: Option<String>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    is_contract_admin(&deps.querier, &env, &info.sender)?;
    let mut attributes = vec![];

    if let Some(new_total) = new_total {
        CAP.update(deps.storage, |c| -> Result<Cap, ContractError> {
            Ok(c.update_total_cap(new_total))
        })?;
        attributes.push(Attribute {
            key: "new_total".to_string(),
            value: new_total.to_string(),
        })
    }

    if let Some(new_cap_admin) = new_cap_admin {
        let new_cap_admin_validated = deps.api.addr_validate(&new_cap_admin)?;
        CAP.update(deps.storage, |c| -> Result<Cap, ContractError> {
            Ok(c.update_cap_admin(new_cap_admin_validated))
        })?;
        attributes.push(Attribute {
            key: "new_cap_admin".to_string(),
            value: new_cap_admin,
        })
    }

    Ok(Response::new()
        .add_attribute("action", "update_cap")
        .add_attributes(attributes)
        .add_attribute("success", "true"))
}

pub fn force_unbond(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    addresses: Vec<String>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    is_contract_admin(&deps.querier, &env, &info.sender)?;
    let mut res = Response::new();
    let investment = INVESTMENT.load(deps.as_ref().storage)?;

    for address in addresses {
        let address = deps.api.addr_validate(&address)?;
        let balance =
            cw20_base::contract::query_balance(deps.as_ref(), address.to_string())?.balance;

        // only unbond if balance is greater than min_withdrawal to avoid getting an error
        if balance > investment.min_withdrawal {
            // workaround to pass the user address instead of the contract admin address
            let user_info = MessageInfo {
                sender: address.clone(),
                funds: vec![],
            };
            let start_unbond_response =
                do_start_unbond(deps.branch(), &env, &user_info, Some(balance))?
                    .unwrap_or(Response::new());

            let start_unbond_msgs = start_unbond_response
                .messages
                .iter()
                .map(|sm| sm.msg.clone());

            res = res
                .add_messages(start_unbond_msgs)
                .add_attributes(start_unbond_response.attributes);
        } else {
            res = res.add_attributes(vec![
                attr("action", "skipped_start_unbond"),
                attr("from", address.to_string()),
            ]);
        }
    }
    Ok(res)
}

pub fn force_claim(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    addresses: Vec<String>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    is_contract_admin(&deps.querier, &env, &info.sender)?;
    let mut res = Response::new();

    for address in addresses {
        let address = deps.api.addr_validate(&address)?;

        let user_info = MessageInfo {
            sender: address.clone(),
            funds: vec![],
        };
        let unbond_response =
            do_unbond(deps.branch(), &env, &user_info)?.unwrap_or(Response::new());

        let unbond_msgs = unbond_response.messages.iter().map(|sm| sm.msg.clone());

        res = res
            .add_messages(unbond_msgs)
            .add_attributes(unbond_response.attributes);
    }
    Ok(res)
}

pub fn execute_transfer_quasar(
    deps: DepsMut,
    env: Env,
    destination_address: Addr,
    amounts: Vec<Coin>,
    sender: Addr,
) -> Result<Response, ContractError> {
    // validate admin
    is_contract_admin(&deps.querier, &env, &sender)?;

    // validate destination address on local chain
    let to_address = deps.api.addr_validate(destination_address.as_str())?;

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: to_address.to_string(),
            amount: amounts,
        }))
        .add_event(Event::new("transfer_on_quasar")
            .add_attribute(
                "destination_address",
                destination_address.clone().to_string().clone(),
            ))
    )
}

#[cfg(test)]
mod tests {
    use crate::callback::on_bond;
    use crate::msg::PrimitiveInitMsg;
    use crate::state::VAULT_REWARDS;
    use crate::tests::{mock_deps_with_primitives, TEST_ADMIN};

    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{attr, Addr, Coin, CosmosMsg, Uint128};

    use cw20_base::state::{MinterData, TokenInfo, TOKEN_INFO};
    use lp_strategy::msg::InstantiateMsg;

    // this test tests 2 on_bond callbacks and a start unbond. The amounts returned slightly 'weird'. The main idea is that after the on_bond callbacks,
    // our user owns 10% of the vault. When we then start to unbond, we expect the user to get 10% of the value in each primitive
    #[test]
    fn test_correct_start_unbond_amount() {
        let primitive_states = vec![
            (
                "contract1".to_string(),
                "ibc/ED07".to_string(),
                // we init state with 1 primitve share being 10 tokens
                Uint128::from(500u128),
                Uint128::from(5000u128),
            ),
            (
                "contract2".to_string(),
                "ibc/ED07".to_string(),
                Uint128::from(500u128),
                Uint128::from(5000u128),
            ),
        ];
        // mock the queries so the primitives exist
        let mut deps = mock_deps_with_primitives(primitive_states);
        let env = mock_env();
        let info = mock_info("user", &[Coin::new(10000, "token")]);

        let instantiate_msg_1 = InstantiateMsg {
            lock_period: 3600,
            pool_id: 2,
            pool_denom: "gamm/pool/2".to_string(),
            local_denom: "ibc/ED07".to_string(),
            base_denom: "uosmo".to_string(),
            quote_denom: "usdc".to_string(),
            transfer_channel: "channel-0".to_string(),
            return_source_channel: "channel-1".to_string(),
            expected_connection: "connection-0".to_string(),
        };

        let instantiate_msg_2 = InstantiateMsg {
            lock_period: 7200,
            pool_id: 5,
            pool_denom: "gamm/pool/5".to_string(),
            local_denom: "ibc/ED07".to_string(),
            base_denom: "uosmo".to_string(),
            quote_denom: "uatom".to_string(),
            transfer_channel: "channel-2".to_string(),
            return_source_channel: "channel-3".to_string(),
            expected_connection: "connection-1".to_string(),
        };

        // set up the contract state
        let min_withdrawal = Uint128::new(100);
        let invest = InvestmentInfo {
            primitives: vec![
                PrimitiveConfig {
                    address: "contract1".to_string(),
                    weight: Decimal::percent(50),
                    init: PrimitiveInitMsg::LP(instantiate_msg_1),
                },
                PrimitiveConfig {
                    address: "contract2".to_string(),
                    weight: Decimal::percent(50),
                    init: PrimitiveInitMsg::LP(instantiate_msg_2),
                },
            ],
            min_withdrawal,
            owner: Addr::unchecked("bob"),
            deposit_denom: "ibc/ED07".to_string(),
        };
        let bond_seq = Uint128::new(1);

        INVESTMENT.save(deps.as_mut().storage, &invest).unwrap();
        BONDING_SEQ.save(deps.as_mut().storage, &bond_seq).unwrap();
        VAULT_REWARDS
            .save(deps.as_mut().storage, &Addr::unchecked("rewards-contract"))
            .unwrap();

        // store token info using cw20-base format
        let token_info = TokenInfo {
            name: "token".to_string(),
            symbol: "token".to_string(),
            decimals: 6,
            total_supply: Uint128::new(5000),
            // set self as minter, so we can properly execute mint and burn
            mint: Some(MinterData {
                minter: env.contract.address.clone(),
                cap: None,
            }),
        };
        TOKEN_INFO.save(deps.as_mut().storage, &token_info).unwrap();

        BONDING_SEQ_TO_ADDR
            .save(deps.as_mut().storage, "1".to_string(), &"user".to_string())
            .unwrap();

        //  mock an unfilfilled stub, do 2 callbacks to fullfill the stubs, and mint shares for the user such that the user owns 50% of the shares
        BOND_STATE
            .save(
                deps.as_mut().storage,
                "1".to_string(),
                &vec![
                    BondingStub {
                        address: "contract1".to_string(),
                        bond_response: None,
                        amount: Uint128::new(5000),
                        primitive_value: None,
                    },
                    BondingStub {
                        address: "contract2".to_string(),
                        bond_response: None,
                        amount: Uint128::new(5000),
                        primitive_value: None,
                    },
                ],
            )
            .unwrap();

        // update the querier to return underlying shares of the vault, in total our vault has 5000 internal shares
        // we expect the vault to unbond 10% of the shares it owns in each primitive, so if contract 1 returns
        // 4000 shares and contract 2 returns 3000 shares, and we unbond 10% of the shares, we should unbond 400 and 300 shares respectively
        deps.querier.update_state(vec![
            // the primitives were mocked with 500 shares and 5000 tokens, we should have deposited 5000 more tokens so get 500 more prim shares
            ("contract1", Uint128::new(1000), Uint128::new(10000)),
            ("contract2", Uint128::new(1000), Uint128::new(10000)),
        ]);

        // we do 2 callbacks, reflecting the updated state of the primitives
        on_bond(
            deps.as_mut(),
            env.clone(),
            MessageInfo {
                sender: Addr::unchecked("contract1"),
                funds: vec![],
            },
            Uint128::new(500),
            "1".to_string(),
        )
        .unwrap();
        on_bond(
            deps.as_mut(),
            env.clone(),
            MessageInfo {
                sender: Addr::unchecked("contract2"),
                funds: vec![],
            },
            Uint128::new(500),
            "1".to_string(),
        )
        .unwrap();

        // start trying withdrawals
        // our succesful withdrawal should show that it is possible for the vault contract to unbond a different amount than 350 and 150 shares

        // case 1: amount is zero, skip start unbond
        let amount = None;
        let res = do_start_unbond(deps.as_mut(), &env, &info, amount).unwrap();
        assert_eq!(res, None);

        // case 2: amount is less than min_withdrawal, error
        let amount = Some(Uint128::new(50));
        let res = do_start_unbond(deps.as_mut(), &env, &info, amount);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ContractError::UnbondTooSmall {
                min_bonded: min_withdrawal
            }
        );

        // case 3: amount is valid, execute start unbond on all primitive contracts
        let amount = cw20_base::contract::query_balance(deps.as_ref(), "user".to_string())
            .unwrap()
            .balance;

        let res = do_start_unbond(deps.as_mut(), &env, &info, Some(amount))
            .unwrap()
            .unwrap();
        assert_eq!(res.attributes.len(), 4);
        assert_eq!(res.messages.len(), 3);

        // check the messages sent to each primitive contract
        let msg1 = &res.messages[0];
        let msg2 = &res.messages[1];

        // start unbond is independent of the amounts in the callback, but is dependent on the vault's amount of shares in the primitive
        assert_eq!(
            msg1.msg,
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "contract1".to_string(),
                msg: to_json_binary(&lp_strategy::msg::ExecuteMsg::StartUnbond {
                    id: bond_seq.to_string(),
                    share_amount: Uint128::new(500),
                })
                .unwrap(),
                funds: vec![],
            })
        );
        assert_eq!(
            msg2.msg,
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "contract2".to_string(),
                msg: to_json_binary(&lp_strategy::msg::ExecuteMsg::StartUnbond {
                    id: bond_seq.to_string(),
                    share_amount: Uint128::new(500),
                })
                .unwrap(),
                funds: vec![],
            })
        );
    }

    #[test]
    fn test_correct_start_unbond_amount_uneven_weights() {
        let primitive_states = vec![
            (
                "contract1".to_string(),
                "ibc/ED07".to_string(),
                // we init state with 1 primitve share being 10 tokens
                Uint128::from(900u128),
                Uint128::from(9000u128),
            ),
            (
                "contract2".to_string(),
                "ibc/ED07".to_string(),
                Uint128::from(100u128),
                Uint128::from(1000u128),
            ),
        ];
        // mock the queries so the primitives exist
        let mut deps = mock_deps_with_primitives(primitive_states);
        let env = mock_env();
        let info = mock_info("user", &[Coin::new(10000, "token")]);

        let instantiate_msg_1 = InstantiateMsg {
            lock_period: 3600,
            pool_id: 2,
            pool_denom: "gamm/pool/2".to_string(),
            local_denom: "ibc/ED07".to_string(),
            base_denom: "uosmo".to_string(),
            quote_denom: "usdc".to_string(),
            transfer_channel: "channel-0".to_string(),
            return_source_channel: "channel-1".to_string(),
            expected_connection: "connection-0".to_string(),
        };

        let instantiate_msg_2 = InstantiateMsg {
            lock_period: 7200,
            pool_id: 5,
            pool_denom: "gamm/pool/5".to_string(),
            local_denom: "ibc/ED07".to_string(),
            base_denom: "uosmo".to_string(),
            quote_denom: "uatom".to_string(),
            transfer_channel: "channel-2".to_string(),
            return_source_channel: "channel-3".to_string(),
            expected_connection: "connection-1".to_string(),
        };

        // set up the contract state
        let min_withdrawal = Uint128::new(100);
        let invest = InvestmentInfo {
            primitives: vec![
                PrimitiveConfig {
                    address: "contract1".to_string(),
                    weight: Decimal::percent(90),
                    init: PrimitiveInitMsg::LP(instantiate_msg_1),
                },
                PrimitiveConfig {
                    address: "contract2".to_string(),
                    weight: Decimal::percent(10),
                    init: PrimitiveInitMsg::LP(instantiate_msg_2),
                },
            ],
            min_withdrawal,
            owner: Addr::unchecked("bob"),
            deposit_denom: "ibc/ED07".to_string(),
        };
        let bond_seq = Uint128::new(1);

        INVESTMENT.save(deps.as_mut().storage, &invest).unwrap();
        BONDING_SEQ.save(deps.as_mut().storage, &bond_seq).unwrap();
        VAULT_REWARDS
            .save(deps.as_mut().storage, &Addr::unchecked("rewards-contract"))
            .unwrap();

        // store token info using cw20-base format
        let token_info = TokenInfo {
            name: "token".to_string(),
            symbol: "token".to_string(),
            decimals: 6,
            total_supply: Uint128::new(5000),
            // set self as minter, so we can properly execute mint and burn
            mint: Some(MinterData {
                minter: env.contract.address.clone(),
                cap: None,
            }),
        };
        TOKEN_INFO.save(deps.as_mut().storage, &token_info).unwrap();

        BONDING_SEQ_TO_ADDR
            .save(deps.as_mut().storage, "1".to_string(), &"user".to_string())
            .unwrap();

        BOND_STATE
            .save(
                deps.as_mut().storage,
                "1".to_string(),
                &vec![
                    BondingStub {
                        address: "contract1".to_string(),
                        bond_response: None,
                        amount: Uint128::new(9000),
                        primitive_value: None,
                    },
                    BondingStub {
                        address: "contract2".to_string(),
                        bond_response: None,
                        amount: Uint128::new(1000),
                        primitive_value: None,
                    },
                ],
            )
            .unwrap();
        // update the querier to return underlying shares of the vault, in total our vault has 5000 internal shares
        // we expect the vault to unbond 10% of the shares it owns in each primitive, so if contract 1 returns
        // 4000 shares and contract 2 returns 3000 shares, and we unbond 10% of the shares, we should unbond 400 and 300 shares respectively
        deps.querier.update_state(vec![
            // the primitives were mocked with 500 shares and 5000 tokens, we should have deposited 5000 more tokens so get 500 more prim shares
            ("contract1", Uint128::new(1800), Uint128::new(18000)),
            ("contract2", Uint128::new(200), Uint128::new(2000)),
        ]);

        // we do 2 callbacks, reflecting the user deposit and the primitive state update
        on_bond(
            deps.as_mut(),
            env.clone(),
            MessageInfo {
                sender: Addr::unchecked("contract1"),
                funds: vec![],
            },
            Uint128::new(900),
            "1".to_string(),
        )
        .unwrap();
        on_bond(
            deps.as_mut(),
            env.clone(),
            MessageInfo {
                sender: Addr::unchecked("contract2"),
                funds: vec![],
            },
            Uint128::new(100),
            "1".to_string(),
        )
        .unwrap();

        // start trying withdrawals
        // our succesful withdrawal should show that it is possible for the vault contract to unbond a different amount than 350 and 150 shares

        // case 1: amount is zero, skip start unbond
        let amount = None;
        let res = do_start_unbond(deps.as_mut(), &env, &info, amount).unwrap();
        assert_eq!(res, None);

        // case 2: amount is less than min_withdrawal, error
        let amount = Some(Uint128::new(50));
        let res = do_start_unbond(deps.as_mut(), &env, &info, amount);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ContractError::UnbondTooSmall {
                min_bonded: min_withdrawal
            }
        );

        // case 3: amount is valid, execute start unbond on all primitive contracts
        let amount = cw20_base::contract::query_balance(deps.as_ref(), "user".to_string())
            .unwrap()
            .balance;
        let _total_supply = cw20_base::contract::query_token_info(deps.as_ref()).unwrap();

        let res = do_start_unbond(deps.as_mut(), &env, &info, Some(amount))
            .unwrap()
            .unwrap();
        assert_eq!(res.attributes.len(), 4);
        assert_eq!(res.messages.len(), 3);

        // check the messages sent to each primitive contract
        let msg1 = &res.messages[0];
        let msg2 = &res.messages[1];

        // start unbond is independent of the amounts in the callback, but is dependent on the vault's amount of shares in the primitive
        // TODO make sure these numbers make sense
        assert_eq!(
            msg1.msg,
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "contract1".to_string(),
                msg: to_json_binary(&lp_strategy::msg::ExecuteMsg::StartUnbond {
                    id: bond_seq.to_string(),
                    share_amount: Uint128::new(900),
                })
                .unwrap(),
                funds: vec![],
            })
        );
        assert_eq!(
            msg2.msg,
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "contract2".to_string(),
                msg: to_json_binary(&lp_strategy::msg::ExecuteMsg::StartUnbond {
                    id: bond_seq.to_string(),
                    share_amount: Uint128::new(100),
                })
                .unwrap(),
                funds: vec![],
            })
        );
    }

    #[test]
    fn test_proper_update_cap() {
        let mut deps = mock_deps_with_primitives(vec![(
            "abc".to_string(),
            "123".to_string(),
            100u128.into(),
            100u128.into(),
        )]);
        let env = mock_env();
        let info = mock_info(TEST_ADMIN, &[]);
        CAP.save(
            &mut deps.storage,
            &Cap::new(Addr::unchecked(TEST_ADMIN.to_string()), Uint128::new(100)),
        )
        .unwrap();

        let cap = Uint128::new(1000);
        let res = update_cap(deps.as_mut(), env.clone(), info.clone(), Some(cap), None).unwrap();
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.attributes[0], attr("action", "update_cap"));
        assert_eq!(res.attributes[1], attr("new_total", cap.to_string()));
        assert_eq!(res.messages.len(), 0);

        // update again
        let cap = Uint128::new(5000);
        let res = update_cap(deps.as_mut(), env.clone(), info.clone(), Some(cap), None).unwrap();
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.attributes[0], attr("action", "update_cap"));
        assert_eq!(res.attributes[1], attr("new_total", cap.to_string()));
        assert_eq!(res.messages.len(), 0);

        // clear cap
        let res = update_cap(deps.as_mut(), env, info, None, None).unwrap();
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.attributes[0], attr("action", "update_cap"));
        assert_eq!(res.messages.len(), 0);
    }

    #[test]
    fn test_unauthorized_update_cap() {
        let mut deps = mock_deps_with_primitives(vec![(
            "abc".to_string(),
            "123".to_string(),
            100u128.into(),
            100u128.into(),
        )]);
        let env = mock_env();
        let info = mock_info("not_admin", &[]);

        CAP.save(
            &mut deps.storage,
            &Cap::new(Addr::unchecked(TEST_ADMIN.to_string()), Uint128::new(100)),
        )
        .unwrap();

        let cap = Uint128::new(1000);
        let res = update_cap(deps.as_mut(), env, info, Some(cap), None);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ContractError::Unauthorized {});
    }
}
