use std::str::FromStr;

use cosmwasm_std::{
    attr, coin, to_binary, Attribute, BankMsg, Coin, Decimal256, DepsMut, Env, Fraction,
    MessageInfo, Response, SubMsg, SubMsgResult, Uint128, Uint256,
};

use osmosis_std::types::{
    cosmos::bank::v1beta1::BankQuerier,
    osmosis::{
        concentratedliquidity::v1beta1::{ConcentratedliquidityQuerier, MsgCreatePositionResponse},
        tokenfactory::v1beta1::MsgMint,
    },
};

use crate::{
    error::ContractResult,
    helpers::{get_liquidity_amount_for_unused_funds, must_pay_one_or_two, sort_tokens},
    msg::{ExecuteMsg, MergePositionMsg},
    reply::Replies,
    state::{CurrentDeposit, CURRENT_DEPOSIT, POOL_CONFIG, POSITION, SHARES, VAULT_DENOM},
    vault::concentrated_liquidity::{create_position, get_position},
    ContractError,
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

/// Try to deposit as much user funds as we can into the a position and
/// refund the rest to the caller
pub(crate) fn execute_exact_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    // Unwrap recipient or use caller's address
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let position_id = (POSITION.load(deps.storage)?).position_id;
    let position = ConcentratedliquidityQuerier::new(&deps.querier)
        .position_by_id(position_id)?
        .position
        .ok_or(ContractError::PositionNotFound)?
        .position
        .ok_or(ContractError::PositionNotFound)?;

    let pool = POOL_CONFIG.load(deps.storage)?;
    let (mut token0, mut token1) = must_pay_one_or_two(&info, (pool.token0, pool.token1))?;

    // Notice: checked_sub has been replaced with saturating_sub due to overflowing response from dex
    token0.amount = token0.amount.saturating_sub(Uint128::one());
    token1.amount = token1.amount.saturating_sub(Uint128::one());

    CURRENT_DEPOSIT.save(
        deps.storage,
        &CurrentDeposit {
            token0_in: token0.amount,
            token1_in: token1.amount,
            sender: recipient,
        },
    )?;

    // Create coins_to_send with no zero amounts
    let mut coins_to_send = vec![];
    if !token0.amount.is_zero() {
        coins_to_send.push(token0.clone());
    }
    if !token1.amount.is_zero() {
        coins_to_send.push(token1.clone());
    }

    let create_position_msg = create_position(
        deps,
        &env,
        position.lower_tick,
        position.upper_tick,
        sort_tokens(coins_to_send),
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            create_position_msg,
            Replies::DepositCreatePosition as u64,
        ))
        .add_attribute("method", "exact_deposit")
        .add_attribute("action", "exact_deposit")
        .add_attribute("amount0", token0.amount)
        .add_attribute("amount1", token1.amount))
}

