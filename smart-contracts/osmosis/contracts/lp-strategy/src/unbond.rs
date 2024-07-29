use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Coin, CosmosMsg, Env, IbcTimeout, Order, QuerierWrapper,
    Storage, SubMsg, Timestamp, Uint128, WasmMsg,
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
    ibc_util::calculate_token_out_min_amount,
    msg::ExecuteMsg,
    state::{
        RawAmount, CONFIG, IBC_TIMEOUT_TIME, ICA_CHANNEL, PENDING_UNBOND_QUEUE, RETURNING,
        RETURN_SOURCE_PORT, UNBONDING_CLAIMS, UNBOND_QUEUE,
    },
};

/// do_unbond is used to check whether an unbonding claim can be processed, keeps record of unbonding attempts,
/// and manages the workflow of unbonding operations.
///
/// It first retrieves the corresponding unbonding claim from storage and checks if the unlock time
/// of the unbonding claim has already passed by comparing it to the current block time. If the unlock time
/// has not yet passed, the function returns an error indicating that the shares are not yet available for unbonding.
///
/// If the unlock time has already passed, the `attempted` field of the unbonding claim is set to `true`.
/// This operation marks that an unbonding attempt has been made for these shares and prevents the execution of
/// multiple unbonding attempts for the same shares.
///
/// Finally, the unbonding claim is moved into the PENDING_UNBOND_QUEUE for later processing.
pub fn do_unbond(
    storage: &mut dyn Storage,
    env: &Env,
    owner: Addr,
    id: String,
) -> Result<(), ContractError> {
    let mut unbond = UNBONDING_CLAIMS.load(storage, (owner.clone(), id.clone()))?;

    if unbond.unlock_time.nanos() > env.block.time.nanos() {
        return Err(ContractError::SharesNotYetUnbonded);
    }

    unbond.attempted = true;
    UNBONDING_CLAIMS.save(storage, (owner, id), &unbond)?;

    Ok(PENDING_UNBOND_QUEUE.push_back(storage, &unbond)?)
}

pub fn batch_unbond(storage: &mut dyn Storage, env: &Env) -> Result<Option<SubMsg>, ContractError> {
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
        total_exit = total_exit
            .checked_add(unbond.lp_shares)
            .map_err(|err| ContractError::TracedOverflowError(err, "cal_total_exit".to_string()))?;
        // add the unbond to the pending unbonds
        pending.push(ReturningUnbond {
            amount: RawAmount::LpShares(unbond.lp_shares),
            owner: unbond.owner,
            id: unbond.id,
        });
    }

    // important to use lp_shares before it gets updated
    let token_out_min_amount = calculate_token_out_min_amount(storage)?;

    let msg = exit_swap(
        storage,
        env,
        total_exit,
        token_out_min_amount,
        PendingReturningUnbonds { unbonds: pending },
    )?;
    Ok(Some(msg))
}

pub(crate) fn exit_swap(
    storage: &mut dyn Storage,
    env: &Env,
    total_exit: Uint128,
    token_out_min_amount: Uint128,
    pending: PendingReturningUnbonds,
) -> Result<SubMsg, ContractError> {
    let pkt = do_exit_swap(storage, env, token_out_min_amount, total_exit)?;

    let channel = ICA_CHANNEL.load(storage)?;

    Ok(create_ibc_ack_submsg(
        storage,
        IbcMsgKind::Ica(IcaMessages::ExitPool(pending)),
        pkt,
        channel,
    )?)
}

