---
title: "QVesting"
excerpt: ""
---

# The QVesting Module

The `x/qvesting` module provides a way for managing vesting schedules in the Quasar foundation to
use `create-vesting-account` message for investors who had invested but failed to provide address at the time of
genesis.

The `x/qvesting` module is a custom implementation built on top of the built-in `x/auth/vesting` module to address the
limitations. This custom module extends the functionality of the `x/auth/vesting` module to
provide better flexibility and support for defining the start time for vesting schedules as well as implementing queries for spendable balances and iterate existing vesting accounts.

## Keeper functions

- `CreateVestingAccount()`
- `AddVestingAccount()`
- `IterateVestingAccounts()`

## State

Misc

- `GenesisState`

## Queries

- `QueryParams`
- `QueryVestingAccounts`
- `QueryQVestingAccounts`
- `QuerySpendableBalances`
- `QuerySpendableSupply`
- `QueryVestingLockedSupply`

## Events

`qvesting` module emits the following events:

Type: Attribute Key &rarr; Attribute Value
--------------------------------------------------

create_vesting_account:
- module &rarr; qvesting
- amount &rarr; {denomAmount}
- start_time &rarr; {unixTimestamp}
- end_time &rarr; {unixTimestamp}
- acc &rarr; {accAddress}