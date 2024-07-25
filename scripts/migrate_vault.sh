set -e

CHAIN_ID="qsr-questnet-04"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
RPC="http://node3.tst4.qsr.network:26657/"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"

VAULT_ADDR="quasar1xt4ahzz2x8hpkc0tk6ekte9x6crw4w6u0r67cyt3kz9syh24pd7slqr7s5"
quasard tx wasm migrate $VAULT_ADDR 5 '{}' --from test-laurens --keyring-backend test $TXFLAG