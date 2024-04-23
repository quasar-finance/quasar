#!/bin/sh

BINARY=quasarnoded
HOME_QSR=$HOME/.quasarnode
CHAIN_ID=quasar

INIT1='{"lock_period":6,"pool_id":1,"pool_denom":"gamm/pool/1","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"stake","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
INIT2='{"lock_period":6,"pool_id":3,"pool_denom":"gamm/pool/3","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"fakestake","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
INIT3='{"lock_period":6,"pool_id":2,"pool_denom":"gamm/pool/2","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"usdc","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'

cd ../../smart-contracts

#docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer-arm64:0.12.11

platform='unknown'
unamestr=$(uname)
if [ "$unamestr" = 'Linux' ]; then
    platform='linux'
elif [ "$unamestr" = 'Darwin' ]; then
    platform='macos'
fi

echo "Running store code"
if [ $platform = 'macos' ]; then
    RES=$(quasarnoded tx wasm store artifacts/lp_strategy-aarch64.wasm --from alice --keyring-backend test -y --output json -b block --chain-id $CHAIN_ID --gas auto --fees 10000uqsr)
elif [ $platform = 'linux' ]; then
    RES=$(quasarnoded tx wasm store artifacts/lp_strategy.wasm --from alice --keyring-backend test -y --output json -b block --chain-id $CHAIN_ID --gas auto --fees 10000uqsr)
fi
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
OUT1=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT1" --from alice --keyring-backend test --label "my first contract" --gas auto --fees 10000uqsr -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --chain-id $CHAIN_ID)
ADDR1=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR1"

ADDR1=quasar14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sy9numu
rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

OUT2=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT2" --from alice --keyring-backend test --label "my first contract" --gas auto --fees 10000uqsr -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --chain-id $CHAIN_ID)
ADDR2=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[1]')
echo "Got address of deployed contract = $ADDR2"

rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

# sleep 6

OUT3=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT3" --from alice --keyring-backend test --label "my first contract" --gas auto --fees 10000uqsr -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --chain-id $CHAIN_ID)
ADDR3=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[2]')
echo "Got address of deployed contract = $ADDR3"

rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

echo "Running store code for reward contract"
if [ $platform = 'macos' ]; then
    RES=$(quasarnoded tx wasm store artifacts/vault_rewards-aarch64.wasm --from alice --keyring-backend test -y --output json -b block --chain-id $CHAIN_ID --gas auto --fees 10000uqsr)
elif [ $platform = 'linux' ]; then
    RES=$(quasarnoded tx wasm store artifacts/vault_rewards.wasm --from alice --keyring-backend test -y --output json -b block --chain-id $CHAIN_ID --gas auto --fees 10000uqsr)
fi
VR_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')

VAULT_INIT='{"total_cap":"200000000000","deposit_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","thesis":"test","vault_rewards_code_id":'$VR_CODE_ID',"reward_token":{"native":"uqsr"},"reward_distribution_schedules":[],"decimals":6,"symbol":"ORN","min_withdrawal":"1","name":"ORION","primitives":[{"address":"'$ADDR1'","weight":"0.5","init":{"l_p":'$INIT1'}},{"address":"'$ADDR2'","weight":"0.5","init":{"l_p":'$INIT2'}},{"address":"'$ADDR3'","weight":"0.5","init":{"l_p":'$INIT3'}}]}'
echo $VAULT_INIT

echo "Running store code (vault)"
if [ $platform = 'macos' ]; then
    RES=$(quasarnoded tx wasm store artifacts/basic_vault-aarch64.wasm --from alice --keyring-backend test -y --output json -b block --chain-id $CHAIN_ID --gas auto --fees 10000uqsr)
elif [ $platform = 'linux' ]; then
    RES=$(quasarnoded tx wasm store artifacts/basic_vault.wasm --from alice --keyring-backend test -y --output json -b block --chain-id $CHAIN_ID --gas auto --fees 10000uqsr)
fi
VAULT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got CODE_ID = $VAULT_CODE_ID"

echo "Deploying contract (vault)"
# # swallow output
OUT=$(quasarnoded tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from alice --keyring-backend test --label "my first contract" --gas auto --fees 10000uqsr -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --chain-id $CHAIN_ID)
VAULT_ADDR=$(quasarnoded query wasm list-contract-by-code $VAULT_CODE_ID --output json | jq -r '.contracts[0]')
echo "Got address of deployed contract = $VAULT_ADDR (vault)"

