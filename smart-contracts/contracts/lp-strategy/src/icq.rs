use cosmwasm_std::{
    to_binary, Decimal, Env, Fraction, IbcMsg, IbcTimeout, QuerierWrapper, StdError, Storage,
    SubMsg, Uint128,
};
use osmosis_std::types::{
    cosmos::{bank::v1beta1::QueryBalanceRequest, base::v1beta1::Coin as OsmoCoin},
    osmosis::{
        gamm::{
            v1beta1::{QueryCalcExitPoolCoinsFromSharesRequest, QueryCalcJoinPoolSharesRequest},
            v2::QuerySpotPriceRequest,
        },
        lockup::LockedRequest,
    },
};
use prost::Message;
use quasar_types::icq::{InterchainQueryPacketData, Query};

use crate::{
    error::ContractError,
    helpers::{check_icq_channel, create_ibc_ack_submsg, get_ica_address, IbcMsgKind},
    state::{
        BOND_QUEUE, CONFIG, FAILED_JOIN_QUEUE, IBC_LOCK, ICA_CHANNEL, ICQ_CHANNEL, LP_SHARES,
        OSMO_LOCK, PENDING_BOND_QUEUE, PENDING_UNBOND_QUEUE, SIMULATED_JOIN_AMOUNT_IN,
        UNBOND_QUEUE,
    },
};

/// try_icq only does something if the IBC_LOCK is unlocked. When it is unlocked,
/// all pending bonds are moved into the active bond queue.
///
/// It then prepares the following queries:
///     - ICA balance in base denom.
///     - ICA balance in quote denom.
///     - ICA balance in LP shares.
///     - SimulateJoinPool with the total pending bonds amount to estimate slippage and saves the total pending bonds amount in state.
///     - SimulateExitPool with the entire ICA locked amount to get the total value in lp tokens.
///     - SpotPrice of base denom and quote denom to convert quote denom from exitpool to the base denom.
///     - LockedByID to get the current lock state.
///
/// It also moves all pending unbonds into the active unbond queue and returns an IBC send packet with the queries as a submessage.
pub fn try_icq(
    storage: &mut dyn Storage,
    _querier: QuerierWrapper,
    env: Env,
) -> Result<Option<SubMsg>, ContractError> {
    if IBC_LOCK.load(storage)?.is_unlocked() {
        // TODO fetching ICQ channel and confirming vs handshake version can be a single function
        let icq_channel = ICQ_CHANNEL.load(storage)?;
        check_icq_channel(storage, icq_channel.clone())?;

        let mut pending_bonds_value = Uint128::zero();
        // we dump pending bonds into the active bond queue
        while !PENDING_BOND_QUEUE.is_empty(storage)? {
            let bond = PENDING_BOND_QUEUE.pop_front(storage)?;
            if let Some(bond) = bond {
                BOND_QUEUE.push_back(storage, &bond)?;
                pending_bonds_value = pending_bonds_value.checked_add(bond.amount)?;
            }
        }

        let failed_bonds_amount = FAILED_JOIN_QUEUE
            .iter(storage)?
            .try_fold(Uint128::zero(), |acc, val| -> Result<Uint128, StdError> {
                Ok(acc + val?.amount)
            })?;

        // the bonding amount that we want to calculate the slippage for is the amount of funds in new bonds and the amount of funds that have
        // previously failed to join the pool. These funds are already located on Osmosis and should not be part of the transfer to Osmosis.
        let bonding_amount = pending_bonds_value + failed_bonds_amount;
        // deposit needs to internally rebuild the amount of funds under the smart contract
        let packet = prepare_full_query(storage, env.clone(), bonding_amount)?;

        let send_packet_msg = IbcMsg::SendPacket {
            channel_id: icq_channel,
            data: to_binary(&packet)?,
            timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(7200)),
        };

        while !PENDING_UNBOND_QUEUE.is_empty(storage)? {
            let unbond = PENDING_UNBOND_QUEUE.pop_front(storage)?;
            if let Some(unbond) = unbond {
                UNBOND_QUEUE.push_back(storage, &unbond)?;
            }
        }

        let channel = ICQ_CHANNEL.load(storage)?;

        Ok(Some(create_ibc_ack_submsg(
            storage,
            IbcMsgKind::Icq,
            send_packet_msg,
            channel,
        )?))
    } else {
        Ok(None)
    }
}

