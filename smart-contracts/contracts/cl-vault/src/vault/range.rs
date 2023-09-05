use std::str::FromStr;
use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Coin, Decimal, Decimal256, Deps, DepsMut, Env, Fraction, MessageInfo, QuerierWrapper, Response, Storage, SubMsg, SubMsgResult, to_binary, Uint128};

use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin,
    osmosis::{
        concentratedliquidity::{
            v1beta1::{MsgCreatePosition, MsgCreatePositionResponse, MsgWithdrawPosition, MsgWithdrawPositionResponse},
        },
        gamm::v1beta1::MsgSwapExactAmountInResponse,
    },
};

use crate::{
    vault::concentrated_liquidity::get_position,
    helpers::{
        get_deposit_amounts_for_liquidity_needed, get_liquidity_needed_for_tokens, get_spot_price,
        with_slippage,
    },
    math::tick::price_to_tick,
    vault::merge::MergeResponse,
    reply::Replies,
    state::{
        ModifyRangeState, Position, SwapDepositMergeState, MODIFY_RANGE_STATE,
        POOL_CONFIG, POSITION, RANGE_ADMIN, SWAP_DEPOSIT_MERGE_STATE, VAULT_CONFIG,
    },
    swap::swap,
    ContractError,
};
use crate::helpers::{get_single_sided_deposit_0_to_1_swap_amount, get_single_sided_deposit_1_to_0_swap_amount};
use crate::msg::{ExecuteMsg, MergePositionMsg};
use crate::state::{CURRENT_REMAINDERS, CURRENT_SWAP};
use crate::vault::concentrated_liquidity::create_position;

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

pub fn execute_update_range(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    lower_price: Uint128,
    upper_price: Uint128,
    max_slippage: Decimal
) -> Result<Response, ContractError> {
    let storage = deps.storage;
    let querier = deps.querier;

    let lower_tick = price_to_tick(storage, Decimal256::from_atomics(lower_price, 0)?)?;
    let upper_tick = price_to_tick(storage, Decimal256::from_atomics(upper_price, 0)?)?;

    execute_update_range_ticks(
        storage,
        &querier,
        env,
        info,
        lower_tick.try_into().unwrap(),
        upper_tick.try_into().unwrap(),
        max_slippage,
    )
}

