use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Coin, Decimal, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    SubMsg, Uint128, WasmMsg,
};
use quasar_types::coinlist::CoinList;

use crate::{
    msg::MigrateMsg, replies::REPLY_ON_GAUGE_INIT, state::{ADMIN, GAUGES, GAUGE_CODE, GAUGE_FEES, GAUGE_IN_PROCESS, GAUGE_KINDS}, ContractError
};

#[cw_serde]
pub struct BlockPeriod {
    /// start of the period
    pub start: u64,

    /// end of the period
    pub end: u64,

    /// expiration of the period
    pub expiry: u64,
}

impl BlockPeriod {
    pub fn check_conf(&self, env: Env) -> Result<(), ContractError> {
        if env.block.height.ge(&self.start) {
            return Err(ContractError::StartTimeMustBeAhead);
        }

        if env.block.height.ge(&self.end) {
            return Err(ContractError::EndTimeMustBeAhead);
        }

        if env.block.height.ge(&self.expiry) {
            return Err(ContractError::ExpiryTimeMustBeAhead);
        }

        if self.end.le(&self.start) {
            return Err(ContractError::EndTimeBiggerThanStart);
        }

        if self.expiry.le(&self.start) {
            return Err(ContractError::ExpiryTimeBiggerThanStart);
        }

        Ok(())
    }
}

#[cw_serde]
pub struct GaugeInProcess {
    pub gauge: Gauge,
    pub kind: GaugeKind,
    pub fee: Fee,
}

#[cw_serde]
pub struct Gauge {
    pub period: BlockPeriod,
    pub incentives: Vec<Coin>,
    pub clawback: String,
}

impl Gauge {
    pub fn new(period: BlockPeriod, incentives: Vec<Coin>, clawback: String) -> Self {
        // let fee = Fee::new(fee_address, fee, CoinList::new(total_incentives.clone()));
        Self {
            period,
            incentives,
            clawback,
        }
    }

    /// write the code for the guage contract
    pub fn code_update(
        deps: DepsMut,
        info: MessageInfo,
        code: u64,
    ) -> Result<Response, ContractError> {
        ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
        GAUGE_CODE.save(deps.storage, &code)?;
        Ok(Response::default().add_attribute("action", "code_update"))
    }

    /// verify that the guage exists in our map
    pub fn check_gauge_exists(deps: Deps, contract_addr: Addr) -> Result<(), ContractError> {
        if !GAUGES.has(deps.storage, contract_addr.clone()) {
            return Err(ContractError::NoSuchGauge {
                addr: contract_addr.into_string(),
            });
        }
        Ok(())
    }

    /// initializes a gauge contract
    pub fn create(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        gauge: Gauge,
        fee: Fee,
        kind: GaugeKind,
    ) -> Result<Response, ContractError> {
        // instantiate an instance of the incentive gauge
        // save the instance in the Gauge overview
        // depending on the gauge type, execute verification, eg verify that a vault is a quasar cl vault
        let code_id = GAUGE_CODE.load(deps.storage)?;
        let factory = env.contract.address.clone();

        gauge.period.check_conf(env)?;

        let msg = merkle_incentives::msg::InstantiateMsg {
            config: merkle_incentives::state::Config {
                clawback_address: deps.api.addr_validate(&gauge.clawback)?,
                start_block: gauge.period.start,
                end_block: gauge.period.end,
                expiration_block: gauge.period.expiry,
            },
        };

        // check fee reciever is a valid address
        deps.api.addr_validate(&fee.reciever)?;

        // pre save the gauge data
        // it will be copied to the relevant maps when the gauge contract replies on init
        GAUGE_IN_PROCESS.save(deps.storage, &GaugeInProcess { gauge, kind, fee })?;

        Ok(Response::default()
            .add_attribute("action", "gauge_create")
            .add_submessage(SubMsg::<Empty>::reply_on_success(
                WasmMsg::Instantiate {
                    label: "Incentives gauge".into(),
                    admin: Some(factory.into_string()),
                    code_id,
                    funds: info.funds,
                    msg: to_json_binary(&msg)?,
                },
                REPLY_ON_GAUGE_INIT,
            )))
    }

