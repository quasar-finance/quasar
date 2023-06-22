#!/bin/sh

#lock period = 14 * 24 * 60 * 60

# osmo denoms:
# uatom: ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2
# stAtom: ibc/C140AFD542AE77BD7DCC83F13FDD8C5E5BB8C4929785E6EC2F4C636F98F17901
# qAtom: ibc/FA602364BEC305A696CBDF987058E99D8B479F0318E47314C49173E8838C5BAC

# quasar uatom: ibc/FA0006F056DB6719B8C16C551FC392B62F5729978FC0B125AC9A432DBB2AA1A5
INIT1='{"lock_period":1209601,"pool_id":1,"pool_denom":"gamm/pool/1","base_denom":"ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2","local_denom":"ibc/FA0006F056DB6719B8C16C551FC392B62F5729978FC0B125AC9A432DBB2AA1A5","quote_denom":"uosmo","return_source_channel":"channel-688","transfer_channel":"channel-1","expected_connection":"connection-1"}'
INIT2='{"lock_period":1209601,"pool_id":803,"pool_denom":"gamm/pool/803","base_denom":"ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2","local_denom":"ibc/FA0006F056DB6719B8C16C551FC392B62F5729978FC0B125AC9A432DBB2AA1A5","quote_denom":"ibc/C140AFD542AE77BD7DCC83F13FDD8C5E5BB8C4929785E6EC2F4C636F98F17901","return_source_channel":"channel-688","transfer_channel":"channel-1","expected_connection":"connection-1"}'
INIT3='{"lock_period":1209601,"pool_id":944,"pool_denom":"gamm/pool/944","base_denom":"ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2","local_denom":"ibc/FA0006F056DB6719B8C16C551FC392B62F5729978FC0B125AC9A432DBB2AA1A5","quote_denom":"ibc/FA602364BEC305A696CBDF987058E99D8B479F0318E47314C49173E8838C5BAC","return_source_channel":"channel-688","transfer_channel":"channel-1","expected_connection":"connection-1"}'

CODE_ID=32

quasarnoded tx wasm instantiate $CODE_ID "$INIT1" --from laurens-ledger --sign-mode amino-json --label "primitive 4 - pool 1" --gas-prices 1ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1u67rhvzs2t24m4qpseggl6qe68psq6lpjrrx9g" --node https://quasar-rpc.polkachu.com:443 --chain-id quasar-1
quasarnoded tx wasm instantiate $CODE_ID "$INIT2" --from laurens-ledger --sign-mode amino-json --label "primitive 5 - pool 803" --gas-prices 1ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1u67rhvzs2t24m4qpseggl6qe68psq6lpjrrx9g" --node https://quasar-rpc.polkachu.com:443 --chain-id quasar-1
quasarnoded tx wasm instantiate $CODE_ID "$INIT3" --from laurens-ledger --sign-mode amino-json --label "primitive 6 - pool 944" --gas-prices 1ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1u67rhvzs2t24m4qpseggl6qe68psq6lpjrrx9g" --node https://quasar-rpc.polkachu.com:443 --chain-id quasar-1

cd ../smart-contracts
quasarnoded tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from laurens-ledger --sign-mode amino-json --label "vault ATOM PRO" --gas-prices 1ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1u67rhvzs2t24m4qpseggl6qe68psq6lpjrrx9g" --node https://quasar-rpc.polkachu.com:443 --chain-id quasar-1
# docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.11
ADDR1="quasar1l468h9metf7m8duvay5t4fk2gp0xl94h94f3e02mfz4353de2ykqh6rcts"
ADDR2="quasar1jgn70d6pf7fqtjke0q63luc6tf7kcavdty67gvfpqhwwsx52xmjq7kd34f"
ADDR3="quasar1ch4s3kkpsgvykx5vtz2hsca4gz70yks5v55nqcfaj7mgsxjsqypsxqtx2a"

rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-1","host_connection_id":"connection-2240"}' --override

rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-1","host_connection_id":"connection-2240"}' --override

rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-1","host_connection_id":"connection-2240"}' --override


VAULT_INIT='{
  "thesis": "ATOM PRO is depositing into the most battle tested ATOM pools on Osmosis with the biggest liquidity and highest volume - offering an easy way to get exposure to the deepest pools with automatic compounding and incentive distribution - Osmosis, evolved.",
  "vault_rewards_code_id": 24,
  "reward_token": {
    "native": "uqsr"
  },
  "reward_distribution_schedules": [],
  "decimals": 6,
  "symbol": "APRO",
  "min_withdrawal": "1000000",
  "name": "ATOM PRO",
  "total_cap": "100000000000",
  "primitives": [
    {
      "address": "'$ADDR1'",
      "weight": "0.5",
      "init": {
        "l_p": '$INIT1'
      }
    },
    {
      "address": "'$ADDR2'",
      "weight": "0.35",
      "init": {
        "l_p": '$INIT2'
      }
    },
    {
      "address": "'$ADDR3'",
      "weight": "0.15",
      "init": {
        "l_p": '$INIT3'
      }
    }
  ]
}'
echo $VAULT_INIT

VAULT_CODE_ID=30

echo "Deploying contract (vault)"
# swallow output
quasarnoded tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from laurens-ledger --sign-mode amino-json --label "vault ATOM PRO" --gas-prices 1ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1u67rhvzs2t24m4qpseggl6qe68psq6lpjrrx9g" --node https://quasar-rpc.polkachu.com:443 --chain-id quasar-1
