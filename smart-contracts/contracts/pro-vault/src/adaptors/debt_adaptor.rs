use cosmwasm_std::{Addr, Coin, DepsMut, Env, QuerierWrapper, Response, StdError, StdResult};
use abstract_money_market_standard::MoneyMarketCommand;
use crate::adaptors::base_adaptor::{DebtAdapter, Adapter, AdapterMetadata, load_metadata};
use crate::adaptors::mars_wrapper::MarsWrapper;
use crate::error::ContractError;
use cw_asset::Asset; // Recheck against abstract Asset type
use cw_asset::AssetInfo; // Recheck against abstract AssetInfo type
use cosmwasm_std::CosmosMsg;
pub struct MarsMoneyMarketAdapter {
    market_impl: MarsWrapper,
}

impl Adapter for MarsMoneyMarketAdapter {
    fn metadata(&self) -> AdapterMetadata {
        // Implement the logic to return metadata for MarsMoneyMarketAdapter
        AdapterMetadata {
            name: "Mars Money Market Adapter".to_string(),
            desc: "Adapter for Mars Money Market".to_string(),
            dest_chain_id: "mars_chain".to_string(),
            dest_contract_addr: "mars_contract_addr".to_string(),
            dest_market_type: crate::adaptors::base_adaptor::MarketType::Debt,
        }
    }

    fn query_net_assets(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {
        // Implement the logic to get the net assets
        unimplemented!()
    }

    fn query_expected_available_assets(&self, querier: &QuerierWrapper, env: Env) -> Result<Vec<Coin>, StdError> {
        // Implement the logic to get the expected available assets
        unimplemented!()
    }

    fn query_allocated_shares(&self, querier: &QuerierWrapper, env: Env) -> Result<Coin, StdError> {
        // Implement the logic to get the allocated shares
        unimplemented!()
    }
}

impl DebtAdapter for MarsMoneyMarketAdapter {
    type AdapterError = ContractError;
    /*
    fn deposit_collateral(&self, deps: DepsMut, assets: Vec<Coin>) -> Result<Response, Self::AdapterError> {
        let metadata = load_metadata(deps.storage)?;
        let msgs: Result<Vec<_>, _> = assets.into_iter().map(|asset| {
            let asset = Asset {
                info: AssetInfo::Native(asset.denom.clone()),
                amount: asset.amount,
            };
            self.market_impl.provide_collateral(deps.as_ref(), Addr::unchecked(metadata.dest_contract_addr.clone()), asset)
        }).collect();
        Ok(Response::new().add_messages(msgs))
    }
    */

    fn deposit_collateral(&self, deps: DepsMut, assets: Vec<Coin>) -> Result<Response, Self::AdapterError> {
        let metadata = load_metadata(deps.storage)?;
        let msgs: Result<Vec<CosmosMsg>, _> = assets.into_iter().map(|asset| {
            let asset = Asset {
                info: AssetInfo::Native(asset.denom.clone()),
                amount: asset.amount,
            };
            self.market_impl.provide_collateral(deps.as_ref(), Addr::unchecked(metadata.dest_contract_addr.clone()), asset)
        }).collect::<Result<Vec<_>, _>>().map(|v| v.into_iter().flatten().collect());
    
        Ok(Response::new().add_messages(msgs?))
    }

    fn withdraw_collateral(&self, deps: DepsMut, shares: Coin) -> Result<Response, Self::AdapterError> {
        let metadata = load_metadata(deps.storage)?;
        let asset = Asset {
            info: AssetInfo::Native(shares.denom.clone()),
            amount: shares.amount,
        };
        let msgs = self.market_impl.withdraw_collateral(deps.as_ref(), Addr::unchecked(metadata.dest_contract_addr.clone()), asset)?;
        Ok(Response::new().add_messages(msgs))
    }
}

/* 
fn load_metadata(storage: &dyn cosmwasm_std::Storage) -> StdResult<AdapterMetadata> {
    crate::adaptors::base_adaptor::ADAPTER_METADATA.load(storage)
}
*/
