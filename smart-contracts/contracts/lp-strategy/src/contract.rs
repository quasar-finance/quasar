use cosmos_sdk_proto::traits::Message;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, IbcMsg, IbcTimeout,
    MessageInfo, Order, Reply, Response, StdError, StdResult, Storage, Timestamp, Uint128,
};
use cw2::set_contract_version;
use osmosis_std::shim::Any;
use osmosis_std::types::cosmos::base::v1beta1::Coin;
use quasar_types::ibc::{ChannelInfo, ChannelType};
use serde::{Serialize, Deserialize};

use crate::error::ContractError;
use crate::error::ContractError::PaymentError;
use crate::helpers::{create_reply, parse_seq, IbcMsgKind, IcaMessages, MsgKind};
use crate::msg::{ChannelsResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::queue::{dequeue, enqueue};
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

/// CosmosTx contains a list of sdk.Msg's. It should be used when sending transactions to an SDK host chain.
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct CosmosTx {
    #[prost(message, repeated, tag = "1")]
    pub messages: Vec<Any>,
}

/// InterchainAccountPacketData is comprised of a raw transaction, type of transaction and optional memo field.
#[derive(Clone, PartialEq, ::prost::Message,  Serialize, Deserialize)]
pub struct InterchainAccountPacketData {
    #[prost(enumeration = "Type", tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub memo: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq,   Serialize, Deserialize)]
pub struct MyInterchainAccountPacketData {
    #[serde(rename="type")]
    pub r#type: String,
    pub data: Vec<u8>,
    pub memo: String,
}

/// Type defines a classification of message issued from a controller chain to its associated interchain accounts
/// host
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum Type {
    /// Default zero value enumeration
    Unspecified = 0,
    /// Execute a transaction on an interchain accounts host chain
    ExecuteTx = 1,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::Unspecified => "TYPE_UNSPECIFIED",
            Type::ExecuteTx => "TYPE_EXECUTE_TX",
        }
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
        channel_ty,
        counter_party_address,
    } = channel.channel_type
    {
        if counter_party_address.is_none() {
            return Err(ContractError::NoCounterpartyIcaAddress);
        }
        let delegate = CosmosMsg::Staking(cosmwasm_std::StakingMsg::Delegate { validator: (), amount: () })
        // setup the first IBC message to send, and save the entire sequence so we have acces to it on acks
        let join = osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn {
            sender: counter_party_address.unwrap(),
            pool_id,
            token_in: Some(Coin {
                denom,
                amount: amount.to_string(),
            }),
            share_out_min_amount: share_out_min_amount.to_string(),
        };

        let anys: Vec<Any>= vec![Any{ type_url: osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn::TYPE_URL.to_string(), value: join.encode_to_vec() }];

        let packet = MyInterchainAccountPacketData {
            r#type: "TYPE_EXECUTE_TX".to_string(),
            // TODO data needs to be a cosmos tx
            data: base64::decode("CrABCiMvY29zbW9zLnN0YWtpbmcudjFiZXRhMS5Nc2dEZWxlZ2F0ZRKIAQpBY29zbW9zMTVjY3NoaG1wMGdzeDI5cXBxcTZnNHptbHRubnZnbXl1OXVldWFkaDl5Mm5jNXpqMHN6bHM1Z3RkZHoSNGNvc21vc3ZhbG9wZXIxcW5rMm40bmxrcHc5eGZxbnRsYWRoNzR3NnVqdHVsd25teG5oM2saDQoFc3Rha2USBDEwMDA=").map_err(|err| StdError::SerializeErr{source_type : "base64".into(), msg: err.to_string()})?,
            memo: "".into(),
        };
// 
// &serde_json_wasm::to_vec(&packet).map_err(|err| ContractError::Std(StdError::SerializeErr { source_type: "serialize intechain packet".into(), msg: err.to_string() }))?)
        let send_packet_msg = IbcMsg::SendPacket {
            channel_id: channel_id,
            data: to_binary(&serde_json_wasm::to_vec(&packet).map_err(|err| StdError::SerializeErr { source_type: "encode packet".into(), msg: err.to_string() })?)?,
            timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
        };

        // save the left over data so we can access everything we need in the ack to lock the tokens
        REPLACEME.save(deps.storage, &Tmp { lock_period })?;
        let resp = create_reply(
            deps.storage,
            MsgKind::Ibc(IbcMsgKind::Ica(IcaMessages::JoinSwapExternAmountIn)),
            send_packet_msg,
        )?;
        Ok(resp.add_attribute("joining_swap_extern_amount_in_on_pool", pool_id.to_string()))
    } else {
        Err(ContractError::NoIcaChannel)
    }
}

pub fn do_ibc_lock_tokens(
    deps: &mut dyn Storage,
    token_amount: String,
) -> Result<CosmosMsg, ContractError> {
    todo!()
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
    enqueue::<WithdrawRequest>(
        deps.branch(),
        WithdrawRequest {
            denom,
            amount,
            owner,
        },
        WITHDRAW_QUEUE,
    )?;
    let res = try_withdraw(deps, env)?;
    Ok(res)
}

