use cosmwasm_std::{CosmosMsg, DepsMut, Uint128};

use crate::ContractError;
use osmosis_std::types::cosmos::bank::v1beta1::BankQuerier;

fn claim_rewards() -> Result<CosmosMsg, ContractError> {
    // silence error
    unimplemented!()
}

fn update_rewards_map(deps: DepsMut, _total_rewards: Uint128) -> Result<(), ContractError> {
    let bank_querier: BankQuerier<'_, cosmwasm_std::Empty> = BankQuerier::new(&deps.querier);
    let _balances = bank_querier.denoms_metadata(None)?;
    let _total = bank_querier.total_supply(None)?;

    todo!()
}
