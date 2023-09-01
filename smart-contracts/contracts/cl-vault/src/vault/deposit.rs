use std::str::FromStr;

use cosmwasm_std::{
    coin, to_binary, Attribute, BankMsg, Coin, Decimal, DepsMut, Env, Fraction, MessageInfo,
    Response, SubMsg, SubMsgResult, Uint128,
};

use osmosis_std::types::{
    cosmos::bank::v1beta1::BankQuerier,
    osmosis::{
        concentratedliquidity::v1beta1::{ConcentratedliquidityQuerier, MsgCreatePositionResponse},
        tokenfactory::v1beta1::MsgMint,
    },
};

use crate::{
    concentrated_liquidity::{create_position, get_position},
    error::ContractResult,
    msg::{ExecuteMsg, MergePositionMsg},
    reply::Replies,
    state::{CurrentDeposit, CURRENT_DEPOSIT, POOL_CONFIG, POSITION, VAULT_DENOM},
    ContractError,
};
use crate::{helpers::must_pay_two, state::SHARES};

// execute_any_deposit is a nice to have feature for the cl vault.
// but left out of the current release.
pub(crate) fn execute_any_deposit(
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
    info: &MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    // Unwrap recipient or use caller's address
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let pool = POOL_CONFIG.load(deps.storage)?;
    let (token0, token1) = must_pay_two(info, (pool.token0, pool.token1))?;

    let position = POSITION.load(deps.storage)?;
    let range = ConcentratedliquidityQuerier::new(&deps.querier)
        .position_by_id(position.position_id)?
        .position
        .ok_or(ContractError::PositionNotFound)?
        .position
        .ok_or(ContractError::PositionNotFound)?;

    let create_msg = create_position(
        deps.storage,
        &env,
        range.lower_tick,
        range.upper_tick,
        vec![token0.clone(), token1.clone()],
        Uint128::zero(),
        Uint128::zero(),
    )?;

    CURRENT_DEPOSIT.save(
        deps.storage,
        &CurrentDeposit {
            token0_in: token0.amount,
            token1_in: token1.amount,
            sender: recipient,
        },
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_always(
            create_msg,
            Replies::DepositCreatePosition as u64,
        ))
        .add_attribute("method", "exact-deposit")
        .add_attribute("action", "exact-deposit")
        .add_attribute("amount0", token0.amount)
        .add_attribute("amount1", token1.amount))
}

