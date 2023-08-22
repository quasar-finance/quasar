use cosmwasm_std::{
    Addr, Decimal256, Deps, DepsMut, Env, Fraction, MessageInfo, QuerierWrapper, Response, Storage,
    SubMsg, SubMsgResult, Uint128,
};

use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin,
    osmosis::{
        concentratedliquidity::{
            self,
            v1beta1::{
                MsgCreatePosition, MsgCreatePositionResponse, MsgFungifyChargedPositionsResponse,
                MsgWithdrawPosition, MsgWithdrawPositionResponse,
            },
        },
        gamm::v1beta1::MsgSwapExactAmountInResponse,
    },
};

use crate::{
    concentrated_liquidity::{create_position, get_position, may_get_position},
    helpers::{
        get_deposit_amounts_for_liquidity_needed, get_liquidity_needed_for_tokens, get_spot_price,
        with_slippage,
    },
    math::tick::price_to_tick,
    reply::Replies,
    state::{
        ModifyRangeState, Position, SwapDepositMergeState, SwapDirection, MODIFY_RANGE_STATE,
        POOL_CONFIG, POSITION, RANGE_ADMIN, SWAP_DEPOSIT_MERGE_STATE, VAULT_CONFIG,
    },
    swap::swap,
    ContractError,
};

fn assert_range_admin(storage: &mut dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let admin = RANGE_ADMIN.load(storage)?;
    if admin != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

fn get_range_admin(deps: Deps) -> Result<Addr, ContractError> {
    Ok(RANGE_ADMIN.load(deps.storage)?)
}

pub fn execute_modify_range(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    lower_price: Uint128,
    upper_price: Uint128,
) -> Result<Response, ContractError> {
    // let lower_tick = price_to_tick(price, exponent_at_price_one)

    let storage = deps.storage;
    let querier = deps.querier;

    let lower_tick = price_to_tick(storage, Decimal256::from_atomics(lower_price, 0)?)?;
    let upper_tick = price_to_tick(storage, Decimal256::from_atomics(upper_price, 0)?)?;

    execute_modify_range_ticks(storage, &querier, env, info, lower_tick, upper_tick)
}

/// This function is the entrypoint into the dsm routine that will go through the following steps
/// * how much liq do we have in current range
/// * so how much of each asset given liq would we have at current price
/// * how much of each asset do we need to move to get to new range
/// * deposit up to max liq we can right now, then swap remaining over and deposit again
pub fn execute_modify_range_ticks(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: Env,
    info: MessageInfo,
    lower_tick: i128,
    upper_tick: i128,
) -> Result<Response, ContractError> {
    assert_range_admin(storage, &info.sender)?;

    // todo: prevent re-entrancy by checking if we have anything in MODIFY_RANGE_STATE (redundant check but whatever)

    let _pool_config = POOL_CONFIG.load(storage)?;
    let _vault_config = VAULT_CONFIG.load(storage)?;

    // this will error if we dont have a position anyway
    let position_breakdown = get_position(storage, querier, &env)?;

    let position = match position_breakdown.position {
        Some(position) => position,
        None => {
            // todo: i guess we can support range modification without a position too? then we'd need to check if we have any balance.
            // however line 83 (6 lines up from here) will error if we dont have a position anyway
            return Err(ContractError::PositionNotFound {});
        }
    };

    let withdraw_msg = MsgWithdrawPosition {
        position_id: position.position_id,
        sender: env.contract.address.to_string(),
        liquidity_amount: position.liquidity.clone(),
    };

    let msg = SubMsg::reply_always(withdraw_msg, Replies::WithdrawPosition.into());

    MODIFY_RANGE_STATE.save(
        storage,
        // todo: should modifyrangestate be an enum?
        &Some(ModifyRangeState {
            lower_tick,
            upper_tick,
            new_range_position_ids: vec![],
        }),
    )?;

    Ok(Response::default()
        .add_submessage(msg)
        .add_attribute("action", "modify_range")
        .add_attribute("method", "withdraw_position")
        .add_attribute("position_id", position.position_id.to_string())
        .add_attribute("liquidity_amount", position.liquidity))
}

// do create new position
pub fn handle_withdraw_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let msg: MsgWithdrawPositionResponse = data.try_into()?;

    let modify_range_state = match MODIFY_RANGE_STATE.load(deps.storage)? {
        Some(modify_range_state) => modify_range_state,
        None => return Err(ContractError::ModifyRangeStateNotFound {}),
    };

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let vault_config = VAULT_CONFIG.load(deps.storage)?;

    let amount0 = msg.amount0;
    let amount1 = msg.amount1;

    // should move this into the reply of withdraw position
    let (liquidity_needed_0, liquidity_needed_1) = get_liquidity_needed_for_tokens(
        amount0.clone(),
        amount1.clone(),
        modify_range_state.lower_tick,
        modify_range_state.upper_tick,
    )?;

    let (deposit, _remainders) = get_deposit_amounts_for_liquidity_needed(
        liquidity_needed_0,
        liquidity_needed_1,
        amount0,
        amount1,
    )?;

    // we can naively re-deposit up to however much keeps the proportion of tokens the same. Then swap & re-deposit the proper ratio with the remaining tokens
    let create_position_msg = MsgCreatePosition {
        pool_id: pool_config.pool_id,
        sender: env.contract.address.to_string(),
        lower_tick: modify_range_state
            .lower_tick
            .try_into()
            .expect("Could not convert lower_tick to i64 from i128"),
        upper_tick: modify_range_state
            .upper_tick
            .try_into()
            .expect("Could not convert upper_tick to i64 from i128"),
        tokens_provided: vec![
            OsmoCoin {
                denom: pool_config.token0.clone(),
                amount: deposit.0.to_string(),
            },
            OsmoCoin {
                denom: pool_config.token1.clone(),
                amount: deposit.1.to_string(),
            },
        ],
        // slippage is a mis-nomer here, we won't suffer any slippage. but the pool may still return us more of one of the tokens. This is fine.
        token_min_amount0: with_slippage(deposit.0, vault_config.create_position_max_slippage)?
            .to_string(),
        token_min_amount1: with_slippage(deposit.1, vault_config.create_position_max_slippage)?
            .to_string(),
    };

    let msg: SubMsg = SubMsg::reply_always(
        create_position_msg,
        Replies::RangeInitialCreatePosition.into(),
    );

    Ok(Response::new()
        .add_submessage(msg)
        .add_attribute("action", "modify_range")
        .add_attribute("method", "create_position")
        .add_attribute("lower_tick", format!("{:?}", modify_range_state.lower_tick))
        .add_attribute("upper_tick", format!("{:?}", modify_range_state.upper_tick))
        .add_attribute("token0", format!("{:?}{:?}", deposit.0, pool_config.token0))
        .add_attribute("token1", format!("{:?}{:?}", deposit.1, pool_config.token1)))
}

