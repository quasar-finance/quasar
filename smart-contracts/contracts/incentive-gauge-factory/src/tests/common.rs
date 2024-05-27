pub use anyhow::Result;
use cosmwasm_std::{testing::mock_env, Addr};
pub use derivative::Derivative;
use quasar_types::coinlist::CoinList;

use crate::types::{BlockPeriod, Fee, Gauge, GaugeKind};
pub use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};
pub use cosmwasm_std::{coin, BlockInfo, Coin, Decimal, Empty, StdResult, Uint128};
pub use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};

// pub const USER: &str = "user";
// pub const DEPLOYER: &str = "deployer";
// pub const EXECUTOR: &str = "executor";
// pub const DENOM: &str = "uosmo";
// pub const LOCAL_DENOM: &str = "ibc/ilovemymom";

// let ADMIN = app.api.addr_make("admin");

pub fn contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    )
    .with_reply(crate::contract::reply);

    Box::new(contract)
}

pub fn incentives_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        merkle_incentives::contract::execute,
        merkle_incentives::contract::instantiate,
        merkle_incentives::contract::query,
    );
    Box::new(contract)
}

pub fn new_init_msg(admin: Option<String>, gauge_codeid: Option<u64>) -> InstantiateMsg {
    InstantiateMsg {
        admin,
        gauge_codeid,
    }
}

pub fn contract_init(
    app: &mut App,
    admin: String,
    init_msg: InstantiateMsg,
) -> Result<(u64, Addr), anyhow::Error> {
    let code_id = app.store_code(contract());
    Ok((
        code_id,
        app.instantiate_contract(
            code_id,
            Addr::unchecked(&admin),
            &init_msg,
            &[],
            "Incentive Gauge Factory",
            Some(admin),
        )?,
    ))
}

pub fn merkle_incentives_upload(app: &mut App) -> u64 {
    app.store_code(incentives_contract())
}

pub fn get_creat_gauge_msg() -> crate::msg::ExecuteMsg {
    let env = mock_env();

    crate::msg::ExecuteMsg::GaugeMsg(crate::msg::GaugeMsg::Create {
        kind: GaugeKind::new_vault(
            Addr::unchecked("vault_addr"),
            None,
            Some(Uint128::zero()),
            Some(Uint128::one()),
        ),
        gauge: Gauge {
            period: BlockPeriod {
                start: env.block.height + 1u64,
                end: env.block.height + 10u64,
                expiry: env.block.height + 100u64,
            },
            incentives: vec![coin(1000, "ucosm")],
            clawback: "clawback_addr".to_string(),
        },
        fee: Fee::new(
            "reciever".to_string(),
            Decimal::from_ratio(Uint128::from(500u16), Uint128::one()),
            CoinList::new(vec![coin(100, "ucosm")]),
        ),
    })
}
