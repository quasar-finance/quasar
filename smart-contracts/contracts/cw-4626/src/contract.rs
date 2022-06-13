#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, BankMsg, coins};
use cosmwasm_std::CosmosMsg::Bank;

use cw2::set_contract_version;
use cw20::{
    EmbeddedLogo, Logo, LogoInfo,
    MarketingInfoResponse,
};

// TODO decide if we want to use deduct allowance
use cw20_base::allowances::{
    deduct_allowance, execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from, execute_burn_from, query_allowance,
};
use cw20_base::contract::{
    execute_burn, execute_mint, execute_send, execute_transfer, execute_upload_logo, execute_update_marketing, query_balance, query_token_info,
    query_download_logo, query_marketing_info, query_minter,
};
use cw20_base::state::{MinterData, TokenInfo, TOKEN_INFO, MARKETING_INFO, LOGO};
use cw20_base::enumerable::{query_all_accounts, query_all_allowances};
use cw_utils::must_pay;


use share_distributor::single_token::SingleToken;

use crate::error::ContractError;
use crate::msg::{
    ConvertToSharesResponse, ExecuteMsg, InstantiateMsg, QueryMsg, VaultInfoResponse,
};
use crate::state::{
    Distributor, VaultInfo,
    OUTSTANDING_SHARES, VAULT_RESERVES, VAULT_DISTRIBUTOR, VAULT_INFO,
};

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
    VAULT_INFO.save(deps.storage, &VaultInfo { reserve_denom: msg.reserve_denom.to_string(), total_supply: msg.reserve_total_supply })?;

    // TODO see if we want to add some logic to differentiate between single and multiple token vaults
    // set the share distributor of the vault to the single token distributor
    let vault_distributor = Distributor {
        dist: SingleToken {},
    };
    VAULT_DISTRIBUTOR.save(deps.storage, &vault_distributor)?;

    // store token info
    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
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
        ExecuteMsg::Deposit { } => {
            execute_deposit(deps, env, info)
        }
        ExecuteMsg::Withdraw { amount, owner } => {
            execute_withdraw(deps, env, info, amount, owner)
        }
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
        ExecuteMsg::Mint { recipient, amount } => Ok(execute_mint(deps, env, info, recipient, amount)?),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_increase_allowance(deps, env, info, spender, amount, expires)?),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_decrease_allowance(deps, env, info, spender, amount, expires)?),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => Ok(execute_transfer_from(deps, env, info, owner, recipient, amount)?),
        ExecuteMsg::BurnFrom { owner, amount } => Ok(execute_burn_from(deps, env, info, owner, amount)?),
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => Ok(execute_send_from(deps, env, info, owner, contract, amount, msg)?),
        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => Ok(execute_update_marketing(deps, env, info, project, description, marketing)?),
        ExecuteMsg::UploadLogo(logo) => Ok(execute_upload_logo(deps, env, info, logo)?),
        ExecuteMsg::Receive { .. } => {todo!()}
    }
}

pub fn execute_deposit(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {

    // do all reasonable checks
    let denom = VAULT_INFO.load(deps.storage)?.reserve_denom;
    // only accept one token, change this for multi asset
    // TODO this is a hardcoded amount of a linear 1-1 price, we need to change this with customizable curves
    let amount = must_pay(&info, denom.as_str())?;


    // call into cw20-base to mint the token, call as self as no one else is allowed
    let sub_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };
    // exchange all the free balance for shares, mint shares of the underlying cw-20
    execute_mint(deps, env, sub_info, info.sender.to_string(), amount)?;

    let res = Response::new()
        .add_attribute("action", "buy")
        .add_attribute("from", info.sender)
        // TODO change shares once the curves are customizable
        .add_attribute("reserve", amount)
        .add_attribute("shares", amount);
    Ok(res)
}

