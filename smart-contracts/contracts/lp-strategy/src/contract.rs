use cosmos_sdk_proto::traits::Message;
use cosmos_sdk_proto::{ibc::applications::interchain_accounts::v1::CosmosTx, Any};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, IbcMsg, IbcTimeout,
    MessageInfo, Order, Reply, Response, StdError, StdResult, Storage, Timestamp, Uint128,
};
use cw2::set_contract_version;
use osmosis_std::shim::Duration;
use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn;
use osmosis_std::types::osmosis::lockup::MsgLockTokens;
use quasar_types::ibc::{ChannelInfo, ChannelType};
use quasar_types::ica::packet::{InterchainAccountPacketData, Type};
use quasar_types::ica::traits::Pack;

use crate::error::ContractError;
use crate::error::ContractError::PaymentError;
use crate::helpers::{create_reply, parse_seq, IbcMsgKind, IcaMessages, MsgKind};
use crate::msg::{ChannelsResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    Tmp, WithdrawRequest, CHANNELS, OUTSTANDING_FUNDS, PENDING_ACK, REPLACEME, REPLIES,
    WITHDRAW_QUEUE,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:lp-strategy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;

    // TODO fill in the instantiation
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    // Save the ibc message together with the sequence number, to be handled properly later at the ack
    let kind = REPLIES.load(deps.storage, msg.id)?;
    match kind {
        MsgKind::Ibc(ibc_kind) => {
            let seq = parse_seq(msg)?;
            PENDING_ACK.save(deps.storage, seq, &ibc_kind)?;
        }
    }
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit { .. } => execute_deposit(deps, env, info),
        ExecuteMsg::Transfer {
            channel,
            to_address,
        } => execute_transfer(deps, env, info, channel, to_address),
        ExecuteMsg::DepositAndLockTokens {
            channel,
            pool_id,
            amount,
            lock_period,
            denom,
            share_out_min_amount,
        } => execute_deposit_and_lock_tokens(
            deps,
            env,
            info,
            channel,
            pool_id,
            denom,
            amount,
            lock_period,
            share_out_min_amount,
        ),
    }
}

pub fn execute_deposit_and_lock_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    channel_id: String,
    pool_id: u64,
    denom: String,
    amount: Uint128,
    lock_period: Uint128,
    share_out_min_amount: Uint128,
) -> Result<Response, ContractError> {
    let channel = CHANNELS.load(deps.storage, channel_id.clone())?;
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

        // save the left over data so we can access everything we need in the ack to lock the tokens
        REPLACEME.save(
            deps.storage,
            &Tmp {
                lock_period,
                pool_id,
            },
        )?;
        let resp = create_reply(
            deps.storage,
            MsgKind::Ibc(IbcMsgKind::Ica(IcaMessages::JoinSwapExternAmountIn)),
            send_packet_msg,
        )?;
        Ok(resp
            .add_attribute("joining_swap_extern_amount_in_on_pool", pool_id.to_string())
            .add_attribute("denom", denom))
    } else {
        Err(ContractError::NoIcaChannel)
    }
}

pub fn do_ibc_lock_tokens(
    deps: &mut dyn Storage,
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

pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    channel: String,
    to_address: String,
) -> Result<Response, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::PaymentError(
            cw_utils::PaymentError::MultipleDenoms {},
        ));
    }

    // TODO implement this check with more advanced logic
    // we want to check that we send funds to our ica address, we check that the address exists as in our channels
    // if !CHANNELS
    //     .range(deps.storage, None, None, Order::Ascending)
    //     .any(|channel| {
    //         let (_, chan) = channel.unwrap();
    //         if let ChannelType::Ica {
    //             channel_ty: _,
    //             counter_party_address,
    //         } = chan.channel_type
    //         {
    //             if counter_party_address.is_some() {
    //                 // this check does not play nice with packet forwarding addresses; to fix we should support (at least) up to two hops
    //                 return counter_party_address.unwrap() == to_address;
    //             } else {
    //                 return false;
    //             }
    //         } else {
    //             false
    //         }
    //     })
    // {
    //     return Err(ContractError::NoCounterpartyIcaAddress);
    // }

    let funds = info.funds[0].clone();
    let transfer = IbcMsg::Transfer {
        channel_id: channel.clone(),
        to_address: to_address.clone(),
        amount: funds,
        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
    };
    Ok(Response::new()
        .add_message(transfer)
        .add_attribute("ibc-tranfer-channel", channel)
        .add_attribute("ibc-transfer-receiver", to_address))
}

