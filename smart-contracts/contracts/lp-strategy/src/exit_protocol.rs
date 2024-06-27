use crate::error::ContractError;
use crate::helpers::{
    create_ibc_ack_submsg, get_ica_address, is_contract_admin, IbcMsgKind, IcaMessages,
};
use crate::state::{IBC_TIMEOUT_TIME, ICA_CHANNEL};
use crate::unbond::{exit_swap, PendingReturningUnbonds};
use cosmwasm_std::{
    Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, Event, IbcTimeout, Response, SubMsg, Uint128,
};
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend as OsmoMsgSend;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
use quasar_types::ica::packet::ica_send;

pub(crate) fn execute_exit_pool(
    deps_mut: DepsMut,
    env: Env,
    share_amount_in: Uint128,
    token_out_min_amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    // check admin
    is_contract_admin(&deps_mut.querier, &env, &sender)?;

    // perform an exit swap
    let msg = exit_swap(
        deps_mut.storage,
        &env,
        share_amount_in,
        token_out_min_amount,
        PendingReturningUnbonds { unbonds: vec![] },
    )?;

    Ok(Response::new().add_submessage(msg).add_event(
        Event::new("exit-pool")
            .add_attribute("share-amount-in", share_amount_in.to_string().clone())
            .add_attribute(
                "token_out_min_amount",
                token_out_min_amount.to_string().clone(),
            ),
    ))
}

pub fn execute_transfer_on_osmosis(
    deps: DepsMut,
    env: Env,
    destination_address: Addr,
    amounts: Vec<OsmoCoin>,
    sender: Addr,
) -> Result<Response, ContractError> {
    // validate admin
    is_contract_admin(&deps.querier, &env, &sender)?;

    // get the ica address
    let ica_address = get_ica_address(deps.storage, ICA_CHANNEL.load(deps.storage)?)?;

    // prepare bank send message on osmosis
    let msg = OsmoMsgSend {
        from_address: ica_address,
        to_address: destination_address.clone().to_string(),
        amount: amounts,
    };

    // prepare ica packet to be sent on osmosis
    let pkt = ica_send::<OsmoMsgSend>(
        msg.clone(),
        ICA_CHANNEL.load(deps.storage)?,
        IbcTimeout::with_timestamp(env.block.time.plus_seconds(IBC_TIMEOUT_TIME)),
    )?;

    let channel = ICA_CHANNEL.load(deps.storage)?;

    let message: SubMsg = create_ibc_ack_submsg(
        deps.storage,
        IbcMsgKind::Ica(IcaMessages::BankSend(
            destination_address.clone(),
            msg.clone().amount,
        )),
        pkt,
        channel,
    )?;

    Ok(Response::new()
        .add_submessage(message)
        .add_event(Event::new("ica_send_amount"))
        .add_attribute(
            "destination_address",
            destination_address.clone().to_string(),
        ))
}

pub fn execute_transfer_on_quasar(
    deps: DepsMut,
    env: Env,
    destination_address: Addr,
    amounts: Vec<Coin>,
    sender: Addr,
) -> Result<Response, ContractError> {
    // validate admin
    is_contract_admin(&deps.querier, &env, &sender)?;

    // validate destination address on local chain
    let to_address = deps.api.addr_validate(destination_address.as_str())?;

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: to_address.to_string(),
            amount: amounts,
        }))
        .add_event(Event::new("transfer_on_quasar"))
        .add_attribute(
            "destination_address",
            destination_address.clone().to_string().clone(),
        ))
}
