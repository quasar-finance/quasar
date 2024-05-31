use cosmwasm_std::{BankMsg, DepsMut, Env, Response, StdError, SubMsg, SubMsgResult};
use osmosis_std::try_proto_to_cosmwasm_coins;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    MsgCollectIncentives, MsgCollectIncentivesResponse, MsgCollectSpreadRewards, MsgCollectSpreadRewardsResponse
};

use crate::helpers::sort_tokens;
use crate::state::{MigrationStatus, MIGRATION_STATUS};
use crate::{reply::Replies, state::VAULT_CONFIG, ContractError};

use super::helpers::CoinList;
use super::{get_collect_incentives_msg, get_collect_spread_rewards_msgs};

/// claim_rewards claims rewards from Osmosis and update the rewards map to reflect each users rewards
pub fn execute_collect_rewards(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let migration_status = MIGRATION_STATUS.load(deps.storage)?;

    if matches!(migration_status, MigrationStatus::Open) {
        return Err(ContractError::MigrationStatusOpen {});
    }

    let spread_rewards: Vec<_> = get_collect_spread_rewards_msgs(deps.as_ref(), env)?
        .into_iter()
        .map(|m| SubMsg::reply_on_success(m, Replies::CollectSpreadRewards.into()))
        .collect();
    let incentives: Vec<_> = get_collect_incentives_msg(deps.as_ref(), env)?
        .into_iter()
        .map(|m| SubMsg::reply_on_success(m, Replies::CollectIncentives.into()))
        .collect();

    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "collect_rewards")
        .add_submessages(spread_rewards)
        .add_submessages(incentives))
}

pub fn handle_collect_spread_rewards_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let data: Result<MsgCollectSpreadRewardsResponse, ContractError> = data
        .into_result()
        .map_err(StdError::generic_err)?
        .data
        .map(|b| Ok(b.try_into()?))
        .unwrap_or(Ok(MsgCollectSpreadRewardsResponse {
            collected_spread_rewards: vec![],
        }));

    let response: MsgCollectSpreadRewardsResponse = data?;
    let mut response_coin_list = CoinList::new();
    response_coin_list.merge(try_proto_to_cosmwasm_coins(
        response.clone().collected_spread_rewards,
    )?)?;

    // calculate the strategist fee and remove the share at source
    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let strategist_fee = response_coin_list.sub_ratio(vault_config.performance_fee)?;

    let mut response = Response::new()
        .add_attribute(
            "collected_spread_rewards",
            format!("{:?}", response.clone().collected_spread_rewards),
        )
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_collect_spread_rewards");

    // Conditionally add a bank send message if the strategist fee is not empty
    if !strategist_fee.is_empty() {
        let bank_send_msg = BankMsg::Send {
            to_address: vault_config.treasury.to_string(),
            amount: strategist_fee.coins(),
        };
        response = response
            .add_message(bank_send_msg.clone())
            .add_attribute("strategist_fee", format!("{:?}", strategist_fee.coins()));
    }

    Ok(response)
}

pub fn handle_collect_incentives_reply(
    deps: DepsMut,
    _env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let data: Result<MsgCollectIncentivesResponse, ContractError> = data
        .into_result()
        .map_err(StdError::generic_err)?
        .data
        .map(|b| Ok(b.try_into()?))
        .unwrap_or(Ok(MsgCollectIncentivesResponse {
            collected_incentives: vec![],
            forfeited_incentives: vec![],
        }));

    let response: MsgCollectIncentivesResponse = data?;
    let mut response_coin_list = CoinList::new();
    response_coin_list.merge(try_proto_to_cosmwasm_coins(
        response.clone().collected_incentives,
    )?)?;

    // calculate the strategist fee and remove the share at source
    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let strategist_fee: CoinList = response_coin_list.sub_ratio(vault_config.performance_fee)?;

    // Create the base response object
    let mut response = Response::new()
        .add_attribute(
            "collected_incentives",
            format!("{:?}", response.clone().collected_incentives),
        )
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_collect_incentives");

    // Conditionally add a bank send message if the strategist fee is not empty
    if !strategist_fee.is_empty() {
        let bank_send_msg = BankMsg::Send {
            to_address: vault_config.treasury.to_string(),
            amount: sort_tokens(strategist_fee.coins()),
        };
        response = response
            .add_message(bank_send_msg)
            .add_attribute("strategist_fee", format!("{:?}", strategist_fee.coins()));
    }

    Ok(response)
}
