#!/bin/bash

echo "User 1 balances  -"
quasard q bank balances $(quasard keys show user1 -a --keyring-backend test --home ~/.quasarnode/) --node tcp://localhost:26659


quasard tx bank send $(quasard keys show alice -a --keyring-backend test --home ~/.quasarnode) $(quasard keys show user1 -a --keyring-backend test --home ~/.quasarnode) 1000uqsr --chain-id quasar --node tcp://localhost:26659 --from alice --keyring-backend test --home ~/.quasarnode
quasard tx bank send $(quasard keys show bob -a --keyring-backend test --home ~/.quasarnode) $(quasard keys show user1 -a --keyring-backend test --home ~/.quasarnode) 1000uqsr --chain-id quasar --node tcp://localhost:26659 --from bob --keyring-backend test --home ~/.quasarnode


alice_pub_key=$(quasard keys show alice -p --keyring-backend test --home ~/.quasarnode/)
bob_pub_key=$(quasard keys show bob -p --keyring-backend test --home ~/.quasarnode/)

echo "alice pub key $alice_pub_key"
echo "bob pub key $bob_pub_key"
# The below commands is to be done in the shared machine in the real-world scenario.

## Create multi sig address in shared machine.
quasard keys add alice-bob-multisig --multisig alice,bob --multisig-threshold 2 --keyring-backend test --home ~/.quasarnode

multi_sig_addr=$(quasard keys show alice-bob-multisig -a --keyring-backend test --home ~/.quasarnode)
echo "multi sig address - $multi_sig_addr"
echo "Sending 1000uqsr to the multisig"

quasard tx bank send $(quasard keys show alice -a --keyring-backend test --home ~/.quasarnode) $(quasard keys show alice-bob-multisig -a --keyring-backend test --home ~/.quasarnode) 1000uqsr --chain-id quasar --node tcp://localhost:26659 --from alice --keyring-backend test --home ~/.quasarnode

sleep 5

echo "Sending 1000uqsr from multi-sig to user 1"
echo "Gen unsinged tx"
## Generate multi-sig in shared tx file. Bank tx to user1. Assuming user1 address is already in the shared machine keyring. 
## user1 address is quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf
quasard tx bank send $(quasard keys show alice-bob-multisig -a --keyring-backend test --home ~/.quasarnode) $(quasard keys show user1 -a --keyring-backend test --home ~/.quasarnode) 1000uqsr  --generate-only --chain-id quasar --node tcp://localhost:26659 > tx.json


## To be done in alice machine in real-world scenario
echo "Sign from alice"
sleep 5
quasard tx sign tx.json --multisig=$(quasard keys show -a alice-bob-multisig --keyring-backend test --home ~/.quasarnode)  --sign-mode amino-json --chain-id  quasar --node tcp://localhost:26659 --keyring-backend test --home ~/.quasarnode --from alice > tx-signed-alice.json


echo "Sign from bob"

sleep 5
## To be done in bob machine in real-world scenario
quasard tx sign tx.json --multisig=$(quasard keys show -a alice-bob-multisig --keyring-backend test --home ~/.quasarnode)  --sign-mode amino-json --chain-id  quasar --node tcp://localhost:26659 --keyring-backend test --home ~/.quasarnode --from bob > tx-signed-bob.json

sleep 5
echo "Multi sign"
# Combine signatures into single tx
quasard tx multisign tx.json alice-bob-multisig tx-signed-alice.json tx-signed-bob.json --chain-id quasar --keyring-backend test --home ~/.quasarnode --from alice-bob-multisig  > tx_ms.json


echo "Broadcast"
sleep 5
quasard tx broadcast tx_ms.json --chain-id quasar

echo "User1 balances"
quasard q bank balances $(quasard keys show user1 -a --keyring-backend test --home ~/.quasarnode/) --node tcp://localhost:26659