// TODO decide on whether to add owner here for allowance support
pub fn execute_withdraw(mut deps: DepsMut, env: Env, info: MessageInfo, amount: Option<Uint128>, owner: String) ->Result<Response, ContractError> {
    let state = VAULT_INFO.load(deps.storage)?;

    // if amount is None, sell all shares of sender
    let shares = if amount.is_some() {
        amount.unwrap()
    } else {
        // TODO remove clone
        query_balance(deps.as_ref(), info.clone().sender.into_string())?.balance
    };

    // execute_burn will error if the sender does not have enough tokens to burn
    // TODO remove clone
    execute_burn(deps.branch(), env, info.clone(), shares)?;

    // we know that the sender has amount of shares, we can release fund based on that amount
    // TODO add customizable curves here
    let released = shares;

    let msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coins(released.u128(), state.reserve_denom),
    };

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "sell")
        .add_attribute("to", &info.sender)
        .add_attribute("amount", shares);
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
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
        QueryMsg::Asset { .. } => {
            todo!()
        }
        QueryMsg::TotalAssets { .. } => {
            todo!()
        }
        QueryMsg::ConvertToShares { .. } => todo!(), //to_binary(&query_convert_to_shares(deps, assets)?),
        QueryMsg::ConvertToAssets { .. } => {
            todo!()
        }
        QueryMsg::MaxDeposit { .. } => {
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
        QueryMsg::VaultInfo { .. } => to_binary(&query_vault_info(deps)?),
    }
}

// // TODO write a test for this
// pub fn query_convert_to_shares(
//     deps: Deps,
//     assets: Vec<Cw20Coin>,
// ) -> StdResult<ConvertToSharesResponse> {
//     // get the distributor from the state
//     let mut dist = VAULT_DISTRIBUTOR.load(deps.storage)?;
//     let mut state: Vec<Cw20Coin> = vec![];
//     for token in VAULT_BALANCES.range(deps.storage, None, None, Order::Ascending) {
//         // now we want to aggregate the balance per token
//         let (token, token_balance) = token?;
//         let mut total = Uint128::zero();
//         for (_, amount) in token_balance.range(deps.storage, None, None, Order::Ascending) {
//             total += amount;
//         }
//         state.push(Cw20Coin {
//             address: token.to_string(),
//             amount: total,
//         });
//     }
//     // decide on how many shares one would get
//     let amount = dist.deposit_funds(&assets, &state)?;
//     Ok(ConvertToSharesResponse { amount })
// }


pub fn query_vault_info(deps: Deps) -> StdResult<VaultInfoResponse> {
    let info = VAULT_INFO.load(deps.storage)?;
    let res = VaultInfoResponse {
        reserve_denom: info.reserve_denom,
        total_supply: info.total_supply
    };
    Ok(res)
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, Decimal, OverflowError, OverflowOperation, StdError, SubMsg};
    use cw_utils::PaymentError;

    const DENOM: &str = "satoshi";
    const CREATOR: &str = "creator";
    const INVESTOR: &str = "investor";
    const BUYER: &str = "buyer";

    fn default_instantiate(
        decimals: u8,
        reserve_decimals: u8,
        reserve_supply: Uint128
    ) -> InstantiateMsg {
        InstantiateMsg {
            name: "Bonded".to_string(),
            symbol: "EPOXY".to_string(),
            reserve_denom: DENOM.to_string(),
            reserve_total_supply: reserve_supply,
            decimals,
            initial_balances: vec![],
            mint: None,
            marketing: None
        }
    }

    fn get_balance<U: Into<String>>(deps: Deps, addr: U) -> Uint128 {
        query_balance(deps, addr.into()).unwrap().balance
    }

    fn setup_test(deps: DepsMut, decimals: u8, reserve_decimals: u8, reserve_supply: Uint128) {
        // this matches `linear_curve` test case from curves.rs
        let creator = String::from(CREATOR);
        let msg = default_instantiate(decimals, reserve_decimals, reserve_supply);
        let info = mock_info(&creator, &[]);

        // make sure we can instantiate with this
        let res = instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
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
}
