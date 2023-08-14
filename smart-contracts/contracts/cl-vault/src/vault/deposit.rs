use std::str::FromStr;

use apollo_cw_asset::{Asset, AssetInfo};
use cosmwasm_std::{
    coin, BankMsg, Binary, Coin, Decimal, DepsMut, Env, Fraction, MessageInfo, Response, SubMsg,
    Uint128,
};
use cw_dex_router::helpers::receive_asset;

use osmosis_std::try_proto_to_cosmwasm_coins;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMintResponse;
use osmosis_std::types::{
    cosmos::bank::v1beta1::BankQuerier,
    osmosis::{
        concentratedliquidity::v1beta1::{
            ConcentratedliquidityQuerier, MsgCreatePositionResponse, MsgFungifyChargedPositions,
        },
        tokenfactory::v1beta1::{MsgMint, TokenfactoryQuerier},
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
        vec![token0, token1],
        Uint128::zero(),
        Uint128::zero(),
    )?;

    CURRENT_DEPOSIT.save(
        deps.storage,
        &CurrentDeposit {
            token0_in: token0.amount,
            token1_in: token1.amount,
            sender: info.sender,
        },
    );

    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        create_msg,
        Replies::DepositCreatePosition as u64,
    )))
}

fn handle_deposit_create_position(
    deps: DepsMut,
    env: Env,
    data: Binary,
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
        .supply_of(vault_denom)?
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
    LOCKED_SHARES.save(deps.storage, current_deposit.sender, &user_shares)?;

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
    let response = Response::new().add_submessage(fungify).add_message(mint);

    // if we have any funds to refund, refund them
    if let Some(msg) = bank_msg {
        response.add_message(msg);
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
    if refund0.is_zero() {
        coins.push(coin(refund0.u128(), denom0))
    }
    if refund1.is_zero() {
        coins.push(coin(refund1.u128(), denom1))
    }
    let bank_msg: Option<BankMsg> = if !coins.is_empty() {
        Some(BankMsg::Send {
            to_address: env.contract.address.to_string(),
            amount: coins,
        })
    } else {
        None
    };
    Ok(bank_msg)
}

/// returns the Coin of the needed denoms in the order given in denoms
fn must_pay_two(info: &MessageInfo, denoms: (String, String)) -> ContractResult<(Coin, Coin)> {
    let funds = info.funds.into_iter();
    let token0 = funds
        .find(|coin| coin.denom == denoms.0)
        .ok_or(cw_utils::PaymentError::MissingDenom(denoms.0))?;
    let token1 = funds
        .find(|coin| coin.denom == denoms.1)
        .ok_or(cw_utils::PaymentError::MissingDenom(denoms.1))?;
    Ok((token0, token1))
}
