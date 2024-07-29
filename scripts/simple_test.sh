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

quasard config keyring-backend $KEYRING
quasard config chain-id $CHAINID

# if $KEY exists it should be deleted
quasard keys add $KEY --keyring-backend $KEYRING --algo $KEYALGO

# Set moniker and chain-id for cosmic-ether (Moniker can be anything, chain-id must be str-int)
quasard init $MONIKER --chain-id $CHAINID

# Allocate genesis accounts (cosmos formatted addresses)
quasard add-genesis-account $KEY 100000000000000000000000000stake --keyring-backend $KEYRING

# Sign genesis transaction
quasard gentx $KEY 1000000000000000000000stake --keyring-backend $KEYRING --chain-id $CHAINID

# Collect genesis tx
quasard collect-gentxs

# Run this to ensure everything worked and that the genesis file is setup correctly
quasard validate-genesis

if [[ $1 == "pending" ]]; then
  echo "pending mode is on, please wait for the first block committed."
fi

# Start the node (remove the --pruning=nothing flag if historical queries are not needed)
quasard start --pruning=nothing