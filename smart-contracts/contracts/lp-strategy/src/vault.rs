use cosmwasm_std::{
    to_binary, Addr, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Order, Storage, SubMsg, Uint128,
};
use cw_utils::must_pay;
use osmosis_std::types::{
    cosmos::bank::v1beta1::QueryBalanceRequest,
    cosmos::base::v1beta1::Coin as OsmoCoin,
    osmosis::gamm::{v1beta1::QueryCalcExitPoolCoinsFromSharesRequest, v2::QuerySpotPriceRequest},
};
use prost::Message;
use quasar_types::icq::{InterchainQueryPacketData, Query};

use crate::{
    error::{ContractError, OngoingDeposit},
    helpers::{check_icq_channel, create_ibc_ack_submsg, get_ica_address, IbcMsgKind},
    lock::{enqueue, DWType, Deposit, Lock},
    state::{
        PendingAck, CLAIMS, CONFIG, ICA_CHANNEL, ICQ_CHANNEL, LOCK, LOCK_QUEUE, LP_SHARES, SHARES,
        TRANSFER_CHANNEL,
    },
    strategy::do_transfer,
};

// A deposit starts of by querying the state of the ica counterparty contract
pub fn do_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Option<SubMsg>, ContractError> {
    let amount = must_pay(&info, &CONFIG.load(deps.storage)?.local_denom)?;

    let icq_channel = ICQ_CHANNEL.load(deps.storage)?;
    check_icq_channel(deps.storage, icq_channel.clone())?;

    // if the contract is locked, enqueue and return, else lock, enqueue and dispatch balance query
    if LOCK.load(deps.storage)? == Lock::Locked {
        enqueue(
            deps.storage,
            DWType::Deposit(Deposit {
                amount: amount,
                owner: info.sender,
            }),
        )?;
        return Ok(None);
    } else {
        LOCK.save(deps.storage, &Lock::Locked)?;
        enqueue(
            deps.storage,
            DWType::Deposit(Deposit {
                amount: amount,
                owner: info.sender,
            }),
        )?;
    }

    // deposit need to internally rebuild the amount of funds under the smart contract
    let packet = prepare_total_balance_query(deps.storage, ICA_CHANNEL.load(deps.storage)?)?;

    let send_packet_msg = IbcMsg::SendPacket {
        channel_id: icq_channel,
        data: to_binary(&packet)?,
        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
    };

    Ok(Some(create_ibc_ack_submsg(
        deps.storage,
        PendingAck {
            kind: IbcMsgKind::Icq,
            // since we don't empty the queue here, we don't pass any deposits
            deposits: vec![],
        },
        send_packet_msg,
    )?))
}

// after the balance query, we can calculate the amount of the claim we need to create, we update the claims and transfer the funds
pub fn handle_query_ack(
    storage: &mut dyn Storage,
    env: Env,
    _pending: PendingAck,
    query_balance: Uint128,
) -> Result<SubMsg, ContractError> {
    let transfer_chan = TRANSFER_CHANNEL.load(storage)?;
    let to_address = get_ica_address(storage, ICA_CHANNEL.load(storage)?)?;

    let (amount, deposits) = fold_queue(storage, query_balance)?;

    do_transfer(storage, env, amount, transfer_chan, to_address, deposits)
}

/// fold_queue folds the queue and attributes shares to the depositors according to the given total value
pub fn fold_queue(
    storage: &mut dyn Storage,
    total_balance: Uint128,
) -> Result<(Uint128, Vec<OngoingDeposit>), ContractError> {
    let mut total = Uint128::zero();
    let mut deposits: Vec<OngoingDeposit> = vec![];
    while !LOCK_QUEUE.is_empty(storage)? {
        let item: DWType = LOCK_QUEUE
            .pop_front(storage)?
            .ok_or(ContractError::QueueItemNotFound)?;
        match item {
            DWType::Withdraw(_) => todo!(),
            DWType::Deposit(val) => {
                let claim_amount =
                    create_claim(storage, val.amount, val.owner.clone(), total_balance)?;
                total = total.checked_add(val.amount)?;
                deposits.push(OngoingDeposit {
                    claim_amount,
                    owner: val.owner,
                });
            }
        }
    }
    Ok((total, deposits))
}

