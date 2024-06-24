use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, Deps, DepsMut, QuerierWrapper, Response, StdError, StdResult, Uint128, WasmMsg, wasm_execute, Decimal,
};
use abstract_sdk::feature_objects::AnsHost;
use abstract_std::objects::{ans_host::AnsHostError, AssetEntry, ContractEntry};
use abstract_adapter_utils::identity::Identify;
use abstract_money_market_standard::{MoneyMarketCommand, MoneyMarketError};
use mars_red_bank_types::red_bank::ExecuteMsg;
use mars_red_bank_types::oracle::{PriceResponse, QueryMsg as OracleQueryMsg};
use cw_asset::{AssetInfo, Asset};
use crate::error::ContractError;
#[derive(Default)]
pub struct Mars {
    pub oracle_contract: Option<Addr>,
}

impl Identify for Mars {
    fn name(&self) -> &'static str {
        "mars"
    }

    fn is_available_on(&self, chain_name: &str) -> bool { 
        // Implement the logic to check availability on the given chain
        // TODO - This is a placeholder implementation. Replace with actual logic.
        chain_name == "mars_chain"
    }
}

pub struct MarsWrapper {
    pub mars: Mars,
    pub oracle_contract: Option<Addr>,
}

impl Identify for MarsWrapper {
    fn name(&self) -> &'static str {
        "mars"
    }

    fn is_available_on(&self, chain_name: &str) -> bool {
        self.mars.is_available_on(chain_name)
    }
}

impl MoneyMarketCommand for MarsWrapper {
    fn fetch_data(&mut self, addr_as_sender: Addr, querier: &QuerierWrapper, ans_host: &AnsHost) -> Result<(), MoneyMarketError> {
        let contract_entry = ContractEntry {
            protocol: self.name().to_string(),
            contract: "oracle".to_string(),
        };

        self.oracle_contract = Some(ans_host.query_contract(querier, &contract_entry)?);
        Ok(())
    }

    fn deposit(&self, deps: Deps, contract_addr: Addr, asset: Asset) -> Result<Vec<CosmosMsg>, MoneyMarketError> {
        let vault_msg = ExecuteMsg::Deposit { on_behalf_of: None };
        let msg = wasm_execute(contract_addr, &vault_msg, vec![asset.try_into()?])?;
        Ok(vec![msg.into()])
    }

    fn withdraw(&self, deps: Deps, contract_addr: Addr, lending_asset: Asset) -> Result<Vec<CosmosMsg>, MoneyMarketError> {
        let denom = unwrap_native(lending_asset.info)?;
        let vault_msg = ExecuteMsg::Withdraw {
            recipient: None,
            denom,
            amount: Some(lending_asset.amount),
        };
        let msg = wasm_execute(contract_addr, &vault_msg, vec![])?;
        Ok(vec![msg.into()])
    }

    fn provide_collateral(&self, deps: Deps, contract_addr: Addr, asset: Asset) -> Result<Vec<CosmosMsg>, MoneyMarketError> {
        let vault_msg = ExecuteMsg::Deposit { on_behalf_of: None };
        let msg = wasm_execute(contract_addr, &vault_msg, vec![asset.try_into()?])?;
        Ok(vec![msg.into()])
    }

    fn withdraw_collateral(&self, deps: Deps, contract_addr: Addr, asset: Asset) -> Result<Vec<CosmosMsg>, MoneyMarketError> {
        let vault_msg = ExecuteMsg::Withdraw {
            recipient: None,
            denom: unwrap_native(asset.info)?,
            amount: Some(asset.amount),
        };
        let msg = wasm_execute(contract_addr, &vault_msg, vec![])?;
        Ok(vec![msg.into()])
    }

    fn borrow(&self, deps: Deps, contract_addr: Addr, asset: Asset) -> Result<Vec<CosmosMsg>, MoneyMarketError> {
        let vault_msg = ExecuteMsg::Borrow {
            recipient: None,
            denom: unwrap_native(asset.info)?,
            amount: asset.amount,
        };
        let msg = wasm_execute(contract_addr, &vault_msg, vec![])?;
        Ok(vec![msg.into()])
    }

    fn repay(&self, deps: Deps, contract_addr: Addr, asset: Asset) -> Result<Vec<CosmosMsg>, MoneyMarketError> {
        let vault_msg = ExecuteMsg::Repay { on_behalf_of: None };
        let msg = wasm_execute(contract_addr, &vault_msg, vec![asset.try_into()?])?;
        Ok(vec![msg.into()])
    }
  
    fn price(&self, deps: Deps, base: AssetInfo, quote: AssetInfo) -> Result<Decimal, MoneyMarketError> {
            let oracle_contract = self
                .oracle_contract
                .as_ref()
                .ok_or(ContractError::OracleContractNotSet)
                .map_err(to_money_market_error)?;

        let base_price: PriceResponse = deps.querier.query_wasm_smart(
            oracle_contract,
            &OracleQueryMsg::Price {
                denom: unwrap_native(base)?,
            },
        )?;
        let quote_price: PriceResponse = deps.querier.query_wasm_smart(
            oracle_contract,
            &OracleQueryMsg::Price {
                denom: unwrap_native(quote)?,
            },
        )?;
        Ok(base_price.price.checked_div(quote_price.price)?)
    }

