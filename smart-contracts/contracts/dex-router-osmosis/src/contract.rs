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
    MsgSwapExactAmountIn, PoolmanagerQuerier, SwapAmountInRoute, TotalPoolLiquidityResponse,
};

use quasar_types::error::assert_fund_length;
use std::str::FromStr;

use crate::error::{assert_non_empty_path, ContractError};
use crate::msg::{BestPathForPairResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{RecipientInfo, OWNER, PATHS, RECIPIENT_INFO};

const _CONTRACT_NAME: &str = "quasar:dex-router-osmosis";
const CONTRACT_NAME: &str = "crates.io:dex-router-osmosis";
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
        } => swap(deps, env, info, path, out_denom, minimum_receive),
        ExecuteMsg::SetPath {
            offer_denom,
            ask_denom,
            path,
            bidirectional,
        } => set_path(deps, info, offer_denom, ask_denom, path, bidirectional),
        ExecuteMsg::RemovePath {
            offer_denom,
            ask_denom,
            path,
            bidirectional,
        } => remove_path(deps, info, offer_denom, ask_denom, path, bidirectional),
        ExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
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
    path: Option<Vec<SwapAmountInRoute>>,
    out_denom: String,
    minimum_receive: Option<Uint128>,
) -> Result<Response, ContractError> {
    assert_fund_length(info.funds.len(), 1)?;
    let swap_path = if let Some(path) = path {
        assert_non_empty_path(&path)?;
        path
    } else if let Some(best_path) =
        query_best_path_for_pair(&deps.as_ref(), info.funds[0].clone(), out_denom.clone())?
    {
        best_path.path
    } else {
        return Err(ContractError::NoPathFound {
            offer: info.funds[0].denom.clone(),
            ask: out_denom,
        });
    };

    let msg = MsgSwapExactAmountIn {
        sender: env.contract.address.to_string(),
        routes: swap_path,
        token_in: Some(cosmwasm_to_proto_coins(info.funds.clone())[0].clone()),
        token_out_min_amount: minimum_receive.unwrap_or_default().to_string(),
    };
    RECIPIENT_INFO.save(
        deps.storage,
        &RecipientInfo {
            address: info.sender.clone(),
            denom: out_denom,
        },
    )?;
    let event = Event::new(_CONTRACT_NAME)
        .add_attribute("operation", "swap")
        .add_attribute("offer_amount", info.funds[0].amount)
        .add_attribute("to", info.sender.to_string());
    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(msg, SWAP_REPLY_ID))
        .add_event(event))
}

