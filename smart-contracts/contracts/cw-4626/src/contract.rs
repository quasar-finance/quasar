#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128,
};
use cw2::set_contract_version;
use cw20::{EmbeddedLogo, Logo, LogoInfo, MarketingInfoResponse};

// TODO decide if we want to use deduct allowance
use cw20_base::allowances::{
    deduct_allowance, execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from, query_allowance,
};
use cw20_base::contract::{
    execute_burn, execute_mint, execute_send, execute_transfer, execute_update_marketing,
    execute_upload_logo, query_balance, query_download_logo, query_marketing_info, query_minter,
    query_token_info,
};
use cw20_base::enumerable::{query_all_accounts, query_all_allowances};
use cw20_base::state::{MinterData, TokenInfo, LOGO, MARKETING_INFO, TOKEN_INFO};
use cw_utils::{must_pay, nonpayable};

use quasar_types::curve::{CurveType, DecimalPlaces};
use strategy::contract::{execute_deposit as execute_strategy_deposit, execute_withdraw_request};

use crate::error::ContractError;
use crate::msg::{
    AssetResponse, ConvertToAssetsResponse, ConvertToSharesResponse, ExecuteMsg, InstantiateMsg,
    MaxDepositResponse, QueryMsg, TotalAssetResponse, VaultInfoResponse,
};
use crate::state::{VaultInfo, VAULT_CURVE, VAULT_INFO};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-4626";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const LOGO_SIZE_CAP: usize = 5 * 1024;

/// Checks if data starts with XML preamble
fn verify_xml_preamble(data: &[u8]) -> Result<(), cw20_base::ContractError> {
    // The easiest way to perform this check would be just match on regex, however regex
    // compilation is heavy and probably not worth it.

    let preamble = data
        .split_inclusive(|c| *c == b'>')
        .next()
        .ok_or(cw20_base::ContractError::InvalidXmlPreamble {})?;

    const PREFIX: &[u8] = b"<?xml ";
    const POSTFIX: &[u8] = b"?>";

    if !(preamble.starts_with(PREFIX) && preamble.ends_with(POSTFIX)) {
        Err(cw20_base::ContractError::InvalidXmlPreamble {})
    } else {
        Ok(())
    }

    // Additionally attributes format could be validated as they are well defined, as well as
    // comments presence inside of preable, but it is probably not worth it.
}

/// Validates XML logo
fn verify_xml_logo(logo: &[u8]) -> Result<(), cw20_base::ContractError> {
    verify_xml_preamble(logo)?;

    if logo.len() > LOGO_SIZE_CAP {
        Err(cw20_base::ContractError::LogoTooBig {})
    } else {
        Ok(())
    }
}

/// Validates png logo
fn verify_png_logo(logo: &[u8]) -> Result<(), cw20_base::ContractError> {
    // PNG header format:
    // 0x89 - magic byte, out of ASCII table to fail on 7-bit systems
    // "PNG" ascii representation
    // [0x0d, 0x0a] - dos style line ending
    // 0x1a - dos control character, stop displaying rest of the file
    // 0x0a - unix style line ending
    const HEADER: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
    if logo.len() > LOGO_SIZE_CAP {
        Err(cw20_base::ContractError::LogoTooBig {})
    } else if !logo.starts_with(&HEADER) {
        Err(cw20_base::ContractError::InvalidPngHeader {})
    } else {
        Ok(())
    }
}

/// Checks if passed logo is correct, and if not, returns an error
fn verify_logo(logo: &Logo) -> Result<(), cw20_base::ContractError> {
    match logo {
        Logo::Embedded(EmbeddedLogo::Svg(logo)) => verify_xml_logo(logo),
        Logo::Embedded(EmbeddedLogo::Png(logo)) => verify_png_logo(logo),
        Logo::Url(_) => Ok(()), // Any reasonable url validation would be regex based, probably not worth it
    }
}

