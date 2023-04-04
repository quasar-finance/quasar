use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, Env, IbcMsg, IbcTimeout, QuerierWrapper, Response, Storage, SubMsg,
    Uint128, WasmMsg,
};

use osmosis_std::types::{cosmos::base::v1beta1::Coin, osmosis::lockup::MsgBeginUnlocking};
use quasar_types::{
    callback::{Callback, StartUnbondResponse},
    ica::packet::ica_send,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ContractError,
    helpers::get_total_primitive_shares,
    helpers::{
        create_callback_submsg, create_ibc_ack_submsg, get_ica_address, IbcMsgKind, IcaMessages,
    },
    ibc_lock::Lock,
    state::{
        LpCache, PendingSingleUnbond, Unbond, CONFIG, IBC_LOCK, IBC_TIMEOUT_TIME, ICA_CHANNEL,
        LP_SHARES, OSMO_LOCK, PENDING_UNBONDING_CLAIMS, SHARES, START_UNBOND_QUEUE,
        UNBONDING_CLAIMS,
    },
};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StartUnbond {
    pub owner: Addr,
    pub id: String,
    pub primitive_shares: Uint128,
}

pub fn do_start_unbond(
    storage: &mut dyn Storage,
    unbond: StartUnbond,
) -> Result<(), ContractError> {
    if UNBONDING_CLAIMS.has(storage, (unbond.owner.clone(), unbond.id.clone())) {
        return Err(ContractError::DuplicateKey);
    }

    //verify here against the amount in the queue aswell
    let queued_shares = START_UNBOND_QUEUE
        .iter(storage)?
        .map(|val| {
            let v = val?;
            if v.id == unbond.id {
                Err(ContractError::DuplicateKey)
            } else {
                Ok(v)
            }
        })
        .try_fold(
            Uint128::zero(),
            |acc, val| -> Result<Uint128, ContractError> {
                let v = val?;
                Ok(acc + v.primitive_shares)
            },
        )?;

    if SHARES.load(storage, unbond.owner.clone())? < (unbond.primitive_shares + queued_shares) {
        return Err(ContractError::InsufficientFunds);
    }

    Ok(START_UNBOND_QUEUE.push_back(storage, &unbond)?)
}

// batch unbond tries to unbond a batch of unbondings, should be called after the icq query has returned for deposits
pub fn batch_start_unbond(
    storage: &mut dyn Storage,
    env: &Env,
) -> Result<Option<SubMsg>, ContractError> {
    let mut to_unbond = Uint128::zero();
    let mut unbonds: Vec<PendingSingleUnbond> = vec![];

    if START_UNBOND_QUEUE.is_empty(storage)? {
        return Ok(None);
    }

    let total_lp_shares = LP_SHARES.load(storage)?;

    while !START_UNBOND_QUEUE.is_empty(storage)? {
        let unbond =
            START_UNBOND_QUEUE
                .pop_front(storage)?
                .ok_or(ContractError::QueueItemNotFound {
                    queue: "start_unbond".to_string(),
                })?;
        let lp_shares = single_unbond(storage, env, &unbond, total_lp_shares.locked_shares)?;
        to_unbond = to_unbond.checked_add(lp_shares)?;
        unbonds.push(PendingSingleUnbond {
            lp_shares,
            primitive_shares: unbond.primitive_shares,
            owner: unbond.owner,
            id: unbond.id,
        })
    }

    let pkt = do_begin_unlocking(storage, env, to_unbond)?;

    let channel = ICA_CHANNEL.load(storage)?;

    Ok(Some(create_ibc_ack_submsg(
        storage,
        IbcMsgKind::Ica(IcaMessages::BeginUnlocking(unbonds, to_unbond)),
        pkt,
        channel,
    )?))
}

pub fn do_begin_unlocking(
    storage: &mut dyn Storage,
    env: &Env,
    to_unbond: Uint128,
) -> Result<IbcMsg, ContractError> {
    let config = CONFIG.load(storage)?;
    let ica_address = get_ica_address(storage, ICA_CHANNEL.load(storage)?)?;

    let msg = MsgBeginUnlocking {
        owner: ica_address,
        id: OSMO_LOCK.load(storage)?,
        coins: vec![Coin {
            denom: config.pool_denom,
            amount: to_unbond.to_string(),
        }],
    };

    let pkt = ica_send::<MsgBeginUnlocking>(
        msg,
        ICA_CHANNEL.load(storage)?,
        IbcTimeout::with_timestamp(env.block.time.plus_seconds(IBC_TIMEOUT_TIME)),
    )?;

    Ok(pkt)
}