/// This function is the entrypoint into the dsm routine that will go through the following steps
/// * how much liq do we have in current range
/// * so how much of each asset given liq would we have at current price
/// * how much of each asset do we need to move to get to new range
/// * deposit up to max liq we can right now, then swap remaining over and deposit again
pub fn execute_update_range_ticks(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: Env,
    info: MessageInfo,
    lower_tick: i64,
    upper_tick: i64,
    max_slippage: Decimal,
) -> Result<Response, ContractError> {
    assert_range_admin(storage, &info.sender)?;

    // todo: prevent re-entrancy by checking if we have anything in MODIFY_RANGE_STATE (redundant check but whatever)

    // this will error if we dont have a position anyway
    let position_breakdown = get_position(storage, querier, &env)?;
    let position = position_breakdown.position.unwrap();

    let withdraw_msg = MsgWithdrawPosition {
        position_id: position.position_id,
        sender: env.contract.address.to_string(),
        liquidity_amount: Decimal::from_str(position.liquidity.as_str())?
            .atomics()
            .to_string(),
    };

    MODIFY_RANGE_STATE.save(
        storage,
        // todo: should ModifyRangeState be an enum?
        &Some(ModifyRangeState {
            lower_tick,
            upper_tick,
            new_range_position_ids: vec![],
            max_slippage,
        }),
    )?;

    Ok(Response::default()
        .add_submessage(
            SubMsg::reply_on_success(withdraw_msg, Replies::WithdrawPosition.into())
        )
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

    let modify_range_state = MODIFY_RANGE_STATE.load(deps.storage)?.unwrap();
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    let amount0 = msg.amount0;
    let amount1 = msg.amount1;

    // should move this into the reply of withdraw position
    let (liquidity_needed_0, liquidity_needed_1) = get_liquidity_needed_for_tokens(
        amount0.clone(),
        amount1.clone(),
        modify_range_state.lower_tick,
        modify_range_state.upper_tick,
    )?;

    let (deposit, remainders) = get_deposit_amounts_for_liquidity_needed(
        liquidity_needed_0,
        liquidity_needed_1,
        amount0,
        amount1,
    )?;

    // Save current remainders at state
    CURRENT_REMAINDERS.save(deps.storage, &remainders)?;

    // we can naively re-deposit up to however much keeps the proportion of tokens the same. Then swap & re-deposit the proper ratio with the remaining tokens
    let create_position_msg = MsgCreatePosition {
        pool_id: pool_config.pool_id,
        sender: env.contract.address.to_string(),
        lower_tick: modify_range_state.lower_tick,
        upper_tick: modify_range_state.upper_tick,
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
        token_min_amount0: with_slippage(deposit.0, modify_range_state.max_slippage)?.to_string(),
        token_min_amount1: with_slippage(deposit.1, modify_range_state.max_slippage)?.to_string(),
    };

    Ok(Response::new()
        .add_submessage(
            SubMsg::reply_on_success(
                create_position_msg,
                Replies::RangeInitialCreatePosition.into(),
            )
        )
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

    // start swap workflow

    // get remaining balance in contract for each token (one of these should be zero i think)
    // @notice: this actually works if this function (do_swap_deposit_merge) is called by
    // handle_initial_create_position_reply, double check if implementing it somewhere else
    let (balance0, balance1) = CURRENT_REMAINDERS.load(storage)?;

    //TODO: further optimizations can be made by increasing the swap amount by half of our expected slippage,
    // to reduce the total number of non-deposited tokens that we will then need to refund
    let (swap_amount, swap_direction) = if !balance0.is_zero() {
        (
            get_single_sided_deposit_0_to_1_swap_amount(
                storage,
                querier,
                balance0,
                target_lower_tick,
                target_upper_tick,
            )?,
            SwapDirection::ZeroToOne,
        )
    } else if !balance1.is_zero() {
        (
            get_single_sided_deposit_1_to_0_swap_amount(
                storage,
                querier,
                balance1,
                target_lower_tick,
                target_upper_tick,
            )?,
            SwapDirection::OneToZero,
        )
    } else {
        // we shouldn't reach here
        panic!("You cannot swap two zero balances");
    };

    // todo check that this math is right with spot price (numerators, denominators) if taken by legacy gamm module instead of poolmanager
    let spot_price = get_spot_price(storage, querier)?;
    let (token_in_denom, token_out_ideal_amount, left_over_amount) = match swap_direction {
        SwapDirection::ZeroToOne => (
            pool_config.token0,
            swap_amount.checked_multiply_ratio(spot_price.numerator(), spot_price.denominator()),
            balance0.checked_sub(swap_amount)?
        ),
        SwapDirection::OneToZero => (
            pool_config.token1,
            swap_amount.checked_multiply_ratio(spot_price.denominator(), spot_price.numerator()),
            balance1.checked_sub(swap_amount)?
        ),
    };

    CURRENT_SWAP.save(storage, &(swap_direction, left_over_amount))?;

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

    Ok(Response::new()
        .add_submessage(
            SubMsg::reply_always(swap_msg, Replies::Swap.into())
        )
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
    let resp: MsgSwapExactAmountInResponse = data.try_into()?;
    let (swap_direction, left_over_amount) = CURRENT_SWAP.load(deps.storage)?;

    let swap_deposit_merge_state = match SWAP_DEPOSIT_MERGE_STATE.may_load(deps.storage)? {
        Some(swap_deposit_merge) => swap_deposit_merge,
        None => return Err(ContractError::SwapDepositMergeStateNotFound {}),
    };

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let modify_range_state = MODIFY_RANGE_STATE.load(deps.storage)?.unwrap();

    // get post swap balances to create positions with

    let (balance0, balance1): (Uint128, Uint128) = match swap_direction {
        SwapDirection::ZeroToOne => (
            left_over_amount,
            Uint128::new(resp.token_out_amount.parse()?),
        ),
        SwapDirection::OneToZero => (
            Uint128::new(resp.token_out_amount.parse()?),
            left_over_amount
        ),
    };

    // Create the position after swapped the leftovers based on swap direction
    let create_position_msg = create_position(
        deps.storage,
        &env,
        swap_deposit_merge_state.target_lower_tick,
        swap_deposit_merge_state.target_upper_tick,
        vec![
            Coin {
                denom: pool_config.token0.clone(),
                amount: balance0,
            },
            Coin {
                denom: pool_config.token1.clone(),
                amount: balance1,
            }
        ],
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(
            SubMsg::reply_always(
                create_position_msg,
                Replies::RangeInitialCreatePosition.into(),
            )
        )
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
            format!("{:?}{:?}", balance0, pool_config.token0),
        )
        .add_attribute(
            "token1",
            format!("{:?}{:?}", balance1, pool_config.token1),
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

    // add the position id to the ones we need to merge
    swap_deposit_merge_state
        .target_range_position_ids
        .push(create_position_message.position_id);

    // call merge
    let merge_msg =
        ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Merge(MergePositionMsg {
            position_ids: swap_deposit_merge_state.target_range_position_ids.clone(),
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

    // clear state to allow for new liquidity movement operations
    SWAP_DEPOSIT_MERGE_STATE.remove(deps.storage);

    Ok(Response::new()
        .add_submessage(merge_submsg)
        .add_attribute("action", "swap_deposit_merge")
        .add_attribute("method", "fungify_positions")
        .add_attribute(
            "position_ids",
            format!("{:?}", swap_deposit_merge_state.target_range_position_ids),
        ))
}

// store new position id and exit
pub fn handle_merge_response(deps: DepsMut, data: SubMsgResult) -> Result<Response, ContractError> {
    let merge_response: MergeResponse = data.try_into()?;

    POSITION.save(
        deps.storage,
        &Position {
            position_id: merge_response.new_position_id,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "swap_deposit_merge")
        .add_attribute("method", "fungify_positions_success")
        .add_attribute("swap_deposit_merge_status", "success")
        .add_attribute("status", "success"))
}

#[cw_serde]
pub enum SwapDirection {
    ZeroToOne,
    OneToZero,
}

#[cfg(test)]
mod tests {
    use std::{marker::PhantomData, str::FromStr};

    use cosmwasm_std::{
        from_binary,
        testing::{
            mock_dependencies, mock_env, mock_info, MockApi, MockStorage, MOCK_CONTRACT_ADDR,
        },
        to_binary, Addr, Binary, ContractResult, Decimal, Empty, MessageInfo, OwnedDeps, Querier,
        QuerierResult, QueryRequest,
    };
    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::concentratedliquidity::v1beta1::{
            FullPositionBreakdown, Position as OsmoPosition, PositionByIdRequest,
            PositionByIdResponse,
        },
    };

    use crate::state::{PoolConfig, VaultConfig, POOL_CONFIG, POSITION, RANGE_ADMIN, VAULT_CONFIG};

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
    fn test_execute_update_range() {
        let info = mock_info("addr0000", &[]);
        let mut deps = mock_deps_with_querier(&info);

        let env = mock_env();
        let lower_price = 1_000_000_000_000_000_000u128;
        let upper_price = 1_000_000_000_000_000_000u128;
        let max_slippage = Decimal::from_str("0.5").unwrap();

        let res = super::execute_update_range(
            deps.as_mut(),
            env,
            info,
            lower_price.into(),
            upper_price.into(),
            max_slippage,
        )
        .unwrap();

        assert_eq!(res.messages.len(), 1);
        assert_eq!(res.attributes[0].value, "modify_range");
        assert_eq!(res.attributes[1].value, "withdraw_position");
        assert_eq!(res.attributes[2].value, "1");
        assert_eq!(res.attributes[3].value, "1000000.1");
    }
}