// TODO add the curve of the vault here
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;

    // set the share distributor of the vault to the single token distributor
    VAULT_INFO.save(
        deps.storage,
        &VaultInfo {
            reserve_denom: msg.reserve_denom.to_string(),
            total_supply: msg.reserve_total_supply,
            decimals: DecimalPlaces {
                supply: msg.supply_decimals as u32,
                reserve: msg.reserve_decimals as u32,
            },
        },
    )?;

    // store token info
    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.supply_decimals,
        total_supply: Uint128::zero(),
        // set self as minter, so we can properly execute mint and burn
        mint: Some(MinterData {
            minter: env.contract.address,
            cap: None,
        }),
    };
    TOKEN_INFO.save(deps.storage, &data)?;

    if let Some(marketing) = msg.marketing {
        let logo = if let Some(logo) = marketing.logo {
            verify_logo(&logo)?;
            LOGO.save(deps.storage, &logo)?;

            match logo {
                Logo::Url(url) => Some(LogoInfo::Url(url)),
                Logo::Embedded(_) => Some(LogoInfo::Embedded),
            }
        } else {
            None
        };

        let data = MarketingInfoResponse {
            project: marketing.project,
            description: marketing.description,
            marketing: marketing
                .marketing
                .map(|addr| deps.api.addr_validate(&addr))
                .transpose()?,
            logo,
        };
        MARKETING_INFO.save(deps.storage, &data)?;
    }

    VAULT_CURVE.save(deps.storage, &msg.curve_type)?;

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
        // the vault execute messages
        ExecuteMsg::Deposit {} => execute_deposit(deps, env, info),
        ExecuteMsg::Withdraw { amount } => execute_withdraw(deps, env, info, amount),
        // the cw-20 execute messages
        ExecuteMsg::Transfer { recipient, amount } => {
            Ok(execute_transfer(deps, env, info, recipient, amount)?)
        }
        ExecuteMsg::Burn { amount } => Ok(execute_burn(deps, env, info, amount)?),
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => Ok(execute_send(deps, env, info, contract, amount, msg)?),
        ExecuteMsg::Mint { recipient, amount } => {
            Ok(execute_mint(deps, env, info, recipient, amount)?)
        }
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_increase_allowance(
            deps, env, info, spender, amount, expires,
        )?),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_decrease_allowance(
            deps, env, info, spender, amount, expires,
        )?),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => Ok(execute_transfer_from(
            deps, env, info, owner, recipient, amount,
        )?),
        ExecuteMsg::BurnFrom { owner, amount } => {
            Ok(execute_burn_from(deps, env, info, owner, amount)?)
        }
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => Ok(execute_send_from(
            deps, env, info, owner, contract, amount, msg,
        )?),
        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => Ok(execute_update_marketing(
            deps,
            env,
            info,
            project,
            description,
            marketing,
        )?),
        ExecuteMsg::UploadLogo(logo) => Ok(execute_upload_logo(deps, env, info, logo)?),
        ExecuteMsg::Receive { .. } => {
            todo!()
        }
    }
}

pub fn execute_deposit(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // do all reasonable checks
    let vault_info = VAULT_INFO.load(deps.storage)?;
    let curve = VAULT_CURVE.load(deps.storage)?;
    let curve_fn = curve.to_curve_fn();
    let curve = curve_fn(vault_info.decimals);
    // only accept one token, change this for multi asset

    // TODO this is a hardcoded amount of a linear 1-1 price, we need to change this with customizable curves
    let amount = must_pay(&info, vault_info.reserve_denom.as_str())?;

    // deposit the funds into the strategy
    // TODO remove clones and decide whether we directly pass info
    execute_strategy_deposit(deps.branch(), env.clone(), info.clone())?;

    let shares = curve.deposit(&amount);
    // call into cw20-base to mint the token, call as self as no one else is allowed
    let sub_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };
    // exchange all the free balance for shares, mint shares of the underlying cw-20
    execute_mint(deps, env, sub_info, info.sender.to_string(), shares)?;

    let res = Response::new()
        .add_attribute("action", "buy")
        .add_attribute("from", info.sender)
        .add_attribute("reserve", amount)
        .add_attribute("shares", shares);
    Ok(res)
}

