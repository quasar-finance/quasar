use std::str::FromStr;

use apollo_cw_asset::{Asset, AssetInfo};
use cosmwasm_std::{
    coin, BankMsg, Binary, Coin, Decimal, DepsMut, Env, Fraction, MessageInfo, Response, SubMsg,
    SubMsgResult, Uint128,
};
use cw_dex_router::helpers::receive_asset;

use osmosis_std::types::{
    cosmos::bank::v1beta1::BankQuerier,
    osmosis::{
        concentratedliquidity::v1beta1::{
            ConcentratedliquidityQuerier, MsgCreatePositionResponse, MsgFungifyChargedPositions,
        },
        tokenfactory::v1beta1::MsgMint,
    },
};

use crate::state::LOCKED_SHARES;
use crate::{
    concentrated_liquidity::{create_position, get_position},
    error::ContractResult,
    reply::Replies,
    state::{CurrentDeposit, PoolConfig, CURRENT_DEPOSIT, POOL_CONFIG, POSITION, VAULT_DENOM},
    ContractError,
};

pub(crate) fn execute_deposit(
    deps: DepsMut,
    env: Env,
    info: &MessageInfo,
    amount: Uint128,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    // Unwrap recipient or use caller's address
    let _recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let pool_config = POOL_CONFIG.load(deps.storage)?;

    // Receive the assets to the contract
    let _receive_res = receive_asset(
        info,
        &env,
        &Asset::new(AssetInfo::Native(pool_config.token0), amount),
    )?;

    todo!()

    // // Compound. Also stakes the users deposit
    // let compound_res = self.compound(deps, &env, user_deposit_amount)?;

    // // Mint vault tokens to recipient
    // let mint_res = Response::new().add_message(
    //     CallbackMsg::MintVaultToken {
    //         amount,
    //         recipient: recipient.clone(),
    //     }
    //     .into_cosmos_msg(&env)?,
    // );

    // let event = Event::new("apollo/vaults/execute_staking").add_attributes(vec![
    //     attr("action", "deposit"),
    //     attr("recipient", recipient),
    //     attr("amount", amount),
    // ]);

    // // Merge responses and add message to mint vault token
    // Ok(merge_responses(vec![receive_res, compound_res, mint_res]).add_event(event))
}

pub(crate) fn execute_multi_deposit(
    deps: DepsMut,
    env: Env,
    info: &MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    // Unwrap recipient or use caller's address
    let recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let pool = POOL_CONFIG.load(deps.storage)?;
    let (token0, token1) = must_pay_two(&info, (pool.token0, pool.token1))?;

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

    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        create_msg,
        Replies::DepositCreatePosition as u64,
    )))
}

pub fn handle_deposit_create_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> ContractResult<Response> {
    let resp: MsgCreatePositionResponse = data.try_into()?;
    let current_deposit = CURRENT_DEPOSIT.load(deps.storage)?;
    let bq = BankQuerier::new(&deps.querier);
    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    // we mint shares according to the liquidity created
    let liquidity = Decimal::from_str(resp.liquidity_created.as_str())?;

    let total_position = get_position(deps.storage, &deps.querier, &env)?
        .position
        .ok_or(ContractError::PositionNotFound)?;
    let total_liquidity = Decimal::from_str(total_position.liquidity.as_str())?;

    // TODO change error type to something more descriptive
    let total_shares: Uint128 = bq
        .supply_of(vault_denom.clone())?
        .amount
        .ok_or(ContractError::IncorrectShares)?
        .amount
        .parse::<u128>()?
        .into();

    let ratio = liquidity.checked_div(total_liquidity)?;
    let user_shares = total_shares.multiply_ratio(ratio.numerator(), ratio.denominator());

    // TODO the locking of minted shares is a band-aid for giving out rewards to users,
    // once tokenfactory has send hooks, we can remove the lockup and have the users
    // own the shares in their balance
    // we mint shares to the contract address here, so we can lock those shares for the user later in the same call
    let mint = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(coin(user_shares.u128(), vault_denom).into()),
        mint_to_address: env.contract.address.to_string(),
    };
    // save the shares in the user map
    LOCKED_SHARES.update(
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
        &env,
        current_deposit,
        &resp,
        pool_config.token0,
        pool_config.token1,
    )?;

    // merge our position with the main position
    let fungify = SubMsg::reply_on_success(
        MsgFungifyChargedPositions {
            position_ids: vec![total_position.position_id, resp.position_id],
            sender: env.contract.address.to_string(),
        },
        Replies::Fungify.into(),
    );

    //fungify our positions together and mint the user shares to the cl-vault
    let mut response = Response::new().add_submessage(fungify).add_message(mint);

    // if we have any funds to refund, refund them
    if let Some(msg) = bank_msg {
        response = response.add_message(msg);
    }

    Ok(response)
}

