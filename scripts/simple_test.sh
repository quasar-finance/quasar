KEY="mykey"
CHAINID="test-1"
MONIKER="localtestnet"
KEYALGO="secp256k1"
KEYRING="test"
LOGLEVEL="info"
# to trace evm
#TRACE="--trace"
TRACE=""

# remove existing daemon
rm -rf ~/.quasarnode*

quasarnoded config keyring-backend $KEYRING
quasarnoded config chain-id $CHAINID

# if $KEY exists it should be deleted
quasarnoded keys add $KEY --keyring-backend $KEYRING --algo $KEYALGO

# Set moniker and chain-id for cosmic-ether (Moniker can be anything, chain-id must be str-int)
quasarnoded init $MONIKER --chain-id $CHAINID

# Allocate genesis accounts (cosmos formatted addresses)
quasarnoded add-genesis-account $KEY 100000000000000000000000000stake --keyring-backend $KEYRING

# Sign genesis transaction
quasarnoded gentx $KEY 1000000000000000000000stake --keyring-backend $KEYRING --chain-id $CHAINID

# Collect genesis tx
quasarnoded collect-gentxs

# Run this to ensure everything worked and that the genesis file is setup correctly
quasarnoded validate-genesis

if [[ $1 == "pending" ]]; then
  echo "pending mode is on, please wait for the first block committed."
fi

# Start the node (remove the --pruning=nothing flag if historical queries are not needed)
quasarnoded start --pruning=nothing