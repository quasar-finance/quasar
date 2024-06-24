#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, Event, MessageInfo, Order, Reply,
    Response, StdError, StdResult, SubMsg, Uint128,
};
use cw2::set_contract_version;
use cw_asset::{AssetInfo, AssetInfoUnchecked};
use mars_owner::OwnerInit::SetInitialOwner;
use osmosis_std::cosmwasm_to_proto_coins;
use osmosis_std::types::osmosis::poolmanager::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute};
use quasar_types::error::assert_fund_length;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
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

    // check if we have any exisiting items under the offer_asset, ask_asset pair
    // we are looking for the highest ID so we can increment it, this should be under Order::Descending in the first item
    let ps: Result<Vec<(u64, Vec<SwapAmountInRoute>)>, StdError> = PATHS
        .prefix((offer_asset.to_string(), ask_asset.to_string()))
        .range(deps.storage, None, None, Order::Descending)
        .collect();
    let paths = ps?;
    let last_id = paths.first().map(|(val, _)| val).unwrap_or(&0);

    let new_id = last_id + 1;
    PATHS.save(
        deps.storage,
        (offer_asset.to_string(), ask_asset.to_string(), new_id),
        &path,
    )?;

    // reverse path and store if `bidirectional` is true
    if bidirectional {
        let ps: Result<Vec<(u64, Vec<SwapAmountInRoute>)>, StdError> = PATHS
            .prefix((ask_asset.to_string(), offer_asset.to_string()))
            .range(deps.storage, None, None, Order::Descending)
            .collect();
        let paths = ps?;
        let last_id = paths.first().map(|(val, _)| val).unwrap_or(&0);

        let new_id = last_id + 1;
        let mut path = path;
        path.reverse();
        PATHS.save(
            deps.storage,
            (ask_asset.to_string(), offer_asset.to_string(), new_id),
            &path,
        )?;
    }

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        // QueryMsg::SimulateSwapOperations { offer, operations } => {
        //     to_json_binary(&simulate_swap_operations(deps, offer, operations)?)
        // }
        QueryMsg::PathsForPair {
            offer_asset,
            ask_asset,
        } => Ok(to_json_binary(&query_paths_for_pair(
            deps,
            offer_asset.check(deps.api, None)?,
            ask_asset.check(deps.api, None)?,
        )?)?),
        // QueryMsg::BestPathForPair {
        //     offer_asset,
        //     offer_amount,
        //     ask_asset,
        //     exclude_paths,
        // } => to_json_binary(&query_best_path_for_pair(
        //     deps,
        //     offer_amount,
        //     offer_asset.check(deps.api)?,
        //     ask_asset.check(deps.api)?,
        //     exclude_paths,
        // )?),
        QueryMsg::SupportedOfferAssets { ask_asset } => Ok(to_json_binary(
            &query_supported_offer_assets(deps, ask_asset)?,
        )?),
        QueryMsg::SupportedAskAssets { offer_asset } => Ok(to_json_binary(
            &query_supported_ask_assets(deps, offer_asset)?,
        )?),
    }
}

// pub fn simulate_swap_operations(
//     deps: Deps,
//     offer: Coin,
//     routes: Vec<SwapAmountInRoute>,
// ) -> Result<Uint128, ContractError> {
//     let querier = PoolmanagerQuerier::new(&deps.querier);
//     let res = querier

//     let mut offer_asset = Asset::from(offer);
//     for route in routes.into_iter() {
//         let receive_info = AssetInfo::native(route.token_out_denom);
//         let receive_amount = operation
//             .pool
//             .simulate_swap(deps, offer_asset, receive_info)?;
//         offer_asset = Asset::new(receive_info, receive_amount);
//     }

//     Ok(offer_amount)
// }

pub fn query_paths_for_pair(
    deps: Deps,
    offer_asset: AssetInfo,
    ask_asset: AssetInfo,
) -> Result<Vec<(u64, Vec<SwapAmountInRoute>)>, ContractError> {
    let ps: StdResult<Vec<(u64, Vec<SwapAmountInRoute>)>> = PATHS
        .prefix((offer_asset.to_string(), ask_asset.to_string()))
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let paths = ps?;
    if paths.is_empty() {
        Err(ContractError::NoPathFound {
            offer: offer_asset.to_string(),
            ask: ask_asset.to_string(),
        })
    } else {
        Ok(paths)
    }
}

// pub fn query_best_path_for_pair(
//     deps: Deps,
//     offer_amount: Uint128,
//     offer_asset: AssetInfo,
//     ask_asset: AssetInfo,
//     exclude_paths: Option<Vec<u64>>,
// ) -> Result<Option<BestPathForPairResponse>, ContractError> {
//     let paths = query_paths_for_pair(deps, offer_asset, ask_asset)?;
//     let excluded = exclude_paths.unwrap_or(vec![]);
//     let paths: Vec<(u64, Vec<SwapAmountInRoute>)> = paths
//         .into_iter()
//         .filter(|(id, _)| !excluded.contains(id))
//         .collect();

//     if paths.is_empty() {
//         return Err(ContractError::NoPathsToCheck {});
//     }

//     let swap_paths: Result<Vec<BestPathForPairResponse>, ContractError> = paths
//         .into_iter()
//         .map(|(_, swaps)| {
//             let out = simulate_swap_operations(deps, offer_amount, swaps.clone().into())?;
//             Ok(BestPathForPairResponse {
//                 operations: swaps,
//                 return_amount: out,
//             })
//         })
//         .collect();

//     let best_path = swap_paths?
//         .into_iter()
//         .max_by(|a, b| a.return_amount.cmp(&b.return_amount));

//     Ok(best_path)
// }

pub fn query_supported_offer_assets(
    deps: Deps,
    ask_asset: AssetInfoUnchecked,
) -> Result<Vec<AssetInfo>, ContractError> {
    let mut offer_assets: Vec<AssetInfo> = vec![];
    for x in PATHS.range(deps.storage, None, None, Order::Ascending) {
        let ((offer_asset, path_ask_asset, _), _) = x?;
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
        let ((path_offer_asset, ask_asset, _), _) = x?;
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