    /// primarly updates the gauge information locally
    /// NOTE: this might need more work
    pub fn update(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        addr: String,
        new_gauge: Gauge,
        new_fees: Option<Fee>,
        new_kind: Option<GaugeKind>,
    ) -> Result<Response, ContractError> {
        ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

        let addr = deps.api.addr_validate(&addr)?;

        Self::check_gauge_exists(deps.as_ref(), addr.clone())?;

        new_gauge.period.check_conf(env)?;

        GAUGES.save(deps.storage, addr.clone(), &new_gauge)?;

        if let Some(fees) = new_fees {
            GAUGE_FEES.save(deps.storage, addr.clone(), &fees)?;
        }

        if let Some(kind) = new_kind {
            GAUGE_KINDS.save(deps.storage, addr, &kind)?;
        }

        Ok(Response::default().add_attribute("action", "gauge_update"))
    }

    /// validates gauge exists and then removes it and its dependencies
    pub fn remove(
        deps: DepsMut,
        info: MessageInfo,
        addr: String,
    ) -> Result<Response, ContractError> {
        ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

        let contract_addr = deps.api.addr_validate(&addr)?;

        Self::check_gauge_exists(deps.as_ref(), contract_addr.clone())?;

        GAUGES.remove(deps.storage, contract_addr.clone());
        GAUGE_KINDS.remove(deps.storage, contract_addr.clone());
        GAUGE_FEES.remove(deps.storage, contract_addr);

        Ok(Response::default().add_attribute("action", "gauge_remove"))
    }

    /// migrate gauge to a new version
    pub fn migrate(
        deps: DepsMut,
        info: MessageInfo,
        addr: String,
        new_code_id: u64,
        version: String,
    ) -> Result<Response, ContractError> {
        ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
        let contract_addr = deps.api.addr_validate(&addr)?;

        Self::check_gauge_exists(deps.as_ref(), contract_addr)?;

        Ok(Response::default()
            .add_attribute("action", "gauge_migrate")
            .add_message(WasmMsg::Migrate {
                contract_addr: addr,
                new_code_id,
                msg: to_json_binary(&MigrateMsg {
                    version,
                })?,
            }))
    }

    /// sends a merkle root to the gauge if the gauge exists
    pub fn merkle_update(
        deps: DepsMut,
        info: MessageInfo,
        addr: String,
        merkle: String,
    ) -> Result<Response, ContractError> {
        ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

        let contract_addr = deps.api.addr_validate(&addr)?;

        Self::check_gauge_exists(deps.as_ref(), contract_addr)?;

        Ok(Response::default()
            .add_attribute("action", "merkle_update")
            .add_message(WasmMsg::Execute {
                contract_addr: addr.clone(),
                msg: to_json_binary(&merkle_incentives::msg::ExecuteMsg::AdminMsg(
                    merkle_incentives::admin::execute::AdminExecuteMsg::UpdateMerkleRoot {
                        new_root: merkle,
                    },
                ))?,
                funds: vec![],
            }))
    }
}

#[cw_serde]
pub struct Fee {
    /// this is the address that is configured to recieve the fee
    pub reciever: String,

    /// ratio to charge
    pub ratio: Decimal,

    /// total calculated fees
    pub total: CoinList,

    /// remaining fees to be collected
    pub remaining: CoinList,
}

impl Fee {
    pub fn new(reciever: String, ratio: Decimal, total_incentives: CoinList) -> Self {
        let total = total_incentives.mul_ratio(ratio);

        Self {
            reciever,
            ratio,
            total: total.clone(),
            remaining: total,
        }
    }

