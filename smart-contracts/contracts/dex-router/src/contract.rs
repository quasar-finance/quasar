use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_json_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, Event, MessageInfo, Order,
    Reply, Response, StdResult, SubMsg, Uint128,
};
use cw2::set_contract_version;
use cw_asset::{AssetInfo, AssetInfoUnchecked};
use mars_owner::OwnerInit::SetInitialOwner;
use osmosis_std::cosmwasm_to_proto_coins;
use osmosis_std::types::osmosis::poolmanager::v1beta1::{
    MsgSwapExactAmountIn, PoolmanagerQuerier, SwapAmountInRoute,
};
use quasar_types::error::assert_fund_length;

use crate::error::ContractError;
use crate::msg::{BestPathForPairResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{RecipientInfo, OWNER, PATHS, RECIPIENT_INFO};

const CONTRACT_NAME: &str = "crates.io:cw-dex-router";
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
            routes,
            minimum_receive,
            to,
        } => swap(deps, env, info, routes, minimum_receive, to),
        ExecuteMsg::SetPath {
            offer_asset,
            ask_asset,
            path,
            bidirectional,
        } => {
            let api = deps.api;
            set_path(
                deps,
                info,
                offer_asset.check(api, None)?,
                ask_asset.check(api, None)?,
                path,
                bidirectional,
            )
        }
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
    routes: Vec<SwapAmountInRoute>,
    minimum_receive: Option<Uint128>,
    to: Option<String>,
) -> Result<Response, ContractError> {
    assert_fund_length(info.funds.len(), 1)?;
    //Validate input or use sender address if None
    let recipient = to.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    let out_denom = routes.last().unwrap().token_out_denom.clone();
    let msg = MsgSwapExactAmountIn {
        sender: env.contract.address.to_string(),
        routes,
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

pub fn set_path(
    deps: DepsMut,
    info: MessageInfo,
    offer_asset: AssetInfo,
    ask_asset: AssetInfo,
    path: Vec<SwapAmountInRoute>,
    bidirectional: bool,
) -> Result<Response, ContractError> {
    OWNER.assert_owner(deps.storage, &info.sender)?;

    // Validate the path
    if path
        .last()
        .is_some_and(|route| AssetInfo::native(route.token_out_denom.clone()) != ask_asset)
    {
        return Err(ContractError::InvalidSwapPath {
            path,
            reason: ask_asset.to_string(),
        });
    }

    let mut new_paths = vec![path.clone()];
    PATHS.update(
        deps.storage,
        (offer_asset.to_string(), ask_asset.to_string()),
        |paths| -> StdResult<_> {
            if let Some(paths) = paths {
                new_paths.extend(paths.into_iter());
            }
            Ok(new_paths)
        },
    )?;

    // reverse path and store if `bidirectional` is true
    if bidirectional {
        let mut path = path;
        path.reverse();
        let mut new_paths = vec![path];
        PATHS.update(
            deps.storage,
            (ask_asset.to_string(), offer_asset.to_string()),
            |paths| -> StdResult<_> {
                if let Some(paths) = paths {
                    new_paths.extend(paths.into_iter());
                }
                Ok(new_paths)
            },
        )?;
    }

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::SimulateSwapOperations { offer, operations } => {
            Ok(to_json_binary(&simulate_swap_operations(
                deps,
                coin(offer.amount.into(), offer.info.inner()),
                operations,
            )?)?)
        }
        QueryMsg::PathsForPair {
            offer_asset,
            ask_asset,
        } => Ok(to_json_binary(&query_paths_for_pair(
            deps,
            offer_asset.check(deps.api, None)?,
            ask_asset.check(deps.api, None)?,
        )?)?),
        QueryMsg::BestPathForPair {
            offer_asset,
            offer_amount,
            ask_asset,
        } => Ok(to_json_binary(&query_best_path_for_pair(
            deps,
            offer_amount,
            offer_asset.check(deps.api, None)?,
            ask_asset.check(deps.api, None)?,
        )?)?),
        QueryMsg::SupportedOfferAssets { ask_asset } => Ok(to_json_binary(
            &query_supported_offer_assets(deps, ask_asset)?,
        )?),
        QueryMsg::SupportedAskAssets { offer_asset } => Ok(to_json_binary(
            &query_supported_ask_assets(deps, offer_asset)?,
        )?),
    }
}

pub fn simulate_swap_operations(
    deps: Deps,
    offer: Coin,
    routes: Vec<SwapAmountInRoute>,
) -> Result<Uint128, ContractError> {
    let querier = PoolmanagerQuerier::new(&deps.querier);
    let response = querier.estimate_swap_exact_amount_in(0, offer.to_string(), routes)?;

    Ok(Uint128::from_str(&response.token_out_amount)?)
}

pub fn query_paths_for_pair(
    deps: Deps,
    offer_asset: AssetInfo,
    ask_asset: AssetInfo,
) -> Result<Vec<Vec<SwapAmountInRoute>>, ContractError> {
    let paths = PATHS.load(
        deps.storage,
        (offer_asset.to_string(), ask_asset.to_string()),
    )?;
    if paths.is_empty() {
        Err(ContractError::NoPathFound {
            offer: offer_asset.to_string(),
            ask: ask_asset.to_string(),
        })
    } else {
        Ok(paths)
    }
}

pub fn query_best_path_for_pair(
    deps: Deps,
    offer_amount: Uint128,
    offer_asset: AssetInfo,
    ask_asset: AssetInfo,
) -> Result<Option<BestPathForPairResponse>, ContractError> {
    let paths = query_paths_for_pair(deps, offer_asset.clone(), ask_asset)?;
    if paths.is_empty() {
        return Err(ContractError::NoPathsToCheck {});
    }
    let offer = coin(offer_amount.into(), offer_asset.inner());
    let swap_paths: Result<Vec<BestPathForPairResponse>, ContractError> = paths
        .into_iter()
        .map(|swaps| {
            let out = simulate_swap_operations(deps, offer.clone(), swaps.clone().into())?;
            Ok(BestPathForPairResponse {
                operations: swaps,
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
    ask_asset: AssetInfoUnchecked,
) -> Result<Vec<AssetInfo>, ContractError> {
    let mut offer_assets: Vec<AssetInfo> = vec![];
    for x in PATHS.range(deps.storage, None, None, Order::Ascending) {
        let ((offer_asset, path_ask_asset), _) = x?;
        if AssetInfo::native(path_ask_asset) == ask_asset.check(deps.api, None)? {
            offer_assets.push(AssetInfo::native(offer_asset));
        }
    }
    Ok(offer_assets)
}

pub fn query_supported_ask_assets(
    deps: Deps,
    offer_asset: AssetInfoUnchecked,
) -> Result<Vec<AssetInfo>, ContractError> {
    let mut ask_assets: Vec<AssetInfo> = vec![];
    for x in PATHS.range(deps.storage, None, None, Order::Ascending) {
        let ((path_offer_asset, ask_asset), _) = x?;
        if AssetInfo::native(path_offer_asset) == offer_asset.check(deps.api, None)? {
            ask_assets.push(AssetInfo::native(ask_asset));
        }
    }
    Ok(ask_assets)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