// do swap
pub fn handle_initial_create_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let create_position_message: MsgCreatePositionResponse = data.try_into()?;

    // target range for our imminent swap
    let target_lower_tick = create_position_message.lower_tick;
    let target_upper_tick = create_position_message.upper_tick;

    do_swap_deposit_merge(
        deps.storage,
        &deps.querier,
        env,
        target_lower_tick,
        target_upper_tick,
    )
}

// TODO move this to a callback execute msg on the contract?
/// this function assumes that we are swapping and depositing into a valid range
///
/// It also calculates the exact amount we should be swapping based on current balances and the new range
pub fn do_swap_deposit_merge(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: Env,
    target_lower_tick: i64,
    target_upper_tick: i64,
) -> Result<Response, ContractError> {
    // target range for our imminent swap
    let target_lower_tick = create_position_message.lower_tick;
    let target_upper_tick = create_position_message.upper_tick;

    do_swap_deposit_merge(storage, querier, env, target_lower_tick, target_upper_tick)
}

/// this function assumes that we are swapping and depositing into a valid range
///
/// It also calculates the exact amount we should be swapping based on current balances and the new range
pub fn do_swap_deposit_merge(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: Env,
    target_lower_tick: i64,
    target_upper_tick: i64,
) -> Result<Response, ContractError> {
    let swap_deposit_merge_state = SWAP_DEPOSIT_MERGE_STATE.may_load(storage)?;
    if swap_deposit_merge_state.is_some() {
        return Err(ContractError::SwapInProgress {});
    }

    let pool_config = POOL_CONFIG.load(storage)?;
    let vault_config = VAULT_CONFIG.load(storage)?;

    // will we ever call this where current position has other ticks?
    let current_position = match get_position(storage, querier, &env)?.position {
        Some(position) => position,
        None => return Err(ContractError::PositionNotFound {}),
    };

    SWAP_DEPOSIT_MERGE_STATE.save(
        storage,
        &SwapDepositMergeState {
            target_lower_tick,
            target_upper_tick,
            target_range_position_ids: vec![current_position.position_id],
        },
    )?;

    // start swap math
    // get remaining balance in contract for each token (one of these should be zero i think)
    let balance0 =
        querier.query_balance(env.contract.address.clone(), pool_config.token0.clone())?;
    let balance1 =
        querier.query_balance(env.contract.address.clone(), pool_config.token1.clone())?;

    let (swap_amount, swap_direction) = if !balance0.amount.is_zero() {
        (balance0.amount, SwapDirection::ZeroToOne)
    } else if !balance1.amount.is_zero() {
        (balance1.amount, SwapDirection::OneToZero)
    } else {
        // we shouldn't reach here
        (Uint128::zero(), SwapDirection::ZeroToOne)
    };

    // todo check that this math is right with spot price (numerators, denominators)
    let spot_price = get_spot_price(storage, querier)?;
    let (token_in_denom, token_out_ideal_amount) = match swap_direction {
        SwapDirection::ZeroToOne => (
            pool_config.token0,
            swap_amount.checked_multiply_ratio(spot_price.numerator(), spot_price.denominator()),
        ),
        SwapDirection::OneToZero => (
            pool_config.token1,
            swap_amount.checked_multiply_ratio(spot_price.denominator(), spot_price.numerator()),
        ),
    };

    let token_out_min_amount = token_out_ideal_amount?.checked_multiply_ratio(
        vault_config.swap_max_slippage.numerator(),
        vault_config.swap_max_slippage.denominator(),
    )?;

    let swap_msg = swap(
        querier,
        storage,
        &env,
        swap_amount,
        &token_in_denom,
        token_out_min_amount,
    )?;

    let msg: SubMsg = SubMsg::reply_always(swap_msg, Replies::Swap.into());

    Ok(Response::new()
        .add_submessage(msg)
        .add_attribute("action", "swap_deposit_merge")
        .add_attribute("method", "swap")
        .add_attribute("token_in", format!("{:?}{:?}", swap_amount, token_in_denom))
        .add_attribute("token_out_min", format!("{:?}", token_out_min_amount)))
}

