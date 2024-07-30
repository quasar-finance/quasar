#!/bin/sh

cp ./hermes_config.toml ~/.hermes/config.toml

hermes keys add --chain quasar --mnemonic-file keys/qsr.key

hermes keys add --chain cosmos --mnemonic-file keys/gaia.key

hermes keys add --chain osmosis --mnemonic-file keys/osmo.key


# BANDCHAIN="band-laozi-testnet5"
# hermes keys restore --mnemonic "machine danger crush duck always will liberty popular security shoulder bargain day repair focus fog evoke market gossip love curious question kingdom armor crazy"  --hd-path "m/44'/494'/0'/0/0" band-laozi-testnet5

## Checking balance
quasard q bank balances quasar143wwmxhsd8nkwu7j8gzpv9ca503g8j55h059ew --node tcp://localhost:26659
gaiad q bank balances cosmos1lrelhs37akgz2wht0y377uerxjm9fh33ke3ksc  --node tcp://localhost:26669
osmosisd q bank balances osmo194580p9pyxakf3y3nqqk9hc3w9a7x0yrnv7wcz --node tcp://localhost:26679
# bandd q bank balances band1cjx30d7n4k4pedgqkeqztz90q2l465gqrcymgf --node https://rpc.laozi-testnet5.bandchain.org:443


# Create connection
echo "creating connection for quasar cosmos"
hermes create connection --a-chain quasar --b-chain cosmos >> ./logs/hermes_qc.log 2>&1

echo "creating connection for quasar osmosis"
hermes create connection --a-chain quasar --b-chain osmosis >> ./logs/hermes_qc.log 2>&1

echo "creating connection for osmosis cosmos"
hermes create connection --a-chain osmosis --b-chain cosmos >> ./logs/hermes_oc.log 2>&1

# hermes create connection quasar $BANDCHAIN

# Create channel
echo "creating default channel for quasar cosmos"
hermes create channel --a-chain cosmos --a-connection connection-0 --a-port transfer --b-port transfer >> ./logs/hermes_qc.log 2>&1

echo "creating default channel for cosmos osmosis"
hermes create channel --a-chain cosmos --a-connection connection-1 --a-port transfer --b-port transfer >> ./logs/hermes_oc.log 2>&1

echo "creating default channel for quasar osmosis"
hermes create channel --a-chain quasar --a-connection connection-1 --a-port transfer --b-port transfer >> ./logs/hermes_qo.log 2>&1

# hermes create channel --port-a qoracle --port-b oracle quasar connection-2 -v bandchain-1
