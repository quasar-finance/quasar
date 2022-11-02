use cosmos_sdk_proto::ibc::applications::interchain_accounts::v1::InterchainAccountPacketData;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, IbcMsg, IbcQuery, IbcTimeout, MessageInfo, Order,
    PortIdResponse, Reply, Response, StdResult, SubMsg, Uint64,
};
use prost::Message;

use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinPool;

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::helpers::{handle_reply_sample, set_reply};
use crate::msg::{
    ChannelResponse, ConfigResponse, ExecuteMsg, InitMsg,
    ListChannelsResponse, MigrateMsg, PortResponse, QueryMsg,
};

use crate::state::{Config, Origin, CHANNEL_INFO, CONFIG, QUERY_RESULT_COUNTER, REPLIES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:icq";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let cfg = Config {
        default_timeout: msg.default_timeout,
    };
    CONFIG.save(deps.storage, &cfg)?;
    QUERY_RESULT_COUNTER.save(deps.storage, &0)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::JoinPool {
            channel,
            sender,
            pool_id,
            share_out_amount,
            token_in_maxs,
        } => execute_bank_send(
            deps,
            env,
            channel,
            sender,
            pool_id,
            share_out_amount,
            token_in_maxs,
        ),
    }
}

fn execute_bank_send(
    deps: DepsMut,
    env: Env,
    channel: String,
    sender: String,
    pool_id: Uint64,
    share_out_amount: String,
    token_in_maxs: Vec<OsmoCoin>,
) -> Result<Response, ContractError> {
    let msg = MsgJoinPool {
        sender,
        pool_id: pool_id.u64(),
        share_out_amount,
        token_in_maxs,
    };
    let packet = InterchainAccountPacketData {
        r#type: 1,
        data: msg.encode_to_vec(),
        memo: "".into(),
    };

    let send_packet_msg = IbcMsg::SendPacket {
        channel_id: channel,
        data: to_binary(&packet.encode_to_vec())?,
        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
    };

    let id = set_reply(deps, &Origin::Sample)?;
    let res = Response::new()
        .add_submessage(SubMsg::reply_on_success(send_packet_msg, id))
        .add_attribute("action", "query");
    Ok(res)
}

#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let origin = REPLIES.load(deps.storage, msg.id)?;
    match origin {
        Origin::Sample => handle_reply_sample(deps, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Port {} => to_binary(&query_port(deps)?),
        QueryMsg::ListChannels {} => to_binary(&query_list(deps)?),
        QueryMsg::Channel { id } => to_binary(&query_channel(deps, id)?),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_port(deps: Deps) -> StdResult<PortResponse> {
    let query = IbcQuery::PortId {}.into();
    let PortIdResponse { port_id } = deps.querier.query(&query)?;
    Ok(PortResponse { port_id })
}

fn query_list(deps: Deps) -> StdResult<ListChannelsResponse> {
    let channels = CHANNEL_INFO
        .range_raw(deps.storage, None, None, Order::Ascending)
        .map(|r| r.map(|(_, v)| v))
        .collect::<StdResult<_>>()?;
    Ok(ListChannelsResponse { channels })
}

// make public for ibc tests
pub fn query_channel(deps: Deps, id: String) -> StdResult<ChannelResponse> {
    let info = CHANNEL_INFO.load(deps.storage, &id)?;
    Ok(ChannelResponse { info })
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let cfg = CONFIG.load(deps.storage)?;
    let res = ConfigResponse {
        default_timeout: cfg.default_timeout,
    };
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    
    use crate::test_helpers::*;

    use cosmwasm_std::testing::{mock_env};
    use cosmwasm_std::{from_binary, StdError};

    #[test]
    fn setup_and_query() {
        let deps = setup(&["channel-3", "channel-7"]);

        let raw_list = query(deps.as_ref(), mock_env(), QueryMsg::ListChannels {}).unwrap();
        let list_res: ListChannelsResponse = from_binary(&raw_list).unwrap();
        assert_eq!(2, list_res.channels.len());
        assert_eq!(mock_channel_info("channel-3"), list_res.channels[0]);
        assert_eq!(mock_channel_info("channel-7"), list_res.channels[1]);

        let raw_channel = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Channel {
                id: "channel-3".to_string(),
            },
        )
        .unwrap();
        let chan_res: ChannelResponse = from_binary(&raw_channel).unwrap();
        assert_eq!(chan_res.info, mock_channel_info("channel-3"));

        let err = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Channel {
                id: "channel-10".to_string(),
            },
        )
        .unwrap_err();
        assert_eq!(err, StdError::not_found("icq::state::ChannelInfo"));
    }
}
