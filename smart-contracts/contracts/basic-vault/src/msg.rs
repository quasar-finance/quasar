use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::{Binary, Coin, Decimal, Uint128};
use cw20::Expiration;
use cw20::{AllowanceResponse, BalanceResponse, TokenInfoResponse};
pub use cw_controllers::ClaimsResponse;
use quasar_types::callback::{Callback, BondResponse, StartUnbondResponse, UnbondResponse};

use crate::state::BondingStub;

#[cw_serde]
pub enum PrimitiveInitMsg {
    LP(lp_strategy::msg::InstantiateMsg),
}

#[cw_serde]
pub struct PrimitiveConfig {
    // the weighting of this strategy that the vault should subscribe to (e.g. 30%)
    // weights are normalized accross strategies, so values don't need to add up to 100%
    pub weight: Decimal,
    // the contract address of the stored primitive contract on the chain
    pub address: String,
    // the Instantiation message for the primitive.
    pub init: PrimitiveInitMsg,
}

#[cw_serde]
pub struct InstantiateMsg {
    /// name of the derivative token
    pub name: String,
    /// symbol / ticker of the derivative token
    pub symbol: String,
    /// decimal places of the derivative token (for UI)
    pub decimals: u8,

    /// This is the minimum amount we will pull out to reinvest, as well as a minimum
    /// that can be unbonded (to avoid needless staking tx)
    pub min_withdrawal: Uint128,
    // the array of primitives to subscribe to for this vault
    pub primitives: Vec<PrimitiveConfig>,
    // // to be extended & discussed later
    // pub entry_fee: Decimal,
    // pub exit_fee: Decimal,
    // pub fee_receiver: String, // address of the fee receiver
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Bond will bond all staking tokens sent with the message and release derivative tokens
    Bond {},
    /// Unbond will "burn" the given amount of derivative tokens and send the unbonded
    /// staking tokens to the message sender (after exit tax is deducted)
    Unbond {
        amount: Uint128,
    },
    /// Claim is used to claim your native tokens that you previously "unbonded"
    /// after the chain-defined waiting period (eg. 3 weeks)
    Claim {},
    /// Reinvest will check for all accumulated rewards, withdraw them, and
    /// re-bond them to the same validator. Anyone can call this, which updates
    /// the value of the token (how much under custody).
    Reinvest {},
    /// _BondAllTokens can only be called by the contract itself, after all rewards have been
    /// withdrawn. This is an example of using "callbacks" in message flows.
    /// This can only be invoked by the contract itself as a return from Reinvest
    _BondAllTokens {},

    // Callback(Callback),
    BondResponse(BondResponse),
    StartUnbondResponse(StartUnbondResponse),
    UnbondResponse(UnbondResponse),

    /// Implements CW20. Transfer is a base message to move tokens to another account without triggering actions
    Transfer {
        recipient: String,
        amount: Uint128,
    },
    /// Implements CW20. Burn is a base message to destroy tokens forever
    Burn {
        amount: Uint128,
    },
    /// Implements CW20.  Send is a base message to transfer tokens to a contract and trigger an action
    /// on the receiving contract.
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Implements CW20 "approval" extension. Allows spender to access an additional amount tokens
    /// from the owner's (env.sender) account. If expires is Some(), overwrites current allowance
    /// expiration with this one.
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Implements CW20 "approval" extension. Lowers the spender's access of tokens
    /// from the owner's (env.sender) account by amount. If expires is Some(), overwrites current
    /// allowance expiration with this one.
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Implements CW20 "approval" extension. Transfers amount tokens from owner -> recipient
    /// if `env.sender` has sufficient pre-approval.
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    /// Implements CW20 "approval" extension. Sends amount tokens from owner -> contract
    /// if `env.sender` has sufficient pre-approval.
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Implements CW20 "approval" extension. Destroys tokens forever
    BurnFrom {
        owner: String,
        amount: Uint128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Claims shows the number of tokens this address can access when they are done unbonding
    #[returns(ClaimsResponse)]
    Claims { address: String },
    /// Investment shows metadata on the staking info of the contract
    #[returns(InvestmentResponse)]
    Investment {},
    /// DepositRatio shows the ratio of tokens that should be sent for a deposit given list of available tokens
    #[returns(DepositRatioResponse)]
    DepositRatio { funds: Vec<Coin> },

    /// PendingBonds shows the bonds that are currently in the process of being deposited for a user
    #[returns(PendingBondsResponse)]
    PendingBonds { address: String },

    /// GetDebug shows us debug string info
    #[returns(GetDebugResponse)]
    GetDebug {},

    /// Implements CW20. Returns the current balance of the given address, 0 if unset.
    #[returns(BalanceResponse)]
    Balance { address: String },
    /// Implements CW20. Returns metadata on the contract - name, decimals, supply, etc.
    #[returns(TokenInfoResponse)]
    TokenInfo {},
    /// Implements CW20 "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    #[returns(AllowanceResponse)]
    Allowance { owner: String, spender: String },
}

#[cw_serde]
pub struct InvestmentResponse {
    /// owner created the contract and takes a cut
    pub owner: String,
    /// This is the minimum amount we will pull out to reinvest, as well as a minimum
    /// that can be unbonded (to avoid needless staking tx)
    pub min_withdrawal: Uint128,
    // the array of primitives to subscribe to for this vault
    pub primitives: Vec<PrimitiveConfig>,
}

#[cw_serde]
pub struct DepositRatioResponse {
    /// the ratio of tokens that should be sent for a deposit given list of available tokens
    pub primitive_funding_amounts: Vec<Coin>,
    pub remainder: Vec<Coin>,
}

#[cw_serde]
pub struct PendingBondsResponse {
    /// the bonds that are currently in the process of being deposited for a user
    pub pending_bonds: Vec<BondingStub>,
    /// the bond ids that are registered as pending for a user
    pub pending_bond_ids: Vec<String>,
}

#[cw_serde]
pub struct GetDebugResponse {
    /// the debug string
    pub debug: String,
}