// TODO decide on whether to add owner here for allowance support
pub fn execute_withdraw(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    // check that no funds are sent with the withdraw
    nonpayable(&info)?;

    // TODO make this a bit nicer with a helper
    let vault_info = VAULT_INFO.load(deps.storage)?;
    let curve_type = VAULT_CURVE.load(deps.storage)?;
    let curve_fn = curve_type.to_curve_fn();
    let curve = curve_fn(vault_info.decimals);

    // if amount is None, sell all shares of sender
    let shares = if amount.is_some() {
        amount.unwrap()
    } else {
        // TODO remove clone
        query_balance(deps.as_ref(), info.clone().sender.into_string())?.balance
    };

    let amount = curve.withdraw(&shares);

    // TODO here we need to withdraw from the strategy, if the strategy has the funds, we can simply withdraw
    //  if the withdraw is queued, do we already burn and mint again if we need to reverse or not burn at all?
    let withdraw_res = execute_withdraw_request(
        deps.branch(),
        env.clone(),
        info.clone(),
        info.sender.to_string(),
        vault_info.reserve_denom.clone(),
        amount,
    )?;

    // execute_burn will error if the sender does not have enough tokens to burn
    // TODO remove clone
    // TODO make sure that we can discard the result of execute_burn
    execute_burn(deps.branch(), env, info.clone(), shares)?;

    // we have created the Send bankmessage in the strategy contract, thus we pass res.messages here
    let res = Response::new()
        .add_submessages(withdraw_res.messages)
        .add_attributes(withdraw_res.attributes)
        .add_attribute("action", "sell")
        .add_attribute("to", &info.sender)
        .add_attribute("amount", shares);
    Ok(res)
}

// TODO implement burn_from to support the cw20 allowances
// execute_burn_from tries to burn the shares from owner if sender has an allowance over those tokens
// in the cw20 contract. The underlying reserve tokens should be returned to who? probably the sender
// and not the owner
pub fn execute_burn_from(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    todo!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Minter {} => to_binary(&query_minter(deps)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
        QueryMsg::AllAllowances {
            owner,
            start_after,
            limit,
        } => to_binary(&query_all_allowances(deps, owner, start_after, limit)?),
        QueryMsg::AllAccounts { start_after, limit } => {
            to_binary(&query_all_accounts(deps, start_after, limit)?)
        }
        QueryMsg::MarketingInfo {} => to_binary(&query_marketing_info(deps)?),
        QueryMsg::DownloadLogo {} => to_binary(&query_download_logo(deps)?),
        QueryMsg::Asset {} => to_binary(&query_asset(deps)?),
        QueryMsg::TotalAssets {} => to_binary(&query_total_assets(deps, env)?),
        QueryMsg::ConvertToShares { assets } => to_binary(&query_convert_to_shares(deps, assets)?),
        QueryMsg::ConvertToAssets { shares } => to_binary(&query_convert_to_assets(deps, shares)?),
        QueryMsg::MaxDeposit { receiver } => {
            // max deposit needs to check the underlying cw-20 token for the maximum supply, convert that
            // to the amout
            todo!()
        }
        QueryMsg::PreviewDeposit { .. } => {
            todo!()
        }
        QueryMsg::MaxMint { .. } => {
            todo!()
        }
        QueryMsg::PreviewMint { .. } => {
            todo!()
        }
        QueryMsg::MaxWithdraw { .. } => {
            todo!()
        }
        QueryMsg::PreviewWithdraw { .. } => {
            todo!()
        }
        QueryMsg::MaxRedeem { .. } => {
            todo!()
        }
        QueryMsg::PreviewRedeem { .. } => {
            todo!()
        }
        QueryMsg::VaultInfo {} => to_binary(&query_vault_info(deps)?),
    }
}

pub fn query_asset(deps: Deps) -> StdResult<AssetResponse> {
    let vault_info = VAULT_INFO.load(deps.storage)?;
    Ok(AssetResponse {
        denom: vault_info.reserve_denom,
    })
}

pub fn query_total_assets(deps: Deps, env: Env) -> StdResult<TotalAssetResponse> {
    let vault_info = VAULT_INFO.load(deps.storage)?;
    let balance = deps
        .querier
        .query_balance(env.contract.address, vault_info.reserve_denom)?;
    Ok(TotalAssetResponse {
        total_managed_assets: balance.amount,
    })
}

pub fn query_convert_to_shares(
    deps: Deps,
    assets: Vec<Coin>,
) -> StdResult<ConvertToSharesResponse> {
    let vault_info = VAULT_INFO.load(deps.storage)?;
    let curve_type = VAULT_CURVE.load(deps.storage)?;
    let curve_fn = curve_type.to_curve_fn();
    let curve = curve_fn(vault_info.decimals);

    // error on wrong amount of assets
    if assets.len() != 1 {
        return Err(StdError::generic_err("Query only supports one asset"));
    }

    // error on wrong denom
    if assets[0].denom != vault_info.reserve_denom {
        return Err(StdError::generic_err(format!(
            "Expected {} instead of {}",
            vault_info.reserve_denom, assets[0].denom
        )));
    }

    let shares = curve.deposit(&assets[0].amount);

    Ok(ConvertToSharesResponse { amount: shares })
}