/// handles the reply to creating a position for a user deposit
/// and calculates the refund for the user
pub fn handle_deposit_create_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> ContractResult<Response> {
    let resp: MsgCreatePositionResponse = data.try_into()?;
    let current_deposit = CURRENT_DEPOSIT.load(deps.storage)?;
    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    // we mint shares according to the liquidity created in the position creation
    // this return value is a uint128 with 18 decimals, eg: 101017752467168561172212170
    let liquidity = Decimal::raw(resp.liquidity_created.parse()?);

    let total_position = get_position(deps.storage, &deps.querier, &env)?
        .position
        .ok_or(ContractError::PositionNotFound)?;

    // the total liquidity, an actual decimal, eg: 2020355.049343371223444243"
    let total_liquidity = Decimal::from_str(total_position.liquidity.as_str())?;

    let total_shares: Uint128 = BankQuerier::new(&deps.querier)
        .supply_of(vault_denom.clone())?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();

    let user_shares: Uint128 = if total_shares.is_zero() {
        liquidity.to_uint_floor().try_into().unwrap()
    } else {
        total_shares
            .multiply_ratio(liquidity.numerator(), liquidity.denominator())
            .multiply_ratio(total_liquidity.denominator(), total_liquidity.numerator())
            .try_into()
            .unwrap()
    };

    // TODO the locking of minted shares is a band-aid for giving out rewards to users,
    // once tokenfactory has send hooks, we can remove the lockup and have the users
    // own the shares in their balance
    // we mint shares to the contract address here, so we can lock those shares for the user later in the same call
    // this is blocked by Osmosis v17 update
    let mint = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(coin(user_shares.into(), vault_denom).into()),
        mint_to_address: env.contract.address.to_string(),
    };
    // save the shares in the user map
    SHARES.update(
        deps.storage,
        current_deposit.sender.clone(),
        |old| -> Result<Uint128, ContractError> {
            if let Some(old_shares) = old {
                Ok(user_shares + old_shares)
            } else {
                Ok(user_shares)
            }
        },
    )?;

    // resp.amount0 and resp.amount1 are the amount of tokens used for the position, we want to refund any unused tokens
    // thus we calculate which tokens are not used
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let bank_msg = refund_bank_msg(
        current_deposit.clone(),
        &resp,
        pool_config.token0,
        pool_config.token1,
    )?;

    let position_ids = vec![total_position.position_id, resp.position_id];
    let merge_attrs = vec![Attribute::new("positions", format!("{:?}", position_ids))];
    let merge_msg =
        ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Merge(MergePositionMsg {
            position_ids,
        }));
    // merge our position with the main position
    let merge = SubMsg::reply_on_success(
        cosmwasm_std::WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&merge_msg)?,
            funds: vec![],
        },
        Replies::Merge.into(),
    );

    let mint_attrs = vec![
        Attribute::new("mint-share-amount", user_shares),
        Attribute::new("receiver", current_deposit.sender),
    ];

    //fungify our positions together and mint the user shares to the cl-vault
    let mut response = Response::new()
        .add_submessage(merge)
        .add_attributes(merge_attrs)
        .add_message(mint)
        .add_attributes(mint_attrs)
        .add_attribute("method", "create-position-reply")
        .add_attribute("action", "exact-deposit");

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
    let refund0 = current_deposit
        .token0_in
        .checked_sub(Uint128::new(resp.amount0.parse::<u128>()?))?;

    let refund1 = current_deposit
        .token1_in
        .checked_sub(Uint128::new(resp.amount1.parse::<u128>()?))?;

    let mut attr: Vec<Attribute> = vec![];

    let mut coins: Vec<Coin> = vec![];
    if !refund0.is_zero() {
        attr.push(Attribute::new("refund0-amount", refund0));
        attr.push(Attribute::new("refund0-denom", denom0.as_str()));

        coins.push(coin(refund0.u128(), denom0))
    }
    if !refund1.is_zero() {
        attr.push(Attribute::new("refund1-amount", refund1));
        attr.push(Attribute::new("refund1-denom", denom1.as_str()));

        coins.push(coin(refund1.u128(), denom1))
    }
    let result: Option<(BankMsg, Vec<Attribute>)> = if !coins.is_empty() {
        Some((
            BankMsg::Send {
                to_address: current_deposit.sender.to_string(),
                amount: coins,
            },
            attr,
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
        from_binary,
        testing::{mock_env, MockApi, MockStorage, MOCK_CONTRACT_ADDR},
        to_binary, Addr, Decimal256, Empty, OwnedDeps, Querier, QuerierResult, QueryRequest,
        SubMsgResponse, Uint256, WasmMsg,
    };
    use cosmwasm_std::{Binary, ContractResult as CwContractResult};
    use osmosis_std::types::cosmos::bank::v1beta1::{QuerySupplyOfRequest, QuerySupplyOfResponse};
    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::concentratedliquidity::v1beta1::{
            FullPositionBreakdown, Position as OsmoPosition, PositionByIdRequest,
            PositionByIdResponse,
        },
    };

    use crate::state::{PoolConfig, Position};

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
            Uint128::new(50)
        );
        assert_eq!(
            response.messages[1],
            SubMsg::new(MsgMint {
                sender: env.contract.address.to_string(),
                amount: Some(OsmoCoin {
                    denom: "money".to_string(),
                    amount: 50.to_string()
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

        let user_shares: Uint128 = if total_shares.is_zero() && total_liquidity.is_zero() {
            liquidity.to_uint_floor().try_into().unwrap()
        } else {
            let _ratio = liquidity.checked_div(total_liquidity).unwrap();
            total_shares
                .multiply_ratio(liquidity.numerator(), liquidity.denominator())
                .multiply_ratio(total_liquidity.denominator(), total_liquidity.numerator())
                .try_into()
                .unwrap()
        };

        println!("{}", user_shares);
    }

    #[test]
    fn refund_bank_msg_2_leftover() {
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
                amount: vec![coin(50, "uosmo"), coin(150, "uatom")]
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
                    println!("{}", path.as_str());
                    println!("{}", PositionByIdRequest::TYPE_URL);
                    match path.as_str() {
                        "/osmosis.concentratedliquidity.v1beta1.Query/PositionById" => {
                            let position_by_id_request: PositionByIdRequest =
                                prost::Message::decode(data.as_slice()).unwrap();
                            let position_id = position_by_id_request.position_id;
                            if position_id == self.position.position.clone().unwrap().position_id {
                                QuerierResult::Ok(CwContractResult::Ok(
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
                        "/cosmos.bank.v1beta1.Query/SupplyOf" => {
                            let query_supply_of_request: QuerySupplyOfRequest =
                                prost::Message::decode(data.as_slice()).unwrap();
                            let denom = query_supply_of_request.denom;
                            QuerierResult::Ok(CwContractResult::Ok(
                                to_binary(&QuerySupplyOfResponse {
                                    amount: Some(OsmoCoin {
                                        denom,
                                        amount: 100.to_string(),
                                    }),
                                })
                                .unwrap(),
                            ))
                        }
                        &_ => QuerierResult::Err(cosmwasm_std::SystemError::UnsupportedRequest {
                            kind: format!("Unmocked stargate query path: {path:?}"),
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

    fn mock_deps_with_querier() -> OwnedDeps<MockStorage, MockApi, QuasarQuerier, Empty> {
        OwnedDeps {
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
            }),
            custom_query_type: PhantomData,
        }
    }
}