pub(crate) fn do_exit_swap(
    storage: &mut dyn Storage,
    env: &Env,
    token_out_min_amount: Uint128,
    total_exit: Uint128,
) -> Result<cosmwasm_std::IbcMsg, ContractError> {
    let ica_address = get_ica_address(storage, ICA_CHANNEL.load(storage)?)?;
    let config = CONFIG.load(storage)?;

    let msg = MsgExitSwapShareAmountIn {
        sender: ica_address,
        pool_id: config.pool_id,
        token_out_denom: config.base_denom,
        share_in_amount: total_exit.to_string(),
        token_out_min_amount: token_out_min_amount.to_string(),
    };

    let pkt = ica_send::<MsgExitSwapShareAmountIn>(
        msg,
        ICA_CHANNEL.load(storage)?,
        IbcTimeout::with_timestamp(env.block.time.plus_seconds(IBC_TIMEOUT_TIME)),
    )?;
    Ok(pkt)
}

// TODO the total tokens parameter and pending is maybe a little weird, check whether we want to fold pending to get total_tokens (with gas costs etc)
pub fn transfer_batch_unbond(
    storage: &mut dyn Storage,
    env: &Env,
    pending: PendingReturningUnbonds,
    total_tokens: Uint128,
) -> Result<SubMsg, ContractError> {
    let pkt = do_transfer_batch_unbond(storage, env, total_tokens, pending.clone())?;

    // this is an ica channel in transfer batch unbond which is fine because even though
    // we are doing a transfer, its a return transfer which must be triggered by an ICA
    let channel = ICA_CHANNEL.load(storage)?;

    Ok(create_ibc_ack_submsg(
        storage,
        IbcMsgKind::Ica(IcaMessages::ReturnTransfer(pending)),
        pkt,
        channel,
    )?)
}

pub(crate) fn do_transfer_batch_unbond(
    storage: &mut dyn Storage,
    env: &Env,
    total_tokens: Uint128,
    pending: PendingReturningUnbonds,
) -> Result<cosmwasm_std::IbcMsg, ContractError> {
    // TODO, assert that raw amounts equal amount
    let timeout_timestamp =
        IbcTimeout::with_timestamp(env.block.time.plus_seconds(IBC_TIMEOUT_TIME));
    let msg = return_transfer(
        storage,
        env,
        total_tokens,
        timeout_timestamp.timestamp().unwrap(),
        pending,
    )?;
    let pkt = ica_send::<MsgTransfer>(
        msg,
        ICA_CHANNEL.load(storage)?,
        IbcTimeout::with_timestamp(env.block.time.plus_seconds(IBC_TIMEOUT_TIME)),
    )?;
    Ok(pkt)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ReturningUnbond {
    pub amount: RawAmount,
    pub owner: Addr,
    pub id: String,
}

// TODO this only works for the happy path in the receiver
pub fn finish_unbond(
    storage: &dyn Storage,
    querier: QuerierWrapper,
    unbond: &ReturningUnbond,
) -> Result<CosmosMsg, ContractError> {
    let amount = match unbond.amount {
        RawAmount::LocalDenom(val) => val,
        RawAmount::LpShares(_) => return Err(ContractError::IncorrectRawAmount),
    };
    let msg: CosmosMsg = if querier
        .query_wasm_contract_info(unbond.owner.as_str())
        .is_ok()
    {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: unbond.owner.to_string(),
            msg: to_json_binary(&Callback::UnbondResponse(UnbondResponse {
                unbond_id: unbond.id.clone(),
            }))?,
            funds: vec![Coin {
                denom: CONFIG.load(storage)?.local_denom,
                amount,
            }],
        })
    } else {
        CosmosMsg::Bank(BankMsg::Send {
            to_address: unbond.owner.to_string(),
            amount: vec![Coin {
                denom: CONFIG.load(storage)?.local_denom,
                amount,
            }],
        })
    };
    Ok(msg)
}

