#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, Event, MessageInfo, Order, Reply,
    Response, StdError, StdResult, SubMsg, Uint128,
};
use cw2::set_contract_version;
use mars_owner::OwnerInit::SetInitialOwner;
use osmosis_std::cosmwasm_to_proto_coins;
use osmosis_std::types::osmosis::poolmanager::v1beta1::{
    MsgSwapExactAmountIn, PoolResponse, PoolmanagerQuerier, SwapAmountInRoute,
};

use prost::Message;
use quasar_types::error::assert_fund_length;
use std::str::FromStr;

use crate::error::ContractError;
use crate::msg::{BestPathForPairResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{RecipientInfo, OWNER, PATHS, RECIPIENT_INFO};

const CONTRACT_NAME: &str = "crates.io:dex-router";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const SWAP_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    OWNER.initialize(
        deps.storage,
        deps.api,
        SetInitialOwner {
            owner: info.sender.to_string(),
        },
    )?;

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
        ExecuteMsg::Swap {
            path,
            out_denom,
            minimum_receive,
            to,
        } => swap(deps, env, info, path, out_denom, minimum_receive, to),
        ExecuteMsg::SetPath {
            offer_denom,
            ask_denom,
            path,
            bidirectional,
        } => set_path(deps, info, offer_denom, ask_denom, path, bidirectional),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        SWAP_REPLY_ID => {
            let recipient_info = RECIPIENT_INFO.load(deps.storage)?;
            let balance = deps
                .querier
                .query_balance(env.contract.address.to_string(), recipient_info.denom)?;
            RECIPIENT_INFO.remove(deps.storage);
            Ok(Response::default().add_message(BankMsg::Send {
                to_address: recipient_info.address.to_string(),
                amount: vec![balance],
            }))
        }
        _ => panic!("not implemented"),
    }
}

pub fn swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    path: Vec<SwapAmountInRoute>,
    out_denom: String,
    minimum_receive: Option<Uint128>,
    to: Option<String>,
) -> Result<Response, ContractError> {
    assert_fund_length(info.funds.len(), 1)?;
    let recipient = to.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;
    let mut path = path;
    if path.is_empty() {
        if let Some(best_path) =
            query_best_path_for_pair(&deps.as_ref(), info.funds[0].clone(), out_denom.clone())?
        {
            path = best_path.path;
        } else {
            return Err(ContractError::NoPathFound {
                offer: info.funds[0].denom.clone(),
                ask: out_denom,
            });
        }
    }

    let msg = MsgSwapExactAmountIn {
        sender: env.contract.address.to_string(),
        routes: path,
        token_in: Some(cosmwasm_to_proto_coins(info.funds.clone())[0].clone()),
        token_out_min_amount: minimum_receive.unwrap_or_default().to_string(),
    };
    RECIPIENT_INFO.save(
        deps.storage,
        &RecipientInfo {
            address: info.sender,
            denom: out_denom,
        },
    )?;
    let event = Event::new("quasar dex-router")
        .add_attribute("operation", "swap")
        .add_attribute("offer_amount", info.funds[0].amount)
        .add_attribute("to", recipient.to_string());
    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(msg, SWAP_REPLY_ID))
        .add_event(event))
}

fn get_denoms(pool: PoolResponse) -> (String, String) {
    if let Some(pool) = pool.pool {
        if let Ok(pool) =
            osmosis_std::types::osmosis::gamm::poolmodels::stableswap::v1beta1::Pool::decode(
                pool.value.as_slice(),
            )
        {
            return (
                pool.pool_liquidity[0].denom.clone(),
                pool.pool_liquidity[1].denom.clone(),
            );
        }
        let cl_pool: Result<osmosis_std::types::osmosis::concentratedliquidity::v1beta1::Pool, _> =
            pool.try_into();
        if let Ok(pool) = cl_pool {
            return (pool.token0, pool.token1);
        }
    }

    panic!("Looks like we forgot to support some pools from osmosis")
}

