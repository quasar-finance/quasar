# Burn Coins Contract

## Overview

The Burn Coins Contract is a smart contract designed for the CosmWasm framework. Its primary function is to burn the coins sent to the contract by users.

## Purpose

The purpose of this contract is straightforward: to permanently remove the coins that users send to it. This can be useful in scenarios where token burning is required, such as reducing the supply of a token to increase its scarcity and value, or as part of a token redistribution mechanism.

## Features

- **Coin Burning**: Users can send coins to the contract, and the contract will permanently remove these coins from circulation.

## Usage

1. **Instantiate the Contract**: The contract needs to be instantiated before it can be used. During instantiation, the contract version is set in storage, and the `AMOUNT_BURNT` storage item is initialized with an empty vector.

2. **Execute Coin Burning**: Users can execute the coin burning functionality by sending coins to the contract using the `Burn` execute message. Upon receiving coins, the contract will burn them, i.e., permanently remove them from circulation.

3. **Query Total Burnt Amount**: Users can query the total amount of coins burnt using the `TotalBurntQuery` query message. This allows users to retrieve information about the total amount of coins burnt since the contract was instantiated.

## Getting Started

To deploy and interact with the Burn Coins Contract, follow these steps:

1. **Setup Environment**: Make sure you have the necessary tools and environment set up for CosmWasm contract development. Refer to the CosmWasm documentation for instructions on setting up your development environment.

2. **Compile Contract**: Compile the Burn Coins Contract using the CosmWasm compiler.

3. **Instantiate Contract**: Instantiate the contract on the desired blockchain network. During instantiation, provide any required initialization parameters.

4. **Interact with the Contract**: Users can interact with the contract by executing the coin burning functionality and querying the total burnt amount as needed.

## Example

Here's an example of how to interact with the Burn Coins Contract:

1. **Instantiate the Contract**:
    ```
    quasarnoded tx wasm instantiate <vault code ID> "" --from <account-name> --keyring-backend <name-of-keyring> --label "burn coins contract" --gas auto --fees <amount-denom> -b block -y --admin <admin-address>
    ```

2. **Execute Coin Burning**:
    ```
    quasarnoded tx wasm execute <contract-address> '{"burn":{}}' -y --from <account-name> --keyring-backend <name-of-keyring> --gas auto --fees <amount-denom> --chain-id <chain-id> --amount <denom-amount,denom-amount>
    ```

3. **Query Total Burnt Amount**:
    ```
    # quasarnoded query wasm contract-state smart <contract-address> '{"total_burnt_query":{}}' --output json
    ```

## Considerations

- **Irreversible**: Coin burning is irreversible. Once coins are burnt, they cannot be recovered.
- **Auditability**: Users should carefully review the contract code and ensure its correctness before interacting with it.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute the code as per the terms of the license.

## Disclaimer

This software is provided as-is, without any express or implied warranty. Use it at your own risk. The authors and contributors of this project are not liable for any damages or losses arising from its use.

---
By [Your Name/Team Name]