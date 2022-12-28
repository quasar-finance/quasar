#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Empty, Env, IbcMsg, IbcTimeout,
    MessageInfo, Order, Reply, Response, StdError, StdResult, Storage, SubMsg, Timestamp, Uint128,
};
use cw2::set_contract_version;
use quasar_types::ibc::{ChannelInfo, ChannelType};
use quasar_types::ica::packet::{InterchainAccountPacketData, Type};
use quasar_types::ica::traits::Pack;

use crate::error::ContractError;
use crate::helpers::{create_reply, create_submsg, parse_seq, IbcMsgKind, IcaMessages, MsgKind};
use crate::msg::{ChannelsResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CHANNELS, CONFIG, PENDING_ACK, REPLIES};
use crate::strategy::do_ibc_join_pool_swap_extern_amount_in;

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
    CONFIG.save(
        deps.storage,
        &Config {
            lock_period: msg.lock_period,
            pool_id: msg.pool_id,
            pool_denom: msg.pool_denom,
            denom: msg.denom,
        },
    );
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    // Save the ibc message together with the sequence number, to be handled properly later at the ack, we can pass the ibc_kind one to one
    // TODO this needs and error check and error handling
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
        ExecuteMsg::TransferJoinLock {
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
        } => execute_join_pool(
            deps,
            env,
            channel,
            pool_id,
            denom,
            amount,
            share_out_min_amount,
        ),
    }
}

// transfer funds sent to the contract to an address on osmosis, this needs an extra change to always
// always send funds to the contracts ICA address
pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    channel: String, // TODO see if we can move channel mapping to a more zone like approach
    to_address: String,
) -> Result<Response, ContractError> {
    let transfer = do_transfer(deps.storage, env, info.funds, channel.clone(), to_address.clone())?;

    Ok(Response::new()
        .add_submessage(transfer)
        .add_attribute("ibc-tranfer-channel", channel)
        .add_attribute("ibc-transfer-receiver", to_address))
}

pub fn execute_join_pool(
    deps: DepsMut,
    env: Env,
    channel_id: String,
    pool_id: u64,
    denom: String,
    amount: Uint128,
    share_out_min_amount: Uint128,
) -> Result<Response, ContractError> {
    let join = do_ibc_join_pool_swap_extern_amount_in(
        deps.storage,
        env,
        channel_id.clone(),
        pool_id,
        denom.clone(),
        amount,
        share_out_min_amount,
    )?;

    Ok(Response::new()
        .add_submessage(join)
        .add_attribute("ibc-join-pool-channel", channel_id)
        .add_attribute("denom", denom))
}

fn do_transfer(
    storage: &mut dyn Storage,
    env: Env,
    funds: Vec<Coin>,
    channel_id: String,
    to_address: String,
) -> Result<SubMsg, ContractError> {
    if funds.len() != 1 {
        return Err(ContractError::PaymentError(
            cw_utils::PaymentError::MultipleDenoms {},
        ));
    }
    // todo check denom of funds once we have denom mapping done

    let timeout = IbcTimeout::with_timestamp(env.block.time.plus_seconds(300));
    let transfer = IbcMsg::Transfer {
        channel_id,
        to_address,
        amount: funds[0].clone(),
        timeout,
    };

    Ok(create_submsg(
        storage,
        MsgKind::Ibc(IbcMsgKind::Transfer),
        transfer,
    )?)
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
        InstantiateMsg {
            lock_period: todo!(),
            pool_id: todo!(),
            pool_denom: todo!(),
            denom: todo!(),
        }
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