// do deposit
pub fn handle_swap_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let msg: MsgSwapExactAmountInResponse = data.try_into()?;

    let swap_deposit_merge_state = match SWAP_DEPOSIT_MERGE_STATE.may_load(deps.storage)? {
        Some(swap_deposit_merge) => swap_deposit_merge,
        None => return Err(ContractError::SwapDepositMergeStateNotFound {}),
    };

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let vault_config = VAULT_CONFIG.load(deps.storage)?;

    // get post swap balances to create positions with
    let balance0 = deps
        .querier
        .query_balance(env.contract.address.clone(), pool_config.token0.clone())?;
    let balance1 = deps
        .querier
        .query_balance(env.contract.address.clone(), pool_config.token1.clone())?;

    // todo: extract this to a function
    let create_position_msg = MsgCreatePosition {
        pool_id: pool_config.pool_id,
        sender: env.contract.address.to_string(),
        lower_tick: swap_deposit_merge_state
            .target_lower_tick
            .try_into()
            .expect("Could not convert lower_tick to i64 from i128"),
        upper_tick: swap_deposit_merge_state
            .target_upper_tick
            .try_into()
            .expect("Could not convert upper_tick to i64 from i128"),
        tokens_provided: vec![
            OsmoCoin {
                denom: pool_config.token0.clone(),
                amount: balance0.amount.to_string(),
            },
            OsmoCoin {
                denom: pool_config.token1.clone(),
                amount: balance1.amount.to_string(),
            },
        ],
        // slippage is a mis-nomer here, we won't suffer any slippage. but the pool may still return us more of one of the tokens. This is fine.
        token_min_amount0: with_slippage(
            balance0.amount,
            vault_config.create_position_max_slippage,
        )?
        .to_string(),
        token_min_amount1: with_slippage(
            balance1.amount,
            vault_config.create_position_max_slippage,
        )?
        .to_string(),
    };

    let msg: SubMsg = SubMsg::reply_always(
        create_position_msg,
        Replies::RangeInitialCreatePosition.into(),
    );

    Ok(Response::new()
        .add_submessage(msg)
        .add_attribute("action", "swap_deposit_merge")
        .add_attribute("method", "create_position2")
        .add_attribute(
            "lower_tick",
            format!("{:?}", swap_deposit_merge_state.target_lower_tick),
        )
        .add_attribute(
            "upper_tick",
            format!("{:?}", swap_deposit_merge_state.target_upper_tick),
        )
        .add_attribute(
            "token0",
            format!("{:?}{:?}", balance0.amount, pool_config.token0),
        )
        .add_attribute(
            "token1",
            format!("{:?}{:?}", balance1.amount, pool_config.token1),
        ))
}

