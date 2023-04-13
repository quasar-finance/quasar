# x/vestingcustom: Custom Vesting Module for Quasar

## Overview

The `x/vestingcustom` module is a custom implementation built on top of the built-in `x/auth/vesting` module to address the
limitations identified in the POC. This custom module extends the functionality of the `x/auth/vesting` module to provide
better flexibility and support for defining the start time for vesting schedules.

### Features

- Ability to define the start time for vesting schedules
- Compatibility with the built-in `x/auth/vesting` module
- Integration with other modules, such as governance, distribution, and staking

### Usage

#### Creating a custom vesting account

To create a custom vesting account, you can use the following command:

```bash
quasarnoded tx vestingcustom create-vesting-account <account_address> <original_vesting> <start_time> <end_time> --from my_treasury --chain-id quasarnode --keyring-backend test
```

For a delayed vesting account, use the `--delayed` flag:

```bash
quasarnoded tx vestingcustom create-vesting-account <account_address> <original_vesting> <start_time> <end_time> --delayed --from my_treasury --chain-id quasarnode --keyring-backend test
```

#### Querying custom vesting account information

To query the custom vesting account information, you can use the following command:

```bash
quasarnoded query auth account <account_address>
```

#### Interacting with custom vesting accounts

Custom vesting accounts can interact with other modules in the same way as regular accounts. For example, you can
delegate, undelegate, or redelegate tokens to validators, participate in governance proposals, or claim rewards from the
distribution module.

### Implementation Details

The `x/vestingcustom` module is implemented as a wrapper around the `x/auth/vesting` module, with additional functionality
for custom start times. It achieves this by using the underlying x/auth KVStore, which allows for the deprecation of the
custom module once the project upgrades to version 0.47 of the Cosmos SDK, if desired.

When a transaction is processed involving a custom vesting account, the module checks the account's vesting schedule and
updates the account's locked tokens accordingly. This allows the custom vesting module to maintain compatibility with
other modules in the Cosmos SDK while still providing the desired vesting functionality.

### Future Work

The `x/vestingcustom` module provides a foundation for managing vesting schedules in the Quasar blockchain project. In
the future, further enhancements and optimizations can be explored, such as implementing periodic vesting, which has
been marked as a nice-to-have feature. Our investors are likely to use the `create-vesting-account` command with a future
`start_time` for continuous vesting, which is released using the same formula included in the POC.

By implementing this custom vesting module, we can provide better flexibility and control over token release schedules
for investors, improving the overall security and long-term viability of the Quasar blockchain project.