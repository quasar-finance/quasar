#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_dependencies_with_balances, mock_env, mock_info},
        Api, Coin, CustomQuery, Decimal, DepsMut, Empty, Env, MessageInfo, OwnedDeps, Querier,
        Response, Storage, Uint128,
    };
    use quasar_types::callback::BondResponse;

    use crate::{
        contract::execute,
        contract::instantiate,
        msg::{ExecuteMsg, InstantiateMsg, PrimitiveConfig, PrimitiveInitMsg},
    };

    const TEST_CREATOR: &str = "creator";

    fn init_msg() -> InstantiateMsg {
        InstantiateMsg {
            name: "Blazar Vault".to_string(),
            symbol: "BLZR".to_string(),
            decimals: 6,
            min_withdrawal: Uint128::one(),
            primitives: vec![PrimitiveConfig {
                weight: Decimal::one(),
                address: "quasar123".to_string(),
                init: PrimitiveInitMsg::LP(lp_strategy::msg::InstantiateMsg {
                    lock_period: 14, // this is supposed to be nanos i think
                    pool_id: 1,
                    pool_denom: "gamm/pool/1".to_string(),
                    local_denom: "ibc/uosmo".to_string(),
                    base_denom: "uosmo".to_string(),
                    quote_denom: "uatom".to_string(),
                    transfer_channel: "what".to_string(),
                    return_source_channel: "should this be".to_string(),
                }),
            }],
        }
    }

    fn init<'a>(deps: DepsMut, msg: &InstantiateMsg, env: &Env, info: &MessageInfo) -> Response {
        let res = instantiate(deps, env.clone(), info.clone(), msg.clone()).unwrap();

        res
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balances(&[]);
        let msg = init_msg();
        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), &msg, &env, &info);

        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn proper_on_bond_callback() {
        let mut deps = mock_dependencies_with_balances(&[]);
        let msg = init_msg();
        let info = mock_info(TEST_CREATOR, &[]);
        let env = mock_env();
        _ = init(deps.as_mut(), &msg, &env, &info);

        let execute_msg = ExecuteMsg::BondResponse(BondResponse {
            share_amount: Uint128::from(100u128),
            bond_id: Uint128::from(1u128).to_string(),
        });

        let res = execute(deps.as_mut(), env, info, execute_msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
