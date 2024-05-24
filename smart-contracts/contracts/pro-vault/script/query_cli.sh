

#!/bin/bash

# Check if a contract address is provided as an argument
if [ -z "$1" ]; then
  echo "Usage: $0 <contract-address> <query_json>"
  exit 1
fi

CONTRACT=$1
#NODE_URL=${2:-tcp://localhost:26679}
QUERY_MSG=${2:-"'{}'"}

echo "CONTRACT - $CONTRACT"
echo "QUERY_MSG - $QUERY_MSG"


# Function to check if the osmosisd command is available
check_osmosisd() {
  if ! command -v osmosisd &> /dev/null; then
    echo "Error: osmosisd command not found. Please install it and ensure it is in your PATH."
    exit 1
  fi
}


osmosisd q  wasm contract-state smart $CONTRACT $QUERY_MSG --node tcp://localhost:26679  