pub fn set_path(
    deps: DepsMut,
    info: MessageInfo,
    offer_denom: String,
    ask_denom: String,
    path: Vec<u64>,
    bidirectional: bool,
) -> Result<Response, ContractError> {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    let pool_querier = PoolmanagerQuerier::new(&deps.querier);
    let pools: Result<Vec<PoolResponse>, StdError> = path
        .iter()
        .map(|pool_id| pool_querier.pool(*pool_id))
        .collect();
    let pools = pools?;
    let denoms: Vec<(String, String)> = pools.into_iter().map(|pool| get_denoms(pool)).collect();

    let mut offer_denom = offer_denom;
    let mut out_denoms = vec![];
    let mut in_denoms = vec![];
    for denom_pair in denoms {
        in_denoms.push(offer_denom.clone());
        if offer_denom == denom_pair.0 {
            offer_denom = denom_pair.1;
        } else if offer_denom == denom_pair.1 {
            offer_denom = denom_pair.0;
        } else {
            return Err(ContractError::InvalidSwapPath {
                path,
                reason: format!(
                    "Could not find {}, available denoms: {:?}",
                    offer_denom, denom_pair
                ),
            });
        }
        out_denoms.push(offer_denom.clone());
    }

    let mut new_paths = vec![path
        .iter()
        .zip(out_denoms.iter())
        .map(|(pool_id, denom)| SwapAmountInRoute {
            pool_id: pool_id.clone(),
            token_out_denom: denom.clone(),
        })
        .collect()];
    PATHS.update(
        deps.storage,
        (offer_denom.clone(), ask_denom.clone()),
        |paths| -> StdResult<_> {
            if let Some(paths) = paths {
                new_paths.extend(paths.into_iter());
            }
            Ok(new_paths)
        },
    )?;

    if bidirectional {
        let mut new_paths = vec![path
            .iter()
            .rev()
            .zip(in_denoms.iter().rev())
            .map(|(pool_id, denom)| SwapAmountInRoute {
                pool_id: pool_id.clone(),
                token_out_denom: denom.clone(),
            })
            .collect()];
        PATHS.update(
            deps.storage,
            (ask_denom.clone(), offer_denom.clone()),
            |paths| -> StdResult<_> {
                if let Some(paths) = paths {
                    new_paths.extend(paths.into_iter());
                }
                Ok(new_paths)
            },
        )?;
    }

    Ok(Response::default()
        .add_attribute("action", "set path")
        .add_attribute("key", format!("{:?}", (offer_denom, ask_denom))))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::SimulateSwaps { offer, path } => {
            Ok(to_json_binary(&simulate_swaps(&deps, &offer, path)?)?)
        }
        QueryMsg::PathsForPair {
            offer_denom,
            ask_denom,
        } => Ok(to_json_binary(&query_paths_for_pair(
            &deps,
            offer_denom,
            ask_denom,
        )?)?),
        QueryMsg::BestPathForPair { offer, ask_denom } => Ok(to_json_binary(
            &query_best_path_for_pair(&deps, offer, ask_denom)?,
        )?),
        QueryMsg::SupportedOfferAssets { ask_denom } => Ok(to_json_binary(
            &query_supported_offer_assets(deps, ask_denom)?,
        )?),
        QueryMsg::SupportedAskAssets { offer_denom } => Ok(to_json_binary(
            &query_supported_ask_assets(deps, offer_denom)?,
        )?),
    }
}

pub fn simulate_swaps(
    deps: &Deps,
    offer: &Coin,
    path: Vec<SwapAmountInRoute>,
) -> Result<Uint128, ContractError> {
    let querier = PoolmanagerQuerier::new(&deps.querier);
    let response = querier.estimate_swap_exact_amount_in(0, offer.to_string(), path)?;

    Ok(Uint128::from_str(&response.token_out_amount)?)
}

pub fn query_paths_for_pair(
    deps: &Deps,
    offer_denom: String,
    ask_denom: String,
) -> Result<Vec<Vec<SwapAmountInRoute>>, ContractError> {
    let paths = PATHS.may_load(deps.storage, (offer_denom.clone(), ask_denom.clone()))?;
    if let Some(paths) = paths {
        if !paths.is_empty() {
            return Ok(paths);
        }
    }

    Err(ContractError::NoPathFound {
        offer: offer_denom,
        ask: ask_denom,
    })
}

pub fn query_best_path_for_pair(
    deps: &Deps,
    offer: Coin,
    ask_denom: String,
) -> Result<Option<BestPathForPairResponse>, ContractError> {
    let paths = query_paths_for_pair(deps, offer.denom.clone(), ask_denom)?;
    if paths.is_empty() {
        return Err(ContractError::NoPathsToCheck {});
    }
    let swap_paths: Result<Vec<BestPathForPairResponse>, ContractError> = paths
        .into_iter()
        .map(|path| {
            let out = simulate_swaps(deps, &offer, path.clone().into())?;
            Ok(BestPathForPairResponse {
                path,
                return_amount: out,
            })
        })
        .collect();

    let best_path = swap_paths?
        .into_iter()
        .max_by(|a, b| a.return_amount.cmp(&b.return_amount));

    Ok(best_path)
}

pub fn query_supported_offer_assets(
    deps: Deps,
    ask_denom: String,
) -> Result<Vec<String>, ContractError> {
    let mut offer_denoms: Vec<String> = vec![];
    for x in PATHS.range(deps.storage, None, None, Order::Ascending) {
        let ((offer_denom, path_ask_denom), _) = x?;
        if path_ask_denom == ask_denom {
            offer_denoms.push(offer_denom);
        }
    }
    Ok(offer_denoms)
}

pub fn query_supported_ask_assets(
    deps: Deps,
    offer_denom: String,
) -> Result<Vec<String>, ContractError> {
    let mut ask_denoms: Vec<String> = vec![];
    for x in PATHS.range(deps.storage, None, None, Order::Ascending) {
        let ((path_offer_denom, ask_denom), _) = x?;
        if path_offer_denom == offer_denom {
            ask_denoms.push(ask_denom);
        }
    }
    Ok(ask_denoms)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
