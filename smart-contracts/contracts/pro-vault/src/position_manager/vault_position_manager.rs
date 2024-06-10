use cosmwasm_std::{Addr, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

use crate::proshare::share_allocator::{ShareConfig, calculate_shares};

// Define the item for the total shares in the vault and a map for deposits
pub const PROVAULT_POSITION: Item<ProVaultPosition> = Item::new("provault_position");
pub const DEPOSITS: Map<u64, Deposit> = Map::new("deposits");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProVaultPosition {
    pub provault_share_balance: Vec<Coin>,  // Total pro vault shares
    pub outstanding_shares: Uint128,
    pub outstanding_deposit_amount: Uint128,
}

impl ProVaultPosition {
    pub fn new(provault_share_balance: Vec<Coin>, outstanding_shares: Uint128, outstanding_deposit_amount: Uint128) -> Self {
        Self {
            provault_share_balance,
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
    PROVAULT_POSITION.save(deps.storage, &ProVaultPosition::new(vec![], Uint128::zero(), Uint128::zero()))?;

    Ok(Response::new().add_attribute("method", "initialize"))
}

pub fn deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    config: &ShareConfig,
) -> StdResult<Response> {
    // Calculate the number of shares to allocate based on the amount deposited
    let shares = calculate_shares(amount, config);

    // Load the provault position or create a new one if it doesn't exist
    let mut provault_position = PROVAULT_POSITION.load(deps.storage)?;

    // Update the provault position with the new shares
    let mut found = false;
    for coin in &mut provault_position.provault_share_balance {
        if coin.denom == config.share_denom {
            coin.amount += shares;
            found = true;
            break;
        }
    }
    if !found {
        provault_position.provault_share_balance.push(Coin {
            denom: config.share_denom.clone(),
            amount: shares,
        });
    }

    // Update outstanding shares and deposit amount. This represent the
    // amount of share to be distributed further to yield destination via 
    // adaptors by the strategy.
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

    // Create a new deposit record. 
    // TODO - This record should be used by the strategy to seamlessly 
    // distribute the deposited funds to yeild adaptors to earn yield.
    // NOTE- Deposit object and outstanding_share/deposit_amount should
    // be updated in sync
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
