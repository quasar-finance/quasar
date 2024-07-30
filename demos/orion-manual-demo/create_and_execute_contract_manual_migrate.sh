#!/bin/sh

set -e


CHAIN_ID="quasar"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
RPC="http://127.0.0.1:26659"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"
echo $NODE
#     lock_period is  60 for 60 sec, in prod should be at least: 60 sec/min * 60 min/hr * 24hr * 14days "1209600"
#     pool_id is hardcoded to 1 for this testing setup, expected to be done by the instantiater on local/testnet
#     pool_denom should be looked up and hardcoded aswell
#     base_denom: base_denom should be the denom of the token on osmosos, for now uosmo
#     local_denom: the denom of the token used locally, in this testing case: the denom of the path transfer/channel-1/uosmo
#     quote_denom is the denom other denom in the pool, stake for now
INIT1='{"lock_period":6,"pool_id":1,"pool_denom":"gamm/pool/1","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"stake","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
# INIT2='{"lock_period":6,"pool_id":3,"pool_denom":"gamm/pool/3","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"fakestake","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
# INIT3='{"lock_period":6,"pool_id":3,"pool_denom":"gamm/pool/3","base_denom":"fakestake","local_denom":"ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1","quote_denom":"uosmo","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'

cd ../../smart-contracts

docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.11