pub fn query_convert_to_assets(deps: Deps, shares: Uint128) -> StdResult<ConvertToAssetsResponse> {
    let vault_info = VAULT_INFO.load(deps.storage)?;
    let curve_type = VAULT_CURVE.load(deps.storage)?;
    let curve_fn = curve_type.to_curve_fn();
    let curve = curve_fn(vault_info.decimals);

    let amount = curve.withdraw(&shares);
    Ok(ConvertToAssetsResponse {
        assets: coins(amount.u128(), vault_info.reserve_denom),
    })
}

fn query_max_deposit(deps: Deps) -> StdResult<MaxDepositResponse> {
    let vault_info = VAULT_INFO.load(deps.storage)?;
    let curve_type = VAULT_CURVE.load(deps.storage)?;
    let curve_fn = curve_type.to_curve_fn();
    let curve = curve_fn(vault_info.decimals);

    let minter = query_minter(deps)?;
    let mut free_tokens = Uint128::zero();
    if minter.is_some() {
        let min = minter.unwrap();
        // calculate the outstanding tokens
        let accounts = query_all_accounts(deps, None, None)?;
        let mut outstanding = Uint128::zero();
        for acc in accounts.accounts {
            let balance = query_balance(deps, acc)?;
            outstanding = outstanding.checked_add(balance.balance)?
        }
        // if we have a cap, calculate the difference between current outstanding supply and cap
        if min.cap.is_some() {
            free_tokens = min.cap.unwrap() - outstanding;
        } else {
            // if there is no cap, what do we do? We can return Uint128::MAX - outstanding tokens
            // This does create an issue with normalizing so that probably is not best.
            // thus we have to return None here to indicate the vault has no cap
            return Ok(MaxDepositResponse { max_assets: None });
        }
    } else {
        return Err(StdError::generic_err("no minter found in vault"));
    }
    // in order to get this many shares, we calculate F^-1(X), or withdraw, since withdraw and deposit are reversible
    let asset = curve.deposit(&free_tokens);
    Ok(MaxDepositResponse {
        max_assets: Some(coins(asset.u128(), vault_info.reserve_denom)),
    })
}

