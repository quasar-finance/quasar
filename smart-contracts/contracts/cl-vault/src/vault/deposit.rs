use std::str::FromStr;

use cosmwasm_std::{
    attr, coin, to_binary, Addr, Attribute, BankMsg, Coin, Decimal256, DepsMut, Env, Fraction,
    MessageInfo, Order, Response, StdResult, SubMsg, SubMsgResult, Uint128, Uint256,
};

use osmosis_std::types::{
    cosmos::bank::v1beta1::BankQuerier,
    osmosis::{
        concentratedliquidity::v1beta1::{
            ConcentratedliquidityQuerier, MsgAddToPosition, MsgAddToPositionResponse,
            MsgCreatePositionResponse,
        },
        tokenfactory::v1beta1::MsgMint,
    },
};

use crate::{
    error::ContractResult,
    helpers::{
        allocate_funds_per_position, get_asset0_value, get_one_or_two, get_spot_price,
        get_unused_balances, must_pay_one_or_two,
    },
    msg::{ExecuteMsg, MergePositionMsg},
    reply::Replies,
    rewards::CoinList,
    state::{
        CurrentDeposit, CURRENT_DEPOSIT, CURRENT_DEPOSITOR, CURRENT_DEPOSIT_LEFTOVER,
        POOL_CONFIG, POSITIONS, SHARES, VAULT_DENOM,
    },
    vault::concentrated_liquidity::{create_position, get_positions},
    ContractError, query::query_total_assets,
};

// execute_any_deposit is a nice to have feature for the cl vault.
// but left out of the current release.
pub(crate) fn _execute_any_deposit(
    _deps: DepsMut,
    _env: Env,
    _info: &MessageInfo,
    _amount: Uint128,
    _recipient: Option<String>,
) -> Result<Response, ContractError> {
    // Unwrap recipient or use caller's address
    unimplemented!()
}

/// allocate as many user funds as we can to the current positions, after adding to those positions
pub(crate) fn execute_exact_deposit(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    // Unwrap recipient or use caller's address
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;
    CURRENT_DEPOSITOR.save(deps.storage, &recipient)?;

    let positions = get_positions(deps.storage, &deps.querier)?;

    let pool = POOL_CONFIG.load(deps.storage)?;
    let (token0, token1) = must_pay_one_or_two(&info, (pool.token0, pool.token1))?;

    let spot_price = get_spot_price(deps.storage, &deps.querier)?;


    let psf = allocate_funds_per_position(
        deps.branch(),
        positions.clone(),
        spot_price,
        token0.amount,
        token1.amount,
    )?;

    let position_funds = psf.iter().zip(positions);

    // sum up the amount that allocate_funds_per_position allocated to each position
    let mut total_allocated = (Uint128::zero(), Uint128::zero());

    // create a position for each position
    let msgs: Result<Result<Vec<SubMsg>, ContractError>, ContractError> = position_funds
        .map(|((pos1, amount0, amount1), (pos2, fp))| {
            if pos1.position_id != pos2.position_id {
                // this would be a programming error
                panic!("Position ids in the zipped iterator don't match")
            }

            // we want to make sure deposit 1 is item 1 in the queue, deposit 2 item 2 in the queue
            // TODO can this be less complex?
            CURRENT_DEPOSIT.push_back(
                deps.storage,
                &CurrentDeposit {
                    token0_in: *amount0,
                    token1_in: *amount1,
                    refund0: None,
                    refund1: None,
                    liquidity_out: None,
                    original_id: pos1.position_id,
                },
            )?;

            total_allocated = (total_allocated.0 + *amount0, total_allocated.1 + amount1);

            create_position(
                deps.branch(),
                &env,
                fp.position.as_ref().unwrap().lower_tick,
                fp.position.unwrap().upper_tick,
                CoinList::from_coins(vec![
                    coin(amount0.u128(), &token0.denom),
                    coin(amount1.u128(), &token1.denom),
                ])
                .coins(),
                Uint128::zero(),
                Uint128::zero(),
            )
        })
        .map(|msg| {
            msg.map(|m| -> Result<SubMsg<_>, ContractError> {
                Ok(SubMsg::reply_on_success(
                    m,
                    Replies::DepositCreatePosition as u64,
                ))
            })
        })
        .collect();

    // create the mint callback message, and set that after the create_position msges, such that after we have created all positions for the user,
    // we can figure out how to mint by the state of CURRENT_DEPOSIT
    let mint = ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CallbackExecuteMsg(
        crate::msg::CallbackExecuteMsg::MintUserDeposit {},
    ));

    // TODO save any funds we did not use in the positions so we can refund them, allocate_funds_per_position does not guarantee that all funds are used
    CURRENT_DEPOSIT_LEFTOVER.save(
        deps.storage,
        &(
            token0.amount - total_allocated.0,
            token1.amount - total_allocated.1,
        ),
    )?;

    Ok(Response::new()
        .add_submessages(msgs??)
        .add_message(cosmwasm_std::WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&mint)?,
            funds: vec![],
        })
        .add_attribute("method", "exact_deposit")
        .add_attribute("action", "exact_deposit")
        .add_attribute("amount0", token0.amount)
        .add_attribute("amount1", token1.amount))
}

