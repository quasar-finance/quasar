use cosmwasm_std::{
    coin, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, Response, StdError, SubMsg, SubMsgResult,
    Uint128,
};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    MsgCreatePositionResponse, Pool,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    MsgCreateDenom, MsgCreateDenomResponse, MsgMint,
};

use crate::msg::InstantiateMsg;
use crate::reply::Replies;
use crate::ContractError;

pub fn handle_instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    todo!()
}
