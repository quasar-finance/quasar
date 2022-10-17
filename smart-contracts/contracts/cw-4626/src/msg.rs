use cosmwasm_std::{Binary, Coin, StdError, StdResult, Uint128, Uint256};
use cw20::{Cw20Coin, Logo, MinterResponse};
use cw_utils::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMarketingInfo {
    pub project: Option<String>,
    pub description: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    // the token accepted by the contract.
    pub reserve_denom: String,
    // the total amount of tokens that can be put in the reserve.
    // TODO change to Option<Uint128> to allow for no cap on reserve supply
    pub reserve_total_supply: Uint128,
    pub reserve_decimals: u8,
    // supply is the amount of tokens this vault has issued.
    // supply_decimals is the amount of decimals of the supply token
    pub supply_decimals: u8,
    // TODO should this be an Option to not need the field in the initialization json
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
    pub marketing: Option<InstantiateMarketingInfo>,

    pub curve_type: quasar_types::curve::CurveType,
}

impl InstantiateMsg {
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|v| v.cap)
    }

    pub fn validate(&self) -> StdResult<()> {
        // Check name, symbol, decimals
        if !is_valid_name(&self.name) {
            return Err(StdError::generic_err(
                "Name is not in the expected format (3-50 UTF-8 bytes)",
            ));
        }
        if !is_valid_symbol(&self.symbol) {
            return Err(StdError::generic_err(
                "Ticker symbol is not in expected format [a-zA-Z\\-]{3,12}",
            ));
        }
        if self.reserve_denom.is_empty() {
            return Err(StdError::generic_err("No reserve denom"));
        }
        if self.supply_decimals > 18 {
            return Err(StdError::generic_err("Decimals must not exceed 18"));
        }
        Ok(())
    }
}

fn is_valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() < 3 || bytes.len() > 50 {
        return false;
    }
    true
}

