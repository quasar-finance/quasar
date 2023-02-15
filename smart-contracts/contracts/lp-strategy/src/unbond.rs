use cosmwasm_std::{
    to_binary, Addr, Coin, Env, IbcTimeout, Order, Storage, SubMsg, Timestamp, Uint128, WasmMsg,
};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin, osmosis::gamm::v1beta1::MsgExitSwapShareAmountIn,
};
use quasar_types::{
    callback::{Callback, UnbondResponse},
    ibc::MsgTransfer,
    ica::packet::ica_send,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ContractError,
    helpers::{create_ibc_ack_submsg, get_ica_address, IbcMsgKind, IcaMessages},
    msg::ExecuteMsg,
    state::{
        RawAmount, CONFIG, ICA_CHANNEL, LP_SHARES, RETURNING, RETURN_SOURCE_PORT, UNBONDING_CLAIMS,
        UNBOND_QUEUE,
    },
};

// if unbond time has expired, we unbond the shares and transfer back the funds
pub fn do_unbond(
    storage: &mut dyn Storage,
    env: &Env,
    owner: Addr,
    id: String,
) -> Result<(), ContractError> {
    let unbond = UNBONDING_CLAIMS.load(storage, (owner, id))?;

    if unbond.unlock_time.nanos() > env.block.time.nanos() {
        return Err(ContractError::SharesNotYetUnbonded);
    }

    Ok(UNBOND_QUEUE.push_back(storage, &unbond)?)
}

pub fn batch_unbond(
    storage: &mut dyn Storage,
    env: &Env,
    _vault_value: Uint128,
    _total_lp_shares: Uint128,
) -> Result<Option<SubMsg>, ContractError> {
    let mut total_exit = Uint128::zero();
    let mut pending: Vec<ReturningUnbond> = vec![];

    if UNBOND_QUEUE.is_empty(storage)? {
        return Ok(None);
    }

    // aggregate the current unbond queue, all items in this queue should be able to unbond
    while !UNBOND_QUEUE.is_empty(storage)? {
        let unbond = UNBOND_QUEUE
            .pop_front(storage)?
            .ok_or(ContractError::QueueItemNotFound {
                queue: "unbond".to_string(),
            })?;
        total_exit = total_exit.checked_add(unbond.lp_shares)?;
        // add the unbond to the pending unbonds
        pending.push(ReturningUnbond {
            amount: RawAmount::LpShares(unbond.lp_shares),
            owner: unbond.owner,
            id: unbond.id,
        });
    }

    let ica_address = get_ica_address(storage, ICA_CHANNEL.load(storage)?)?;
    let config = CONFIG.load(storage)?;

    LP_SHARES.update(storage, |old| -> Result<Uint128, ContractError> {
        Ok(old.checked_sub(total_exit)?)
    })?;

    let msg = MsgExitSwapShareAmountIn {
        sender: ica_address,
        pool_id: config.pool_id,
        token_out_denom: config.base_denom,
        share_in_amount: total_exit.to_string(),
        // TODO add a more robust estimation
        token_out_min_amount: Uint128::one().to_string(),
    };

    let pkt = ica_send::<MsgExitSwapShareAmountIn>(
        msg,
        ICA_CHANNEL.load(storage)?,
        IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
    )?;

    Ok(Some(create_ibc_ack_submsg(
        storage,
        &IbcMsgKind::Ica(IcaMessages::ExitPool(PendingReturningUnbonds {
            unbonds: pending,
        })),
        pkt,
    )?))
}