echo "setting depositor"
sleep 6
quasarnoded tx wasm execute $ADDR1 '{"set_depositor":{"depositor":"'$VAULT_ADDR'"}}' --from alice --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID -y
sleep 6
quasarnoded tx wasm execute $ADDR2 '{"set_depositor":{"depositor":"'$VAULT_ADDR'"}}' --from alice --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID -y
sleep 6
quasarnoded tx wasm execute $ADDR3 '{"set_depositor":{"depositor":"'$VAULT_ADDR'"}}' --from alice --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID -y
sleep 6

quasarnoded tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from user1 --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID --amount 1000000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518

sleep 20
rly transact flush
sleep 10
quasarnoded tx wasm execute $VAULT_ADDR '{"clear_cache":{}}' -y --from user1 --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID
sleep 10

quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf"}}' --output json

quasarnoded tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from user1 --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID --amount 1000000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518
sleep 6
quasarnoded tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from bob --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID --amount 1000000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518
sleep 6
quasarnoded tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from alice --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID --amount 1000000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518
sleep 6
quasarnoded tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from user2 --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID --amount 1000000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518
sleep 6

for n in {1..20}; do
  KEY_NAME="test"$n
  quasarnoded tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from $KEY_NAME --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID --amount 1000000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518
  sleep 6
done

sleep 20
rly transact flush
sleep 10
quasarnoded tx wasm execute $VAULT_ADDR '{"clear_cache":{}}' -y --from user1 --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID

VAULT_ADDR=quasar1unyuj8qnmygvzuex3dwmg9yzt9alhvyeat0uu0jedg2wj33efl5qtefy4k
quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf"}}' --output json
quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu"}}' --output json
quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"}}' --output json
quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar185fflsvwrz0cx46w6qada7mdy92m6kx4xruj7p"}}' --output json



if [ $platform = 'macos' ]; then
    say "contracts deployment ready"
fi


#quasarnoded tx wasm execute $VAULT_ADDR '{"force_unbond":{"addresses":["quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf", "quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu"]}}' -y --from alice --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID
#sleep 20
#
#quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf"}}' --output json
#quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu"}}' --output json
#quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"}}' --output json
#quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar185fflsvwrz0cx46w6qada7mdy92m6kx4xruj7p"}}' --output json
#
#
#sleep 20
#quasarnoded tx wasm execute $VAULT_ADDR '{"force_claim":{"addresses":["quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf", "quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu"]}}' -y --from alice --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID
#quasarnoded tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"100000"}}' -y --from user1 --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID
#quasarnoded tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"400000"}}' -y --from bob --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID
#quasarnoded tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"400000"}}' -y --from alice --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID
#quasarnoded tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"400000"}}' -y --from user2 --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID

#quasarnoded tx wasm execute $VAULT_ADDR '{"claim":{}}' -y --from bob --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID


#rly q unrelayed-packets quasar_osmosis channel-0
#rly q unrelayed-packets quasar_osmosis channel-1
#rly q unrelayed-packets quasar_osmosis channel-2
#rly q unrelayed-packets quasar_osmosis channel-3
#rly q unrelayed-packets quasar_osmosis channel-4
#rly q unrelayed-packets quasar_osmosis channel-5
#rly q unrelayed-packets quasar_osmosis channel-6
#
#
#rly q unrelayed-acknowledgements quasar_osmosis channel-0
#rly q unrelayed-acknowledgements quasar_osmosis channel-1
#rly q unrelayed-acknowledgements quasar_osmosis channel-2
#rly q unrelayed-acknowledgements quasar_osmosis channel-3
#rly q unrelayed-acknowledgements quasar_osmosis channel-4
#rly q unrelayed-acknowledgements quasar_osmosis channel-5
#rly q unrelayed-acknowledgements quasar_osmosis channel-6

#osmosisd tx gov submit-proposal software-upgrade v21 --upgrade-height 100 --deposit 20000000uosmo --title "test" --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y -b block --gas 2000000 --fees 100000uosmo --description "test"
#osmosisd tx gov vote 1 yes --from alice --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y -b block --gas 200000 --fees 100000uosmo