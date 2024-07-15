use crate::msg::{
    ExtensionExecuteMsg, ExtensionQueryMsg, VaultInfoResponse, VaultStandardInfoResponse,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, Coin, CosmosMsg, Empty, StdResult, Uint128, WasmMsg};
use cw20::{
    AllAccountsResponse, AllAllowancesResponse, AllowanceResponse, BalanceResponse,
    DownloadLogoResponse, MarketingInfoResponse, TokenInfoResponse,
};
use cw20::{Expiration, Logo};
use schemars::JsonSchema;

// TODO update for multi asset support

/// The default ExecuteMsg variants that a vault using the Cw4626 extension must
/// implement. This includes all of the variants from the default
/// VaultStandardExecuteMsg, plus the variants from the CW20 standard. This enum
/// can be extended with additional variants by defining an extension enum and
/// then passing it as the generic argument `T` to this enum.
#[cw_serde]
pub enum Cw4626ExecuteMsg<T = ExtensionExecuteMsg> {
    //--------------------------------------------------------------------------
    // Standard CW20 ExecuteMsgs
    //--------------------------------------------------------------------------
    /// Transfer is a base message to move tokens to another account without
    /// triggering actions
    Transfer { recipient: String, amount: Uint128 },
    /// Send is a base message to transfer tokens to a contract and trigger an
    /// action on the receiving contract.
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with "approval" extension. Allows spender to access an additional
    /// amount tokens from the owner's (env.sender) account. If expires is
    /// Some(), overwrites current allowance expiration with this one.
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Only with "approval" extension. Lowers the spender's access of tokens
    /// from the owner's (env.sender) account by amount. If expires is Some(),
    /// overwrites current allowance expiration with this one.
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Only with "approval" extension. Transfers amount tokens from owner ->
    /// recipient if `env.sender` has sufficient pre-approval.
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    /// Only with "approval" extension. Sends amount tokens from owner ->
    /// contract if `env.sender` has sufficient pre-approval.
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with the "marketing" extension. If authorized, updates marketing
    /// metadata. Setting None/null for any of these will leave it
    /// unchanged. Setting Some("") will clear this field on the contract
    /// storage
    UpdateMarketing {
        /// A URL pointing to the project behind this token.
        project: Option<String>,
        /// A longer description of the token and it's utility. Designed for
        /// tooltips or such
        description: Option<String>,
        /// The address (if any) who can update this data structure
        marketing: Option<String>,
    },
    /// If set as the "marketing" role on the contract, upload a new URL, SVG,
    /// or PNG for the token
    UploadLogo(Logo),

    //--------------------------------------------------------------------------
    // Vault Standard ExecuteMsgs
    //--------------------------------------------------------------------------
    /// Called to deposit into the vault. Native assets are passed in the funds
    /// parameter.
    Deposit {
        /// The amount of base tokens to deposit
        amount: Uint128,
        /// An optional field containing the recipient of the vault token. If
        /// not set, the caller address will be used instead.
        recipient: Option<String>,
    },

    /// Called to redeem vault tokens and receive assets back from the vault.
    /// The native vault token must be passed in the funds parameter, unless the
    /// lockup extension is called, in which case the vault token has already
    /// been passed to ExecuteMsg::Unlock.
    Redeem {
        /// Amount of vault tokens to redeem
        amount: Uint128,
        /// An optional field containing which address should receive the
        /// withdrawn base tokens. If not set, the caller address will
        /// be used instead.
        recipient: Option<String>,
    },

    /// Called to execute functionality of any enabled extensions.
    VaultExtension(T),
}

impl Cw4626ExecuteMsg {
    /// Convert a [`Cw4626ExecuteMsg`] into a [`CosmosMsg`].
    pub fn into_cosmos_msg(self, contract_addr: String, funds: Vec<Coin>) -> StdResult<CosmosMsg> {
        Ok(WasmMsg::Execute {
            contract_addr,
            msg: to_json_binary(&self)?,
            funds,
        }
        .into())
    }
}

