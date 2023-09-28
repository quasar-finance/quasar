#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order, Reply, Response,
    StdError, StdResult, SubMsg, SubMsgResult, Uint128, WasmMsg,
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
use cw_utils::parse_instantiate_response_data;
use lp_strategy::msg::ConfigResponse;
use vault_rewards::msg::InstantiateMsg as VaultRewardsInstantiateMsg;

use crate::callback::{on_bond, on_start_unbond, on_unbond};
use crate::error::ContractError;
use crate::execute::{bond, claim, execute_force_claim, execute_force_unbond, unbond, update_cap};
use crate::helpers::update_user_reward_index;
use crate::msg::{
    ExecuteMsg, GetCapResponse, GetDebugResponse, InstantiateMsg, MigrateMsg, PrimitiveConfig,
    QueryMsg, VaultTokenInfoResponse,
};
use crate::query::{
    query_deposit_ratio, query_investment, query_pending_bonds, query_pending_bonds_by_id,
    query_pending_unbonds, query_pending_unbonds_by_id, query_tvl_info,
};
use crate::state::{
    AdditionalTokenInfo, Cap, InvestmentInfo, ADDITIONAL_TOKEN_INFO, BONDING_SEQ, CAP, CLAIMS,
    CONTRACT_NAME, CONTRACT_VERSION, DEBUG_TOOL, INVESTMENT, VAULT_REWARDS,
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
            minter: env.contract.address.clone(),
            cap: None,
        }),
    };
    let additional_info = AdditionalTokenInfo {
        creation_time: env.block.time,
        thesis: msg.thesis,
    };
    TOKEN_INFO.save(deps.storage, &token_info)?;
    ADDITIONAL_TOKEN_INFO.save(deps.storage, &additional_info)?;

    CAP.save(deps.storage, &Cap::new(info.sender.clone(), msg.total_cap))?;

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
        owner: info.sender.clone(),
        min_withdrawal: msg.min_withdrawal,
        primitives: msg.primitives,
        deposit_denom: msg.deposit_denom,
    };
    invest.normalize_primitive_weights();
    INVESTMENT.save(deps.storage, &invest)?;

    // initialize bonding sequence num
    BONDING_SEQ.save(deps.storage, &Uint128::one())?;

    DEBUG_TOOL.save(deps.storage, &"Empty".to_string())?;

    let init_vault_rewards = SubMsg::reply_always(
        WasmMsg::Instantiate {
            admin: Some(info.sender.to_string()),
            code_id: msg.vault_rewards_code_id,
            msg: to_binary(&VaultRewardsInstantiateMsg {
                vault_token: env.contract.address.to_string(),
                reward_token: msg.reward_token,
                distribution_schedules: msg.reward_distribution_schedules,
            })?,
            funds: vec![],
            label: "vault-rewards".to_string(),
        },
        REPLY_INIT_VAULT_REWARDS,
    );

    Ok(Response::new().add_submessage(init_vault_rewards))
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

        // calls the TryIcq ExecuteMsg in the lp-strategy contract
        ExecuteMsg::ClearCache {} => {
            let try_icq_msg = lp_strategy::msg::ExecuteMsg::TryIcq {};

            let mut msgs: Vec<WasmMsg> = vec![];

            let primitives = INVESTMENT.load(deps.storage)?.primitives;
            primitives.iter().try_for_each(
                |pc: &PrimitiveConfig| -> Result<(), ContractError> {
                    let clear_cache_msg = WasmMsg::Execute {
                        contract_addr: pc.address.to_string(),
                        funds: vec![],
                        msg: to_binary(&try_icq_msg)?,
                    };
                    msgs.push(clear_cache_msg);
                    Ok(())
                },
            )?;
            Ok(Response::new().add_messages(msgs))
        }

        // Callbacks entrypoint
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

        // Admin messages
        ExecuteMsg::SetCap {
            new_total,
            new_cap_admin,
        } => update_cap(deps, env, info, new_total, new_cap_admin),

        // these all come from cw20-base to implement the cw20 standard
        ExecuteMsg::Transfer { recipient, amount } => {
            let recipient = deps.api.addr_validate(&recipient)?;
            let update_user_reward_indexes = vec![
                update_user_reward_index(deps.storage, &info.sender)?,
                update_user_reward_index(deps.storage, &recipient)?,
            ];
            Ok(
                execute_transfer(deps, env, info, recipient.to_string(), amount)?
                    .add_messages(update_user_reward_indexes),
            )
        }
        ExecuteMsg::Burn { amount } => {
            let update_user_reward_index = update_user_reward_index(deps.storage, &info.sender)?;
            Ok(execute_burn(deps, env, info, amount)?.add_message(update_user_reward_index))
        }
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => {
            let contract = deps.api.addr_validate(&contract)?;
            let update_user_reward_indexes = vec![
                update_user_reward_index(deps.storage, &info.sender)?,
                update_user_reward_index(deps.storage, &contract)?,
            ];
            Ok(
                execute_send(deps, env, info, contract.to_string(), amount, msg)?
                    .add_messages(update_user_reward_indexes),
            )
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
        } => {
            let recipient = deps.api.addr_validate(&recipient)?;
            let update_user_reward_indexes = vec![
                update_user_reward_index(deps.storage, &deps.api.addr_validate(&owner)?)?,
                update_user_reward_index(deps.storage, &recipient)?,
            ];
            Ok(
                execute_transfer_from(deps, env, info, owner, recipient.to_string(), amount)?
                    .add_messages(update_user_reward_indexes),
            )
        }
        ExecuteMsg::BurnFrom { owner, amount } => {
            let owner = deps.api.addr_validate(&owner)?;
            let update_user_reward_index = update_user_reward_index(deps.storage, &owner)?;
            Ok(
                execute_burn_from(deps, env, info, owner.to_string(), amount)?
                    .add_message(update_user_reward_index),
            )
        }
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => {
            let contract = deps.api.addr_validate(&contract)?;
            let update_user_reward_indexes = vec![
                update_user_reward_index(deps.storage, &deps.api.addr_validate(&owner)?)?,
                update_user_reward_index(deps.storage, &contract)?,
            ];
            Ok(
                execute_send_from(deps, env, info, owner, contract.to_string(), amount, msg)?
                    .add_messages(update_user_reward_indexes),
            )
        }
        ExecuteMsg::ForceUnbond { addresses } => execute_force_unbond(deps, env, info, addresses),
        ExecuteMsg::ForceClaim { addresses } => execute_force_claim(deps, env, info, addresses),
    }
}

