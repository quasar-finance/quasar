use cosmwasm_std::{
    Addr, BankMsg, Decimal, DepsMut, Env, MessageInfo, Response, SubMsg, Timestamp, Uint128,
};
use cw20_base::contract::execute_mint;
use quasar_types::callback::{BondResponse, UnbondResponse};

use crate::{
    helpers::update_user_reward_index,
    state::{
        Unbond, BONDING_SEQ_TO_ADDR, BOND_STATE, DEBUG_TOOL, INVESTMENT, PENDING_BOND_IDS,
        PENDING_UNBOND_IDS, TOTAL_SUPPLY, UNBOND_STATE,
    },
    types::FromUint128,
    ContractError,
};

pub fn on_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    share_amount: Uint128,
    bond_id: String,
) -> Result<Response, ContractError> {
    DEBUG_TOOL.save(
        deps.storage,
        &format!("We hit on_bond with bond_id: {bond_id}"),
    )?;

    // load investment info
    let invest = INVESTMENT.load(deps.storage)?;

    let mut bond_stubs = BOND_STATE.load(deps.storage, bond_id.clone())?;

    // lets find the primitive for this response
    let primitive_config = invest.primitives.iter().find(|p| p.address == info.sender);

    // if we don't find a primitive, this is an unauthorized call
    if primitive_config.is_none() {
        return Err(ContractError::Unauthorized {});
    }

    // if we find a bond_stub coming from a primitive that already sent us one. fail
    if bond_stubs
        .iter()
        .any(|s| s.address == info.sender && s.bond_response.is_some())
    {
        return Err(ContractError::DuplicateBondResponse { bond_id });
    }


    // update deposit state here before doing anything else & save!
    bond_stubs.iter_mut().for_each(|s| {
        if s.address == info.sender {
            // we should probably return the primitive value in the bond response
            let primitive_value: lp_strategy::msg::IcaBalanceResponse = deps
                .querier
                .query_wasm_smart(
                    info.sender.clone(),
                    &lp_strategy::msg::QueryMsg::IcaBalance {},
                )
                .unwrap();
            s.bond_response = Some(BondResponse {
                share_amount,
                bond_id: bond_id.clone(),
            });
            s.primitive_value = Some(primitive_value.amount.amount);
        }
    });

    BOND_STATE.save(deps.storage, bond_id.clone(), &bond_stubs)?;

    // if still waiting on successful bonds, then return
    if bond_stubs.iter().any(|s| s.bond_response.is_none()) {
        return Ok(Response::new()
            .add_attribute("action", "on_bond")
            .add_attribute(
                "state",
                bond_stubs
                    .iter()
                    .fold(0u32, |acc, stub| {
                        if stub.bond_response.is_none() {
                            acc + 1
                        } else {
                            acc
                        }
                    })
                    .to_string()
                    + "pending bonds",
            ));
    }
    // at this point we know that the deposit has succeeded fully, and we can mint shares

    let user_address = BONDING_SEQ_TO_ADDR.load(deps.storage, bond_id.clone())?;
    let validated_user_address = deps.api.addr_validate(&user_address)?;
    // lets updated all pending deposit info
    PENDING_BOND_IDS.update(deps.storage, validated_user_address.clone(), |ids| {
        if let Some(mut bond_ids) = ids {
            let bond_index = bond_ids.iter().position(|id| id.eq(&bond_id)).ok_or(
                ContractError::IncorrectCallbackId {
                    expected: bond_id.clone(),
                    ids: bond_ids.clone(),
                },
            )?;
            bond_ids.remove(bond_index);
            Ok::<Vec<String>, ContractError>(bond_ids)
        } else {
            Ok(vec![])
        }
    })?;

    BOND_STATE.save(deps.storage, bond_id, &bond_stubs)?;

    // calculate the shares to mint by value
    // at the time of bonding, we want to figure out what percentage of value in the vault the user has

    // User value per primitive = BondResponse Primitive Shares / Total Primitive Shares  * ICA_BALANCE or funds send in the bond
    // Total Vault Value = Total Vault shares in Primitive / Total Primitive shares * ICA_BALANCE
    let total_vault_value = bond_stubs.iter().try_fold(
        Uint128::zero(),
        |acc, stub| -> Result<Uint128, ContractError> {
            Ok(acc
                + stub
                    .primitive_value
                    .ok_or(ContractError::BondResponseIsEmpty {})?)
        },
    )?;

    // User Vault Shares =  Sum(user value per primitive) / Total Vault value * Total Vault Shares
    let total_user_value = bond_stubs
        .iter()
        .fold(Uint128::zero(), |acc, stub| acc + stub.amount);

    let token_info = cw20_base::contract::query_token_info(deps.as_ref())?;
    // equal to the cw20 base total supply
    let total_vault_shares: Uint128 = token_info.total_supply;

    //if either is zero, then we just mint the user value
    let tmp = total_vault_shares.checked_mul(total_vault_value).unwrap();
    let mut shares_to_mint = Uint128::zero();
    if tmp.is_zero() {
        shares_to_mint = total_user_value;
    } else {
        shares_to_mint =
            total_user_value.checked_multiply_ratio(total_vault_shares, total_vault_value)?;
    };

    // update total supply
    let mut supply = TOTAL_SUPPLY.load(deps.storage)?;

    supply.issued += shares_to_mint;
    TOTAL_SUPPLY.save(deps.storage, &supply)?;

    // call into cw20-base to mint the token, call as self as no one else is allowed
    let sub_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };

    let update_user_rewards_idx_msg =
        update_user_reward_index(deps.as_ref().storage, &validated_user_address)?;
    execute_mint(deps, env, sub_info, user_address, shares_to_mint)?;

    let res = Response::new()
        .add_submessage(SubMsg::new(update_user_rewards_idx_msg))
        .add_attribute("action", "on_bond")
        .add_attribute("from", info.sender)
        .add_attribute("minted", shares_to_mint)
        .add_attribute("new_total_supply", supply.issued.to_string());
    Ok(res)
}