pub fn handle_start_unbond_ack(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    env: &Env,
    unbonds: Vec<PendingSingleUnbond>,
    total_start_unbonding: Uint128,
) -> Result<Response, ContractError> {
    let mut callback_submsgs: Vec<SubMsg> = vec![];
    for unbond in unbonds {
        if let Some(wasm_msg) = start_internal_unbond(storage, querier, env, unbond)? {
            // convert wasm_msg into cosmos_msg to be handled in create_callback_submsg
            let cosmos_msg = CosmosMsg::Wasm(wasm_msg);
            callback_submsgs.push(create_callback_submsg(storage, cosmos_msg)?);
        }
    }

    IBC_LOCK.update(storage, |lock| -> Result<Lock, ContractError> {
        Ok(lock.unlock_start_unbond())
    })?;

    // TODO, update the actual amount of locked lp shares in the lp cache here aswell
    LP_SHARES.update(storage, |mut cache| -> Result<LpCache, ContractError> {
        cache.w_unlocked_shares = cache.w_unlocked_shares.checked_add(total_start_unbonding)?;
        cache.locked_shares = cache.locked_shares.checked_sub(total_start_unbonding)?;
        Ok(cache)
    })?;

    Ok(Response::new()
        .add_attribute("start-unbond", "succes")
        .add_attribute("callback-submsgs", callback_submsgs.len().to_string())
        .add_messages(callback_submsgs.iter().map(|m| m.msg.clone())))
}

// in single_unbond, we change from using internal primitive to an actual amount of lp-shares that we can unbond
fn single_unbond(
    storage: &mut dyn Storage,
    _env: &Env,
    unbond: &StartUnbond,
    total_lp_shares: Uint128,
) -> Result<Uint128, ContractError> {
    let total_primitive_shares = get_total_primitive_shares(storage)?;

    Ok(unbond
        .primitive_shares
        .checked_mul(total_lp_shares)?
        .checked_div(total_primitive_shares)?)
}

