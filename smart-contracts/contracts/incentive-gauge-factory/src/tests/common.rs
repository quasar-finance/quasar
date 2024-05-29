pub use anyhow::Result;
use cosmwasm_std::{Addr, Timestamp};
pub use derivative::Derivative;
use quasar_types::coinlist::CoinList;

use crate::types::{BlockPeriod, Fee, Gauge, GaugeKind, PoolKind};
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

/// mint tokens in the app
pub fn mint_native(app: &mut App, recipient: String, denom: String, amount: u128) {
    app.sudo(cw_multi_test::SudoMsg::Bank(
        cw_multi_test::BankSudo::Mint {
            to_address: recipient,
            amount: vec![coin(amount, denom)],
        },
    ))
    .unwrap();
}

/// hard resets the time to 1_000 and height 200
pub fn reset_time(app: &mut App) {
    const DEFAULT_TIME: u64 = 1_000;

    app.update_block(|block| {
        block.time = Timestamp::default().plus_seconds(DEFAULT_TIME);
        block.height = DEFAULT_TIME / 5;
    });
}

pub fn contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    )
    .with_reply(crate::contract::reply)
    .with_migrate(crate::contract::migrate);

    Box::new(contract)
}

pub fn incentives_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        merkle_incentives::contract::execute,
        merkle_incentives::contract::instantiate,
        merkle_incentives::contract::query,
    ).with_migrate(merkle_incentives::contract::migrate);

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
    crate::msg::ExecuteMsg::GaugeMsg(crate::msg::GaugeMsg::Create {
        kind: GaugeKind::new_vault(
            Addr::unchecked("vault_addr"),
            None,
            Some(Uint128::zero()),
            Some(Uint128::one()),
        ),
        gauge: Gauge {
            period: BlockPeriod {
                start: 205u64,
                end: 304u64,
                expiry: 304u64,
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

pub fn get_creat_gauge_pool_msg() -> crate::msg::ExecuteMsg {
    crate::msg::ExecuteMsg::GaugeMsg(crate::msg::GaugeMsg::Create {
        kind: GaugeKind::new_pool(
            Addr::unchecked("pool_addr"),
            PoolKind::Liquidity,
            "ucosm".to_string(),
            Some("uatom".to_string()),
        ),
        gauge: Gauge {
            period: BlockPeriod {
                start: 201u64,
                end: 304u64,
                expiry: 304u64,
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