/// handles the reply to adding to a position for the user. The amount of liquidity should be saved here
/// so we can calculate user liquidity later
pub fn handle_deposit_create_position_reply(
    mut deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> ContractResult<Response> {
    let resp: MsgCreatePositionResponse = data.try_into()?;
    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    // we mint shares according to the liquidity created in the position creation
    // this return value is a uint128 with 18 decimals, eg: 101017752467168561172212170
    let user_created_liquidity =
        Decimal256::new(Uint256::from_str(resp.liquidity_created.as_str())?);

    let current = CURRENT_DEPOSIT.pop_front(deps.storage)?.unwrap();

    // save the amounts refunded from this position
    // TODO
    let refunded = (
        current
            .token0_in
            .checked_sub(Uint128::new(resp.amount0.parse::<u128>()?))?,
        current
            .token1_in
            .checked_sub(Uint128::new(resp.amount1.parse::<u128>()?))?,
    );

    // push the updated current deposit to the back of the queue
    CURRENT_DEPOSIT.push_back(
        deps.storage,
        &CurrentDeposit {
            token0_in: current.token0_in,
            token1_in: current.token1_in,
            refund0: Some(refunded.0),
            refund1: Some(refunded.1),
            liquidity_out: Some(user_created_liquidity),
            original_id: current.original_id,
        },
    )?;

    let ratio = POSITIONS.load(deps.storage, current.original_id)?.ratio;

    // TODO this should probably be part of the merging
    // remove the old position before merging
    POSITIONS.remove(deps.storage, current.original_id);

    let merge_msg =
        ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CallbackExecuteMsg(
            crate::msg::CallbackExecuteMsg::Merge(MergePositionMsg {
                ratio,
                position_ids: vec![resp.position_id, current.original_id],
            }),
        ));
    // merge our position with the main position
    let merge_submsg = SubMsg::reply_on_success(
        cosmwasm_std::WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&merge_msg)?,
            funds: vec![],
        },
        Replies::Merge.into(),
    );

    // Merge our positions together and mint the user shares to the cl-vault
    let response = Response::new()
        .add_submessage(merge_submsg)
        .add_attribute("method", "create_position_reply")
        .add_attribute("action", "exact_deposit");

    Ok(response)
}