pub fn prepare_total_balance_query(
    storage: &dyn Storage,
    channel: String,
) -> Result<InterchainQueryPacketData, ContractError> {
    let address = get_ica_address(storage, channel)?;
    let config = CONFIG.load(storage)?;
    let denom = CONFIG.load(storage)?.base_denom;
    // we query the current balance on our ica address
    let balance = QueryBalanceRequest { address, denom };
    // we simulate the result of an exit pool of our entire vault to get the total value in lp tokens
    let exit_pool = QueryCalcExitPoolCoinsFromSharesRequest {
        pool_id: config.pool_id,
        share_in_amount: LP_SHARES.load(storage)?.to_string(),
    };
    // we query the spot price of our base_denom and quote_denom so we can convert the quote_denom from exitpool to the base_denom
    let spot_price = QuerySpotPriceRequest {
        pool_id: config.pool_id,
        base_asset_denom: config.base_denom,
        quote_asset_denom: config.quote_denom,
    };
    Ok(Query::new()
        .add_request(
            balance.encode_to_vec(),
            QueryBalanceRequest::TYPE_URL.to_string(),
        )
        .add_request(
            exit_pool.encode_to_vec(),
            QueryCalcExitPoolCoinsFromSharesRequest::TYPE_URL.to_string(),
        )
        .add_request(
            spot_price.encode_to_vec(),
            QuerySpotPriceRequest::TYPE_URL.to_string(),
        )
        .encode_pkt())
}

// calculate the total balance of the vault using the query from prepare_total_balance_query()
pub fn calc_total_balance(
    storage: &mut dyn Storage,
    ica_balance: Uint128,
    exit_pool: Vec<OsmoCoin>,
    spot_price: Uint128,
) -> Result<Uint128, ContractError> {
    let config = CONFIG.load(storage)?;
    let base = exit_pool
        .iter()
        .find(|coin| coin.denom == config.base_denom)
        .ok_or(ContractError::BaseDenomNotFound)?;
    let quote = exit_pool
        .iter()
        .find(|coin| coin.denom == config.quote_denom)
        .ok_or(ContractError::QuoteDenomNotFound)?;
    // return ica_balance + base_amount + (quote_amount * spot_price)
    Ok(ica_balance
        .checked_add(Uint128::new(base.amount.parse::<u128>()?))?
        .checked_add(Uint128::new(quote.amount.parse::<u128>()?).checked_mul(spot_price)?)?)
}

// create_claim
fn create_claim(
    storage: &mut dyn Storage,
    user_balance: Uint128,
    address: Addr,
    total_balance: Uint128,
) -> Result<Uint128, ContractError> {
    let total_shares = get_total_shares(storage)?;

    // calculate the correct size of the claim
    let claim_amount = calculate_claim(user_balance, total_balance, total_shares)?;
    CLAIMS.save(storage, address, &claim_amount)?;
    Ok(claim_amount)
}

fn get_total_shares(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    let mut sum = Uint128::zero();
    for val in SHARES.range(storage, None, None, Order::Ascending) {
        sum = sum.checked_add(val?.1)?;
    }
    Ok(sum)
}

// create a share and remove the amount from the claim
pub fn create_share(
    storage: &mut dyn Storage,
    owner: Addr,
    amount: Uint128,
) -> Result<(), ContractError> {
    let claim = CLAIMS.load(storage, owner.clone())?;
    if claim < amount {
        return Err(ContractError::InsufficientClaims);
    }

    if claim <= amount {
        CLAIMS.remove(storage, owner.clone());
    } else {
        CLAIMS.save(storage, owner.clone(), &claim.checked_sub(amount)?)?;
    }

    // TODO make shares fungible using cw20
    // call into the minter and mint shares for the according to the claim
    Ok(SHARES.save(storage, owner, &amount)?)
}

/// calculate the amount of for the claim of the user
/// user_shares = (user_balance / vault_balance) * vault_total_shares = (user_balance * vault_total_shares) / vault_balance
fn calculate_claim(
    user_balance: Uint128,
    total_balance: Uint128,
    total_shares: Uint128,
) -> Result<Uint128, ContractError> {
    Ok(user_balance
        .checked_mul(total_shares)?
        .checked_div(total_balance)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO rewrite this to a proptest
    #[test]
    fn calculate_claim_works() {
        let val = calculate_claim(Uint128::new(10), Uint128::new(100), Uint128::new(10)).unwrap();
        assert_eq!(val, Uint128::one())
    }
}