fn try_withdraw(mut deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let withdraw = dequeue::<WithdrawRequest>(deps.branch(), WITHDRAW_QUEUE);
    if withdraw.is_none() {
        return Err(ContractError::QueueError(
            "dequeue was none while queue should be some".to_string(),
        ));
    }
    let w = withdraw.unwrap();
    // check the free balance of the contract
    let free_balance = deps
        .querier
        .query_balance(env.contract.address, w.denom.clone())
        .map_err(|error| ContractError::Std(error))?;
    // if the contract has enough free balance, execute the withdraw
    if w.amount <= free_balance.amount {
        // remove the peeked withdraw request
        do_withdraw(w)
    } else {
        // else we start to unlock funds and return a response
        // TODO determine whether we need to dequeue the withdraw at this point or a later point
        unlock_funds(deps, w)
    }
}

// do_withdraw sends funds from the contract to the owner of the funds according to the withdraw request
fn do_withdraw(withdraw: WithdrawRequest) -> Result<Response, ContractError> {
    let msg = BankMsg::Send {
        to_address: withdraw.owner.clone(),
        amount: coins(withdraw.amount.u128(), withdraw.denom.clone()),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("withdraw", "executed")
        .add_attribute("amount", withdraw.amount)
        .add_attribute("denom", withdraw.denom)
        .add_attribute("owner", withdraw.owner))
}

fn unlock_funds(deps: DepsMut, withdraw: WithdrawRequest) -> Result<Response, ContractError> {
    // TODO this is where funds are locked or not present within the strategy contract. The withdraw happens 'async' here
    // the strategy needs to know where the funds are located, unlock the funds there(or do so)
    let outstanding = OUTSTANDING_FUNDS.load(deps.storage)?;
    if withdraw.amount > outstanding {
        return Err(ContractError::InsufficientOutStandingFunds);
    }
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
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    const DENOM: &str = "satoshi";
    const CREATOR: &str = "creator";
    const INVESTOR: &str = "investor";
    const BUYER: &str = "buyer";

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
        let go_proto: Vec<u8> = vec![10, 93, 10, 47, 47, 111, 115, 109, 111, 115, 105, 115, 46, 103, 97, 109, 109, 46, 118, 49, 98, 101, 116, 97, 49, 46, 77, 115, 103, 74, 111, 105, 110, 83, 119, 97, 112, 69, 120, 116, 101, 114, 110, 65, 109, 111, 117, 110, 116, 73, 110, 18, 42, 10, 21, 99, 111, 117, 110, 116, 101, 114, 95, 112, 97, 114, 116, 121, 95, 97, 100, 100, 114, 101, 115, 115, 16, 1, 26, 12, 10, 4, 117, 113, 115, 114, 18, 4, 49, 48, 48, 48, 34, 1, 49];


        let anys: Vec<Any>= vec![Any{ type_url: osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn::TYPE_URL.to_string(), value: proto_join }];
        let data = CosmosTx{ messages: anys }.encode_to_vec();

        assert_eq!(data, go_proto)
    }

    #[test]
    fn serialize_interchain_packet_works() {
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


        let anys: Vec<Any>= vec![Any{ type_url: osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn::TYPE_URL.to_string(), value: proto_join }];
        let data = CosmosTx{ messages: anys }.encode_to_vec();

        let ica_packet = InterchainAccountPacketData{ r#type: 1, data, memo: "".into() };
        
        let go_proto: Vec<u8> = vec![123, 34, 100, 97, 116, 97, 34, 58, 34, 67, 108, 48, 75, 76, 121, 57, 118, 99, 50, 49, 118, 99, 50, 108, 122, 76, 109, 100, 104, 98, 87, 48, 117, 100, 106, 70, 105, 90, 88, 82, 104, 77, 83, 53, 78, 99, 50, 100, 75, 98, 50, 108, 117, 85, 51, 100, 104, 99, 69, 86, 52, 100, 71, 86, 121, 98, 107, 70, 116, 98, 51, 86, 117, 100, 69, 108, 117, 69, 105, 111, 75, 70, 87, 78, 118, 100, 87, 53, 48, 90, 88, 74, 102, 99, 71, 70, 121, 100, 72, 108, 102, 89, 87, 82, 107, 99, 109, 86, 122, 99, 120, 65, 66, 71, 103, 119, 75, 66, 72, 86, 120, 99, 51, 73, 83, 66, 68, 69, 119, 77, 68, 65, 105, 65 ,84, 69, 61, 34, 44, 34, 109, 101, 109, 111, 34, 58, 34, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 84, 89, 80, 69, 95, 69, 88, 69, 67, 85, 84, 69, 95, 84, 88, 34, 125];
        let go_string = "{\"data\":\"[10,93,10,47,47,111,115,109,111,115,105,115,46,103,97,109,109,46,118,49,98,101,116,97,49,46,77,115,103,74,111,105,110,83,119,97,112,69,120,116,101,114,110,65,109,111,117,110,116,73,110,18,42,10,21,99,111,117,110,116,101,114,95,112,97,114,116,121,95,97,100,100,114,101,115,115,16,1,26,12,10,4,117,113,115,114,18,4,49,48,48,48,34,1,49]\",\"memo\":\"\",\"type\":\"TYPE_EXECUTE_TX\"}";
        let rust_string = serde_json_wasm::to_string(&ica_packet).unwrap();

        println!("{}", rust_string);
        assert_eq!(go_string, rust_string);
        assert_eq!(go_proto, serde_json_wasm::to_vec(&ica_packet).unwrap())
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