pub fn execute_mint_callback(mut deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    // process the users deposits
    let mut deposits: Vec<CurrentDeposit> = vec![];
    // empty the current deposit queue
    while !CURRENT_DEPOSIT.is_empty(deps.storage)? {
        let deposit = CURRENT_DEPOSIT.pop_front(deps.storage)?.unwrap();
        deposits.push(deposit);
    }

    let deposited_assets = deposits.iter().try_fold(
        (Uint128::zero(), Uint128::zero()),
        |(acc0, acc1), deposit| -> Result<(Uint128, Uint128), ContractError> {
            Ok((acc0 + deposit.token0_in, acc1 + deposit.token1_in))
        },
    )?;

    let refunded = deposits.iter().try_fold(
        (Uint128::zero(), Uint128::zero()),
        |(acc0, acc1), c| -> Result<(Uint128, Uint128), ContractError> {
            Ok((
                acc0.checked_add(c.refund0.unwrap())?,
                acc1.checked_add(c.refund1.unwrap())?,
            ))
        },
    )?;

    let spot_price = get_spot_price(deps.storage, &deps.querier)?;

    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    let total_vault_shares: Uint256 = BankQuerier::new(&deps.querier)
        .supply_of(vault_denom.clone())?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();

    let leftover = CURRENT_DEPOSIT_LEFTOVER.load(deps.storage)?;

    let vault_assets = query_total_assets(deps.as_ref(), env.clone())?;
    let user_value = get_asset0_value(deposited_assets.0 - refunded.0 + leftover.0, deposited_assets.1 - refunded.1 + leftover.1, spot_price.into())?;


    // the total_vault_value is the amount of vault assets minus the amount 
    let total_vault_value = get_asset0_value(
        vault_assets.token0.amount - (deposited_assets.0 + leftover.0),
        vault_assets.token1.amount - (deposited_assets.1 + leftover.1),
        spot_price.into(),
    )?;
    
    // this depends on the vault being instantiated with some amount of value
    let user_shares: Uint128 = total_vault_shares
        .checked_mul(user_value.into())?
        .checked_div(total_vault_value.into())?
        .try_into()?;

    // TODO the locking of minted shares is a band-aid for giving out rewards to users,
    // once tokenfactory has send hooks, we can remove the lockup and have the users
    // own the shares in their balance
    // we mint shares to the contract address here, so we can lock those shares for the user later in the same call
    // this is blocked by Osmosis v17 update
    let mint_msg = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(coin(user_shares.into(), vault_denom).into()),
        mint_to_address: env.contract.address.to_string(),
    };

    let current_depositor = CURRENT_DEPOSITOR.load(deps.storage)?;

    let mint_attrs = vec![
        attr("mint_shares_amount", user_shares),
        attr("receiver", current_depositor.as_str()),
    ];

    // save the shares in the user map
    SHARES.update(
        deps.branch().storage,
        current_depositor.clone(),
        |old| -> Result<Uint128, ContractError> {
            if let Some(existing_user_shares) = old {
                Ok(user_shares + existing_user_shares)
            } else {
                Ok(user_shares)
            }
        },
    )?;

    let refund_bank_msg = refund_bank_msg(
        deps.branch(),
        refunded.0 + leftover.0,
        refunded.1 + leftover.1,
        current_depositor,
    )?;

    let mut response = Response::new()
        .add_message(mint_msg)
        .add_attributes(mint_attrs)
        .add_attribute("method", "create_position_reply")
        .add_attribute("action", "exact_deposit")
        .add_attribute("used_token0", deposited_assets.0)
        .add_attribute("used_token1", deposited_assets.1);

    // if we have any funds to refund, refund them
    if let Some((msg, attr)) = refund_bank_msg {
        response = response.add_message(msg).add_attributes(attr);
    }

    Ok(response)
}

