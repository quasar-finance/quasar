use std::str::FromStr;

use cosmwasm_std::{
    attr, coin, to_json_binary, Attribute, BankMsg, Coin, Decimal256, DepsMut, Env, Fraction,
    MessageInfo, Response, SubMsg, SubMsgResult, Uint128, Uint256,
};

use osmosis_std::types::{
    cosmos::bank::v1beta1::BankQuerier,
    osmosis::{
        concentratedliquidity::v1beta1::{ConcentratedliquidityQuerier, MsgCreatePositionResponse},
        poolmanager::v2::PoolmanagerQuerier,
        tokenfactory::v1beta1::MsgMint,
    },
};

use crate::{
    error::ContractResult,
    helpers::{get_liquidity_amount_for_unused_funds, must_pay_one_or_two, sort_tokens},
    msg::{ExecuteMsg, MergePositionMsg},
    query::query_total_assets,
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

    let pool = POOL_CONFIG.load(deps.storage)?;
    let (token0, token1) = must_pay_one_or_two(&info, (pool.token0, pool.token1))?;

    let deposit_ratio: (Uint128, Uint128) = todo!();

    // get the amount of funds we can deposit from this ratio
    let (deposit, refund): ((Uint128, Uint128), (Uint128, Uint128)) = todo!();

    // calculate the amount of shares we can mint for this
    let total_assets = query_total_assets(deps.as_ref(), env)?;
    let total_assets_value = get_asset0_value(
        deps.storage,
        &deps.querier,
        total_assets.token0.amount,
        total_assets.token1.amount,
    )?;

    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total_vault_shares: Uint256 = BankQuerier::new(&deps.querier)
        .supply_of(vault_denom.clone())?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();

    let user_value = get_asset0_value(deps.storage, &deps.querier, deposit.0, deposit.1)?;

    // total_vault_shares.is_zero() should never be zero. This should ideally always enter the else and we are just sanity checking.
    let user_shares: Uint128 = if total_vault_shares.is_zero() {
        total_assets_value
    } else {
        total_vault_shares
            .checked_mul(user_value.into())?
            .checked_mul(total_assets_value.into())?
            .try_into()?
    };

    // save the shares in the user map
    SHARES.update(
        deps.storage,
        recipient,
        |old| -> Result<Uint128, ContractError> {
            if let Some(existing_user_shares) = old {
                Ok(user_shares + existing_user_shares)
            } else {
                Ok(user_shares)
            }
        },
    )?;

    let mint_attrs = vec![
        attr("mint_shares_amount", user_shares),
        attr("receiver", recipient.as_str()),
    ];

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

    Ok(Response::new()
        .add_attribute("method", "exact_deposit")
        .add_attribute("action", "exact_deposit")
        .add_attribute("amount0", token0.amount)
        .add_attribute("amount1", token1.amount)
        .add_message(mint_msg)
        .add_attributes(mint_attrs)
        .add_attribute("method", "create_position_reply")
        .add_attribute("action", "exact_deposit"))
}

/// Calculate the total value of two assets in asset0
fn get_asset0_value(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    token0: Uint128,
    token1: Uint128,
) -> Result<Uint128, ContractError> {
    let pool_config = POOL_CONFIG.load(storage)?;

    let pm_querier = PoolmanagerQuerier::new(querier);
    let spot_price: Decimal = pm_querier
        .spot_price_v2(pool_config.pool_id, pool_config.token0, pool_config.token1)?
        .spot_price
        .parse()?;

    let total = token0
        .checked_add(token1.multiply_ratio(spot_price.denominator(), spot_price.numerator()))?;

    Ok(total)
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
        to_json_binary, Addr, Decimal256, Empty, OwnedDeps, SubMsgResponse, Uint256, WasmMsg,
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

        // the mint amount is dependent on the liquidity returned by MsgCreatePositionResponse, in this case 50% of current liquidty
        assert_eq!(
            SHARES.load(deps.as_ref().storage, sender).unwrap(),
            Uint128::new(50000)
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
