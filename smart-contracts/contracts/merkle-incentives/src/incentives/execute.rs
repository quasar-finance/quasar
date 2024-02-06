use cl_vault::{
    msg::{ClQueryMsg, ExecuteMsg as VaultExecuteMsg, ModifyRangeMsg, QueryMsg as VaultQueryMsg},
    query::PoolResponse,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Coin, Decimal, DepsMut, Env, MessageInfo, Response, WasmMsg};
use cw_dex_router::operations::SwapOperationsListUnchecked;

use crate::ContractError;

#[cw_serde]
pub enum IncentivesExecuteMsg {
    /// Submit a range to the range middleware
    Claim {
        for_user: String,
        claim_coins: Vec<Coin>,
        proof_str: String,
    },
}

pub fn execute_incentives_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    incentives_msg: IncentivesExecuteMsg,
) -> Result<Response, ContractError> {
    match incentives_msg {
        IncentivesExecuteMsg::Claim {
            for_user,
            claim_coins,
            proof_str,
        } => claim(deps, env, info, for_user, claim_coins, proof_str),
    }
}

pub fn claim(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    for_user: String,
    claim_coins: Vec<Coin>,
    proof_str: String,
) -> Result<Response, ContractError> {
    todo!()
}
