use crate::{state::POSITION, ContractError};
use cosmwasm_std::{attr, Addr, Attribute, BankMsg, Coin, Deps, Env, Uint128};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    MsgCollectIncentives, MsgCollectSpreadRewards,
};

pub fn refund_bank_msg(
    receiver: Addr,
    refund0: Option<Coin>,
    refund1: Option<Coin>,
) -> Result<Option<(BankMsg, Vec<Attribute>)>, ContractError> {
    let mut attributes: Vec<Attribute> = vec![];
    let mut coins: Vec<Coin> = vec![];

    if let Some(refund0) = refund0 {
        if refund0.amount > Uint128::zero() {
            attributes.push(attr("refund0", refund0.amount));
            coins.push(refund0)
        }
    }
    if let Some(refund1) = refund1 {
        if refund1.amount > Uint128::zero() {
            attributes.push(attr("refund1", refund1.amount));
            coins.push(refund1)
        }
    }
    let result: Option<(BankMsg, Vec<Attribute>)> = if !coins.is_empty() {
        Some((
            BankMsg::Send {
                to_address: receiver.to_string(),
                amount: coins,
            },
            attributes,
        ))
    } else {
        None
    };
    Ok(result)
}

pub fn collect_incentives_msg(deps: Deps, env: Env) -> Result<MsgCollectIncentives, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectIncentives {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}

pub fn collect_spread_rewards_msg(
    deps: Deps,
    env: Env,
) -> Result<MsgCollectSpreadRewards, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectSpreadRewards {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}