/// handles the reply to creating a position for a user deposit
/// and calculates the refund for the user
pub fn handle_deposit_create_position_reply(
    mut deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> ContractResult<Response> {
    let create_deposit_position_resp: MsgCreatePositionResponse = data.try_into()?;

    let current_deposit = CURRENT_DEPOSIT.load(deps.storage)?;
    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    // we mint shares according to the liquidity created in the position creation
    // this return value is a uint128 with 18 decimals, eg: 101017752467168561172212170
    let user_created_liquidity = Decimal256::new(Uint256::from_str(
        create_deposit_position_resp.liquidity_created.as_str(),
    )?);

    let existing_position = get_position(deps.storage, &deps.querier)?
        .position
        .ok_or(ContractError::PositionNotFound)?;

    // the total liquidity, an actual decimal, eg: 2020355.049343371223444243"
    let existing_liquidity = Decimal256::from_str(existing_position.liquidity.as_str())?;

    let total_vault_shares: Uint256 = BankQuerier::new(&deps.querier)
        .supply_of(vault_denom.clone())?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();

    // Notice: checked_sub has been replaced with saturating_sub due to overflowing response from dex
    let refunded = (
        current_deposit.token0_in.saturating_sub(Uint128::new(
            create_deposit_position_resp.amount0.parse::<u128>()?,
        )),
        current_deposit.token1_in.saturating_sub(Uint128::new(
            create_deposit_position_resp.amount1.parse::<u128>()?,
        )),
    );

    // total_vault_shares.is_zero() should never be zero. This should ideally always enter the else and we are just sanity checking.
    let user_shares: Uint128 = if total_vault_shares.is_zero() {
        existing_liquidity.to_uint_floor().try_into()?
    } else {
        let liquidity_amount_of_unused_funds: Decimal256 =
            get_liquidity_amount_for_unused_funds(deps.branch(), &env, refunded)?;
        let total_liquidity = existing_liquidity.checked_add(liquidity_amount_of_unused_funds)?;

        // user_shares = total_vault_shares * user_liq / total_liq
        total_vault_shares
            .multiply_ratio(
                user_created_liquidity.numerator(),
                user_created_liquidity.denominator(),
            )
            .multiply_ratio(total_liquidity.denominator(), total_liquidity.numerator())
            .try_into()?
    };

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

    // save the shares in the user map
    SHARES.update(
        deps.storage,
        current_deposit.sender.clone(),
        |old| -> Result<Uint128, ContractError> {
            if let Some(existing_user_shares) = old {
                Ok(user_shares + existing_user_shares)
            } else {
                Ok(user_shares)
            }
        },
    )?;

    // resp.amount0 and resp.amount1 are the amount of tokens used for the position, we want to refund any unused tokens
    // thus we calculate which tokens are not used
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    // TODOSN: Document the following refund_bank_msg purpose
    let bank_msg = refund_bank_msg(
        current_deposit.clone(),
        &create_deposit_position_resp,
        pool_config.token0,
        pool_config.token1,
    )?;

    let position_ids = vec![
        existing_position.position_id,
        create_deposit_position_resp.position_id,
    ];
    let merge_msg =
        ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Merge(MergePositionMsg {
            position_ids,
        }));
    // merge our position with the main position
    let merge_submsg = SubMsg::reply_on_success(
        cosmwasm_std::WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&merge_msg)?,
            funds: vec![],
        },
        Replies::Merge.into(),
    );

    let mint_attrs = vec![
        attr("mint_shares_amount", user_shares),
        attr("receiver", current_deposit.sender.as_str()),
    ];

    // clear out the current deposit since it is no longer needed
    CURRENT_DEPOSIT.remove(deps.storage);

    // Merge our positions together and mint the user shares to the cl-vault
    let mut response = Response::new()
        .add_submessage(merge_submsg)
        .add_attribute(
            "position_ids",
            format!(
                "{},{}",
                existing_position.position_id, create_deposit_position_resp.position_id
            ),
        )
        .add_message(mint_msg)
        .add_attributes(mint_attrs)
        .add_attribute("method", "create_position_reply")
        .add_attribute("action", "exact_deposit");

    // if we have any funds to refund, refund them
    if let Some((msg, attr)) = bank_msg {
        response = response.add_message(msg).add_attributes(attr);
    }

    Ok(response)
}

