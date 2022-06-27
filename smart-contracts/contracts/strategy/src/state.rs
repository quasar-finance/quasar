use std::collections::VecDeque;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use quasar_traits::traits::ShareDistributor;
use quasar_types::curve::{CurveType, DecimalPlaces};

use serde::de::DeserializeOwned;
use share_distributor::single_token::SingleToken;


pub struct WithdrawRequest {
    denom: String,
    amount: Uint128,
    // an id so the vault knows which withdrawal is being fulfilled
    id: Uuid,
}

pub const WITHDRAW_QUEUE: Item<VecDeque<WithdrawRequest>> = Item::new("withdraw_queue");
