#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
    Uint128,
};

use cw2::set_contract_version;
use cw20_base::allowances::{
    execute_burn_from, execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from, query_allowance,
};
use cw20_base::contract::{execute_burn, execute_send, execute_transfer, query_balance};
use cw20_base::state::{MinterData, TokenInfo, TOKEN_INFO};
use lp_strategy::msg::ConfigResponse;

use crate::callback::{on_bond, on_start_unbond, on_unbond};
use crate::error::ContractError;
use crate::execute::{bond, claim, unbond};
use crate::msg::{
    ExecuteMsg, GetDebugResponse, InstantiateMsg, MigrateMsg, QueryMsg, VaultTokenInfoResponse,
};
use crate::query::{
    query_deposit_ratio, query_investment, query_pending_bonds, query_pending_unbonds,
    query_tvl_info,
};
use crate::state::{
    AdditionalTokenInfo, InvestmentInfo, Supply, ADDITIONAL_TOKEN_INFO, BONDING_SEQ, CLAIMS,
    CONTRACT_NAME, CONTRACT_VERSION, DEBUG_TOOL, INVESTMENT, TOTAL_SUPPLY,
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
    let token_info = TokenInfo {
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
    let additional_info = AdditionalTokenInfo { thesis: msg.thesis };
    TOKEN_INFO.save(deps.storage, &token_info)?;
    ADDITIONAL_TOKEN_INFO.save(deps.storage, &additional_info)?;

    for prim in msg.primitives.iter() {
        let config: ConfigResponse = deps
            .querier
            .query_wasm_smart(&prim.address, &lp_strategy::msg::QueryMsg::Config {})?;
        match &prim.init {
            crate::msg::PrimitiveInitMsg::LP(init) => {
                assert_eq!(config.config.base_denom, init.base_denom);
                assert_eq!(config.config.expected_connection, init.expected_connection);
                assert_eq!(config.config.local_denom, init.local_denom);
                assert_eq!(config.config.lock_period, init.lock_period);
                assert_eq!(config.config.pool_denom, init.pool_denom);
                assert_eq!(config.config.pool_id, init.pool_id);
                assert_eq!(config.config.quote_denom, init.quote_denom);
                assert_eq!(
                    config.config.return_source_channel,
                    init.return_source_channel
                );
                assert_eq!(config.config.transfer_channel, init.transfer_channel);
            }
        }
    }

    let mut invest = InvestmentInfo {
        owner: info.sender,
        min_withdrawal: msg.min_withdrawal,
        primitives: msg.primitives,
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
        ExecuteMsg::Bond { recipient } => bond(deps, env, info, recipient),
        ExecuteMsg::Unbond { amount } => unbond(deps, env, info, amount),
        ExecuteMsg::Claim {} => claim(deps, env, info),

        // callbacks entrypoint
        // you cant do this - DONT TRY IT (unless you know what you're doing)
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
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // todo remove or augment
        QueryMsg::Claims { address } => {
            to_binary(&CLAIMS.query_claims(deps, &deps.api.addr_validate(&address)?)?)
        }
        QueryMsg::Investment {} => to_binary(&query_investment(deps)?),
        QueryMsg::TokenInfo {} => to_binary(&query_vault_token_info(deps)?),
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
        QueryMsg::DepositRatio { funds } => to_binary(&query_deposit_ratio(deps, funds)?),
        QueryMsg::PendingBonds { address } => to_binary(&query_pending_bonds(deps, address)?),
        QueryMsg::GetDebug {} => to_binary(&query_debug_string(deps)?),
        QueryMsg::GetTvlInfo {} => to_binary(&query_tvl_info(deps)?),
        QueryMsg::PendingUnbonds { address } => to_binary(&query_pending_unbonds(deps, address)?),
    }
}

pub fn query_vault_token_info(deps: Deps) -> StdResult<VaultTokenInfoResponse> {
    let token_info = TOKEN_INFO.load(deps.storage)?;
    let additional_info = ADDITIONAL_TOKEN_INFO.load(deps.storage)?;
    let res = VaultTokenInfoResponse {
        name: token_info.name,
        thesis: additional_info.thesis,
        symbol: token_info.symbol,
        decimals: token_info.decimals,
        total_supply: token_info.total_supply,
    };
    Ok(res)
}

pub fn query_debug_string(deps: Deps) -> StdResult<GetDebugResponse> {
    let debug_string = DEBUG_TOOL.load(deps.storage)?;

    Ok(GetDebugResponse {
        debug: debug_string,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    // for this migration, we only need to rebuild total_supply
    let balances: Result<Vec<(Addr, Uint128)>, StdError> = cw20_base::state::BALANCES
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let total = balances?
        .iter()
        .fold(Uint128::zero(), |sum, val| sum + val.1);
    TOTAL_SUPPLY.update(
        deps.storage,
        |mut supply| -> Result<Supply, ContractError> {
            supply.issued = total;
            Ok(supply)
        },
    )?;

    Ok(Response::new()
        .add_attribute("migrate", CONTRACT_NAME)
        .add_attribute("succes", "true"))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        ContractResult, Decimal, QuerierResult,
    };

    use crate::msg::PrimitiveConfig;

    use super::*;

    #[test]
    fn instantiate_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let info = MessageInfo {
            sender: Addr::unchecked("owner"),
            funds: vec![],
        };

        let msg = InstantiateMsg {
            name: "vault".to_string(),
            thesis: "to generate yield, I guess".to_string(),
            symbol: "VLT".to_string(),
            decimals: 6,
            min_withdrawal: Uint128::new(100),
            primitives: vec![
                PrimitiveConfig {
                    weight: Decimal::from_ratio(Uint128::one(), Uint128::new(3)),
                    address: "prim1".to_string(),
                    init: crate::msg::PrimitiveInitMsg::LP(lp_strategy::msg::InstantiateMsg {
                        lock_period: 300,
                        pool_id: 1,
                        pool_denom: "gamm/pool/1".to_string(),
                        local_denom: "ibc/SOME_DENOM".to_string(),
                        base_denom: "uosmo".to_string(),
                        quote_denom: "uqsr".to_string(),
                        transfer_channel: "channel-0".to_string(),
                        return_source_channel: "channel-0".to_string(),
                        expected_connection: "connection-0".to_string(),
                    }),
                },
                PrimitiveConfig {
                    weight: Decimal::from_ratio(Uint128::one(), Uint128::new(3)),
                    address: "prim2".to_string(),
                    init: crate::msg::PrimitiveInitMsg::LP(lp_strategy::msg::InstantiateMsg {
                        lock_period: 300,
                        pool_id: 1,
                        pool_denom: "gamm/pool/2".to_string(),
                        local_denom: "ibc/OTHER_DENOM".to_string(),
                        base_denom: "uqsr".to_string(),
                        quote_denom: "uosmo".to_string(),
                        transfer_channel: "channel-0".to_string(),
                        return_source_channel: "channel-0".to_string(),
                        expected_connection: "connection-0".to_string(),
                    }),
                },
                PrimitiveConfig {
                    weight: Decimal::from_ratio(Uint128::one(), Uint128::new(3)),
                    address: "prim3".to_string(),
                    init: crate::msg::PrimitiveInitMsg::LP(lp_strategy::msg::InstantiateMsg {
                        lock_period: 300,
                        pool_id: 1,
                        pool_denom: "gamm/pool/3".to_string(),
                        local_denom: "ibc/OTHER_OTHER_DENOM".to_string(),
                        base_denom: "uatom".to_string(),
                        quote_denom: "uqsr".to_string(),
                        transfer_channel: "channel-0".to_string(),
                        return_source_channel: "channel-0".to_string(),
                        expected_connection: "connection-0".to_string(),
                    }),
                },
            ],
        };

        // prepare 3 mock configs for prim1, prim2 and prim3
        deps.querier.update_wasm(|wq| match wq {
            cosmwasm_std::WasmQuery::Smart {
                contract_addr,
                msg: _,
            } => {
                if contract_addr == "prim1" {
                    QuerierResult::Ok(ContractResult::Ok(
                        to_binary(&lp_strategy::msg::ConfigResponse {
                            config: lp_strategy::state::Config {
                                lock_period: 300,
                                pool_id: 1,
                                pool_denom: "gamm/pool/1".to_string(),
                                local_denom: "ibc/SOME_DENOM".to_string(),
                                base_denom: "uosmo".to_string(),
                                quote_denom: "uqsr".to_string(),
                                transfer_channel: "channel-0".to_string(),
                                return_source_channel: "channel-0".to_string(),
                                expected_connection: "connection-0".to_string(),
                            },
                        })
                        .unwrap(),
                    ))
                } else if contract_addr == "prim2" {
                    QuerierResult::Ok(ContractResult::Ok(
                        to_binary(&lp_strategy::msg::ConfigResponse {
                            config: lp_strategy::state::Config {
                                lock_period: 300,
                                pool_id: 1,
                                pool_denom: "gamm/pool/2".to_string(),
                                local_denom: "ibc/OTHER_DENOM".to_string(),
                                base_denom: "uqsr".to_string(),
                                quote_denom: "uosmo".to_string(),
                                transfer_channel: "channel-0".to_string(),
                                return_source_channel: "channel-0".to_string(),
                                expected_connection: "connection-0".to_string(),
                            },
                        })
                        .unwrap(),
                    ))
                } else if contract_addr == "prim3" {
                    QuerierResult::Ok(ContractResult::Ok(
                        to_binary(&lp_strategy::msg::ConfigResponse {
                            config: lp_strategy::state::Config {
                                lock_period: 300,
                                pool_id: 1,
                                pool_denom: "gamm/pool/3".to_string(),
                                local_denom: "ibc/OTHER_OTHER_DENOM".to_string(),
                                base_denom: "uatom".to_string(),
                                quote_denom: "uqsr".to_string(),
                                transfer_channel: "channel-0".to_string(),
                                return_source_channel: "channel-0".to_string(),
                                expected_connection: "connection-0".to_string(),
                            },
                        })
                        .unwrap(),
                    ))
                } else {
                    QuerierResult::Err(cosmwasm_std::SystemError::NoSuchContract {
                        addr: contract_addr.to_string(),
                    })
                }
            }
            cosmwasm_std::WasmQuery::Raw {
                contract_addr: _,
                key: _,
            } => QuerierResult::Err(cosmwasm_std::SystemError::Unknown {}),
            cosmwasm_std::WasmQuery::ContractInfo { contract_addr: _ } => {
                QuerierResult::Err(cosmwasm_std::SystemError::Unknown {})
            }
            _ => panic!("Unimplemented query path"),
        });

        instantiate(deps.as_mut(), env, info, msg).unwrap();
    }
}
