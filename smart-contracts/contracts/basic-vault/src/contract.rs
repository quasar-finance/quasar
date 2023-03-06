#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

use cw2::set_contract_version;
use cw20_base::allowances::{
    execute_burn_from, execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from, query_allowance,
};
use cw20_base::contract::{
    execute_burn, execute_send, execute_transfer, query_balance, query_token_info,
};
use cw20_base::state::{MinterData, TokenInfo, TOKEN_INFO};

use crate::callback::{on_bond, on_start_unbond, on_unbond};
use crate::error::ContractError;
use crate::execute::{_bond_all_tokens, bond, claim, reinvest, unbond};
use crate::msg::{ExecuteMsg, GetDebugResponse, InstantiateMsg, QueryMsg};
use crate::query::{
    query_deposit_ratio, query_investment, query_pending_bonds, query_tvl_info,
    query_unbonding_claims,
};
use crate::state::{
    InvestmentInfo, Supply, BONDING_SEQ, CLAIMS, CONTRACT_NAME, CONTRACT_VERSION, DEBUG_TOOL,
    INVESTMENT, TOTAL_SUPPLY,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // store token info using cw20-base format
    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply: Uint128::zero(),
        // set self as minter, so we can properly execute mint and burn
        mint: Some(MinterData {
            minter: env.contract.address.clone(),
            cap: None,
        }),
    };
    TOKEN_INFO.save(deps.storage, &data)?;

    let mut invest = InvestmentInfo {
        owner: info.sender,
        min_withdrawal: msg.min_withdrawal,
        primitives: msg.primitives.clone(),
    };
    invest.normalize_primitive_weights();
    INVESTMENT.save(deps.storage, &invest)?;

    // initialize bonding sequence num
    BONDING_SEQ.save(deps.storage, &Uint128::one())?;

    // set supply to 0
    let supply = Supply::default();
    TOTAL_SUPPLY.save(deps.storage, &supply)?;

    DEBUG_TOOL.save(deps.storage, &"Empty".to_string())?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Bond {} => bond(deps, env, info),
        ExecuteMsg::Unbond { amount } => unbond(deps, env, info, amount),
        ExecuteMsg::Claim {} => claim(deps, env, info),
        ExecuteMsg::Reinvest {} => reinvest(deps, env, info),
        ExecuteMsg::_BondAllTokens {} => _bond_all_tokens(deps, env, info),

        // callbacks entrypoint
        // you cant do this fuck me
        // ExecuteMsg::Callback(callback_msg) => handle_callback(deps, env, info, callback_msg),
        ExecuteMsg::BondResponse(bond_response) => on_bond(
            deps,
            env,
            info,
            bond_response.share_amount,
            bond_response.bond_id,
        ),
        ExecuteMsg::StartUnbondResponse(start_unbond_response) => on_start_unbond(
            deps,
            env,
            info,
            start_unbond_response.unbond_id,
            start_unbond_response.unlock_time,
        ),
        ExecuteMsg::UnbondResponse(unbond_response) => {
            on_unbond(deps, env, info, unbond_response.unbond_id)
        }

        // these all come from cw20-base to implement the cw20 standard
        ExecuteMsg::Transfer { recipient, amount } => {
            Ok(execute_transfer(deps, env, info, recipient, amount)?)
        }
        ExecuteMsg::Burn { amount } => Ok(execute_burn(deps, env, info, amount)?),
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => Ok(execute_send(deps, env, info, contract, amount, msg)?),
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
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Claims { address } => {
            to_binary(&CLAIMS.query_claims(deps, &deps.api.addr_validate(&address)?)?)
        }
        QueryMsg::Investment {} => to_binary(&query_investment(deps)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
        QueryMsg::DepositRatio { funds } => to_binary(&query_deposit_ratio(deps, funds)?),
        QueryMsg::PendingBonds { address } => to_binary(&query_pending_bonds(deps, address)?),
        QueryMsg::GetDebug {} => to_binary(&query_debug_string(deps)?),
        QueryMsg::GetTvlInfo {} => to_binary(&query_tvl_info(deps)?),
        QueryMsg::PendingUnbonds { address } => {
            to_binary(&query_unbonding_claims(deps, env, address)?)
        }
    }
}

pub fn query_debug_string(deps: Deps) -> StdResult<GetDebugResponse> {
    let debug_string = DEBUG_TOOL.load(deps.storage)?;

    Ok(GetDebugResponse {
        debug: debug_string,
    })
}

// replies not created yet
// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {

// }

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use quasar_types::callback::BondResponse;

    // #[test]
    // fn callback_bond_response() {
    //     let bond_response = BondResponse {
    //         share_amount: Uint128::one(),
    //         bond_id: "id".to_string(),
    //     };
    //     let cb = ExecuteMsg::BondResponse(bond_response);
    //     let mut deps = mock_dependencies();
    //     let env = mock_env();

    //     let info = MessageInfo {
    //         sender: env.clone().contract.address,
    //         funds: Vec::new(),
    //     };
    //     execute(deps.as_mut(), env, info, cb).unwrap();
    //     assert_ne!(DEBUG_TOOL.load(&deps.storage).unwrap().len(), 0);
    //     println!("{:?}", DEBUG_TOOL.load(&deps.storage).unwrap())
    // }
}