/// The default QueryMsg variants that a vault using the Cw4626 extension must
/// implement. This includes all of the variants from the default
/// VaultStandardQueryMsg, plus the variants from the CW20 standard. This enum
/// can be extended with additional variants by defining an extension enum and
/// then passing it as the generic argument `T` to this enum.
#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw4626QueryMsg<T = ExtensionQueryMsg>
where
    T: JsonSchema,
{
    //--------------------------------------------------------------------------
    // Standard CW20 QueryMsgs
    //--------------------------------------------------------------------------
    /// Returns the current balance of the given address, 0 if unset.
    /// Return type: BalanceResponse.
    #[returns(BalanceResponse)]
    Balance { address: String },
    /// Returns metadata on the contract - name, decimals, supply, etc.
    /// Return type: TokenInfoResponse.
    #[returns(TokenInfoResponse)]
    TokenInfo {},
    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    /// Return type: AllowanceResponse.
    #[returns(AllowanceResponse)]
    Allowance { owner: String, spender: String },
    /// Only with "marketing" extension
    /// Returns more metadata on the contract to display in the client:
    /// - description, logo, project url, etc.
    /// Return type: MarketingInfoResponse.
    #[returns(MarketingInfoResponse)]
    MarketingInfo {},
    /// Only with "marketing" extension
    /// Downloads the embedded logo data (if stored on chain). Errors if no logo
    /// data stored for this contract.
    /// Return type: DownloadLogoResponse.
    #[returns(DownloadLogoResponse)]
    DownloadLogo {},
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this owner has approved. Supports pagination.
    /// Return type: AllAllowancesResponse.
    #[returns(AllAllowancesResponse)]
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "enumerable" extension
    /// Returns all accounts that have balances. Supports pagination.
    /// Return type: AllAccountsResponse.
    #[returns(AllAccountsResponse)]
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    //--------------------------------------------------------------------------
    // Vault Standard QueryMsgs
    //--------------------------------------------------------------------------
    /// Returns `VaultStandardInfoResponse` with information on the version of
    /// the vault standard used as well as any enabled extensions.
    #[returns(VaultStandardInfoResponse)]
    VaultStandardInfo {},

    /// Returns `VaultInfoResponse` representing vault requirements, lockup, &
    /// vault token denom.
    #[returns(VaultInfoResponse)]
    Info {},

    /// Returns `Uint128` amount of vault tokens that will be returned for the
    /// passed in `amount` of base tokens.
    ///
    /// Allows an on-chain or off-chain user to simulate the effects of their
    /// deposit at the current block, given current on-chain conditions.
    ///
    /// Must return as close to and no more than the exact amount of vault
    /// tokens that would be minted in a deposit call in the same transaction.
    /// I.e. Deposit should return the same or more vault tokens as
    /// PreviewDeposit if called in the same transaction.
    #[returns(Uint128)]
    PreviewDeposit {
        /// The amount of base tokens to preview depositing.
        amount: Uint128,
    },

    /// Returns `Uint128` amount of base tokens that would be withdrawn in
    /// exchange for redeeming `amount` of vault tokens.
    ///
    /// Allows an on-chain or off-chain user to simulate the effects of their
    /// redeem at the current block, given current on-chain conditions.
    ///
    /// Must return as close to and no more than the exact amount of base tokens
    /// that would be withdrawn in a redeem call in the same transaction.
    #[returns(Uint128)]
    PreviewRedeem {
        /// The amount of vault tokens to preview redeeming.
        amount: Uint128,
    },

    /// Returns the amount of assets managed by the vault denominated in base
    /// tokens. Useful for display purposes, and does not have to confer the
    /// exact amount of base tokens.
    #[returns(Uint128)]
    TotalAssets {},

    /// Returns `Uint128` total amount of vault tokens in circulation.
    #[returns(Uint128)]
    TotalVaultTokenSupply {},

    /// The amount of vault tokens that the vault would exchange for the amount
    /// of assets provided, in an ideal scenario where all the conditions
    /// are met.
    ///
    /// Useful for display purposes and does not have to confer the exact amount
    /// of vault tokens returned by the vault if the passed in assets were
    /// deposited. This calculation should not reflect the "per-user"
    /// price-per-share, and instead should reflect the "average-user’s"
    /// price-per-share, meaning what the average user should expect to see
    /// when exchanging to and from.
    #[returns(Uint128)]
    ConvertToShares {
        /// The amount of base tokens to convert to vault tokens.
        amount: Uint128,
    },

    /// Returns the amount of base tokens that the Vault would exchange for
    /// the `amount` of vault tokens provided, in an ideal scenario where all
    /// the conditions are met.
    ///
    /// Useful for display purposes and does not have to confer the exact amount
    /// of assets returned by the vault if the passed in vault tokens were
    /// redeemed. This calculation should not reflect the "per-user"
    /// price-per-share, and instead should reflect the "average-user’s"
    /// price-per-share, meaning what the average user should expect to see
    /// when exchanging to and from.
    #[returns(Uint128)]
    ConvertToAssets {
        /// The amount of vault tokens to convert to base tokens.
        amount: Uint128,
    },

    /// Handle queries of any enabled extensions.
    #[returns(Empty)]
    VaultExtension(T),
}