pub const REPLY_INIT_VAULT_REWARDS: u64 = 777;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_INIT_VAULT_REWARDS => match msg.result {
            SubMsgResult::Ok(res) => {
                let vault_rewards =
                    parse_instantiate_response_data(res.data.unwrap().as_slice()).unwrap();
                update_rewards_contract(deps, vault_rewards.contract_address)
            }
            SubMsgResult::Err(e) => Err(StdError::generic_err(format!(
                "error instantiating vault rewards contract: {e:?}"
            )))?,
        },
        _ => {
            unimplemented!()
        }
    }
}

fn update_rewards_contract(
    deps: DepsMut,
    rewards_contract: String,
) -> Result<Response, ContractError> {
    deps.api.addr_validate(&rewards_contract)?;

    VAULT_REWARDS.save(deps.storage, &deps.api.addr_validate(&rewards_contract)?)?;

    Ok(Response::default().add_attributes(vec![
        ("action", "init_vault_rewards"),
        ("contract_address", rewards_contract.as_str()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Claims { address } => {
            to_binary(&CLAIMS.query_claims(deps, &deps.api.addr_validate(&address)?)?)
        }
        QueryMsg::Investment {} => to_binary(&query_investment(deps)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::AdditionalTokenInfo {} => to_binary(&query_vault_token_info(deps)?),
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
        QueryMsg::DepositRatio { funds } => to_binary(&query_deposit_ratio(deps, funds)?),
        QueryMsg::PendingBonds { address } => to_binary(&query_pending_bonds(deps, address)?),
        QueryMsg::GetDebug {} => to_binary(&query_debug_string(deps)?),
        QueryMsg::GetTvlInfo {} => to_binary(&query_tvl_info(deps)?),
        QueryMsg::PendingUnbonds { address } => to_binary(&query_pending_unbonds(deps, address)?),
        QueryMsg::GetCap {} => to_binary(&query_cap(deps)?),
        QueryMsg::PendingBondsById { bond_id } => {
            to_binary(&query_pending_bonds_by_id(deps, bond_id)?)
        }
        QueryMsg::PendingUnbondsById { bond_id } => {
            to_binary(&query_pending_unbonds_by_id(deps, bond_id)?)
        }
    }
}

fn query_cap(deps: Deps) -> StdResult<GetCapResponse> {
    Ok(GetCapResponse {
        cap: CAP.load(deps.storage)?,
    })
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
        creation_time: additional_info.creation_time,
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
    let msgs: Result<Vec<WasmMsg>, StdError> = cw20_base::state::BALANCES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|val| {
            let (addr, _) = val?;
            update_user_reward_index(deps.storage, &addr)
        })
        .collect();

    let wrapped_msges = msgs?.into_iter().map(CosmosMsg::Wasm);

    Ok(Response::new()
        .add_attribute(
            "updated-rewards-indexes-msges",
            wrapped_msges.len().to_string(),
        )
        .add_messages(wrapped_msges))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, ContractResult, Decimal, QuerierResult,
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
                        local_denom: "ibc/SOME_DENOM".to_string(),
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
                        local_denom: "ibc/SOME_DENOM".to_string(),
                        base_denom: "uatom".to_string(),
                        quote_denom: "uqsr".to_string(),
                        transfer_channel: "channel-0".to_string(),
                        return_source_channel: "channel-0".to_string(),
                        expected_connection: "connection-0".to_string(),
                    }),
                },
            ],
            vault_rewards_code_id: 123,
            reward_token: cw_asset::AssetInfoBase::Native("uqsr".to_string()),
            reward_distribution_schedules: vec![vault_rewards::state::DistributionSchedule {
                start: 0,
                end: 500,
                amount: Uint128::from(1000u128),
            }],
            total_cap: Uint128::new(10_000_000_000_000),
            deposit_denom: "ibc/SOME_DENOM".to_string(),
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
                                local_denom: "ibc/SOME_DENOM".to_string(),
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
                                local_denom: "ibc/SOME_DENOM".to_string(),
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

    #[test]
    fn test_try_clear_cache() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("lulu", &[]);

        let primitive_configs = vec![
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
                    local_denom: "ibc/SOME_DENOM".to_string(),
                    base_denom: "uqsr".to_string(),
                    quote_denom: "uosmo".to_string(),
                    transfer_channel: "channel-0".to_string(),
                    return_source_channel: "channel-0".to_string(),
                    expected_connection: "connection-0".to_string(),
                }),
            },
        ];

        let investment_info = InvestmentInfo {
            owner: Addr::unchecked("lulu"),
            min_withdrawal: Uint128::from(100u128),
            primitives: primitive_configs,
            deposit_denom: "ibc/SOME_DENOM".to_string(),
        };

        INVESTMENT
            .save(deps.as_mut().storage, &investment_info)
            .unwrap();

        let msg = ExecuteMsg::ClearCache {};
        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(res.messages.len(), 2);
        assert_eq!(
            res.messages[0],
            SubMsg::new(WasmMsg::Execute {
                contract_addr: "prim1".to_string(),
                funds: vec![],
                msg: to_binary(&lp_strategy::msg::ExecuteMsg::TryIcq {}).unwrap(),
            })
        );
        assert_eq!(
            res.messages[1],
            SubMsg::new(WasmMsg::Execute {
                contract_addr: "prim2".to_string(),
                funds: vec![],
                msg: to_binary(&lp_strategy::msg::ExecuteMsg::TryIcq {}).unwrap(),
            })
        );
    }
}