fn return_transfer(
    storage: &mut dyn Storage,
    env: &Env,
    amount: Uint128,
    timeout_timestamp: Timestamp,
    pending: PendingReturningUnbonds,
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
                msg: ExecuteMsg::AcceptReturningFunds { id, pending },
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

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        CosmosMsg,
    };

    use crate::{
        state::{Unbond, LP_SHARES, SIMULATED_EXIT_RESULT},
        test_helpers::default_setup,
    };

    use super::*;

    #[test]
    fn do_unbond_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let mut env = mock_env();
        let id = "my-id".to_string();

        let unbond = Unbond {
            lp_shares: Uint128::new(100),
            unlock_time: env.block.time,
            attempted: true,
            owner: owner.clone(),
            id: id.clone(),
        };
        UNBONDING_CLAIMS
            .save(deps.as_mut().storage, (owner.clone(), id.clone()), &unbond)
            .unwrap();

        let time = mock_env().block.time.plus_seconds(101);
        env.block.time = time;
        do_unbond(deps.as_mut().storage, &env, owner, id).unwrap();
        assert_eq!(
            PENDING_UNBOND_QUEUE
                .pop_front(deps.as_mut().storage)
                .unwrap()
                .unwrap(),
            unbond
        )
    }

    #[test]
    fn do_unbond_early_fails() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let env = mock_env();
        let id = "my-id".to_string();

        UNBONDING_CLAIMS
            .save(
                deps.as_mut().storage,
                (owner.clone(), id.clone()),
                &Unbond {
                    lp_shares: Uint128::new(100),
                    unlock_time: env.block.time.plus_nanos(1),
                    attempted: false,
                    owner: owner.clone(),
                    id: id.clone(),
                },
            )
            .unwrap();

        let err = do_unbond(deps.as_mut().storage, &env, owner, id).unwrap_err();
        assert_eq!(err, ContractError::SharesNotYetUnbonded)
    }

    #[test]
    fn batch_unbond_empty_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();

        let res = batch_unbond(deps.as_mut().storage, &env).unwrap();
        assert!(res.is_none())
    }

    #[test]
    fn batch_unbond_multiple_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();
        let owner = Addr::unchecked("bob");
        let id = "my-id".to_string();

        // test specific setup
        LP_SHARES
            .save(
                deps.as_mut().storage,
                &crate::state::LpCache {
                    locked_shares: Uint128::new(500),
                    w_unlocked_shares: Uint128::zero(),
                    d_unlocked_shares: Uint128::zero(),
                },
            )
            .unwrap();

        let unbonds = vec![
            Unbond {
                lp_shares: Uint128::new(100),
                unlock_time: env.block.time,
                attempted: false,
                owner: owner.clone(),
                id: id.clone(),
            },
            Unbond {
                lp_shares: Uint128::new(101),
                unlock_time: env.block.time,
                attempted: false,
                owner: owner.clone(),
                id: id.clone(),
            },
            Unbond {
                lp_shares: Uint128::new(102),
                unlock_time: env.block.time,
                attempted: false,
                owner,
                id,
            },
        ];

        for unbond in unbonds.iter() {
            UNBOND_QUEUE
                .push_back(deps.as_mut().storage, unbond)
                .unwrap();
        }

        SIMULATED_EXIT_RESULT
            .save(deps.as_mut().storage, &Uint128::from(100u128))
            .unwrap();

        let res = batch_unbond(deps.as_mut().storage, &env).unwrap();
        assert!(res.is_some());

        // checking above we have total exit amount = 100 + 101 + 102
        // while total shares in lp cache is defined at 500, this is important for calculating token_min_out_amount
        // these asserts are just a quality of life improvement since the failure in this test sucks
        let expected_exit_amount = Uint128::from(100u128 + 101u128 + 102u128);
        let actual_exit_amount = unbonds
            .iter()
            .fold(Uint128::zero(), |acc, u| acc + u.lp_shares);
        assert_eq!(expected_exit_amount, actual_exit_amount);

        let token_out_min_amount = calculate_token_out_min_amount(deps.as_mut().storage).unwrap();

        // check that the packet is as we expect
        let ica_address = get_ica_address(
            deps.as_ref().storage,
            ICA_CHANNEL.load(deps.as_ref().storage).unwrap(),
        )
        .unwrap();
        let config = CONFIG.load(deps.as_ref().storage).unwrap();
        let msg = MsgExitSwapShareAmountIn {
            sender: ica_address,
            pool_id: config.pool_id,
            token_out_denom: config.base_denom,
            share_in_amount: Uint128::new(303).to_string(),
            token_out_min_amount: token_out_min_amount.to_string(),
        };

        let pkt = ica_send::<MsgExitSwapShareAmountIn>(
            msg,
            ICA_CHANNEL.load(deps.as_ref().storage).unwrap(),
            IbcTimeout::with_timestamp(env.block.time.plus_seconds(IBC_TIMEOUT_TIME)),
        )
        .unwrap();

        assert_eq!(res.unwrap().msg, CosmosMsg::Ibc(pkt));
    }

    #[test]
    fn transfer_batch_unbond_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();
        let owner = Addr::unchecked("bob");
        let id = "my-id".to_string();

        let pending = PendingReturningUnbonds {
            unbonds: vec![
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(101)),
                    owner: owner.clone(),
                    id: id.clone(),
                },
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(102)),
                    owner: owner.clone(),
                    id: id.clone(),
                },
                ReturningUnbond {
                    amount: RawAmount::LocalDenom(Uint128::new(103)),
                    owner,
                    id,
                },
            ],
        };

        let total_tokens = Uint128::new(306);
        let timeout_timestamp =
            IbcTimeout::with_timestamp(env.block.time.plus_seconds(IBC_TIMEOUT_TIME));

        let res = transfer_batch_unbond(deps.as_mut().storage, &env, pending.clone(), total_tokens)
            .unwrap();

        let msg = return_transfer(
            deps.as_mut().storage,
            &env,
            total_tokens,
            timeout_timestamp.timestamp().unwrap(),
            pending,
        )
        .unwrap();

        let pkt = ica_send::<MsgTransfer>(
            msg,
            ICA_CHANNEL.load(deps.as_ref().storage).unwrap(),
            IbcTimeout::with_timestamp(env.block.time.plus_seconds(IBC_TIMEOUT_TIME)),
        )
        .unwrap();
        assert_eq!(res.msg, CosmosMsg::Ibc(pkt));
    }

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

    #[test]
    fn exit_swap_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();

        let pending = PendingReturningUnbonds {
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

        SIMULATED_EXIT_RESULT
            .save(deps.as_mut().storage, &Uint128::new(3000))
            .unwrap();

        let total_exit = pending
            .unbonds
            .iter()
            .fold(Uint128::zero(), |acc, u| match u.amount {
                RawAmount::LocalDenom(_) => unimplemented!(),
                RawAmount::LpShares(val) => acc + val,
            });

        let token_out_min_amount = calculate_token_out_min_amount(deps.as_mut().storage).unwrap();

        let msg = exit_swap(
            deps.as_mut().storage,
            &env,
            total_exit,
            token_out_min_amount,
            pending,
        )
        .unwrap();

        let ica_address = get_ica_address(
            deps.as_ref().storage,
            ICA_CHANNEL.load(deps.as_ref().storage).unwrap(),
        )
        .unwrap();
        let config = CONFIG.load(deps.as_ref().storage).unwrap();

        let expected = MsgExitSwapShareAmountIn {
            sender: ica_address,
            pool_id: config.pool_id,
            token_out_denom: config.base_denom,
            share_in_amount: total_exit.to_string(),
            // TODO add a more robust estimation
            token_out_min_amount: token_out_min_amount.to_string(),
        };

        let pkt = ica_send::<MsgExitSwapShareAmountIn>(
            expected,
            ICA_CHANNEL.load(deps.as_ref().storage).unwrap(),
            IbcTimeout::with_timestamp(env.block.time.plus_seconds(IBC_TIMEOUT_TIME)),
        )
        .unwrap();

        assert_eq!(msg.msg, CosmosMsg::Ibc(pkt))
    }
}
