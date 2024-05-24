#!/bin/bash
set +x
CONTRACT=$1
#NODE_URL=${2:-tcp://localhost:26679}
QUERY_MSG=${2:-"'{"get_vault_config": {}}'"}

#osmosisd q  wasm contract-state smart $CONTRACT $QUERY_MSG  --node tcp://localhost:26679 --output json
osmosisd q  wasm contract-state smart $CONTRACT '{"get_vault_config": {}}' --node tcp://localhost:26679 --output json
osmosisd q  wasm contract-state smart $CONTRACT '{"get_vault_running_state": {}}' --node tcp://localhost:26679 --output json
#osmosisd query wasm contract-state smart $1 '{"get_vault_config": {}}' --node $2 --output json

