/// The lockup extension can be used to create vaults where the vault tokens are
/// not immediately reedemable. Instead of normally calling the
/// `VaultStandardExecuteMsg::Redeem` variant, the user has to call the `Unlock`
/// variant on the Lockup extension `ExecuteMsg` and wait for a specified period
/// of time before they can withdraw their base tokens via the
/// `WithdrawUnlocked` variant.
#[cfg(feature = "lockup")]
#[cfg_attr(docsrs, doc(cfg(feature = "lockup")))]
pub mod lockup;

/// The force unlock extension can be used to create a vault that also
/// implements the `Lockup` extension, but where some whitelisted addresses are
/// allowed to call the `ForceUnlock` variant on the extension `ExecuteMsg` and
/// immediately unlock the vault tokens of the specified user. This is useful if
/// the vault is used with leverage and a liquidator needs to be able to
/// liquidate the tokens locked in the vault.
#[cfg(feature = "force-unlock")]
#[cfg_attr(docsrs, doc(cfg(feature = "force-unlock")))]
pub mod force_unlock;

/// The keeper extension can be used to add functionality for either whitelisted
/// addresses or anyone to act as a "keeper" for the vault and call functions to
/// perform jobs that need to be done to keep the vault running.
#[cfg(feature = "keeper")]
#[cfg_attr(docsrs, doc(cfg(feature = "keeper")))]
pub mod keeper;

/// The Cw4626 extension is the only extension provided with in this repo that
/// does not extend the standard `ExecuteMsg` and `QueryMsg` enums with by
/// putting its variants inside of a `VaultExtension` variant. Instead it adds
/// more variants at the top level, namely the variants from the [CW20
/// standard](https://github.com/CosmWasm/cw-plus/tree/main/packages/cw20) This
/// is inspired by the [ERC-4626 standard on
/// Ethereum](https://ethereum.org/en/developers/docs/standards/tokens/erc-4626/)
/// and allows the vault to, instead of using a Cosmos native token as the vault
/// token, have the vault contract be it's own vault token by also implementing
/// the CW20 standard. This is useful if you are writing a vault on a chain that
/// does not yet have the [TokenFactory
/// module](https://github.com/CosmWasm/token-factory) available and can
/// therefore not issue a Cosmos native token as the vault token.
#[cfg(feature = "cw4626")]
#[cfg_attr(docsrs, doc(cfg(feature = "cw4626")))]
pub mod cw4626;
