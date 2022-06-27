use std::cmp::min;
use std::collections::VecDeque;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::CosmosMsg::Bank;
use cosmwasm_std::{coins, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, Coin, Addr};
use cw2::set_contract_version;
use cw20::{EmbeddedLogo, Logo, LogoInfo, MarketingInfoResponse};

// TODO decide if we want to use deduct allowance
use cw20_base::allowances::{
    deduct_allowance, execute_decrease_allowance, execute_increase_allowance,
    execute_send_from, execute_transfer_from, query_allowance,
};
use cw20_base::contract::{
    execute_burn, execute_mint, execute_send, execute_transfer, execute_update_marketing,
    execute_upload_logo, query_balance, query_download_logo, query_marketing_info, query_minter,
    query_token_info,
};
use cw20_base::enumerable::{query_all_accounts, query_all_allowances};
use cw20_base::state::{MinterData, TokenInfo, LOGO, MARKETING_INFO, TOKEN_INFO};
use cw_utils::{must_pay, nonpayable};
use quasar_traits::traits::Curve;
use quasar_types::curve::{CurveType, DecimalPlaces};

use share_distributor::single_token::SingleToken;
use crate::ContractError::{PaymentError, Std};
use crate::error::ContractError;
use crate::msg::{AssetResponse, ConvertToAssetsResponse, ConvertToSharesResponse, ExecuteMsg, InstantiateMsg, MaxDepositResponse, QueryMsg, TotalAssetResponse, VaultInfoResponse};
use crate::state::{VaultInfo, VAULT_CURVE, VAULT_INFO, WITHDRAW_QUEUE, WithdrawRequest};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-4626";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const LOGO_SIZE_CAP: usize = 5 * 1024;


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

    // TODO fill in the instantiation
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
        ExecuteMsg::Deposit { .. } => {todo!()}
        ExecuteMsg::WithdrawRequest { .. } => {todo!()}
    }
}

pub fn execute_deposit(deps: DepsMut, env: Env, info: MessageInfo) {
    let mut queue: VecDeque<WithdrawRequest> = WITHDRAW_QUEUE.load(deps)?;
    // we want a fifo queue
    queue.push_back(WithdrawRequest{
    })

}

pub fn execute_withdraw_request(deps: DepsMut, env: Env, info: MessageInfo) {

}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        .. => todo!()
    }
}

mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, Decimal, OverflowError, OverflowOperation, StdError, SubMsg};
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
        InstantiateMsg {}
    }

    fn get_balance<U: Into<String>>(deps: Deps, addr: U) -> Uint128 {
        query_balance(deps, addr.into()).unwrap().balance
    }

    fn setup_test(
        deps: DepsMut,
        supply_decimals: u8,
        reserve_decimals: u8,
        reserve_supply: Uint128,
    ) {}
}