pub fn prepare_full_query(
    storage: &mut dyn Storage,
    _env: Env,
    bonding_amount: Uint128,
) -> Result<InterchainQueryPacketData, ContractError> {
    let ica_channel = ICA_CHANNEL.load(storage)?;
    // todo: query flows should be separated by which flowType we're doing (bond, unbond, startunbond)
    let address = get_ica_address(storage, ica_channel)?;
    let config = CONFIG.load(storage)?;
    // we query the current balance on our ica address
    let base_balance = QueryBalanceRequest {
        address: address.clone(),
        denom: config.base_denom.clone(),
    };
    let quote_balance = QueryBalanceRequest {
        address: address.clone(),
        denom: config.quote_denom.clone(),
    };
    let lp_balance = QueryBalanceRequest {
        address,
        denom: config.pool_denom.clone(),
    };
    // we simulate the result of a join pool to estimate the slippage we can expect during this deposit
    // we use the current balance of local_denom for this query. This is safe because at any point
    // a pending deposit will only use the current balance of the vault. QueryCalcJoinPoolSharesRequest
    // since we're going to be moving the entire pending bond queue to the bond queue in this icq, we  can
    // fold the PENDING_BOND_QUEUE
    // April 27 2023 - we removed get_usable_bond_balance, but we will have to bring it back when we do error
    // recovery for deposits

    // we save the amount to scale the slippage against in the icq ack for other incoming bonds
    SIMULATED_JOIN_AMOUNT_IN.save(storage, &bonding_amount)?;

    let join_pool = QueryCalcJoinPoolSharesRequest {
        pool_id: config.pool_id,
        tokens_in: vec![OsmoCoin {
            denom: config.base_denom.clone(),
            amount: bonding_amount.to_string(),
        }],
    };

    // we simulate the result of an exit pool of our entire locked vault to get the total value in lp tokens
    // any funds still in one of the unlocked states when the contract can dispatch an icq again, should not be
    // taken into account, since they are either unlocking (out of the vault value), or errored in deposit
    let shares = LP_SHARES.load(storage)?.locked_shares;
    let shares_out = if !shares.is_zero() {
        shares
    } else {
        Uint128::one()
    };

    let exit_pool = QueryCalcExitPoolCoinsFromSharesRequest {
        pool_id: config.pool_id,
        share_in_amount: shares_out.to_string(),
    };
    // we query the spot price of our base_denom and quote_denom so we can convert the quote_denom from exitpool to the base_denom
    let spot_price = QuerySpotPriceRequest {
        pool_id: config.pool_id,
        base_asset_denom: config.base_denom,
        quote_asset_denom: config.quote_denom,
    };

    let lock_by_id = LockedRequest {
        lock_id: OSMO_LOCK.may_load(storage)?.unwrap_or(1),
    };

    // path have to be set manually, should be equal to the proto_queries of osmosis-std types
    let mut q = Query::new()
        .add_request(
            base_balance.encode_to_vec().into(),
            "/cosmos.bank.v1beta1.Query/Balance".to_string(),
        )
        .add_request(
            quote_balance.encode_to_vec().into(),
            "/cosmos.bank.v1beta1.Query/Balance".to_string(),
        )
        .add_request(
            lp_balance.encode_to_vec().into(),
            "/cosmos.bank.v1beta1.Query/Balance".to_string(),
        )
        .add_request(
            join_pool.encode_to_vec().into(),
            "/osmosis.gamm.v1beta1.Query/CalcJoinPoolShares".to_string(),
        )
        .add_request(
            exit_pool.encode_to_vec().into(),
            "/osmosis.gamm.v1beta1.Query/CalcExitPoolCoinsFromShares".to_string(),
        )
        .add_request(
            spot_price.encode_to_vec().into(),
            "/osmosis.gamm.v2.Query/SpotPrice".to_string(),
        );

    // only query LockedByID if we have a lock_id
    match OSMO_LOCK.may_load(storage)? {
        Some(lock_id) => {
            let lock_by_id = LockedRequest { lock_id };
            q = q.add_request(
                lock_by_id.encode_to_vec().into(),
                "/osmosis.lockup.Query/LockedByID".to_string(),
            );
        }
        None => todo!(),
    }

    Ok(q.encode_pkt())
}