fn refund_bank_msg(
    current_deposit: CurrentDeposit,
    resp: &MsgCreatePositionResponse,
    denom0: String,
    denom1: String,
) -> Result<Option<(BankMsg, Vec<Attribute>)>, ContractError> {
    // Notice: checked_sub has been replaced with saturating_sub due to overflowing response from dex
    let refund0 = current_deposit
        .token0_in
        .saturating_sub(Uint128::new(resp.amount0.parse::<u128>()?));

    let refund1 = current_deposit
        .token1_in
        .saturating_sub(Uint128::new(resp.amount1.parse::<u128>()?));

    let mut attributes: Vec<Attribute> = vec![];
    let mut coins: Vec<Coin> = vec![];

    // TODOSN: Document this explaining what s happening below
    if !refund0.is_zero() {
        attributes.push(attr("refund0_amount", refund0));
        attributes.push(attr("refund0_denom", denom0.as_str()));

        coins.push(coin(refund0.u128(), denom0))
    }
    if !refund1.is_zero() {
        attributes.push(attr("refund1_amount", refund1));
        attributes.push(attr("refund1_denom", denom1.as_str()));

        coins.push(coin(refund1.u128(), denom1))
    }
    let result: Option<(BankMsg, Vec<Attribute>)> = if !coins.is_empty() {
        Some((
            BankMsg::Send {
                to_address: current_deposit.sender.to_string(),
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

    #[test]
    fn handle_deposit_create_position_works() {
        let mut deps = mock_deps_with_querier();
        let env = mock_env();
        let sender = Addr::unchecked("alice");
        VAULT_DENOM
            .save(deps.as_mut().storage, &"money".to_string())
            .unwrap();
        POSITION
            .save(deps.as_mut().storage, &Position { position_id: 1 })
            .unwrap();

        STRATEGIST_REWARDS
            .save(deps.as_mut().storage, &CoinList::new())
            .unwrap();
        CURRENT_DEPOSIT
            .save(
                deps.as_mut().storage,
                &CurrentDeposit {
                    token0_in: Uint128::new(100),
                    token1_in: Uint128::new(100),
                    sender: sender.clone(),
                },
            )
            .unwrap();
        POOL_CONFIG
            .save(
                deps.as_mut().storage,
                &PoolConfig {
                    pool_id: 1,
                    token0: "token0".to_string(),
                    token1: "token1".to_string(),
                },
            )
            .unwrap();

        let result = SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: Some(
                MsgCreatePositionResponse {
                    position_id: 2,
                    amount0: "100".to_string(),
                    amount1: "100".to_string(),
                    // MsgCreatePositionResponse returns a uint, which represents an 18 decimal in
                    // for the liquidity created to be 500000.1, we expect this number to be 500000100000000000000000
                    liquidity_created: "500000100000000000000000".to_string(),
                    lower_tick: 1,
                    upper_tick: 100,
                }
                .try_into()
                .unwrap(),
            ),
        });

        let response =
            handle_deposit_create_position_reply(deps.as_mut(), env.clone(), result).unwrap();
        assert_eq!(response.messages.len(), 2);
        assert_eq!(
            response.messages[0],
            SubMsg::reply_on_success(
                WasmMsg::Execute {
                    contract_addr: env.contract.address.to_string(),
                    msg: to_binary(&ExecuteMsg::VaultExtension(
                        crate::msg::ExtensionExecuteMsg::Merge(MergePositionMsg {
                            position_ids: vec![1, 2]
                        })
                    ))
                    .unwrap(),
                    funds: vec![]
                },
                Replies::Merge.into()
            )
        );
        // the mint amount is dependent on the liquidity returned by MsgCreatePositionResponse, in this case 50% of current liquidty
        assert_eq!(
            SHARES.load(deps.as_ref().storage, sender).unwrap(),
            Uint128::new(50000)
        );
        assert_eq!(
            response.messages[1],
            SubMsg::new(MsgMint {
                sender: env.contract.address.to_string(),
                amount: Some(OsmoCoin {
                    denom: "money".to_string(),
                    amount: 50000.to_string()
                }),
                mint_to_address: env.contract.address.to_string()
            })
        );
    }

    #[test]
    fn test_shares() {
        let total_shares = Uint256::from(1000000000_u128);
        let total_liquidity = Decimal256::from_str("1000000000").unwrap();
        let liquidity = Decimal256::from_str("5000000").unwrap();

        let _user_shares: Uint128 = if total_shares.is_zero() && total_liquidity.is_zero() {
            liquidity.to_uint_floor().try_into().unwrap()
        } else {
            let _ratio = liquidity.checked_div(total_liquidity).unwrap();
            total_shares
                .multiply_ratio(liquidity.numerator(), liquidity.denominator())
                .multiply_ratio(total_liquidity.denominator(), total_liquidity.numerator())
                .try_into()
                .unwrap()
        };
    }

    #[test]
    fn refund_bank_msg_2_leftover() {
        let _env = mock_env();
        let mut deps = mock_dependencies();
        let user = Addr::unchecked("alice");

        let current_deposit = CurrentDeposit {
            token0_in: Uint128::new(200),
            token1_in: Uint128::new(400),
            sender: user,
        };
        let resp = MsgCreatePositionResponse {
            position_id: 1,
            amount0: 150.to_string(),
            amount1: 250.to_string(),
            liquidity_created: "100000.000".to_string(),
            lower_tick: 1,
            upper_tick: 100,
        };
        let denom0 = "uosmo".to_string();
        let denom1 = "uatom".to_string();

        let response = refund_bank_msg(current_deposit.clone(), &resp, denom0, denom1).unwrap();
        assert!(response.is_some());
        assert_eq!(
            response.unwrap().0,
            BankMsg::Send {
                to_address: current_deposit.sender.to_string(),
                amount: vec![coin(50, "uosmo"), coin(150, "uatom")],
            }
        )
    }

    #[test]
    fn refund_bank_msg_token1_leftover() {
        let _env = mock_env();
        let user = Addr::unchecked("alice");

        let current_deposit = CurrentDeposit {
            token0_in: Uint128::new(200),
            token1_in: Uint128::new(400),
            sender: user,
        };
        let resp = MsgCreatePositionResponse {
            position_id: 1,
            amount0: 200.to_string(),
            amount1: 250.to_string(),
            liquidity_created: "100000.000".to_string(),
            lower_tick: 1,
            upper_tick: 100,
        };
        let denom0 = "uosmo".to_string();
        let denom1 = "uatom".to_string();

        let response = refund_bank_msg(current_deposit.clone(), &resp, denom0, denom1).unwrap();
        assert!(response.is_some());
        assert_eq!(
            response.unwrap().0,
            BankMsg::Send {
                to_address: current_deposit.sender.to_string(),
                amount: vec![coin(150, "uatom")]
            }
        )
    }

    #[test]
    fn refund_bank_msg_token0_leftover() {
        let _env = mock_env();
        let user = Addr::unchecked("alice");

        let current_deposit = CurrentDeposit {
            token0_in: Uint128::new(200),
            token1_in: Uint128::new(400),
            sender: user,
        };
        let resp = MsgCreatePositionResponse {
            position_id: 1,
            amount0: 150.to_string(),
            amount1: 400.to_string(),
            liquidity_created: "100000.000".to_string(),
            lower_tick: 1,
            upper_tick: 100,
        };
        let denom0 = "uosmo".to_string();
        let denom1 = "uatom".to_string();

        let response = refund_bank_msg(current_deposit.clone(), &resp, denom0, denom1).unwrap();
        assert!(response.is_some());
        assert_eq!(
            response.unwrap().0,
            BankMsg::Send {
                to_address: current_deposit.sender.to_string(),
                amount: vec![coin(50, "uosmo")]
            }
        )
    }

    #[test]
    fn refund_bank_msg_none_leftover() {
        let _env = mock_env();
        let user = Addr::unchecked("alice");

        let current_deposit = CurrentDeposit {
            token0_in: Uint128::new(200),
            token1_in: Uint128::new(400),
            sender: user,
        };
        let resp = MsgCreatePositionResponse {
            position_id: 1,
            amount0: 200.to_string(),
            amount1: 400.to_string(),
            liquidity_created: "100000.000".to_string(),
            lower_tick: 1,
            upper_tick: 100,
        };
        let denom0 = "uosmo".to_string();
        let denom1 = "uatom".to_string();

        let response = refund_bank_msg(current_deposit, &resp, denom0, denom1).unwrap();
        assert!(response.is_none());
    }

    fn mock_deps_with_querier() -> OwnedDeps<MockStorage, MockApi, QuasarQuerier, Empty> {
        OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: QuasarQuerier::new(
                FullPositionBreakdown {
                    position: Some(OsmoPosition {
                        position_id: 1,
                        address: MOCK_CONTRACT_ADDR.to_string(),
                        pool_id: 1,
                        lower_tick: 100,
                        upper_tick: 1000,
                        join_time: None,
                        liquidity: "1000000.2".to_string(),
                    }),
                    asset0: Some(OsmoCoin {
                        denom: "token0".to_string(),
                        amount: "1000000".to_string(),
                    }),
                    asset1: Some(OsmoCoin {
                        denom: "token1".to_string(),
                        amount: "1000000".to_string(),
                    }),
                    claimable_spread_rewards: vec![
                        OsmoCoin {
                            denom: "token0".to_string(),
                            amount: "100".to_string(),
                        },
                        OsmoCoin {
                            denom: "token1".to_string(),
                            amount: "100".to_string(),
                        },
                    ],
                    claimable_incentives: vec![],
                    forfeited_incentives: vec![],
                },
                500,
            ),
            custom_query_type: PhantomData,
        }
    }
}
