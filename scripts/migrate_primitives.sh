set -e

CHAIN_ID="qsr-questnet-04"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
RPC="http://node3.tst4.qsr.network:26657/"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"

cd ../smart-contracts

PRIM_CODE_ID=10

PRIM1="quasar1qum2tr7hh4y7ruzew68c64myjec0dq2s2njf6waja5t0w879lutqtvz03d"
PRIM2="quasar1tqwwyth34550lg2437m05mjnjp8w7h5ka7m70jtzpxn4uh2ktsmqqthn5d"
PRIM3="quasar1vguuxez2h5ekltfj9gjd62fs5k4rl2zy5hfrncasykzw08rezpfsah6jpz"
quasarnoded tx wasm migrate $PRIM1 $PRIM_CODE_ID '{}' --from test-laurens --keyring-backend test $TXFLAG 
quasarnoded tx wasm migrate $PRIM2 $PRIM_CODE_ID '{}' --from test-laurens --keyring-backend test $TXFLAG 
quasarnoded tx wasm migrate $PRIM3 $PRIM_CODE_ID '{}' --from test-laurens --keyring-backend test $TXFLAG 
