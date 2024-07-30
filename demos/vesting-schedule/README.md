# Vesting Account Module for Quasar (x/qvesting)

- [Overview](#overview)
- [Features](#features)
- [Implementation Details](#implementation-details)
- [Testing with standard account](#testing-with-standard-account)
- [Testing with Multi-signature Transactions](#testing-with-multi-signature-transactions)
- [Testing with Ledger Hardware Wallet](#testing-with-ledger-hardware-wallet)

## Overview

The `x/qvesting` module provides a way for managing vesting schedules in the Quasar foundation to
use `create-vesting-account` message for investors who had invested but failed to provide address at the time of
genesis.

The `x/qvesting` module is a custom implementation built on top of the built-in `x/auth/vesting` module to address the
limitations identified in the POC. This custom module extends the functionality of the `x/auth/vesting` module to
provide better flexibility and support for defining the start time for vesting schedules.

-----

## Features

- Ability to define the start time for vesting schedules
- Ability to query `spendable-balances` via query CLI command with pagination
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

## Query CLI command with pagination (optionals)
```bash
quasard query qvesting spendable-balances <account_address> (--limit 1) (--count-total)
```

## Testing with standard account

#### Creating a vesting account

To create a vesting account, you can use the following command:

```bash
quasard tx qvesting create-vesting-account <account_address> <original_vesting> <start_time> <end_time> --from my_treasury --chain-id quasar --keyring-backend test
```

#### Querying vesting account information

To query the vesting account information, you can use the following command:

```bash
quasard query auth account <account_address>
```

-----

## Testing with Multi-signature Transactions

Multi-signature transactions require multiple signatures from different accounts to authorize a transaction. To test the
create-vesting-account transaction with multi-sig accounts, follow these steps:

Create multiple accounts to act as signers for the multi-sig transaction:
```bash
quasard keys add signer1 --keyring-backend test
quasard keys add signer2 --keyring-backend test
quasard keys add signer3 --keyring-backend test
```

Create a multi-sig account using the created signer accounts:
```bash
quasard keys add multisig_account --multisig-threshold 2 --multisig "signer1,signer2,signer3" --keyring-backend test
```

Fund the multi-sig account and the signer accounts.
```bash
quasard tx bank send <my_treasury_address> <multisig_account_address> 1000uqsr --from my_treasury --chain-id quasar --keyring-backend test
```

Create a create-vesting-account transaction using the multi-sig account as the signer:
```bash
quasard tx qvesting create-vesting-account <account_address> <original_vesting> <start_time> <end_time> --from multisig_account --chain-id quasar --keyring-backend test --generate-only > tx.json
```

Sign the transaction with each signer:
```bash
quasard tx sign tx.json --from signer1 --multisig <multisig_account_address> --chain-id quasar --keyring-backend test --output-document tx_signed1.json
quasard tx sign tx.json --from signer2 --multisig <multisig_account_address> --chain-id quasar --keyring-backend test --output-document tx_signed2.json
quasard tx sign tx.json --from signer3 --multisig <multisig_account_address> --chain-id quasar --keyring-backend test --output-document tx_signed3.json
```

Assemble the signatures and broadcast the transaction:
```bash
quasard tx multisign tx.json multisig_account tx_signed1.json tx_signed2.json tx_signed3.json --chain-id quasar --keyring-backend test > tx_multisig.json
quasard tx broadcast tx_multisig.json --chain-id quasar --keyring-backend test -y
```

Verify the custom vesting account has been created successfully:
```bash
quasard query auth account <account_address>
```

## Testing with Ledger Hardware Wallet

To test the compatibility of the x/qvesting module with Ledger hardware wallets, follow these steps:

Connect your Ledger device and ensure the Cosmos app is installed and running.

Retrieve the Ledger account address and adding the key to the keyring:
```bash
quasard keys add ledger_account --ledger
```

Fund the Ledger account.

Create a create-vesting-account transaction using the Ledger account as the signer:
```bash
quasard tx qvesting create-vesting-account <account_address> <original_vesting> <start_time> <end_time> --from ledger_account --chain-id quasar --keyring-backend test --generate-only > tx.json
```

Sign the transaction with the Ledger device:
```bash
quasard tx sign tx.json --from ledger_account --chain-id quasar --keyring-backend test --output-document tx_signed.json
```

Broadcast the signed transaction:
```bash
quasard tx broadcast tx_signed.json --chain-id quasar
```

Verify the custom vesting account has been created successfully:
```bash
quasard query auth account <account_address>
```

By performing these tests, you can ensure that the x/qvesting module is compatible with multi-signature
transactions and Ledger hardware wallets, providing a secure and reliable experience for all users.