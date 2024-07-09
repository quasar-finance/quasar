use cosmwasm_std::{Addr, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

use crate::proshare::share_allocator::{ShareConfig, calculate_shares};

// Define the item for the total shares in the vault and a map for deposits
pub const PROVAULT_POSITION: Item<ProVaultPosition> = Item::new("provault_position");
// Map for storing deposits
pub const DEPOSITS: Map<u64, Deposit> = Map::new("deposits");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProVaultPosition {
    pub total_shares: Uint128,             // Total shares in the vault
    pub total_assets: Uint128,             // Total asset under management 
    pub outstanding_shares: Uint128,       // Shares allocated to connected adapters
    pub outstanding_deposit_amount: Uint128, // Total deposit amount in the vault
}

impl ProVaultPosition {
    pub fn new(total_shares: Uint128, 
        total_assets: Uint128, 
        outstanding_shares: Uint128, 
        outstanding_deposit_amount: Uint128) -> Self {
        Self {
            total_shares,
            total_assets,
            outstanding_shares,
            outstanding_deposit_amount,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    pub id: u64,
    pub depositor: Addr,
    pub amount: Uint128,
    pub shares: Uint128,
}

pub fn initialize(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> StdResult<Response> {
    // Initialize the provault position with an empty balance
    PROVAULT_POSITION.save(deps.storage, 
        &ProVaultPosition::new(Uint128::zero(),Uint128::zero(),
         Uint128::zero(), Uint128::zero()))?;

    Ok(Response::new().add_attribute("method", "initialize"))
}

pub fn deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    config: &ShareConfig,
) -> StdResult<Response> {
    // Load the provault position or create a new one if it doesn't exist
    let mut provault_position = PROVAULT_POSITION
        .may_load(deps.storage)?
        .unwrap_or_else(|| ProVaultPosition::new(Uint128::zero(),Uint128::zero(),
                         Uint128::zero(), Uint128::zero()));

    // Calculate the number of shares to allocate based on the amount deposited
    let shares = calculate_shares(amount, 
        provault_position.total_shares,
        provault_position.total_assets);

    // Update the provault position with the new shares, its also total share
    provault_position.total_shares += shares;
    // Track shares and assets allocated to connected adapters
    provault_position.outstanding_shares += shares; 
    provault_position.outstanding_deposit_amount += amount;

    // Save the updated provault position
    PROVAULT_POSITION.save(deps.storage, &provault_position)?;

    // Generate a unique deposit ID
    let deposit_id = DEPOSITS
        .keys(deps.storage, None, None, Order::Descending)
        .next()
        .map(|key| key.unwrap_or(0) + 1)
        .unwrap_or(1);

    // Create a new deposit record
    let deposit = Deposit {
        id: deposit_id,
        depositor: info.sender.clone(),
        amount,
        shares,
    };

    // Save the deposit record
    DEPOSITS.save(deps.storage, deposit_id, &deposit)?;

    Ok(Response::new()
        .add_attribute("method", "deposit")
        .add_attribute("amount", amount.to_string())
        .add_attribute("shares", shares.to_string())
        .add_attribute("deposit_id", deposit_id.to_string()))
}

pub fn query_provault_position(deps: Deps) -> StdResult<ProVaultPosition> {
    let provault_position = PROVAULT_POSITION.load(deps.storage)?;
    Ok(provault_position)
}

pub fn query_deposit(deps: Deps, deposit_id: u64) -> StdResult<Deposit> {
    let deposit = DEPOSITS.load(deps.storage, deposit_id)?;
    Ok(deposit)
}