echo "Running store code"
RES=$(quasard tx wasm store artifacts/lp_strategy.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
OUT1=$(quasard tx wasm instantiate $CODE_ID "$INIT1" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec $NODE --chain-id $CHAIN_ID)
ADDR1=$(quasard query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR1"

rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

# quasard tx wasm execute $ADDR1 '{"bond":{"id": "my-id"}}' -y --from alice --keyring-backend test --amount 1000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518 $TXFLAG
# sleep 6

# OUT2=$(quasard tx wasm instantiate $CODE_ID "$INIT2" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec $NODE --chain-id $CHAIN_ID)
# ADDR2=$(quasard query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[1]')
# echo "Got address of deployed contract = $ADDR2"

# rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icqhost --order unordered --version icq-1 --override
# rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

# sleep 6

# OUT3=$(quasard tx wasm instantiate $CODE_ID "$INIT3" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec $NODE --chain-id $CHAIN_ID)
# ADDR3=$(quasard query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[2]')
# echo "Got address of deployed contract = $ADDR3"

# rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icqhost --order unordered --version icq-1 --override
# rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

# echo "primitive contracts:\n$ADDR1\n$ADDR2\n$ADDR3\n"

# BOND=$(printf 'quasard tx wasm execute %s "{\"bond\":{\"id\":\"my-id\"}}" -y --from alice --keyring-backend test --gas-prices 10uqsr --gas auto --gas-adjustment 1.3 --node http://127.0.0.1:26659 --chain-id quasar --amount 1000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518\n' $ADDR)
# echo "bonding tokens, to replay: "
# echo $BOND
# $BOND

RES=$(quasard tx wasm store artifacts/vault_rewards.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)
VR_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')

VAULT_INIT='{"total_cap":"200000000000","thesis":"yurmom","vault_rewards_code_id":'$VR_CODE_ID',"reward_token":{"native":"uqsr"},"reward_distribution_schedules":[],"decimals":6,"symbol":"ORN","min_withdrawal":"1","name":"ORION","primitives":[{"address":"'$ADDR1'","weight":"0.5","init":{"l_p":'$INIT1'}}]}'
# VAULT_INIT='{"total_cap":"200000000000","thesis":"yurmom","vault_rewards_code_id":'$VR_CODE_ID',"reward_token":{"native":"uqsr"},"reward_distribution_schedules":[],"decimals":6,"symbol":"ORN","min_withdrawal":"1","name":"ORION","primitives":[{"address":"'$ADDR1'","weight":"0.5","init":{"l_p":'$INIT1'}},{"address":"'$ADDR2'","weight":"0.5","init":{"l_p":'$INIT2'}}]}'
#,{"address":"'$ADDR2'","weight":"0.333333333333","init":{"l_p":'$INIT2'}},{"address":"'$ADDR3'","weight":"0.333333333333","init":{"l_p":'$INIT3'}}]}'
echo $VAULT_INIT

echo "Running store code (vault)"
RES=$(quasard tx wasm store artifacts/basic_vault.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)

VAULT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')

echo "Got CODE_ID = $VAULT_CODE_ID"

echo "Deploying contract (vault)"
# # swallow output
OUT=$(quasard tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec $NODE --chain-id $CHAIN_ID)
VAULT_ADDR=$(quasard query wasm list-contract-by-code $VAULT_CODE_ID --output json $NODE | jq -r '.contracts[0]')

echo "Got address of deployed contract = $VAULT_ADDR (vault)"

echo "setting depositor"
sleep 6
quasard tx wasm execute $ADDR1 '{"set_depositor":{"depositor":"'$VAULT_ADDR'"}}' --from alice --keyring-backend test $TXFLAG -y -b block
# sleep 6
# quasard tx wasm execute $ADDR2 '{"set_depositor":{"depositor":"'$VAULT_ADDR'"}}' --from alice --keyring-backend test $TXFLAG -y

echo "VAULT: $VAULT_ADDR"
# echo "Seeding liquidity"
# quasard tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from user1 --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1
# sleep 5

echo "bonding multiple bonds"
# echo "Command: quasard tx wasm execute $VAULT_ADDR '{\"bond\":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1"
quasard tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1 -b block > ../demos/orion-manual-demo/logs/bonds.log
quasard tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1100ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1 -b block >> ../demos/orion-manual-demo/logs/bonds.log
# echo "third bond"
quasard tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1200ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1 -b block >> ../demos/orion-manual-demo/logs/bonds.log
# quasard tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from bob --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1300ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1 -b block >> ./logs/bond.log
# quasard tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from bob --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1400ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1 -b block > ./logs/bond.log
# quasard tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from bob --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1500ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1 -b block > ./logs/bond.log


# echo "Sleeping for 80 seconds"
# sleep 80

echo "Querying alice balance"
quasard query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"}}' --output json

# echo "Running unbond, command: quasard tx wasm execute $VAULT_ADDR '{\"unbond\":{\"amount\":\"100\"}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID"
# quasard tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"100"}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID || true
# echo $(quasard tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"100"}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID)

echo "Alice should not be able to unbond, since we are not relaying and leaving the bonds hanging"
echo "Migrating to manually satisfy the bonds"

MIGRATE1="{\"vault_addr\":\"$VAULT_ADDR\", \"callbacks\": [{\"share_amount\": \"1000\",\"bond_id\":\"1\"}]}"
# satisfy 1 bond id, alice should have a 1000 shares now
echo "quasard tx wasm migrate $ADDR $CODE_ID "$MIGRATE1" -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID"
quasard tx wasm migrate $ADDR1 $CODE_ID "$MIGRATE1" -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID

echo "Querying alice balance again, expect 1000 shares"
quasard query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"}}' --output json

# satisfy 2 bond id, alice should have a 3300 shares now
MIGRATE2="{\"vault_addr\":\"$VAULT_ADDR\", \"callbacks\": [{\"share_amount\": \"1100\",\"bond_id\":\"2\"}, {\"share_amount\": \"1200\",\"bond_id\":\"3\"}]}"
quasard tx wasm migrate $ADDR1 $CODE_ID "$MIGRATE2" -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID

echo "Querying alice balance again, expect 3300 shares"
quasard query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"}}' --output json

# trying to satisfy more bonds here should fail
quasard tx wasm migrate $ADDR1 $CODE_ID "{\"vault_addr\":\"$VAULT_ADDR\", \"callbacks\": [{\"share_amount\": \"1100\",\"bond_id\":\"2\"}, {\"share_amount\": \"1200\",\"bond_id\":\"3\"}]}" -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID || true

# echo "Sleeping 120"
# sleep 120
# echo "Running unbond again to get funds back"
# quasard tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"0"}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID

# echo "Sleeping 60"
# echo "Alice balances after unbond step:"
# quasard query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"}}' --output json
# quasard q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec

cd -