pub fn query_vault_info(deps: Deps) -> StdResult<VaultInfoResponse> {
    let info = VAULT_INFO.load(deps.storage)?;
    let res = VaultInfoResponse {
        reserve_denom: info.reserve_denom,
        total_supply: info.total_supply,
    };
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{
        coin, BankMsg, Decimal, OverflowError, OverflowOperation, StdError, SubMsg,
    };
    use cw_utils::PaymentError;
    use std::borrow::BorrowMut;

    const DENOM: &str = "satoshi";
    const CREATOR: &str = "creator";
    const INVESTOR: &str = "investor";
    const BUYER: &str = "buyer";

    fn default_instantiate(
        supply_decimals: u8,
        reserve_decimals: u8,
        reserve_supply: Uint128,
    ) -> InstantiateMsg {
        InstantiateMsg {
            name: "Bonded".to_string(),
            symbol: "EPOXY".to_string(),
            reserve_denom: DENOM.to_string(),
            reserve_total_supply: reserve_supply,
            reserve_decimals,
            supply_decimals,
            initial_balances: vec![],
            // initiate with a constant 1 to 1 curve
            curve_type: CurveType::Constant {
                value: Uint128::new(10),
                scale: 1,
            },
            mint: None,
            marketing: None,
        }
    }

    fn get_balance<U: Into<String>>(deps: Deps, addr: U) -> Uint128 {
        query_balance(deps, addr.into()).unwrap().balance
    }

    fn setup_test(
        deps: DepsMut,
        supply_decimals: u8,
        reserve_decimals: u8,
        reserve_supply: Uint128,
    ) {
        // this matches `constant_curve` test case from curves.rs
        let creator = String::from(CREATOR);
        let msg = default_instantiate(supply_decimals, reserve_decimals, reserve_supply);
        let info = mock_info(&creator, &[]);

        // make sure we can instantiate with this
        let res = instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    fn setup_test_with_deposit(
        mut deps: DepsMut,
        env: Env,
        supply_decimals: u8,
        reserve_decimals: u8,
        reserve_supply: Uint128,
        deposited_funds: u128,
        received_shares: u128,
    ) {
        // this matches `constant_curve` test case from curves.rs
        let creator = String::from(CREATOR);
        let msg = default_instantiate(supply_decimals, reserve_decimals, reserve_supply);
        let info = mock_info(&creator, &[]);

        // make sure we can instantiate with this
        let res = instantiate(deps.branch(), env.clone(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let alice: &str = "alice";
        let bob: &str = "bobbyb";
        let carl: &str = "carl";

        // setup_test() defaults to a 1-1 curve
        // bob buys some shares, spends 45 9 decimal coins (45_000_000_000) and receives 45 6 decimal (45_000_000) shares
        let info = mock_info(bob, &coins(deposited_funds, DENOM));
        let buy = ExecuteMsg::Deposit {};
        execute(deps.branch(), env, info, buy).unwrap();

        // check that bob has the shares
        assert_eq!(
            get_balance(deps.as_ref(), bob),
            Uint128::new(received_shares)
        );
    }

    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();

        // this matches `linear_curve` test case from curves.rs
        let creator = String::from("creator");
        let msg = default_instantiate(2, 8, Uint128::MAX);
        let info = mock_info(&creator, &[]);

        // make sure we can instantiate with this
        let res = instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());

        // token info is proper
        let token = query_token_info(deps.as_ref()).unwrap();
        assert_eq!(&token.name, &msg.name);
        assert_eq!(&token.symbol, &msg.symbol);
        assert_eq!(token.decimals, 2);
        assert_eq!(token.total_supply, Uint128::zero());
        //
        // // curve state is sensible
        // let state = query_curve_info(deps.as_ref(), curve_type.to_curve_fn()).unwrap();
        let state = VAULT_INFO.load(deps.storage.borrow_mut()).unwrap();
        assert_eq!(state.total_supply, Uint128::MAX);
        assert_eq!(state.reserve_denom.as_str(), DENOM);
        // spot price 0 as supply is 0
        // assert_eq!(state.spot_price, Decimal::zero());

        // no balance
        assert_eq!(get_balance(deps.as_ref(), &creator), Uint128::zero());
    }

    #[test]
    fn deposit_works() {
        let mut deps = mock_dependencies();
        setup_test(deps.as_mut(), 9, 6, Uint128::MAX);

        let alice: &str = "alice";
        let bob: &str = "bobbyb";
        let carl: &str = "carl";

        // setup_test() defaults to a 1-1 curve
        // bob buys some shares, spends 45 9 decimal coins (45_000_000_000) and receives 45 6 decimal (45_000_000) shares
        let info = mock_info(bob, &coins(45_000_000_000, DENOM));
        let buy = ExecuteMsg::Deposit {};
        execute(deps.as_mut(), mock_env(), info, buy).unwrap();

        // check that bob has the shares
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(45_000_000));
    }

    #[test]
    fn deposit_needs_right_denom() {
        let mut deps = mock_dependencies();
        setup_test(deps.as_mut(), 9, 6, Uint128::MAX);

        let alice: &str = "alice";
        let bob: &str = "bobbyb";
        let carl: &str = "carl";

        // setup_test() defaults to a 1-1 curve
        // bob buys some shares, spends 45 9 decimal coins (45_000_000_000) and receives 45 6 decimal (45_000_000) shares
        let info = mock_info(bob, &coins(45_000_000_000, "wrong-denom"));
        let buy = ExecuteMsg::Deposit {};
        execute(deps.as_mut(), mock_env(), info, buy).unwrap_err();

        // check that bob has no shares
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(0));
    }

    fn deposit_needs_funds() {
        let mut deps = mock_dependencies();
        setup_test(deps.as_mut(), 9, 6, Uint128::MAX);

        let alice: &str = "alice";
        let bob: &str = "bobbyb";
        let carl: &str = "carl";

        // setup_test() defaults to a 1-1 curve
        // bob tries to buy some shares without any funds
        let info = mock_info(bob, &[]);
        let buy = ExecuteMsg::Deposit {};
        execute(deps.as_mut(), mock_env(), info, buy).unwrap_err();

        // check that bob has no shares
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(0));
    }

    #[test]
    fn withdraw_works() {
        let mut deps = mock_dependencies();
        // setup_test() defaults to a 1-1 curve
        // bob buys some shares, spends 45 9 decimal coins (45_000_000_000) and receives 45 6 decimal (45_000_000) shares
        setup_test_with_deposit(
            deps.as_mut(),
            mock_env(),
            9,
            6,
            Uint128::MAX,
            45_000_000_000,
            45_000_000,
        );

        deps.querier
            .update_balance(mock_env().contract.address, coins(45_000_000_000, DENOM));

        let alice: &str = "alice";
        let bob: &str = "bobbyb";
        let carl: &str = "carl";

        // check that bob has the shares
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(45_000_000));

        let info = mock_info(bob, &[]);
        // withdraw all shares
        let sell = ExecuteMsg::Withdraw { amount: None };
        let res = execute(deps.as_mut(), mock_env(), info, sell).unwrap();

        // check that bob's balance is completely gone
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::zero());

        assert_eq!(res.messages.len(), 1);
        // check that bob received a bankmessage containing funds
        assert_eq!(
            res.messages[0],
            SubMsg::new(BankMsg::Send {
                to_address: bob.to_string(),
                amount: coins(45_000_000_000, DENOM)
            })
        )
    }

    #[test]
    fn cannot_withdraw_too_many_funds() {
        let mut deps = mock_dependencies();
        setup_test_with_deposit(
            deps.as_mut(),
            mock_env(),
            9,
            6,
            Uint128::MAX,
            45_000_000_000,
            45_000_000,
        );

        let alice: &str = "alice";
        let bob: &str = "bobbyb";
        let carl: &str = "carl";

        // check that bob has the shares
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(45_000_000));

        let info = mock_info(bob, &[]);
        // withdraw too many shares
        let sell = ExecuteMsg::Withdraw {
            amount: Some(Uint128::new(999_000_000)),
        };
        execute(deps.as_mut(), mock_env(), info, sell).unwrap_err();

        // check that bob's balance has not changed
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(45_000_000))
    }

    #[test]
    fn cannot_send_funds_with_withdraw() {
        let mut deps = mock_dependencies();
        setup_test_with_deposit(
            deps.as_mut(),
            mock_env(),
            9,
            6,
            Uint128::MAX,
            45_000_000_000,
            45_000_000,
        );

        let alice: &str = "alice";
        let bob: &str = "bobbyb";
        let carl: &str = "carl";

        // check that bob has the shares
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(45_000_000));

        // check that bob has the shares
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(45_000_000));

        let info = mock_info(bob, &coins(45_000_000_000, DENOM));
        // withdraw too many shares
        let sell = ExecuteMsg::Withdraw {
            amount: Some(Uint128::new(999_000_000)),
        };
        execute(deps.as_mut(), mock_env(), info, sell).unwrap_err();

        // check that bob's balance has not changed
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(45_000_000))
    }

    #[test]
    fn cw20_imports_work() {
        let mut deps = mock_dependencies();
        setup_test(deps.as_mut(), 9, 6, Uint128::MAX);

        let alice: &str = "alice";
        let bob: &str = "bobby";
        let carl: &str = "carl";

        // setup_test() defaults to a 1-1 curve, the supply has 9 decimals and the shares 6 decimals
        // spend 45_000_000 uatom for 45_000 shares
        let info = mock_info(bob, &coins(45_000_000, DENOM));
        let buy = ExecuteMsg::Deposit {};
        execute(deps.as_mut(), mock_env(), info, buy).unwrap();

        // check balances
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(45_000));
        assert_eq!(get_balance(deps.as_ref(), carl), Uint128::zero());

        // send coins to carl
        let bob_info = mock_info(bob, &[]);
        let transfer = ExecuteMsg::Transfer {
            recipient: carl.into(),
            amount: Uint128::new(20_000),
        };
        execute(deps.as_mut(), mock_env(), bob_info.clone(), transfer).unwrap();
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(25_000));
        assert_eq!(get_balance(deps.as_ref(), carl), Uint128::new(20_000));

        // allow alice
        let allow = ExecuteMsg::IncreaseAllowance {
            spender: alice.into(),
            amount: Uint128::new(10_000),
            expires: None,
        };
        execute(deps.as_mut(), mock_env(), bob_info, allow).unwrap();
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(25_000));
        assert_eq!(get_balance(deps.as_ref(), alice), Uint128::zero());
        assert_eq!(
            query_allowance(deps.as_ref(), bob.into(), alice.into())
                .unwrap()
                .allowance,
            Uint128::new(10_000)
        );

        // alice takes some for herself
        let self_pay = ExecuteMsg::TransferFrom {
            owner: bob.into(),
            recipient: alice.into(),
            amount: Uint128::new(5_000),
        };
        let alice_info = mock_info(alice, &[]);
        execute(deps.as_mut(), mock_env(), alice_info, self_pay).unwrap();
        assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(20_000));
        assert_eq!(get_balance(deps.as_ref(), alice), Uint128::new(5_000));
        assert_eq!(get_balance(deps.as_ref(), carl), Uint128::new(20_000));
        assert_eq!(
            query_allowance(deps.as_ref(), bob.into(), alice.into())
                .unwrap()
                .allowance,
            Uint128::new(5_000)
        );

        // test burn from works properly (burn tested in burning_sends_reserve)
        // cannot burn more than they have

        // TODO burn_from needs to be implemented and these tests added back here

        // let info = mock_info(alice, &[]);
        // let burn_from = ExecuteMsg::BurnFrom {
        //     owner: bob.into(),
        //     amount: Uint128::new(3_300_000),
        // };
        // let err = execute(deps.as_mut(), mock_env(), info, burn_from).unwrap_err();
        // assert_eq!(
        //     err,
        //     ContractError::Base(cw20_base::ContractError::Std(StdError::overflow(
        //         OverflowError::new(OverflowOperation::Sub, 5000 as u128, 3300000 as u128)
        //     )))
        // );

        // burn 1_500 EPOXY to get back 1_500 DENOM (constant curve)
        // let info = mock_info(alice, &[]);
        // let burn_from = ExecuteMsg::BurnFrom {
        //     owner: bob.into(),
        //     amount: Uint128::new(1_500),
        // };
        // let res = execute(deps.as_mut(), mock_env(), info, burn_from).unwrap();

        // bob balance is lower, not alice
        // assert_eq!(get_balance(deps.as_ref(), alice), Uint128::new(5000));
        // assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(18500));

        // ensure alice got our money back
        // TODO, currently this is not supported, we will need to support it, in cw-20 bonding curve, this is routed to sell from
        // assert_eq!(1, res.messages.len());
        // assert_eq!(
        //     &res.messages[0],
        //     &SubMsg::new(BankMsg::Send {
        //         to_address: alice.into(),
        //         amount: coins(1_500, DENOM),
        //     })
        // );
    }

    #[test]
    fn query_asset_works() {
        let mut deps = mock_dependencies();
        setup_test(deps.as_mut(), 9, 6, Uint128::MAX);

        let asset = query_asset(deps.as_ref()).unwrap();
        assert_eq!(asset.denom, "satoshi".to_string())
    }

    #[test]
    fn query_total_assets_works() {
        let mut deps = mock_dependencies();
        setup_test(deps.as_mut(), 9, 6, Uint128::MAX);

        // check that total_assets is zero upon instantiation
        let total_assets = query_total_assets(deps.as_ref(), mock_env()).unwrap();
        assert_eq!(total_assets.total_managed_assets, Uint128::new(0));

        // deposit some funds in different accounts and check assets after each deposit
        let alice: &str = "alice";
        let bob: &str = "bobbyb";
        let carl: &str = "carl";

        deps.querier
            .update_balance(mock_env().contract.address, coins(45_000_000_000, DENOM));

        let total_assets = query_total_assets(deps.as_ref(), mock_env()).unwrap();
        assert_eq!(
            total_assets.total_managed_assets,
            Uint128::new(45_000_000_000)
        );
    }

    #[test]
    fn query_convert_to_shares_works() {
        let mut deps = mock_dependencies();
        setup_test(deps.as_mut(), 9, 6, Uint128::MAX);

        let response =
            query_convert_to_shares(deps.as_ref(), coins(45_000_000_000, DENOM)).unwrap();
        assert_eq!(response.amount, Uint128::new(45_000_000))
    }

    #[test]
    fn query_convert_to_assets_works() {
        let mut deps = mock_dependencies();
        setup_test(deps.as_mut(), 9, 6, Uint128::MAX);

        let response = query_convert_to_assets(deps.as_ref(), Uint128::new(45_000_000)).unwrap();
        assert_eq!(response.assets, coins(45_000_000_000, DENOM))
    }

    #[test]
    fn query_max_deposit_works() {
        let mut deps = mock_dependencies();
        setup_test(deps.as_mut(), 9, 6, Uint128::MAX);

        let response = query_max_deposit(deps.as_ref()).unwrap();
        // the vault has no cap to minting by default, thus we expect None
        assert_eq!(response.max_assets, None)
    }
}