    pub fn update(
        deps: DepsMut,
        info: MessageInfo,
        addr: String,
        fees: Fee,
    ) -> Result<Response, ContractError> {
        ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

        let contract_addr = deps.api.addr_validate(&addr)?;

        if GAUGE_FEES.has(deps.storage, contract_addr.clone()) {
            GAUGE_FEES.save(deps.storage, contract_addr, &fees)?;
            return Ok(Response::default().add_attribute("action", "fee_update"));
        }

        Err(ContractError::NoSuchGauge { addr })
    }

    // Note: by removing the fees from the gauge the read cost remains the same but the write cost is reduced
    pub fn distribute(
        deps: DepsMut,
        env: Env,
        gauge_addr: String,
    ) -> Result<Response, ContractError> {
        let gauge_addr = deps.api.addr_validate(&gauge_addr)?;

        let gauge = GAUGES.load(deps.storage, gauge_addr.clone())?;

        let mut fees = GAUGE_FEES.load(deps.storage, gauge_addr.clone())?;

        let elapsed_time = env.block.height - gauge.period.start;
        let total_time = gauge.period.end - gauge.period.start;

        // calculate what % of the gauge has passed
        let elapsed_ratio = Decimal::from_ratio(elapsed_time, total_time);
        let claimable_until_now = fees.total.mul_ratio(elapsed_ratio);

        let claimed = fees.total.checked_sub(&fees.remaining)?;

        // calculate the difference between what fees were already paid out and what is claimable
        let to_receive = claimable_until_now.clone() - claimed;
        let new_remaining_fees = fees.total.checked_sub(&claimable_until_now)?;

        fees.remaining = new_remaining_fees;

        GAUGE_FEES.save(deps.storage, gauge_addr, &fees)?;

        Ok(Response::default()
            .add_attribute("action", "fee_distribute")
            .add_message(BankMsg::Send {
                to_address: fees.reciever.to_string(),
                amount: to_receive.coins(),
            }))
    }
}

#[cw_serde]
pub enum PoolKind {
    Volume = 1,
    Liquidity,
}

/// The different kinds of incentive gauges supported by Quasar
/// Each kind of gauge is created in the incentive gauge factory
/// The offchain infrastructure picks up the settings from the onchain created gauge
#[cw_serde]
pub enum GaugeKind {
    /// The gauge type to incentivize a Quasar vault.
    /// address is the contract address of the corresponding Quasar vault to incentivize
    /// blacklist gives support to blacklist certain addresses, such as contracts that deposit into the vault but do not have the capability to claim any incentives
    /// min_shares is an optional setting to define a minimum amount of shares needed to earn any incentives
    /// max_shares is an optional setting to define a maximum amount of shares a user can earn any incentives over, any users over the max amount are given rewards according to the max_shares amount
    Vault {
        address: Addr,
        blacklist: Option<Vec<Addr>>,
        min_shares: Option<Uint128>,
        max_shares: Option<Uint128>,
    },

    /// Pool guage incentivization.
    /// There are two types of [PoolKind] PoolKind::Volume and PoolKind::Liquidity.
    /// To incentivize a specific token or both use denom_a and denom_b for the names of the tokens.
    Pool {
        address: Addr,
        kind: PoolKind,
        denom_a: String,
        denom_b: Option<String>,
    },
}

impl GaugeKind {
    pub fn new_vault(
        address: Addr,
        blacklist: Option<Vec<Addr>>,
        min_shares: Option<Uint128>,
        max_shares: Option<Uint128>,
    ) -> Self {
        GaugeKind::Vault {
            address,
            blacklist,
            min_shares,
            max_shares,
        }
    }

    pub fn new_pool(
        address: Addr,
        kind: PoolKind,
        denom_a: String,
        denom_b: Option<String>,
    ) -> Self {
        GaugeKind::Pool {
            address,
            kind,
            denom_a,
            denom_b,
        }
    }
}