fn is_valid_symbol(symbol: &str) -> bool {
    let bytes = symbol.as_bytes();
    if bytes.len() < 3 || bytes.len() > 12 {
        return false;
    }
    for byte in bytes.iter() {
        if (*byte != 45) && (*byte < 65 || *byte > 90) && (*byte < 97 || *byte > 122) {
            return false;
        }
    }
    true
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the denomination of the vaults reserve token
    /// Return type: AssetResponse
    Asset {},
    /// Returns the total amount of underlying reserve assets that is managed by the vault
    /// Return type: TotalAssetsResponse
    TotalAssets {},
    /// Returns the current balance of shares of the given address, 0 if unset.
    /// Return type: BalanceResponse.
    Balance { address: String },
    /// Returns the amount of shares the vault would exchange for the underlying reserve asset, in the ideal scenario
    /// Return type: TODO
    ConvertToShares { assets: Vec<Coin> },
    /// Returns the amount of assets the vault would exchange for the amount of shares, in the ideal scenario
    /// Return type: ConvertToSharesResponse
    ConvertToAssets { shares: Uint128 },
    /// Returns the maximum amount of the underlying asset that can be deposited into the Vault for the receiver, through a deposit call.
    /// If None is returned in the response, the vault does not have a cap
    /// Return type: ConvertToAssetsResponse
    MaxDeposit { receiver: String },
    /// Allows an on-chain or off-chain user to simulate the effects of their deposit at the current block, given current on-chain conditions.
    /// Return type: TODO
    PreviewDeposit { assets: Uint256 },
    /// Return the maximum amount of shares that can be minted from the vault for the receiver, through a mint call
    /// Return type: TODO
    MaxMint { receiver: String },
    /// Allows an on-chain or off-chain user to simulate the effects of their mint at the current block, given current on-chain conditions.
    /// Return type: TODO
    PreviewMint { shares: Uint256 },
    /// Returns the maximum amount of the underlying asset that can be withdrawn from the owner balance in the Vault, through a withdraw call.
    /// Return type: TODO
    MaxWithdraw { owner: String },
    /// Allows an on-chain or off-chain user to simulate the effects of their withdrawal at the current block, given current on-chain conditions.
    /// Return type: TODO
    PreviewWithdraw { assets: Uint256 },
    /// Returns the maximum amount of Vault shares that can be redeemed from the owner balance in the Vault, through a redeem call.
    MaxRedeem { owner: String },
    /// Allows an on-chain or off-chain user to simulate the effects of their redeemption at the current block, given current on-chain conditions.
    PreviewRedeem { shares: Uint256 },
    /// Returns metadata on the contract - name, decimals, supply, etc.
    /// Return type: TokenInfoResponse.
    TokenInfo {},
    /// Returns the metadata on the vault - whitelist, etc.
    /// Return type: VaultInfoResponse.
    VaultInfo {},
    /// Only with "mintable" extension.
    /// Returns who can mint and the hard cap on maximum tokens after minting.
    /// Return type: MinterResponse.
    Minter {},
    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    /// Return type: AllowanceResponse.
    Allowance { owner: String, spender: String },
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this owner has approved. Supports pagination.
    /// Return type: AllAllowancesResponse.
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "enumerable" extension
    /// Returns all accounts that have balances. Supports pagination.
    /// Return type: AllAccountsResponse.
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "marketing" extension
    /// Returns more metadata on the contract to display in the client:
    /// - description, logo, project url, etc.
    /// Return type: MarketingInfoResponse
    MarketingInfo {},
    /// Only with "marketing" extension
    /// Downloads the embedded logo data (if stored on chain). Errors if no logo data is stored for this
    /// contract.
    /// Return type: DownloadLogoResponse.
    DownloadLogo {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct AssetResponse {
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TotalAssetResponse {
    pub total_managed_assets: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ConvertToSharesResponse {
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ConvertToAssetsResponse {
    pub assets: Vec<Coin>,
}

/// A None response indicates that the vault has no cap an thus no max deposit
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MaxDepositResponse {
    pub max_assets: Option<Vec<Coin>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct VaultInfoResponse {
    pub reserve_denom: String,
    pub total_supply: Uint128,
}

// we give our own ExecuteMsg instead of the cw-20 executeMsg so we can easily extend it
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Deposits assets and mints shares
    Deposit {},
    /// Burns shares from owner and sends exactly assets of underlying tokens to receiver.
    /// If amount is None, all shares are sold
    Withdraw { amount: Option<Uint128> },

    //cw20-base messages
    /// Transfer is a base message to move tokens to another account without triggering actions
    Transfer { recipient: String, amount: Uint128 },
    /// Burn is a base message to destroy tokens forever
    Burn { amount: Uint128 },
    /// Send is a base message to transfer tokens to a contract and trigger an action
    /// on the receiving contract.
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Receive is a base message to receive tokens from another contract and trigger an action on
    /// this contract. the address of the contract is stored in env.sender. Sender should match the
    /// contract we expect to handle. Sender is the original account moving the tokens. The vault
    /// needs to support sender if we expect people to be able to move their shares out of the vault.
    Receive {
        sender: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with "approval" extension. Allows spender to access an additional amount tokens
    /// from the owner's (env.sender) account. If expires is Some(), overwrites current allowance
    /// expiration with this one.
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Only with "approval" extension. Lowers the spender's access of tokens
    /// from the owner's (env.sender) account by amount. If expires is Some(), overwrites current
    /// allowance expiration with this one.
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Only with "approval" extension. Transfers amount tokens from owner -> recipient
    /// if `env.sender` has sufficient pre-approval.
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    /// Only with "approval" extension. Sends amount tokens from owner -> contract
    /// if `env.sender` has sufficient pre-approval.
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with "approval" extension. Destroys tokens forever
    BurnFrom { owner: String, amount: Uint128 },
    /// Only with the "mintable" extension. If authorized, creates amount new tokens
    /// and adds to the recipient balance.
    Mint { recipient: String, amount: Uint128 },
    /// Only with the "marketing" extension. If authorized, updates marketing metadata.
    /// Setting None/null for any of these will leave it unchanged.
    /// Setting Some("") will clear this field on the contract storage
    UpdateMarketing {
        /// A URL pointing to the project behind this token.
        project: Option<String>,
        /// A longer description of the token and it's utility. Designed for tooltips or such
        description: Option<String>,
        /// The address (if any) who can update this data structure
        marketing: Option<String>,
    },
    /// If set as the "marketing" role on the contract, upload a new URL, SVG, or PNG for the token
    UploadLogo(Logo),
}
