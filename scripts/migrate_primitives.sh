set -e

CHAIN_ID="qsr-questnet-04"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
RPC="http://node3.tst4.qsr.network:26657/"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"

cd ../smart-contracts

PRIM_CODE_ID=14

CONFIG1='{"lock_period":300,"pool_id":1,"pool_denom":"gamm/pool/1","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"ibc/6BEEEE6CC17BA0EE37857A1E2AF6EC53C50DB6B6F89463A3933D0859C4CF6087","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
CONFIG2='{"lock_period":300,"pool_id":2,"pool_denom":"gamm/pool/2","base_denom":"ibc/DA3CEF7DEAF6983032E061030C63E13262957D223E9EDBBB7AF9B69F8F8BA090","local_denom":"uayy","quote_denom":"uosmo","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
CONFIG3='{"lock_period":300,"pool_id":3,"pool_denom":"gamm/pool/3","base_denom":"ibc/6BEEEE6CC17BA0EE37857A1E2AF6EC53C50DB6B6F89463A3933D0859C4CF6087","local_denom":"uqsr","quote_denom":"ibc/DA3CEF7DEAF6983032E061030C63E13262957D223E9EDBBB7AF9B69F8F8BA090","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'


PRIM1="quasar1qum2tr7hh4y7ruzew68c64myjec0dq2s2njf6waja5t0w879lutqtvz03d"
PRIM2="quasar1tqwwyth34550lg2437m05mjnjp8w7h5ka7m70jtzpxn4uh2ktsmqqthn5d"
PRIM3="quasar1vguuxez2h5ekltfj9gjd62fs5k4rl2zy5hfrncasykzw08rezpfsah6jpz"
quasarnoded tx wasm migrate $PRIM1 $PRIM_CODE_ID "{\"config\": $CONFIG1}" --from test-laurens --keyring-backend test -b block $TXFLAG 
quasarnoded tx wasm migrate $PRIM2 $PRIM_CODE_ID "{\"config\": $CONFIG2}" --from test-laurens --keyring-backend test -b block $TXFLAG 
quasarnoded tx wasm migrate $PRIM3 $PRIM_CODE_ID "{\"config\": $CONFIG3}" --from test-laurens --keyring-backend test -b block $TXFLAG 
