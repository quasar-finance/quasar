#!/bin/bash

# Check if a contract address is provided as an argument
if [ -z "$1" ]; then
  echo "Usage: $0 <contract-address>"
  exit 1
fi

CONTRACT=$1
NODE_URL=${2:-tcp://localhost:26679}

# Function to check if the osmosisd command is available
check_osmosisd() {
  if ! command -v osmosisd &> /dev/null; then
    echo "Error: osmosisd command not found. Please install it and ensure it is in your PATH."
    exit 1
  fi
}

# Function to query the contract state
query_contract_state() {
  local contract=$1
  local node_url=$2

  echo "Querying contract state for contract: $contract on node: $node_url"
  osmosisd q wasm contract-state all "$contract" --node "$node_url"
  
  if [ $? -ne 0 ]; then
    echo "Error: Failed to query contract state. Please check the contract address and node URL."
    exit 1
  fi
}

# Main script execution
check_osmosisd
query_contract_state "$CONTRACT" "$NODE_URL"