    fn user_deposit(&self, deps: Deps, contract_addr: Addr, user: Addr, asset: AssetInfo) -> Result<Uint128, MoneyMarketError> {
        let market_msg = mars_red_bank_types::red_bank::QueryMsg::UserCollateral {
            user: user.to_string(),
            denom: unwrap_native(asset)?,
        };
        let query_response: mars_red_bank_types::red_bank::UserCollateralResponse = deps.querier.query_wasm_smart(contract_addr, &market_msg)?;
        Ok(query_response.amount)
    }

    fn user_collateral(&self, deps: Deps, contract_addr: Addr, user: Addr, _borrowed_asset: AssetInfo, collateral_asset: AssetInfo) -> Result<Uint128, MoneyMarketError> {
        let market_msg = mars_red_bank_types::red_bank::QueryMsg::UserCollateral {
            user: user.to_string(),
            denom: unwrap_native(collateral_asset)?,
        };
        let query_response: mars_red_bank_types::red_bank::UserCollateralResponse = deps.querier.query_wasm_smart(contract_addr, &market_msg)?;
        Ok(query_response.amount)
    }

    fn user_borrow(&self, deps: Deps, contract_addr: Addr, user: Addr, borrowed_asset: AssetInfo, _collateral_asset: AssetInfo) -> Result<Uint128, MoneyMarketError> {
        let market_msg = mars_red_bank_types::red_bank::QueryMsg::UserDebt {
            user: user.to_string(),
            denom: unwrap_native(borrowed_asset)?,
        };
        let query_response: mars_red_bank_types::red_bank::UserDebtResponse = deps.querier.query_wasm_smart(contract_addr, &market_msg)?;
        Ok(query_response.amount)
    }

    fn current_ltv(&self, deps: Deps, contract_addr: Addr, user: Addr, _borrowed_asset: AssetInfo, _collateral_asset: AssetInfo) -> Result<Decimal, MoneyMarketError> {
        let market_msg = mars_red_bank_types::red_bank::QueryMsg::UserPosition {
            user: user.to_string(),
        };
        let query_response: mars_red_bank_types::red_bank::UserPositionResponse = deps.querier.query_wasm_smart(contract_addr, &market_msg)?;
        if query_response.total_enabled_collateral.is_zero() {
            return Ok(Decimal::zero());
        }
        Ok(Decimal::from_ratio(
            query_response.total_collateralized_debt,
            query_response.total_enabled_collateral,
        ))
    }

    fn max_ltv(&self, deps: Deps, contract_addr: Addr, user: Addr, _borrowed_asset: AssetInfo, _collateral_asset: AssetInfo) -> Result<Decimal, MoneyMarketError> {
        let market_msg = mars_red_bank_types::red_bank::QueryMsg::UserPosition {
            user: user.to_string(),
        };
        let query_response: mars_red_bank_types::red_bank::UserPositionResponse = deps.querier.query_wasm_smart(contract_addr, &market_msg)?;
        if query_response.total_enabled_collateral.is_zero() {
            return Ok(Decimal::zero());
        }
        Ok(Decimal::from_ratio(
            query_response.weighted_max_ltv_collateral,
            query_response.total_enabled_collateral,
        ))
    }

    fn lending_address(&self, querier: &QuerierWrapper, ans_host: &AnsHost, _lending_asset: AssetEntry) -> Result<Addr, AnsHostError> {
        self.mars.red_bank(querier, ans_host)
    }

    fn collateral_address(&self, querier: &QuerierWrapper, ans_host: &AnsHost, _lending_asset: AssetEntry, _collateral_asset: AssetEntry) -> Result<Addr, AnsHostError> {
        self.mars.red_bank(querier, ans_host)
    }
    
    fn borrow_address(&self, querier: &QuerierWrapper, ans_host: &AnsHost, _lending_asset: AssetEntry, _collateral_asset: AssetEntry) -> Result<Addr, AnsHostError> {
        self.mars.red_bank(querier, ans_host)
    }

    fn max_ltv_address(&self, querier: &QuerierWrapper, ans_host: &AnsHost, _lending_asset: AssetEntry, _collateral_asset: AssetEntry) -> Result<Addr, AnsHostError> {
        self.mars.red_bank(querier, ans_host)
    }

    fn current_ltv_address(&self, querier: &QuerierWrapper, ans_host: &AnsHost, _lending_asset: AssetEntry, _collateral_asset: AssetEntry) -> Result<Addr, AnsHostError> {
        self.mars.red_bank(querier, ans_host)
    }
}

impl Mars {
    fn red_bank(&self, querier: &QuerierWrapper, ans_host: &AnsHost) -> Result<Addr, AnsHostError> {
        let contract_entry = ContractEntry {
            protocol: self.name().to_string(),
            contract: "red-bank".to_string(),
        };
        ans_host.query_contract(querier, &contract_entry).map_err(Into::into)
    }
}

fn unwrap_native(asset: AssetInfo) -> Result<String, MoneyMarketError> {
    match asset {
        cw_asset::AssetInfoBase::Native(denom) => Ok(denom),
        cw_asset::AssetInfoBase::Cw20(_) => Err(MoneyMarketError::ExpectedNative {}),
        _ => todo!(),
    }
}


fn to_money_market_error(err: ContractError) -> MoneyMarketError {
    match err {
        ContractError::OracleContractNotSet => MoneyMarketError::Std(StdError::generic_err("Oracle contract not set")),
        ContractError::ExpectedNative => MoneyMarketError::Std(StdError::generic_err("Expected native asset")),
        _ => MoneyMarketError::Std(StdError::generic_err("Unexpected error")),
    }
}
