# Vesting Account Module for Quasar (x/qvesting)

## Overview

The `x/qvesting` module provides a way for managing vesting schedules in the Quasar foundation to
use `create-vesting-account` message for investors who had invested but failed to provide address at the time of
genesis.

The `x/qvesting` module is a custom implementation built on top of the built-in `x/auth/vesting` module to address the
limitations identified in the POC. This custom module extends the functionality of the `x/auth/vesting` module to
provide better flexibility and support for defining the start time for vesting schedules.

-----

### Features

- Ability to define the start time for vesting schedules
- Compatibility with the built-in `x/auth/vesting` module
- Integration with other modules, such as governance, distribution, and staking

### Implementation Details

The `x/qvesting` module is implemented as a wrapper around the `x/auth/vesting` module, with additional functionality
for start_time. It achieves this by using the underlying x/auth KVStore, which allows for the deprecation of the module
once the project upgrades to version 0.47 of the Cosmos SDK, if desired.

When a transaction is processed involving a vesting account, the module checks the account's vesting schedule and
updates the account's locked tokens accordingly. This allows the vesting module to maintain compatibility with
other modules in the Cosmos SDK while still providing the desired vesting functionality.

Quasar vesting accounts can interact with other modules in the same way as regular accounts. For example, you can
delegate, undelegate, or redelegate tokens to validators, participate in governance proposals, or claim rewards from the
distribution module.

-----

### Usage

#### Creating a vesting account

To create a vesting account, you can use the following command:

```bash
quasarnoded tx qvesting create-vesting-account <account_address> <original_vesting> <start_time> <end_time> --from my_treasury --chain-id quasarnode --keyring-backend test
```

#### Querying vesting account information

To query the vesting account information, you can use the following command:

```bash
quasarnoded query auth account <account_address>
```