pub fn on_start_unbond(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    unbond_id: String,
    unlock_time: Timestamp,
) -> Result<Response, ContractError> {
    let invest = INVESTMENT.load(deps.storage)?;
    let primitive_config = invest.primitives.iter().find(|p| p.address == info.sender);

    // if we don't find a primitive, this is an unauthorized call
    if primitive_config.is_none() {
        return Err(ContractError::Unauthorized {});
    }

    UNBOND_STATE.update(
        deps.storage,
        unbond_id.clone(),
        |s: Option<Unbond>| -> Result<Unbond, ContractError> {
            let mut unbond = s.ok_or(ContractError::UnbondIsEmpty {})?;
            // update the stub where the address is the same as message sender with the unlock time

            unbond
                .stub
                .iter_mut()
                .find(|s| s.address == info.sender)
                .ok_or(ContractError::UnbondStubIsEmpty {})?
                .unlock_time = Option::Some(unlock_time);
            Ok(Unbond {
                stub: unbond.stub,
                shares: unbond.shares,
            })
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "on_start_unbond")
        .add_attribute("unbond_id", unbond_id)
        .add_attribute("unlock_time", unlock_time.to_string()))
}

pub fn on_unbond(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    unbond_id: String,
) -> Result<Response, ContractError> {
    let invest = INVESTMENT.load(deps.storage)?;
    let primitive_config = invest.primitives.iter().find(|p| p.address == info.sender);

    // if we don't find a primitive, this is an unauthorized call
    if primitive_config.is_none() {
        return Err(ContractError::Unauthorized {});
    }

    let mut unbond_stubs = UNBOND_STATE.load(deps.storage, unbond_id.clone())?;

    // edit and save the stub where the address is the same as message sender with the unbond response
    let mut unbonding_stub = unbond_stubs
        .stub
        .iter_mut()
        .find(|s| s.address == info.sender)
        .ok_or(ContractError::UnbondStubIsEmpty {})?;

    // update info
    unbonding_stub.unbond_response = Option::Some(UnbondResponse {
        unbond_id: unbond_id.clone(),
    });
    unbonding_stub.unbond_funds = info.funds;

    UNBOND_STATE.save(deps.storage, unbond_id.clone(), &unbond_stubs)?;

    // if still waiting on successful unbonds, then return
    // todo: should we eagerly send back funds?
    if unbond_stubs
        .stub
        .iter()
        .any(|s| s.unbond_response.is_none())
    {
        return Ok(Response::new());
    }

    let user_address = BONDING_SEQ_TO_ADDR.load(deps.storage, unbond_id.clone())?;
    // Construct message to return these funds to the user
    let return_msgs: Vec<BankMsg> = unbond_stubs
        .stub
        .iter()
        .map(|s| BankMsg::Send {
            to_address: user_address.to_string(),
            amount: s.unbond_funds.clone(),
        })
        .collect();

    // delete this pending unbond id from the state
    UNBOND_STATE.remove(deps.storage, unbond_id.clone());
    PENDING_UNBOND_IDS.update(
        deps.storage,
        Addr::unchecked(user_address),
        |ids| -> Result<Vec<String>, ContractError> {
            Ok(ids
                .ok_or(ContractError::NoPendingUnbonds {})?
                .into_iter()
                .filter(|id| id != &unbond_id)
                .collect())
        },
    )?;

    Ok(Response::new()
        .add_messages(return_msgs)
        .add_attribute("action", "on_unbond")
        .add_attribute("unbond_id", unbond_id))
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::multitest::common::PrimitiveInstantiateMsg;
    use crate::state::{BondingStub, BOND_STATE};
    use crate::tests::{mock_deps_with_primitives, QuasarQuerier};
    use crate::{
        callback::on_bond,
        msg::PrimitiveConfig,
        multitest::common::{DENOM, LOCAL_DENOM},
        state::{InvestmentInfo, INVESTMENT},
        ContractError,
    };
    use cosmwasm_std::{
        testing::{mock_env, mock_info},
        Decimal,
    };
    use cosmwasm_std::{Addr, Uint128};

    #[test]
    fn fail_if_duplicate_bond_id() {
        let primitive_states = vec![
            (
                "addr00001".to_string(),
                LOCAL_DENOM.to_string(),
                Uint128::from(100u128),
                Uint128::from(100u128),
            ),
            (
                "addr00002".to_string(),
                LOCAL_DENOM.to_string(),
                Uint128::from(200u128),
                Uint128::from(400u128),
            ),
        ];
        // mock the queries so the primitives exist
        let mut deps = mock_deps_with_primitives(primitive_states);
        let env = mock_env();
        let info = mock_info("addr00001", &[]);

        INVESTMENT
            .save(
                &mut deps.storage,
                &InvestmentInfo {
                    primitives: vec![
                        PrimitiveConfig {
                            weight: Decimal::from_str("0.33333333333").unwrap(),
                            address: "addr00001".to_string(),
                            init: crate::msg::PrimitiveInitMsg::LP(PrimitiveInstantiateMsg {
                                lock_period: 64,
                                pool_id: 1,
                                pool_denom: "gamm/pool/1".to_string(),
                                local_denom: LOCAL_DENOM.to_string(),
                                base_denom: DENOM.to_string(),
                                quote_denom: "uatom".to_string(),
                                transfer_channel: "channel-0".to_string(),
                                return_source_channel: "channel-0".to_string(),
                                expected_connection: "connection-0".to_string(),
                            }),
                        },
                        PrimitiveConfig {
                            weight: Decimal::from_str("0.33333333333").unwrap(),
                            address: "addr00002".to_string(),
                            init: crate::msg::PrimitiveInitMsg::LP(PrimitiveInstantiateMsg {
                                lock_period: 64,
                                pool_id: 2,
                                pool_denom: "gamm/pool/1".to_string(),
                                local_denom: LOCAL_DENOM.to_string(),
                                base_denom: DENOM.to_string(),
                                quote_denom: "uatom".to_string(),
                                transfer_channel: "channel-0".to_string(),
                                return_source_channel: "channel-0".to_string(),
                                expected_connection: "connection-0".to_string(),
                            }),
                        },
                    ],
                    owner: Addr::unchecked("owner".to_string()),
                    min_withdrawal: 1u128.into(),
                    deposit_denom: LOCAL_DENOM.to_string(),
                },
            )
            .unwrap();

        let share_amount = 100u128;
        let bond_id = "1".to_string();

        BOND_STATE
            .save(
                &mut deps.storage,
                bond_id.clone(),
                &vec![
                    BondingStub {
                        address: "addr00001".to_string(),
                        bond_response: None,
                        primitive_value: None,
                        amount: Uint128::one(),
                    },
                    BondingStub {
                        address: "addr00002".to_string(),
                        bond_response: None,
                        primitive_value: None,
                        amount: Uint128::one(),
                    },
                ],
            )
            .unwrap();

        // first bond should work
        let res = on_bond(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            share_amount.into(),
            bond_id.clone(),
        )
        .unwrap();
        assert_eq!(0, res.messages.len());

        // second bond should fail
        let res = on_bond(deps.as_mut(), env, info, share_amount.into(), bond_id).unwrap_err();
        match res {
            ContractError::DuplicateBondResponse { .. } => {}
            _ => panic!("Unexpected error: {:?}", res),
        }
    }
}
