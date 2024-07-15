# CosmWasm Vault Standard

A standard interface for tokenized vaults written in CosmWasm. This repo contains a set of `ExecuteMsg` and `QueryMsg` variants that should be implemented by a vault contract that adheres to this standard.

## Vault Standard Fundamentals
There are a few things to know about the vault standard:
* Each vault has one specific token that is used for deposits, withdrawals and accounting. This token is called the `base token`.
* Each vault has a `vault token` that represents the users share in the vault. The number of vault tokens the user receives should be based on the the number of base tokens they deposit.

## How to create a vault contract that adheres to this standard

To create a vault contract that adheres to the standard, all you need to do is import the `VaultStandardExecuteMsg` and `VaultStandardQueryMsg` enums and use them in the entrypoints of your contracts.

The `VaultStandardExecuteMsg` and `VaultStandardQueryMsg` enums define a set of variants that should be enough to cover most vault contract use cases, and all vaults that adhere to the standard must implement all of the provided default variants. If however your use case requires additional variants, please see the section on [how to use extensions](#how-to-use-extensions).


## Description and specification of ExecuteMsg variants
Please refer to the [API docs](https://docs.rs/cw-vault-standard) for a complete description of each variant.

## How to use Extensions

If the standard set of `ExecuteMsg` and `QueryMsg` variants are not enough for your use case, you can include additional ones by defining an extension. The preferred way to do this is by creating a new enum that extends the exported `VaultStandardExecuteMsg` and `VaultStandardQueryMsg` enums. For example:

```rust
pub enum MyExtensionExecuteMsg {
    MyVariant1 { ... },
    MyVariant2 { ... },
}
```
This enum can then be included in an enum with all the Extensions that your vault uses and then be passed in as the generic argument `T` to `VaultStandardExecuteMsg<T>`. For example:

```rust
pub enum ExtensionExecuteMsg {
    MyExtension(MyExtensionExecuteMsg),
    Lockup(LockupExecuteMsg),
}

pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;
```

Now you can use the `ExecuteMsg` enum in your contract entrypoints instead of the default `VaultStandardExecuteMsg` enum.

## Included Extensions

The following extensions are included in this repo:
* [Lockup](src/extensions/lockup.rs)
* [ForceUnlock](src/extensions/force_unlock.rs)
* [Keeper](src/extensions/keeper.rs)
* [Cw4626](src/extensions/cw4626.rs)

Each of these extensions are available in this repo via cargo features. To use them, you can import the crate with a feature flag like this:

```toml
cw-vault-standard = { version = "0.2.0", features = ["lockup", "force_unlock"] }
```

A short description of each extension can be found below.

### Lockup
The lockup extension can be used to create vaults where the vault tokens are not immediately reedemable. Instead of normally calling the `VaultStandardExecuteMsg::Redeem` variant, the user has to call the `Unlock` variant on the Lockup extension `ExecuteMsg` and wait for a specified period of time before they can withdraw their base tokens via the `WithdrawUnlocked` variant.

### ForceUnlock
The force unlock extension can be used to create a vault that also implements the `Lockup` extension, but where some whitelisted addresses are allowed to call the `ForceUnlock` variant on the extension `ExecuteMsg` and immediately unlock the vault tokens of the specified user. This is useful if the vault is used  with leverage and a liquidator needs to be able to liquidate the tokens locked in the vault.

### Keeper
The keeper extension can be used to add functionality for either whitelisted addresses or anyone to act as a "keeper" for the vault and call functions to perform jobs that need to be done to keep the vault running.

### Cw4626
The Cw4626 extension is the only extension provided with in this repo that does not extend the standard `VaultStandardExecuteMsg` and `VaultStandardQueryMsg` enums by putting its variants inside of a `VaultExtension` variant. Instead it adds more variants at the top level, namely the variants from the [CW20 standard](https://github.com/CosmWasm/cw-plus/tree/main/packages/cw20) This is inspired by the [ERC-4626 standard on Ethereum](https://ethereum.org/en/developers/docs/standards/tokens/erc-4626/) and allows the vault to, instead of using a Cosmos native token as the vault token, have the vault contract be it's own vault token by also implementing the CW20 standard. This is useful if you are writing a vault on a chain that does not yet have the [TokenFactory module](https://github.com/CosmWasm/token-factory) available and can therefore not issue a Cosmos native token as the vault token.