fn refund_bank_msg(
    env: &Env,
    current_deposit: CurrentDeposit,
    resp: &MsgCreatePositionResponse,
    denom0: String,
    denom1: String,
) -> Result<Option<BankMsg>, ContractError> {
    let refund0 = current_deposit
        .token0_in
        .checked_sub(Uint128::new(resp.amount0.parse::<u128>()?))?;

    let refund1 = current_deposit
        .token1_in
        .checked_sub(Uint128::new(resp.amount1.parse::<u128>()?))?;

    let mut coins: Vec<Coin> = vec![];
    if !refund0.is_zero() {
        coins.push(coin(refund0.u128(), denom0))
    }
    if !refund1.is_zero() {
        coins.push(coin(refund1.u128(), denom1))
    }
    let bank_msg: Option<BankMsg> = if !coins.is_empty() {
        Some(BankMsg::Send {
            to_address: current_deposit.sender.to_string(),
            amount: coins,
        })
    } else {
        None
    };
    Ok(bank_msg)
}

/// returns the Coin of the needed denoms in the order given in denoms

fn must_pay_two(info: &MessageInfo, denoms: (String, String)) -> ContractResult<(Coin, Coin)> {
    if info.funds.len() != 2 {
        return Err(cw_utils::PaymentError::MultipleDenoms {}.into());
    }

    let token0 = info
        .funds
        .clone()
        .into_iter()
        .find(|coin| coin.denom == denoms.0)
        .ok_or(cw_utils::PaymentError::MissingDenom(denoms.0))?;

    let token1 = info
        .funds
        .clone()
        .into_iter()
        .find(|coin| coin.denom == denoms.1)
        .ok_or(cw_utils::PaymentError::MissingDenom(denoms.1))?;

    Ok((token0, token1))
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use cosmwasm_std::ContractResult as CwContractResult;
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, MockApi, MockStorage, MOCK_CONTRACT_ADDR},
        to_binary, Addr, Empty, OwnedDeps, Querier, QuerierResult, QueryRequest, SubMsgResponse,
    };
    use osmosis_std::types::cosmos::bank::v1beta1::{QuerySupplyOfRequest, QuerySupplyOfResponse};
    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::concentratedliquidity::v1beta1::{
            FullPositionBreakdown, Position as OsmoPosition, PositionByIdRequest,
            PositionByIdResponse,
        },
    };
    use prost::Message;

    use crate::state::Position;

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
        POOL_CONFIG.save(
            deps.as_mut().storage,
            &PoolConfig {
                pool_id: 1,
                token0: "token0".to_string(),
                token1: "token1".to_string(),
            },
        ).unwrap();

        let result = SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: Some(
                MsgCreatePositionResponse {
                    position_id: 2,
                    amount0: "100".to_string(),
                    amount1: "100".to_string(),
                    liquidity_created: "500000.1".to_string(),
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
                MsgFungifyChargedPositions {
                    position_ids: vec![1, 2],
                    sender: env.contract.address.to_string()
                },
                Replies::Fungify.into()
            )
        );
        // the mint amount is dependent on the liquidity returned by MsgCreatePositionResponse, in this case 50% of current liquidty
        assert_eq!(
            LOCKED_SHARES.load(deps.as_ref().storage, sender).unwrap(),
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
    fn refund_bank_msg_2_leftover() {
        let env = mock_env();
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

        let response =
            refund_bank_msg(&env, current_deposit.clone(), &resp, denom0, denom1).unwrap();
        assert!(response.is_some());
        assert_eq!(
            response.unwrap(),
            BankMsg::Send {
                to_address: current_deposit.sender.to_string(),
                amount: vec![coin(50, "uosmo"), coin(150, "uatom")]
            }
        )
    }

    #[test]
    fn refund_bank_msg_token1_leftover() {
        let env = mock_env();
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

        let response =
            refund_bank_msg(&env, current_deposit.clone(), &resp, denom0, denom1).unwrap();
        assert!(response.is_some());
        assert_eq!(
            response.unwrap(),
            BankMsg::Send {
                to_address: current_deposit.sender.to_string(),
                amount: vec![coin(150, "uatom")]
            }
        )
    }

    #[test]
    fn refund_bank_msg_token0_leftover() {
        let env = mock_env();
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

        let response =
            refund_bank_msg(&env, current_deposit.clone(), &resp, denom0, denom1).unwrap();
        assert!(response.is_some());
        assert_eq!(
            response.unwrap(),
            BankMsg::Send {
                to_address: current_deposit.sender.to_string(),
                amount: vec![coin(50, "uosmo")]
            }
        )
    }

    #[test]
    fn refund_bank_msg_none_leftover() {
        let env = mock_env();
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

        let response =
            refund_bank_msg(&env, current_deposit.clone(), &resp, denom0, denom1).unwrap();
        assert!(response.is_none());
    }

    #[test]
    fn must_pay_two_works_ordered() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![expected0.clone(), expected1.clone()],
        };
        let (token0, token1) =
            must_pay_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!(expected0, token0);
        assert_eq!(expected1, token1);
    }

    #[test]
    fn must_pay_two_works_unordered() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![expected1.clone(), expected0.clone()],
        };
        let (token0, token1) =
            must_pay_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!(expected0, token0);
        assert_eq!(expected1, token1);
    }

    #[test]
    fn must_pay_two_rejects_three() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![expected1.clone(), expected0.clone(), coin(200, "uqsr")],
        };
        let err = must_pay_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap_err();
    }

    #[test]
    fn must_pay_two_rejects_one() {
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![coin(200, "uqsr")],
        };
        let err = must_pay_two(&info, ("uatom".to_string(), "uosmo".to_string())).unwrap_err();
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
        };
        deps
    }
}
