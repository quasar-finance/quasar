# CL Vault Scripts

This directory contains scripts for managing CL vault operations.

## 1. Query CL Vault Users Script

This script queries all users from a CL vault contract and filters out users with 0 shares.

### Usage

Run the script using npx tsx:

```bash
# Basic usage (queries Osmosis mainnet by default)
CONTRACT_ADDRESS=<your_vault_contract_address> npx tsx smart-contracts/osmosis/scripts/query-cl-vault-users.ts

# With custom RPC endpoint
RPC_ENDPOINT=https://rpc.osmosis.zone CONTRACT_ADDRESS=<your_vault_contract_address> npx tsx smart-contracts/osmosis/scripts/query-cl-vault-users.ts

# Save results to a JSON file
CONTRACT_ADDRESS=<your_vault_contract_address> OUTPUT_FILE=active_users.json npx tsx smart-contracts/osmosis/scripts/query-cl-vault-users.ts
```

### Environment Variables

- `CONTRACT_ADDRESS` (required): The address of the CL vault contract
- `RPC_ENDPOINT` (optional): The RPC endpoint to use (defaults to https://rpc.osmosis.zone)
- `OUTPUT_FILE` (optional): If provided, saves the results to a JSON file

### Output

The script will:
1. Query all users from the vault using pagination
2. Filter out users with 0 shares
3. Display the results in the console
4. Optionally save to a JSON file with the following structure:

```json
{
  "total_users": 1000,
  "active_users": 750,
  "filtered_users": 250,
  "users": [
    {
      "address": "osmo1...",
      "shares": "1000000"
    },
    ...
  ]
}
```

## 2. Generate AutoWithdraw Message Script

After querying active users, this script generates the AutoWithdraw execute message for the CL vault contract.

### Usage

```bash
# Generate from default active_users.json file
npx tsx smart-contracts/osmosis/scripts/generate-autowithdraw-msg.ts

# Specify custom input/output files
INPUT_FILE=my_users.json OUTPUT_FILE=my_withdraw_msg.json npx tsx smart-contracts/osmosis/scripts/generate-autowithdraw-msg.ts
```

### Environment Variables

- `INPUT_FILE` (optional): The input file with active users data (defaults to `active_users.json`)
- `OUTPUT_FILE` (optional): The output file for the execute message (defaults to `autowithdraw_msg.json`)

### Output

The script generates a JSON file with the properly formatted execute message:

```json
{
  "extension": {
    "admin": {
      "auto_withdraw": {
        "users": [
          ["osmo1...", "1000000"],
          ["osmo2...", "2000000"]
        ]
      }
    }
  }
}
```

This JSON can be directly used in a transaction to execute the AutoWithdraw function on the CL vault contract.