// do merge position & exit
pub fn handle_iteration_create_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let create_position_message: MsgCreatePositionResponse = data.try_into()?;

    let mut swap_deposit_merge_state = match SWAP_DEPOSIT_MERGE_STATE.may_load(deps.storage)? {
        Some(swap_deposit_merge) => swap_deposit_merge,
        None => return Err(ContractError::SwapDepositMergeStateNotFound {}),
    };

    swap_deposit_merge_state
        .target_range_position_ids
        .push(create_position_message.position_id);

    SWAP_DEPOSIT_MERGE_STATE.save(deps.storage, &swap_deposit_merge_state)?;

    let fungify_positions_msg = concentratedliquidity::v1beta1::MsgFungifyChargedPositions {
        position_ids: swap_deposit_merge_state.target_range_position_ids.clone(),
        sender: env.contract.address.to_string(),
    };

    let msg: SubMsg = SubMsg::reply_always(fungify_positions_msg, Replies::Fungify.into());

    Ok(Response::new()
        .add_submessage(msg)
        .add_attribute("action", "swap_deposit_merge")
        .add_attribute("method", "fungify_positions")
        .add_attribute(
            "position_ids",
            format!("{:?}", swap_deposit_merge_state.target_range_position_ids),
        ))
}

// store new position id and exit
pub fn handle_fungify_charged_positions_response(
    storage: &mut dyn Storage,
    _querier: &QuerierWrapper,
    _env: Env,
    _info: MessageInfo,
    fungify_positions_msg: MsgFungifyChargedPositionsResponse,
) -> Result<Response, ContractError> {
    deps.api.debug("fungifying");
    let fungify_positions_msg: MsgFungifyChargedPositionsResponse = data.try_into()?;

    SWAP_DEPOSIT_MERGE_STATE.remove(deps.storage);
    POSITION.save(
        deps.storage,
        &Position {
            position_id: fungify_positions_msg.new_position_id,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "swap_deposit_merge")
        .add_attribute("method", "fungify_positions_success")
        .add_attribute("swap_deposit_merge_status", "success")
        .add_attribute("status", "success"))
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use cosmwasm_std::{
        from_binary,
        testing::{
            mock_dependencies, mock_env, mock_info, MockApi, MockStorage, MOCK_CONTRACT_ADDR,
        },
        to_binary, Addr, Binary, ContractResult, Decimal, Empty, MessageInfo, OwnedDeps, Querier,
        QuerierResult, QueryRequest, Storage, Timestamp,
    };
    use osmosis_std::{
        shim::Any,
        types::{
            cosmos::base::v1beta1::Coin as OsmoCoin,
            osmosis::concentratedliquidity::v1beta1::{
                FullPositionBreakdown, Position as OsmoPosition, PositionByIdRequest,
                PositionByIdResponse,
            },
        },
    };
    use prost::Message;

    use crate::state::{
        PoolConfig, Position, VaultConfig, POOL_CONFIG, POSITION, RANGE_ADMIN, VAULT_CONFIG,
    };

    pub struct QuasarQuerier {
        position: FullPositionBreakdown,
    }

    impl QuasarQuerier {
        pub fn new(position: FullPositionBreakdown) -> QuasarQuerier {
            QuasarQuerier { position }
        }
    }

    impl Querier for QuasarQuerier {
        fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
            let request: QueryRequest<Empty> = from_binary(&Binary::from(bin_request)).unwrap();
            match request {
                QueryRequest::Stargate { path, data } => {
                    match prost::Message::decode(data.as_slice()).unwrap() {
                        PositionByIdRequest { position_id } => {
                            if position_id == self.position.position.clone().unwrap().position_id {
                                QuerierResult::Ok(ContractResult::Ok(
                                    to_binary(&PositionByIdResponse {
                                        position: Some(self.position.clone()),
                                    })
                                    .unwrap(),
                                ))
                            } else {
                                QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                                    kind: format!("position id not found: {position_id:?}"),
                                })
                            }
                        }
                        _ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                            kind: format!("Unmocked wasm query type: {path:?}"),
                        }),
                    }
                }
                _ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                    kind: format!("Unmocked query type: {request:?}"),
                }),
            }
            // QuerierResult::Ok(ContractResult::Ok(to_binary(&"hello").unwrap()))
        }
    }

    fn mock_deps_with_querier(
        info: &MessageInfo,
    ) -> OwnedDeps<MockStorage, MockApi, QuasarQuerier, Empty> {
        let mut deps = OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: QuasarQuerier::new(FullPositionBreakdown {
                position: Some(OsmoPosition {
                    position_id: 1,
                    address: MOCK_CONTRACT_ADDR.to_string(),
                    pool_id: 1,
                    lower_tick: 100,
                    upper_tick: 1000,
                    join_time: None,
                    liquidity: "1000000.1".to_string(),
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
            }),
            custom_query_type: PhantomData,
        };

        let storage = &mut deps.storage;

        RANGE_ADMIN.save(storage, &info.sender).unwrap();
        POOL_CONFIG
            .save(
                storage,
                &PoolConfig {
                    pool_id: 1,
                    token0: "token0".to_string(),
                    token1: "token1".to_string(),
                },
            )
            .unwrap();
        VAULT_CONFIG
            .save(
                storage,
                &VaultConfig {
                    performance_fee: Decimal::zero(),
                    treasury: Addr::unchecked("treasure"),
                    create_position_max_slippage: Decimal::from_ratio(1u128, 20u128),
                    swap_max_slippage: Decimal::from_ratio(1u128, 20u128),
                },
            )
            .unwrap();
        POSITION
            .save(storage, &crate::state::Position { position_id: 1 })
            .unwrap();

        deps
    }

    #[test]
    fn test_assert_range_admin() {
        let mut deps = mock_dependencies();
        let info = mock_info("addr0000", &[]);

        RANGE_ADMIN.save(&mut deps.storage, &info.sender).unwrap();

        super::assert_range_admin(&mut deps.storage, &info.sender).unwrap();

        let info = mock_info("addr0001", &[]);
        super::assert_range_admin(&mut deps.storage, &info.sender).unwrap_err();

        let info = mock_info("addr0000", &[]);
        RANGE_ADMIN.save(&mut deps.storage, &info.sender).unwrap();

        super::assert_range_admin(&mut deps.storage, &Addr::unchecked("someoneelse")).unwrap_err();
    }

    #[test]
    fn test_get_range_admin() {
        let mut deps = mock_dependencies();
        let info = mock_info("addr0000", &[]);

        RANGE_ADMIN.save(&mut deps.storage, &info.sender).unwrap();

        assert_eq!(super::get_range_admin(deps.as_ref()).unwrap(), info.sender);
    }

    #[test]
    fn test_execute_modify_range() {
        let info = mock_info("addr0000", &[]);
        let mut deps = mock_deps_with_querier(&info);

        let env = mock_env();
        let lower_price = 1_000_000_000_000_000_000u128;
        let upper_price = 1_000_000_000_000_000_000u128;

        let res = super::execute_modify_range(
            deps.as_mut(),
            env,
            info,
            lower_price.into(),
            upper_price.into(),
        )
        .unwrap();

        assert_eq!(res.messages.len(), 1);
        assert_eq!(res.attributes[0].value, "modify_range");
        assert_eq!(res.attributes[1].value, "withdraw_position");
        assert_eq!(res.attributes[2].value, "1");
        assert_eq!(res.attributes[3].value, "1000000.1");
    }
}
