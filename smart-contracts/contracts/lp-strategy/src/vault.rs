use std::{env, ops::Add};

use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Order,
    Response, Storage, SubMsg, Uint128,
};
use cw_storage_plus::Bound;
use cw_utils::must_pay;
use osmosis_std::types::cosmos::bank::v1beta1::QueryBalanceRequest;
use prost::Message;
use quasar_types::icq::{InterchainQueryPacketData, Query};

use crate::{
    error::ContractError,
    helpers::{check_icq_channel, create_ibc_ack_submsg, get_ica_address, IbcMsgKind},
    state::{
        Claim, PendingAck, CLAIMS, CONFIG, DEPOSIT_SEQ, ICA_CHANNEL, ICQ_CHANNEL, LOCKED_FUNDS,
        SHARES, TRANSFERRED_FUNDS, TRANSFER_CHANNEL,
    },
    strategy::do_transfer,
};

// A deposit starts of by querying the state of the ica counterparty contract
pub fn do_deposit(deps: DepsMut, env: Env, info: MessageInfo) -> Result<SubMsg, ContractError> {
    let amount = must_pay(&info, &CONFIG.load(deps.storage)?.denom)?;
    let icq_channel = ICQ_CHANNEL.load(deps.storage)?;
    check_icq_channel(deps.storage, icq_channel.clone())?;

    // deposit need to internally rebuild the amount of funds under the smart contract, can this be just deposited + already autocompounded?
    let packet = prepare_icq_balance(deps.storage, ICA_CHANNEL.load(deps.storage)?)?;

    let send_packet_msg = IbcMsg::SendPacket {
        channel_id: icq_channel,
        data: to_binary(&packet)?,
        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
    };

    let dep_seq = DEPOSIT_SEQ.load(deps.storage)?;
    DEPOSIT_SEQ.save(deps.storage, &dep_seq.checked_add(Uint128::one())?)?;

    Ok(create_ibc_ack_submsg(
        deps.storage,
        PendingAck {
            kind: IbcMsgKind::Icq,
            address: info.sender,
            amount,
            id: dep_seq,
        },
        send_packet_msg,
    )?)
}

// after the balance query, we can calculate the amount of the claim we need to create, we update the claims and transfer the funds
pub fn handle_deposit_ack(
    storage: &mut dyn Storage,
    env: Env,
    pending: PendingAck,
    query_balance: Uint128,
) -> Result<SubMsg, ContractError> {
    create_claim(
        storage,
        pending.amount,
        pending.address.clone(),
        query_balance,
        pending.id,
    )?;
    let transfer_chan = TRANSFER_CHANNEL.load(storage)?;
    let to_address = get_ica_address(storage, ICA_CHANNEL.load(storage)?)?;
    do_transfer(
        storage,
        env,
        pending.address,
        pending.amount,
        transfer_chan,
        to_address,
        pending.id,
    )
}

pub fn prepare_icq_balance(
    storage: &dyn Storage,
    channel: String,
) -> Result<InterchainQueryPacketData, ContractError> {
    let address = get_ica_address(storage, channel)?;

    let denom = CONFIG.load(storage)?.denom;
    let query = QueryBalanceRequest { address, denom };
    Ok(Query::new()
        .add_request(
            query.encode_to_vec(),
            "/cosmos.bank.v1beta1.Query/Balance".into(),
        )
        .encode_pkt())
}

// create_claim
fn create_claim(
    storage: &mut dyn Storage,
    user_balance: Uint128,
    address: Addr,
    ica_balance: Uint128,
    current: Uint128,
) -> Result<(), ContractError> {
    // calculate the total balance
    let in_transit_user_funds = get_transit_user_funds(storage, current)?;
    let locked_balance = get_locked_balance(storage, current)?;
    let total_balance = ica_balance
        .checked_add(locked_balance)?
        .checked_sub(in_transit_user_funds)?;
    let total_shares = get_total_balance(storage)?;

    // calculate the correct size of the claim
    let claim_amount = calculate_claim(user_balance, total_balance, total_shares)?;
    CLAIMS.save(storage, address, &claim_amount)?;
    Ok(())
}

// get all user funds that are in transit, but not yet in a pool
fn get_transit_user_funds(
    storage: &dyn Storage,
    current: Uint128,
) -> Result<Uint128, ContractError> {
    let mut sum = Uint128::zero();
    for val in TRANSFERRED_FUNDS.range(
        storage,
        None,
        Some(Bound::exclusive(current.u128())),
        Order::Ascending,
    ) {
        sum = sum.checked_add(val?.1)?;
    }
    Ok(sum)
}

fn get_locked_balance(storage: &dyn Storage, current: Uint128) -> Result<Uint128, ContractError> {
    let mut sum = Uint128::zero();
    for val in LOCKED_FUNDS.range(
        storage,
        None,
        Some(Bound::exclusive(current.u128())),
        Order::Ascending,
    ) {
        sum = sum.checked_add(val?.1)?;
    }
    Ok(sum)
}

fn get_total_balance(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    let mut sum = Uint128::zero();
    for val in SHARES.range(storage, None, None, Order::Ascending) {
        sum = sum.checked_add(val?.1)?;
    }
    Ok(sum)
}

fn create_share(claim: Claim) -> Result<Response, ContractError> {
    // call into the minter and mint shares for the according to the claim
    todo!()
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