// TODO add quote denom to base denom conversion
// calculate the total balance of the vault using the query from prepare_total_balance_query()
pub fn calc_total_balance(
    storage: &mut dyn Storage,
    ica_balance: Uint128,
    exit_pool: &Vec<OsmoCoin>,
    spot_price: Decimal,
) -> Result<Uint128, ContractError> {
    let config = CONFIG.load(storage)?;
    // if we receive no tokens in the response, the total balance
    if exit_pool.is_empty() {
        return Ok(ica_balance);
    }

    let base = exit_pool
        .iter()
        .find(|coin| coin.denom == config.base_denom)
        .ok_or(ContractError::BaseDenomNotFound)?;

    let quote = exit_pool
        .iter()
        .find(|coin| coin.denom == config.quote_denom)
        .ok_or(ContractError::QuoteDenomNotFound)?;
    // return ica_balance + base_amount + (quote_amount * spot_price)
    Ok(ica_balance
        .checked_add(Uint128::new(base.amount.parse::<u128>().map_err(
            |err| ContractError::ParseIntError {
                error: format!("ica_balance:{err:?}"),
                value: base.amount.clone(),
            },
        )?))?
        .checked_add(
            Uint128::new(quote.amount.parse::<u128>().map_err(|err| {
                ContractError::ParseIntError {
                    error: format!("quote_denom:{err:?}"),
                    value: quote.amount.clone(),
                }
            })?)
            .checked_multiply_ratio(spot_price.denominator(), spot_price.numerator())?,
        )?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, MockQuerier},
        Empty,
    };

    use crate::{
        ibc_lock::Lock,
        state::{LpCache, IBC_LOCK},
        test_helpers::default_setup,
    };

    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn calc_total_balance_works(ica_balance in 1..u64::MAX as u128, base_amount in 1..u64::MAX as u128, quote_amount in 1..u64::MAX as u128, spot_price in 1..u64::MAX as u128) {
            let mut deps = mock_dependencies();
            default_setup(deps.as_mut().storage).unwrap();
            let config = CONFIG.load(deps.as_ref().storage).unwrap();

            let tokens = vec![OsmoCoin{ denom: config.base_denom, amount: base_amount.to_string() }, OsmoCoin{ denom: config.quote_denom, amount: quote_amount.to_string() }];
            let spot = Decimal::raw(spot_price);
            let total = calc_total_balance(deps.as_mut().storage, Uint128::new(ica_balance), &tokens, spot).unwrap();
            let expecte_quote = Uint128::new(quote_amount).multiply_ratio(spot.denominator(), spot.numerator());
            prop_assert_eq!(total.u128(), ica_balance + base_amount + expecte_quote.u128())
        }
    }

    #[test]
    fn try_icq_unlocked_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();

        LP_SHARES
            .save(
                deps.as_mut().storage,
                &LpCache {
                    locked_shares: Uint128::new(100),
                    w_unlocked_shares: Uint128::zero(),
                    d_unlocked_shares: Uint128::zero(),
                },
            )
            .unwrap();

        // lock the ibc lock
        IBC_LOCK.save(deps.as_mut().storage, &Lock::new()).unwrap();

        let qx: MockQuerier<Empty> = MockQuerier::new(&[]);
        let q = QuerierWrapper::new(&qx);

        let res = try_icq(deps.as_mut().storage, q, env.clone()).unwrap();

        let icq_channel = ICQ_CHANNEL.load(deps.as_mut().storage).unwrap();

        let pkt = IbcMsg::SendPacket {
            channel_id: icq_channel.clone(),
            data: to_binary(
                &prepare_full_query(deps.as_mut().storage, env.clone(), Uint128::new(0)).unwrap(),
            )
            .unwrap(),
            timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(7200)),
        };

        assert_eq!(
            res.unwrap().msg,
            create_ibc_ack_submsg(deps.as_mut().storage, IbcMsgKind::Icq, pkt, icq_channel)
                .unwrap()
                .msg
        )
    }

    #[test]
    fn try_icq_locked_bond_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();

        // lock the ibc lock
        IBC_LOCK
            .save(deps.as_mut().storage, &Lock::new().lock_bond())
            .unwrap();

        let qx: MockQuerier<Empty> = MockQuerier::new(&[]);
        let q = QuerierWrapper::new(&qx);

        let res = try_icq(deps.as_mut().storage, q, env).unwrap();
        assert_eq!(res, None)
    }

    #[test]
    fn try_icq_locked_start_unbond_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();

        // lock the ibc lock
        IBC_LOCK
            .save(deps.as_mut().storage, &Lock::new().lock_start_unbond())
            .unwrap();

        let qx: MockQuerier<Empty> = MockQuerier::new(&[]);
        let q = QuerierWrapper::new(&qx);

        let res = try_icq(deps.as_mut().storage, q, env).unwrap();
        assert_eq!(res, None)
    }

    #[test]
    fn try_icq_locked_unbond_works() {
        let mut deps = mock_dependencies();
        default_setup(deps.as_mut().storage).unwrap();
        let env = mock_env();

        // lock the ibc lock
        IBC_LOCK
            .save(deps.as_mut().storage, &Lock::new().lock_unbond())
            .unwrap();

        let qx: MockQuerier<Empty> = MockQuerier::new(&[]);
        let q = QuerierWrapper::new(&qx);

        let res = try_icq(deps.as_mut().storage, q, env).unwrap();
        assert_eq!(res, None)
    }
}
