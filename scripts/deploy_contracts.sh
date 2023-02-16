#!/bin/sh

set -e

CHAIN_ID="quasar"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
RPC="http://127.0.0.1:26659"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"

INIT1='{"lock_period":300,"pool_id":1,"pool_denom":"gamm/pool/1","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"ibc/6BEEEE6CC17BA0EE37857A1E2AF6EC53C50DB6B6F89463A3933D0859C4CF6087","return_source_channel":"channel-0","transfer_channel":"channel-0"}'
INIT2='{"lock_period":300,"pool_id":2,"pool_denom":"gamm/pool/2","base_denom":"ibc/DA3CEF7DEAF6983032E061030C63E13262957D223E9EDBBB7AF9B69F8F8BA090","local_denom":"ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878","quote_denom":"fakestake","return_source_channel":"channel-0","transfer_channel":"channel-0"}'
INIT3='{"lock_period":300,"pool_id":3,"pool_denom":"gamm/pool/3","base_denom":"fakestake","local_denom":"ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1","quote_denom":"uosmo","return_source_channel":"channel-0","transfer_channel":"channel-0"}'