// unbond starts unbonding an amount of lp shares
fn start_internal_unbond(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    env: &Env,
    unbond: PendingSingleUnbond,
) -> Result<Option<WasmMsg>, ContractError> {
    // check that we can create a new unbond
    if UNBONDING_CLAIMS.has(storage, (unbond.owner.clone(), unbond.id.clone())) {
        return Err(ContractError::DuplicateKey);
    }

    // remove amount of shares
    let left = SHARES
        .load(storage, unbond.owner.clone())?
        .checked_sub(unbond.primitive_shares)
        .map_err(|err| {
            ContractError::TracedOverflowError(err, "lower_shares_to_unbond".to_string())
        })?;
    // subtracting below zero here should trigger an error in check_sub
    if left.is_zero() {
        SHARES.remove(storage, unbond.owner.clone());
    } else {
        SHARES.save(storage, unbond.owner.clone(), &left)?;
    }

    // todo verify logic of unlock times
    let unlock_time = env
        .block
        .time
        .plus_seconds(CONFIG.load(storage)?.lock_period);

    // add amount of unbonding claims
    PENDING_UNBONDING_CLAIMS.save(
        storage,
        (unbond.owner.clone(), unbond.id.clone()),
        &Unbond {
            lp_shares: unbond.lp_shares,
            unlock_time,
            attempted: false,
            id: unbond.id.clone(),
            owner: unbond.owner.clone(),
        },
    )?;

    let msg = Callback::StartUnbondResponse(StartUnbondResponse {
        unbond_id: unbond.id.clone(),
        unlock_time,
    });

    if querier
        .query_wasm_contract_info(unbond.owner.as_str())
        .is_ok()
    {
        Ok(Some(WasmMsg::Execute {
            contract_addr: unbond.owner.to_string(),
            msg: to_binary(&msg)?,
            funds: vec![],
        }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        Addr, Binary, ContractInfoResponse, ContractResult, CosmosMsg, OverflowError,
        OverflowOperation, QuerierResult, StdError, Timestamp, Uint128, WasmMsg,
    };

    use crate::{
        state::{LpCache, PendingSingleUnbond, SHARES},
        test_helpers::default_setup,
    };

    use super::*;

    #[test]
    fn do_start_unbond_exact_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let id = "my-id".to_string();

        SHARES
            .save(deps.as_mut().storage, owner.clone(), &Uint128::new(1000))
            .unwrap();

        let unbond = StartUnbond {
            owner,
            id,
            primitive_shares: Uint128::new(1000),
        };
        do_start_unbond(deps.as_mut().storage, unbond).unwrap()
    }

    #[test]
    fn do_start_unbond_multiple_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let id = "my-id".to_string();

        SHARES
            .save(deps.as_mut().storage, owner.clone(), &Uint128::new(1000))
            .unwrap();

        let unbond1 = StartUnbond {
            owner: owner.clone(),
            id: id.to_string(),
            primitive_shares: Uint128::new(500),
        };
        let unbond2 = StartUnbond {
            owner: owner.clone(),
            id: id.to_string(),
            primitive_shares: Uint128::new(300),
        };
        let unbond3 = StartUnbond {
            owner,
            id,
            primitive_shares: Uint128::new(200),
        };

        do_start_unbond(deps.as_mut().storage, unbond1.clone()).unwrap();
        do_start_unbond(deps.as_mut().storage, unbond2.clone()).unwrap();
        do_start_unbond(deps.as_mut().storage, unbond3.clone()).unwrap();
        assert_eq!(START_UNBOND_QUEUE.len(deps.as_ref().storage).unwrap(), 3);
        assert_eq!(
            START_UNBOND_QUEUE
                .pop_front(deps.as_mut().storage)
                .unwrap()
                .unwrap(),
            unbond1
        );
        assert_eq!(
            START_UNBOND_QUEUE
                .pop_front(deps.as_mut().storage)
                .unwrap()
                .unwrap(),
            unbond2
        );
        assert_eq!(
            START_UNBOND_QUEUE
                .pop_front(deps.as_mut().storage)
                .unwrap()
                .unwrap(),
            unbond3
        )
    }

    #[test]
    fn do_start_unbond_not_enough_shares_fails() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let id = "my-id".to_string();

        SHARES
            .save(deps.as_mut().storage, owner.clone(), &Uint128::new(999))
            .unwrap();

        let unbond = StartUnbond {
            owner,
            id,
            primitive_shares: Uint128::new(1000),
        };
        let err = do_start_unbond(deps.as_mut().storage, unbond).unwrap_err();
        assert_eq!(err, ContractError::InsufficientFunds)
    }

    #[test]
    fn do_start_unbond_duplicate_key_fails() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let id = "my-id".to_string();

        SHARES
            .save(deps.as_mut().storage, owner.clone(), &Uint128::new(999))
            .unwrap();
        UNBONDING_CLAIMS
            .save(
                deps.as_mut().storage,
                (owner.clone(), id.clone()),
                &Unbond {
                    lp_shares: Uint128::new(420),
                    unlock_time: Timestamp::from_seconds(100),
                    attempted: false,
                    owner: owner.clone(),
                    id: id.clone(),
                },
            )
            .unwrap();

        let unbond = StartUnbond {
            owner,
            id,
            primitive_shares: Uint128::new(1000),
        };
        let err = do_start_unbond(deps.as_mut().storage, unbond).unwrap_err();
        assert_eq!(err, ContractError::DuplicateKey)
    }

    #[test]
    fn batch_start_unbond_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let env = mock_env();
        let id = "my-id".to_string();
        //test specific setup
        OSMO_LOCK.save(deps.as_mut().storage, &1).unwrap();
        SHARES
            .save(deps.as_mut().storage, owner.clone(), &Uint128::new(1000))
            .unwrap();

        LP_SHARES
            .save(
                deps.as_mut().storage,
                &LpCache {
                    locked_shares: Uint128::new(1000),
                    w_unlocked_shares: Uint128::zero(),
                    d_unlocked_shares: Uint128::zero(),
                },
            )
            .unwrap();

        let unbond1 = StartUnbond {
            owner,
            id,
            primitive_shares: Uint128::new(1000),
        };

        do_start_unbond(deps.as_mut().storage, unbond1).unwrap();

        let res = batch_start_unbond(deps.as_mut().storage, &env).unwrap();
        assert!(res.is_some());

        // check that the packet is as we expect
        let ica = get_ica_address(
            deps.as_ref().storage,
            ICA_CHANNEL.load(deps.as_ref().storage).unwrap(),
        )
        .unwrap();
        let msg = MsgBeginUnlocking {
            owner: ica,
            id: OSMO_LOCK.load(deps.as_mut().storage).unwrap(),
            coins: vec![Coin {
                denom: CONFIG.load(deps.as_ref().storage).unwrap().pool_denom,
                // integer truncation present here again
                amount: Uint128::new(999).to_string(),
            }],
        };

        let pkt = ica_send::<MsgBeginUnlocking>(
            msg,
            ICA_CHANNEL.load(deps.as_ref().storage).unwrap(),
            IbcTimeout::with_timestamp(env.block.time.plus_seconds(IBC_TIMEOUT_TIME)),
        )
        .unwrap();
        assert_eq!(res.unwrap().msg, CosmosMsg::Ibc(pkt));
    }

    #[test]
    fn single_unbond_big_math() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let env = mock_env();
        let id = "my-id".to_string();

        SHARES
            .save(deps.as_mut().storage, owner.clone(), &Uint128::new(100))
            .unwrap();
        SHARES
            .save(
                deps.as_mut().storage,
                Addr::unchecked("other_user"),
                &Uint128::new(900),
            )
            .unwrap();

        LP_SHARES
            .save(
                deps.as_mut().storage,
                &LpCache {
                    locked_shares: Uint128::new(10_000_000_000),
                    w_unlocked_shares: Uint128::zero(),
                    d_unlocked_shares: Uint128::zero(),
                },
            )
            .unwrap();

        let res = single_unbond(
            deps.as_mut().storage,
            &env,
            &StartUnbond {
                owner,
                id,
                primitive_shares: Uint128::new(100),
            },
            Uint128::new(10_000_000_000),
        )
        .unwrap();

        // assert_eq!(get_total_primitive_shares(deps.as_mut().storage).unwrap(), Uint128::new(1000));
        // we have a share loss here due to truncation, is this avoidable?
        assert_eq!(res, Uint128::new(999000999))
    }

    // this is an excellent first test to write a proptest for
    #[test]
    fn single_unbond_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let env = mock_env();
        let id = "my-id".to_string();

        SHARES
            .save(deps.as_mut().storage, owner.clone(), &Uint128::new(100))
            .unwrap();

        let res = single_unbond(
            deps.as_mut().storage,
            &env,
            &StartUnbond {
                owner,
                id,
                primitive_shares: Uint128::new(100),
            },
            Uint128::new(100),
        )
        .unwrap();
        // we have a share loss here due to truncation, is this avoidable?
        assert_eq!(res, Uint128::new(99))
    }

    #[test]
    fn start_internal_unbond_exact_shares_works() {
        // general setup
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let id = "my-id";
        let env = mock_env();

        deps.querier.update_wasm(|q| match q {
            cosmwasm_std::WasmQuery::ContractInfo { contract_addr: _ } => QuerierResult::Ok(
                ContractResult::Ok(to_binary(&ContractInfoResponse::default()).unwrap()),
            ),
            _ => unimplemented!(),
        });
        let w = QuerierWrapper::new(&deps.querier);

        assert!(w.query_wasm_contract_info(owner.clone()).is_ok());

        // test specific setup
        SHARES
            .save(&mut deps.storage, owner.clone(), &Uint128::new(100))
            .unwrap();
        let unbond = PendingSingleUnbond {
            lp_shares: Uint128::new(100),
            primitive_shares: Uint128::new(100),
            owner: owner.clone(),
            id: id.to_string(),
        };

        let res = start_internal_unbond(&mut deps.storage, w, &env, unbond).unwrap();
        assert_eq!(
            res.unwrap(),
            WasmMsg::Execute {
                contract_addr: owner.to_string(),
                msg: to_binary(&Callback::StartUnbondResponse(StartUnbondResponse {
                    unbond_id: id.to_string(),
                    unlock_time: env
                        .block
                        .time
                        .plus_seconds(CONFIG.load(deps.as_ref().storage).unwrap().lock_period)
                }))
                .unwrap(),
                funds: vec![]
            }
        )
    }

    #[test]
    fn start_internal_unbond_less_shares_works() {
        // general setup
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let id = "my-id";
        let env = mock_env();

        deps.querier.update_wasm(|q| match q {
            cosmwasm_std::WasmQuery::ContractInfo { contract_addr: _ } => QuerierResult::Ok(
                ContractResult::Ok(to_binary(&ContractInfoResponse::default()).unwrap()),
            ),
            _ => unimplemented!(),
        });
        let w = QuerierWrapper::new(&deps.querier);

        // test specific setup
        SHARES
            .save(&mut deps.storage, owner.clone(), &Uint128::new(101))
            .unwrap();
        let unbond = PendingSingleUnbond {
            lp_shares: Uint128::new(100),
            primitive_shares: Uint128::new(100),
            owner: owner.clone(),
            id: id.to_string(),
        };

        let res = start_internal_unbond(&mut deps.storage, w, &env, unbond).unwrap();
        assert_eq!(
            res.unwrap(),
            WasmMsg::Execute {
                contract_addr: owner.to_string(),
                msg: to_binary(&Callback::StartUnbondResponse(StartUnbondResponse {
                    unbond_id: id.to_string(),
                    unlock_time: env
                        .block
                        .time
                        .plus_seconds(CONFIG.load(deps.as_ref().storage).unwrap().lock_period)
                }))
                .unwrap(),
                funds: vec![]
            }
        );
        assert_eq!(
            SHARES.load(deps.as_ref().storage, owner).unwrap(),
            Uint128::one()
        )
    }

    #[test]
    fn start_internal_unbond_duplicate_key_fails() {
        // general setup
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let id = "my-id";
        let env = mock_env();

        deps.querier.update_wasm(|q| match q {
            cosmwasm_std::WasmQuery::ContractInfo { contract_addr: _ } => {
                QuerierResult::Ok(ContractResult::Ok(Binary::from_base64("deadbeef").unwrap()))
            }
            _ => unimplemented!(),
        });
        let w = QuerierWrapper::new(&deps.querier);

        // test specific setup
        SHARES
            .save(&mut deps.storage, owner.clone(), &Uint128::new(99))
            .unwrap();
        let unbond = PendingSingleUnbond {
            lp_shares: Uint128::new(100),
            primitive_shares: Uint128::new(100),
            owner: owner.clone(),
            id: id.to_string(),
        };
        let unlock_time = env
            .block
            .time
            .plus_seconds(CONFIG.load(deps.as_ref().storage).unwrap().lock_period);
        UNBONDING_CLAIMS
            .save(
                &mut deps.storage,
                (owner.clone(), id.to_string()),
                &Unbond {
                    lp_shares: Uint128::new(100),
                    unlock_time,
                    attempted: false,
                    owner,
                    id: id.to_string(),
                },
            )
            .unwrap();

        let res = start_internal_unbond(&mut deps.storage, w, &env, unbond).unwrap_err();
        assert_eq!(res, ContractError::DuplicateKey)
    }

    #[test]
    fn start_internal_unbond_not_enough_shares_fails() {
        // general setup
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let id = "my-id";
        let env = mock_env();

        deps.querier.update_wasm(|q| match q {
            cosmwasm_std::WasmQuery::ContractInfo { contract_addr: _ } => {
                QuerierResult::Ok(ContractResult::Ok(Binary::from_base64("deadbeef").unwrap()))
            }
            _ => unimplemented!(),
        });
        let w = QuerierWrapper::new(&deps.querier);

        // test specific setup
        SHARES
            .save(&mut deps.storage, owner.clone(), &Uint128::new(99))
            .unwrap();
        let unbond = PendingSingleUnbond {
            lp_shares: Uint128::new(100),
            primitive_shares: Uint128::new(100),
            owner,
            id: id.to_string(),
        };

        let res = start_internal_unbond(&mut deps.storage, w, &env, unbond).unwrap_err();
        assert_eq!(
            res,
            ContractError::OverflowError(OverflowError {
                operation: OverflowOperation::Sub,
                operand1: "99".to_string(),
                operand2: "100".to_string()
            })
        )
    }

    #[test]
    fn start_internal_unbond_no_shares_fails() {
        // general setup
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let owner = Addr::unchecked("bob");
        let id = "my-id";
        let env = mock_env();

        deps.querier.update_wasm(|q| match q {
            cosmwasm_std::WasmQuery::ContractInfo { contract_addr: _ } => {
                QuerierResult::Ok(ContractResult::Ok(Binary::from_base64("deadbeef").unwrap()))
            }
            _ => unimplemented!(),
        });
        let w = QuerierWrapper::new(&deps.querier);

        // test specific setup
        let unbond = PendingSingleUnbond {
            lp_shares: Uint128::new(100),
            primitive_shares: Uint128::new(100),
            owner,
            id: id.to_string(),
        };

        let res = start_internal_unbond(&mut deps.storage, w, &env, unbond).unwrap_err();
        assert_eq!(
            res,
            ContractError::Std(StdError::NotFound {
                kind: "cosmwasm_std::math::uint128::Uint128".to_string()
            })
        )
    }
}