fn refund_bank_msg(
    deps: DepsMut,
    amount0: Uint128,
    amount1: Uint128,
    depositor: Addr,
) -> Result<Option<(BankMsg, Vec<Attribute>)>, ContractError> {
    let pool = POOL_CONFIG.load(deps.storage)?;

    let token0 = coin(amount0.u128(), pool.token0);
    let token1 = coin(amount1.u128(), pool.token1);

    let mut attributes: Vec<Attribute> = vec![];
    let mut coins: Vec<Coin> = vec![];

    if !token0.amount.is_zero() {
        attributes.push(attr("refund0_amount", token0.amount));
        attributes.push(attr("refund0_denom", token0.denom.as_str()));

        coins.push(token0)
    }
    if !token1.amount.is_zero() {
        attributes.push(attr("refund1_amount", token1.amount));
        attributes.push(attr("refund1_denom", token1.denom.as_str()));

        coins.push(token1)
    }

    let result: Option<(BankMsg, Vec<Attribute>)> = if !coins.is_empty() {
        Some((
            BankMsg::Send {
                to_address: depositor.to_string(),
                amount: coins,
            },
            attributes,
        ))
    } else {
        None
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use cosmwasm_std::{
        testing::{mock_env, MockApi, MockStorage, MOCK_CONTRACT_ADDR},
        to_binary, Addr, Decimal256, Empty, OwnedDeps, SubMsgResponse, Uint256, WasmMsg,
    };

    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::concentratedliquidity::v1beta1::{
            FullPositionBreakdown, Position as OsmoPosition,
        },
    };

    use crate::{
        rewards::CoinList,
        state::{PoolConfig, Position, STRATEGIST_REWARDS},
        test_helpers::QuasarQuerier,
    };

    use super::*;

    // #[test]
    // fn handle_deposit_create_position_works() {
    //     todo!()
    // }

    // #[test]
    // fn test_shares() {
    //     let total_shares = Uint256::from(1000000000_u128);
    //     let total_liquidity = Decimal256::from_str("1000000000").unwrap();
    //     let liquidity = Decimal256::from_str("5000000").unwrap();

    //     let _user_shares: Uint128 = if total_shares.is_zero() && total_liquidity.is_zero() {
    //         liquidity.to_uint_floor().try_into().unwrap()
    //     } else {
    //         let _ratio = liquidity.checked_div(total_liquidity).unwrap();
    //         total_shares
    //             .multiply_ratio(liquidity.numerator(), liquidity.denominator())
    //             .multiply_ratio(total_liquidity.denominator(), total_liquidity.numerator())
    //             .try_into()
    //             .unwrap()
    //     };
    // }

    // #[test]
    // fn refund_bank_msg_2_leftover() {
    //     let deps = mock_deps_with_querier();
    //     let _env = mock_env();
    //     let user = Addr::unchecked("alice");

    //     let current_deposit = CurrentDeposit {
    //         token0_in: Uint128::new(200),
    //         token1_in: Uint128::new(400),
    //         original_id: todo!(),
    //         refund0: todo!(),
    //         refund1: todo!(),
    //         liquidity_out: todo!(),
    //     };
    //     let resp = MsgCreatePositionResponse {
    //         position_id: 1,
    //         amount0: 150.to_string(),
    //         amount1: 250.to_string(),
    //         liquidity_created: "100000.000".to_string(),
    //         lower_tick: 1,
    //         upper_tick: 100,
    //     };
    //     let denom0 = "uosmo".to_string();
    //     let denom1 = "uatom".to_string();

    //     let response = refund_bank_msg(deps, Uint128).unwrap();
    //     assert!(response.is_some());
    //     assert_eq!(
    //         response.unwrap().0,
    //         BankMsg::Send {
    //             to_address: current_deposit.sender.to_string(),
    //             amount: vec![coin(50, "uosmo"), coin(150, "uatom")],
    //         }
    //     )
    // }

    // #[test]
    // fn refund_bank_msg_token1_leftover() {
    //     let _env = mock_env();
    //     let user = Addr::unchecked("alice");

    //     let current_deposit = CurrentDeposit {
    //         token0_in: Uint128::new(200),
    //         token1_in: Uint128::new(400),
    //         sender: user,
    //     };
    //     let resp = MsgCreatePositionResponse {
    //         position_id: 1,
    //         amount0: 200.to_string(),
    //         amount1: 250.to_string(),
    //         liquidity_created: "100000.000".to_string(),
    //         lower_tick: 1,
    //         upper_tick: 100,
    //     };
    //     let denom0 = "uosmo".to_string();
    //     let denom1 = "uatom".to_string();

    //     let response = refund_bank_msg(current_deposit.clone(), &resp, denom0, denom1).unwrap();
    //     assert!(response.is_some());
    //     assert_eq!(
    //         response.unwrap().0,
    //         BankMsg::Send {
    //             to_address: current_deposit.sender.to_string(),
    //             amount: vec![coin(150, "uatom")]
    //         }
    //     )
    // }

    // #[test]
    // fn refund_bank_msg_token0_leftover() {
    //     let _env = mock_env();
    //     let user = Addr::unchecked("alice");

    //     let current_deposit = CurrentDeposit {
    //         token0_in: Uint128::new(200),
    //         token1_in: Uint128::new(400),
    //         sender: user,
    //     };
    //     let resp = MsgCreatePositionResponse {
    //         position_id: 1,
    //         amount0: 150.to_string(),
    //         amount1: 400.to_string(),
    //         liquidity_created: "100000.000".to_string(),
    //         lower_tick: 1,
    //         upper_tick: 100,
    //     };
    //     let denom0 = "uosmo".to_string();
    //     let denom1 = "uatom".to_string();

    //     let response = refund_bank_msg(current_deposit.clone(), &resp, denom0, denom1).unwrap();
    //     assert!(response.is_some());
    //     assert_eq!(
    //         response.unwrap().0,
    //         BankMsg::Send {
    //             to_address: current_deposit.sender.to_string(),
    //             amount: vec![coin(50, "uosmo")]
    //         }
    //     )
    // }

    // #[test]
    // fn refund_bank_msg_none_leftover() {
    //     let _env = mock_env();
    //     let user = Addr::unchecked("alice");

    //     let current_deposit = CurrentDeposit {
    //         token0_in: Uint128::new(200),
    //         token1_in: Uint128::new(400),
    //         sender: user,
    //     };
    //     let resp = MsgCreatePositionResponse {
    //         position_id: 1,
    //         amount0: 200.to_string(),
    //         amount1: 400.to_string(),
    //         liquidity_created: "100000.000".to_string(),
    //         lower_tick: 1,
    //         upper_tick: 100,
    //     };
    //     let denom0 = "uosmo".to_string();
    //     let denom1 = "uatom".to_string();

    //     let response = refund_bank_msg(current_deposit, &resp, denom0, denom1).unwrap();
    //     assert!(response.is_none());
    // }

    // fn mock_deps_with_querier() -> OwnedDeps<MockStorage, MockApi, QuasarQuerier, Empty> {
    //     OwnedDeps {
    //         storage: MockStorage::default(),
    //         api: MockApi::default(),
    //         querier: QuasarQuerier::new(
    //             FullPositionBreakdown {
    //                 position: Some(OsmoPosition {
    //                     position_id: 1,
    //                     address: MOCK_CONTRACT_ADDR.to_string(),
    //                     pool_id: 1,
    //                     lower_tick: 100,
    //                     upper_tick: 1000,
    //                     join_time: None,
    //                     liquidity: "1000000.2".to_string(),
    //                 }),
    //                 asset0: Some(OsmoCoin {
    //                     denom: "token0".to_string(),
    //                     amount: "1000000".to_string(),
    //                 }),
    //                 asset1: Some(OsmoCoin {
    //                     denom: "token1".to_string(),
    //                     amount: "1000000".to_string(),
    //                 }),
    //                 claimable_spread_rewards: vec![
    //                     OsmoCoin {
    //                         denom: "token0".to_string(),
    //                         amount: "100".to_string(),
    //                     },
    //                     OsmoCoin {
    //                         denom: "token1".to_string(),
    //                         amount: "100".to_string(),
    //                     },
    //                 ],
    //                 claimable_incentives: vec![],
    //                 forfeited_incentives: vec![],
    //             },
    //             500,
    //         ),
    //         custom_query_type: PhantomData,
    //     }
    // }
}
