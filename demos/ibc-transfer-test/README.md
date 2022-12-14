# IBC Transfer Test
This demo tests whether IBC transfers will ack back into the contract when transfers are sent from a contract.

## To run
Launch quasar chain, osmo chain, and relayer
```bash
cd demos/ibc-transfer-test

./quasar-localnet.sh
# in a separate terminal
./osmo-localnet.sh
# in a separate terminal
./setup-go-relayer.sh
rly start # once relayer starts, proceed
```


In a separate terminal
`cd smart-contracts`

### Build
Build the ibc-transfer-test contract using your favorite build strategy:
`cd contracts/ibc-transfer && RUSTFLAGS='-C link-arg=-s' cargo wasm && cd -`

### Store
`~/go/bin/quasarnoded tx wasm  store ./target/wasm32-unknown-unknown/release/ibc_transfer.wasm --from alice --gas auto --chain-id quasar`

### Instantiate
`~/go/bin/quasarnoded tx wasm instantiate 1  "{}" --from alice --label "my first contract" --gas-prices 10uqsr --gas auto --gas-adjustment 1.3 -b block -y --no-admin --chain-id quasar`

Make sure to replace code ID with the correct one above

### Query
`quasarnoded query wasm contract-state smart REPLACE_ME_WITH_INSTANTIATED_CONTRACT_ADDR '{"state":{}}'`

This will return
```
data:
  transfer_happened: false
```

### Execute
`quasarnoded tx wasm execute REPLACE_ME_WITH_INSTANTIATED_CONTRACT_ADDR '{"transfer":{"channel":"channel-0","to_address":"osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq"}}' --from alice --amount 10uqsr --chain-id quasar`

Wait approx 16 seconds

### Query again
`quasarnoded query wasm contract-state smart REPLACE_ME_WITH_INSTANTIATED_CONTRACT_ADDR '{"state":{}}'`

If everything worked, this will return
```
data:
  transfer_happened: true
```