pub fn transfer_batch_unbond(
    storage: &mut dyn Storage,
    env: &Env,
    pending: &PendingReturningUnbonds,
    total_tokens: Uint128,
) -> Result<SubMsg, ContractError> {
    // the return transfer times out 400 seconds after we dispatch the ica msg towards osmosis
    let timeout_timestamp = IbcTimeout::with_timestamp(env.block.time.plus_seconds(400));

    // we can unwrap here since we have just instantiated with a timestamp
    let msg = return_transfer(
        storage,
        env,
        total_tokens,
        timeout_timestamp.timestamp().unwrap(),
    )?;

    let pkt = ica_send::<MsgTransfer>(
        msg,
        ICA_CHANNEL.load(storage)?,
        IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
    )?;

    Ok(create_ibc_ack_submsg(
        storage,
        &IbcMsgKind::Ica(IcaMessages::ReturnTransfer(pending.clone())),
        pkt,
    )?)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PendingReturningUnbonds {
    pub unbonds: Vec<ReturningUnbond>,
}

impl PendingReturningUnbonds {
    /// convert a se of lp shares to a set of local tokens
    pub fn lp_to_local_denom(&mut self, total_local: Uint128) -> Result<Uint128, ContractError> {
        let mut total_lp = Uint128::zero();
        for p in self.unbonds.iter() {
            match p.amount {
                crate::state::RawAmount::LocalDenom(_) => unimplemented!(),
                crate::state::RawAmount::LpShares(val) => total_lp = total_lp.checked_add(val)?,
            }
        }
        for p in self.unbonds.iter_mut() {
            match p.amount {
                // amount of tokens = lp_shares * total / total_lp
                crate::state::RawAmount::LpShares(val) => {
                    p.amount =
                        RawAmount::LocalDenom(val.checked_mul(total_local)?.checked_div(total_lp)?)
                }
                crate::state::RawAmount::LocalDenom(_) => unimplemented!(),
            }
        }
        Ok(total_lp)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ReturningUnbond {
    pub amount: RawAmount,
    pub owner: Addr,
    pub id: String,
}

// TODO this only works for the happy path in the receiver
pub fn finish_unbond(
    storage: &dyn Storage,
    unbond: &ReturningUnbond,
) -> Result<WasmMsg, ContractError> {
    let amount = match unbond.amount {
        RawAmount::LocalDenom(val) => val,
        RawAmount::LpShares(_) => return Err(ContractError::IncorrectRawAmount),
    };
    let msg = WasmMsg::Execute {
        contract_addr: unbond.owner.to_string(),
        msg: to_binary(&Callback::UnbondResponse(UnbondResponse {
            unbond_id: unbond.id.clone(),
        }))?,
        funds: vec![Coin {
            denom: CONFIG.load(storage)?.local_denom,
            amount,
        }],
    };
    Ok(msg)
}

fn return_transfer(
    storage: &mut dyn Storage,
    env: &Env,
    amount: Uint128,
    timeout_timestamp: Timestamp,
) -> Result<MsgTransfer, ContractError> {
    let config = CONFIG.load(storage)?;
    let ica_address = get_ica_address(storage, ICA_CHANNEL.load(storage)?)?;
    let id = get_next_return_id(storage)?;

    RETURNING.save(storage, id, &amount)?;

    Ok(MsgTransfer {
        // TODO do we want to keep the return port a constant? Leaning towards yes since ibc transfer app uses this the same
        source_port: RETURN_SOURCE_PORT.to_string(),
        source_channel: config.return_source_channel,
        token: Some(OsmoCoin {
            denom: config.base_denom,
            amount: amount.to_string(),
        }),
        sender: ica_address,
        receiver: env.contract.address.clone().to_string(),
        // timeout_height is disabled when set to 0
        // since height is kinda difficult to use, we always want to use the timestamp
        timeout_height: None,
        // timeout_timestamp is disabled when set to 0
        timeout_timestamp: Some(timeout_timestamp.nanos()),
        memo: serde_json_wasm::to_string(&IbcHook {
            wasm: Wasm {
                contract: env.contract.address.clone(),
                msg: ExecuteMsg::AcceptReturningFunds { id },
            },
        })
        .map_err(|_| ContractError::SerdeJsonSer)?,
    })
}

fn get_next_return_id(storage: &dyn Storage) -> Result<u64, ContractError> {
    let last = RETURNING
        .range(storage, None, None, Order::Descending)
        .next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0;
    }
    Ok(id)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
struct IbcHook {
    wasm: Wasm,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
struct Wasm {
    contract: Addr,
    msg: ExecuteMsg,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lp_to_local_denom() {
        let mut pending = PendingReturningUnbonds {
            unbonds: vec![
                ReturningUnbond {
                    owner: Addr::unchecked("address"),
                    id: "bla".to_string(),
                    amount: RawAmount::LpShares(Uint128::new(100)),
                },
                ReturningUnbond {
                    owner: Addr::unchecked("address"),
                    id: "bla".to_string(),
                    amount: RawAmount::LpShares(Uint128::new(50)),
                },
                ReturningUnbond {
                    owner: Addr::unchecked("address"),
                    id: "bla".to_string(),
                    amount: RawAmount::LpShares(Uint128::new(150)),
                },
            ],
        };
        pending.lp_to_local_denom(Uint128::new(3000)).unwrap();
        assert_eq!(
            pending.unbonds[0].amount,
            RawAmount::LocalDenom(Uint128::new(1000))
        );
        assert_eq!(
            pending.unbonds[1].amount,
            RawAmount::LocalDenom(Uint128::new(500))
        );
        assert_eq!(
            pending.unbonds[2].amount,
            RawAmount::LocalDenom(Uint128::new(1500))
        )
    }
}