pub fn execute_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // do things here with the funds. This is where the actual strategy starts placing funds on a new deposit
    // in the template version of the contract, all we do is check that funds are present
    if info.funds.is_empty() {
        return Err(PaymentError(cw_utils::PaymentError::NoFunds {}));
    }

    // TODO see if we can package this logic a bit better by moving it to strategy.rs
    // Assume we have Atom from the vault contract, later we can add other tokens and add a swap route or something
    // transfer the tokens to our ICA on cosmos
    // let ica = ICA_STATE.load(deps.storage)?;
    // let msg = IntergammMsg::RegisterIcaOnZone {
    //     zone_id: ica.zone_id,
    // };

    // Stake them in a validator, we can add logic to spread over multiple later

    // ????

    // profit

    // if funds are sent to an outside contract, OUTSTANDING funds should be updated here
    // TODO add some more sensible attributes here
    Ok(Response::new().add_attribute("deposit", info.sender))
}

pub fn execute_withdraw_request(
    mut deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    owner: String,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    todo!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Channels {} => to_binary(&handle_channels_query(deps)?),
    }
}

pub fn handle_channels_query(deps: Deps) -> StdResult<ChannelsResponse> {
    let channels: Vec<ChannelInfo> = CHANNELS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|kv| kv.unwrap().1)
        .collect();
    Ok(ChannelsResponse { channels })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        IbcTimeoutBlock,
    };

    const DENOM: &str = "satoshi";
    const CREATOR: &str = "creator";
    const INVESTOR: &str = "investor";
    const BUYER: &str = "buyer";

    #[test]
    fn interchain_packet_serialization_works() {
        let expected: Vec<u8> = vec![
            123, 34, 64, 116, 121, 112, 101, 34, 58, 34, 84, 89, 80, 69, 95, 69, 88, 69, 67, 85,
            84, 69, 95, 84, 88, 34, 44, 34, 100, 97, 116, 97, 34, 58, 91, 49, 48, 44, 49, 51, 54,
            44, 49, 44, 49, 48, 44, 52, 55, 44, 52, 55, 44, 49, 49, 49, 44, 49, 49, 53, 44, 49, 48,
            57, 44, 49, 49, 49, 44, 49, 49, 53, 44, 49, 48, 53, 44, 49, 49, 53, 44, 52, 54, 44, 49,
            48, 51, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48, 57, 44, 52, 54, 44, 49, 49, 56, 44, 52,
            57, 44, 57, 56, 44, 49, 48, 49, 44, 49, 49, 54, 44, 57, 55, 44, 52, 57, 44, 52, 54, 44,
            55, 55, 44, 49, 49, 53, 44, 49, 48, 51, 44, 55, 52, 44, 49, 49, 49, 44, 49, 48, 53, 44,
            49, 49, 48, 44, 56, 51, 44, 49, 49, 57, 44, 57, 55, 44, 49, 49, 50, 44, 54, 57, 44, 49,
            50, 48, 44, 49, 49, 54, 44, 49, 48, 49, 44, 49, 49, 52, 44, 49, 49, 48, 44, 54, 53, 44,
            49, 48, 57, 44, 49, 49, 49, 44, 49, 49, 55, 44, 49, 49, 48, 44, 49, 49, 54, 44, 55, 51,
            44, 49, 49, 48, 44, 49, 56, 44, 56, 53, 44, 49, 48, 44, 54, 51, 44, 49, 49, 49, 44, 49,
            49, 53, 44, 49, 48, 57, 44, 49, 49, 49, 44, 52, 57, 44, 49, 48, 50, 44, 49, 49, 50, 44,
            49, 49, 52, 44, 49, 48, 52, 44, 49, 48, 49, 44, 49, 49, 48, 44, 49, 48, 51, 44, 49, 48,
            55, 44, 49, 50, 48, 44, 52, 56, 44, 53, 49, 44, 53, 49, 44, 49, 49, 55, 44, 49, 48, 57,
            44, 53, 48, 44, 49, 49, 53, 44, 49, 49, 50, 44, 49, 48, 49, 44, 49, 49, 51, 44, 49, 48,
            49, 44, 53, 49, 44, 49, 48, 51, 44, 49, 49, 48, 44, 53, 55, 44, 53, 49, 44, 49, 48, 55,
            44, 53, 48, 44, 53, 53, 44, 53, 54, 44, 49, 48, 52, 44, 49, 48, 57, 44, 52, 56, 44, 49,
            48, 48, 44, 53, 52, 44, 53, 53, 44, 53, 48, 44, 49, 49, 57, 44, 49, 49, 56, 44, 49, 48,
            48, 44, 49, 49, 54, 44, 53, 52, 44, 53, 55, 44, 49, 48, 52, 44, 49, 49, 54, 44, 49, 49,
            57, 44, 49, 48, 52, 44, 49, 48, 56, 44, 49, 48, 52, 44, 49, 49, 50, 44, 49, 49, 55, 44,
            53, 48, 44, 49, 49, 53, 44, 49, 50, 48, 44, 49, 48, 56, 44, 49, 49, 55, 44, 49, 49, 50,
            44, 49, 48, 51, 44, 49, 49, 57, 44, 49, 54, 44, 49, 44, 50, 54, 44, 49, 51, 44, 49, 48,
            44, 53, 44, 49, 49, 55, 44, 49, 49, 49, 44, 49, 48, 57, 44, 49, 49, 53, 44, 49, 49, 49,
            44, 49, 56, 44, 52, 44, 52, 57, 44, 52, 56, 44, 52, 56, 44, 52, 56, 44, 51, 52, 44, 49,
            44, 52, 57, 93, 44, 34, 109, 101, 109, 111, 34, 58, 34, 34, 125,
        ];
        let join = osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn {
            sender: "osmo1fprhengkx033um2speqe3gn93k278hm0d672wvdt69htwhlhpu2sxlupgw".to_string(),
            pool_id: 1,
            token_in: Some(Coin {
                denom: "uomso".to_string(),
                amount: "1000".to_string(),
            }),
            share_out_min_amount: "1".to_string(),
        };

        let anys: Vec<Any> = vec![Any {
            type_url:
                osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn::TYPE_URL
                    .to_string(),
            value: join.encode_to_vec(),
        }];

        let packet = InterchainAccountPacketData {
            r#type: Type::ExecuteTx,
            // TODO data needs to be a cosmos tx
            data: CosmosTx { messages: anys }.encode_to_vec(),
            memo: "".into(),
        };
        assert_eq!(expected, to_binary(&packet).unwrap())
    }

    fn default_instantiate(
        supply_decimals: u8,
        reserve_decimals: u8,
        reserve_supply: Uint128,
    ) -> InstantiateMsg {
        InstantiateMsg {}
    }

    fn setup_test(
        deps: DepsMut,
        supply_decimals: u8,
        reserve_decimals: u8,
        reserve_supply: Uint128,
    ) {
    }

    #[test]
    fn serialize_cosmos_tx_works() {
        let join = osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn {
            sender: "counter_party_address".into(),
            pool_id: 1,
            token_in: Some(Coin {
                denom: "uqsr".to_string(),
                amount: "1000".to_string(),
            }),
            share_out_min_amount: "1".to_string(),
        };

        let proto_join = join.encode_to_vec();
        // a serialization in done in Go and Cosmos sdk
        let go_proto: Vec<u8> = vec![
            10, 93, 10, 47, 47, 111, 115, 109, 111, 115, 105, 115, 46, 103, 97, 109, 109, 46, 118,
            49, 98, 101, 116, 97, 49, 46, 77, 115, 103, 74, 111, 105, 110, 83, 119, 97, 112, 69,
            120, 116, 101, 114, 110, 65, 109, 111, 117, 110, 116, 73, 110, 18, 42, 10, 21, 99, 111,
            117, 110, 116, 101, 114, 95, 112, 97, 114, 116, 121, 95, 97, 100, 100, 114, 101, 115,
            115, 16, 1, 26, 12, 10, 4, 117, 113, 115, 114, 18, 4, 49, 48, 48, 48, 34, 1, 49,
        ];

        let anys: Vec<Any> = vec![Any {
            type_url:
                osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn::TYPE_URL
                    .to_string(),
            value: proto_join,
        }];
        let data = CosmosTx { messages: anys }.encode_to_vec();

        assert_eq!(data, go_proto)
    }

    // #[test]
    // fn deposit_works() {
    //     let mut deps = mock_dependencies();
    //     let info = mock_info("alice", &coins(100_000, "uqsar"));
    //     execute_deposit(deps.as_mut(), mock_env(), info).unwrap();
    // }

    // #[test]
    // fn withdraw_with_sufficient_funds_works() {
    //     let mut deps = mock_dependencies();
    //     let env = mock_env();
    //     deps.querier
    //         .update_balance(env.clone().contract.address, coins(100_000, "uqsar"));
    //     let res = execute_withdraw_request(
    //         deps.as_mut(),
    //         env,
    //         mock_info("alice", &[]),
    //         "alice".into(),
    //         "uqsar".to_string(),
    //         Uint128::new(100_000),
    //     )
    //     .unwrap();
    //     assert_eq!(res.messages.len(), 1);
    //     assert_eq!(
    //         res.messages[0],
    //         SubMsg::new(BankMsg::Send {
    //             to_address: "alice".to_string(),
    //             amount: coins(100_000, "uqsar")
    //         })
    //     )
    // }
}