fn get_denoms(deps: &Deps, path: &[u64]) -> StdResult<Vec<(String, String)>> {
    let pool_querier = PoolmanagerQuerier::new(&deps.querier);
    let liquidity: Result<Vec<TotalPoolLiquidityResponse>, StdError> = path
        .iter()
        .map(|pool_id| pool_querier.total_pool_liquidity(*pool_id))
        .collect();
    let liquidity = liquidity?;
    Ok(liquidity
        .into_iter()
        .map(|liq| {
            (
                liq.liquidity[0].denom.clone(),
                liq.liquidity[1].denom.clone(),
            )
        })
        .collect())
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
    assert_non_empty_path(&path)?;
    let denoms = get_denoms(&deps.as_ref(), &path)?;
    let key = (offer_denom.clone(), ask_denom.clone());

    let mut offer_denom = offer_denom;
    let mut out_denoms = vec![];
    let mut in_denoms = vec![];
    for denom_pair in &denoms {
        in_denoms.push(offer_denom.clone());
        if offer_denom == denom_pair.0 {
            offer_denom = denom_pair.1.clone();
        } else if offer_denom == denom_pair.1 {
            offer_denom = denom_pair.0.clone();
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
    if offer_denom != ask_denom {
        return Err(ContractError::InvalidSwapPath {
            path,
            reason: format!(
                "Could not find {}, available denoms: {:?}",
                ask_denom,
                denoms.last().unwrap()
            ),
        });
    }

    let mut new_paths = vec![path
        .iter()
        .zip(out_denoms.iter())
        .map(|(pool_id, denom)| SwapAmountInRoute {
            pool_id: *pool_id,
            token_out_denom: denom.clone(),
        })
        .collect()];
    PATHS.update(deps.storage, key.clone(), |paths| -> StdResult<_> {
        if let Some(paths) = paths {
            new_paths.extend(paths.into_iter());
        }
        Ok(new_paths)
    })?;

    let mut event = Event::new(_CONTRACT_NAME)
        .add_attribute("operation", "set path")
        .add_attribute("key", format!("{:?}", key))
        .add_attribute(
            "path",
            path.iter()
                .map(|pool_id| pool_id.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
    if bidirectional {
        let mut new_paths = vec![path
            .iter()
            .rev()
            .zip(in_denoms.iter().rev())
            .map(|(pool_id, denom)| SwapAmountInRoute {
                pool_id: *pool_id,
                token_out_denom: denom.clone(),
            })
            .collect()];
        let reverse_key = (key.1, key.0);
        PATHS.update(deps.storage, reverse_key.clone(), |paths| -> StdResult<_> {
            if let Some(paths) = paths {
                new_paths.extend(paths.into_iter());
            }
            Ok(new_paths)
        })?;
        event = event.add_attribute("key", format!("{:?}", reverse_key));
    }

    Ok(Response::default().add_event(event))
}

fn try_remove_path(
    paths: Option<Vec<Vec<SwapAmountInRoute>>>,
    path: &[u64],
    offer_denom: String,
    ask_denom: String,
) -> Result<Option<Vec<Vec<SwapAmountInRoute>>>, ContractError> {
    if let Some(mut paths) = paths {
        let idx = paths.iter().position(|p| -> bool {
            path.iter()
                .zip(p.iter().map(|p| p.pool_id))
                .all(|(pool_id0, pool_id1)| pool_id0 == &pool_id1)
        });
        if let Some(idx) = idx {
            paths.remove(idx);
            if !paths.is_empty() {
                return Ok(Some(paths));
            } else {
                return Ok(None);
            }
        }
    }

    Err(ContractError::NoPathFound {
        offer: offer_denom,
        ask: ask_denom,
    })
}

pub fn remove_path(
    deps: DepsMut,
    info: MessageInfo,
    offer_denom: String,
    ask_denom: String,
    path: Vec<u64>,
    bidirectional: bool,
) -> Result<Response, ContractError> {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    assert_non_empty_path(&path)?;

    let key = (offer_denom.clone(), ask_denom.clone());
    let paths = PATHS.may_load(deps.storage, key.clone())?;
    let paths = try_remove_path(paths, &path, offer_denom.clone(), ask_denom.clone())?;
    if let Some(paths) = paths {
        PATHS.save(deps.storage, key.clone(), &paths)?;
    } else {
        PATHS.remove(deps.storage, key.clone());
    }

    let mut event = Event::new(_CONTRACT_NAME)
        .add_attribute("operation", "remove path")
        .add_attribute("key", format!("{:?}", key))
        .add_attribute(
            "path",
            path.iter()
                .map(|pool_id| pool_id.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
    if bidirectional {
        let reverse_key = (ask_denom.clone(), offer_denom.clone());
        let paths = PATHS.may_load(deps.storage, reverse_key.clone())?;
        let mut path = path;
        path.reverse();
        let paths = try_remove_path(paths, &path, ask_denom.clone(), offer_denom.clone())?;
        if let Some(paths) = paths {
            PATHS.save(deps.storage, reverse_key.clone(), &paths)?;
        } else {
            PATHS.remove(deps.storage, reverse_key.clone());
        }
        event = event.add_attribute("key", format!("{:?}", reverse_key));
    }

    Ok(Response::default().add_event(event))
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
            let out = simulate_swaps(deps, &offer, path.clone())?;
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
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("migrate", "succesful"))
}
