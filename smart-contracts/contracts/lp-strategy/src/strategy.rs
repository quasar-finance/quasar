use cosmwasm_std::{to_binary, Env, IbcMsg, IbcTimeout, Storage, SubMsg, Uint128};
use osmosis_std::{
    shim::Duration,
    types::{
        cosmos::base::v1beta1::Coin,
        osmosis::{gamm::v1beta1::MsgJoinSwapExternAmountIn, lockup::MsgLockTokens},
    },
};
use quasar_types::{
    ibc::ChannelType,
    ica::{
        packet::{InterchainAccountPacketData, Type},
        traits::Pack,
    },
};

use crate::{
    error::ContractError,
    helpers::{create_submsg, IbcMsgKind, IcaMessages, MsgKind},
    state::CHANNELS,
};

pub fn do_ibc_join_pool_swap_extern_amount_in(
    storage: &mut dyn Storage,
    env: Env,
    channel_id: String,
    pool_id: u64,
    denom: String,
    amount: Uint128,
    share_out_min_amount: Uint128,
) -> Result<SubMsg, ContractError> {
    let channel = CHANNELS.load(storage, channel_id.clone())?;
    if let ChannelType::Ica {
        channel_ty: _,
        counter_party_address,
    } = channel.channel_type
    {
        // make sure we have a counterparty address
        if counter_party_address.is_none() {
            return Err(ContractError::NoCounterpartyIcaAddress);
        }

        // setup the first IBC message to send, and save the entire sequence so we have acces to it on acks
        let join = MsgJoinSwapExternAmountIn {
            sender: counter_party_address.unwrap(),
            pool_id,
            token_in: Some(Coin {
                denom: denom.clone(),
                amount: amount.to_string(),
            }),
            share_out_min_amount: share_out_min_amount.to_string(),
        };

        let packet = InterchainAccountPacketData::new(Type::ExecuteTx, vec![join.pack()], None);

        let send_packet_msg = IbcMsg::SendPacket {
            channel_id: channel_id,
            data: to_binary(&packet)?,
            timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
        };

        Ok(create_submsg(
            storage,
            MsgKind::Ibc(IbcMsgKind::Ica(IcaMessages::JoinSwapExternAmountIn)),
            send_packet_msg,
        )?)
    } else {
        Err(ContractError::NoIcaChannel)
    }
}

pub fn do_ibc_lock_tokens(
    _deps: &mut dyn Storage,
    owner: String,
    coins: Vec<Coin>,
) -> Result<InterchainAccountPacketData, ContractError> {
    // denom in this case is expected to be something like gamm/pool/1
    // duration is  60 sec/min * 60 min/hr * 24hr * 14days
    // TODO move the duration to a package and make it settable
    let lock = MsgLockTokens {
        owner,
        duration: Some(Duration {
            seconds: 1209600,
            nanos: 0,
        }),
        coins,
    };
    Ok(InterchainAccountPacketData::new(
        Type::ExecuteTx,
        vec![lock.pack()],
        None,
    ))
}
