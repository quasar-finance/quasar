use cosmwasm_std::{CosmosMsg, DepsMut, QuerierWrapper, Uint128};

use crate::ContractError;
use osmosis_std::types::cosmos::bank::v1beta1::BankQuerier;

fn claim_rewards() -> Result<CosmosMsg, ContractError> {
    // silence error
    unimplemented!()
}

fn update_rewards_map(deps: DepsMut, total_rewards: Uint128) -> Result<(), ContractError> {
    let bank_querier: BankQuerier<'_, cosmwasm_std::Empty> = BankQuerier::new(&deps.querier);
    let balances = bank_querier.denoms_metadata(None)?;
    let total = bank_querier.total_supply(None)?;

    todo!()
}
