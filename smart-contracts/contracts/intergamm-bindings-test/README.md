# Intergamm test contract
The goal of this contract is to transmit a message from the smart contract to a local osmosis chain

## setting up
build the contract:
```
docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.6
```

Run the chains using the commands from the orion manual demo in a separate window:
```
./cosmos_localnet.sh 
```

```
./osmo_localnet.sh
```

```
./quasar_localnet.sh
```

```
./run_hermes
```

Now store the contract on the quasar chain:
```
EXTRA="--node tcp://localhost:26659 --chain-id quasar"
```
```
quasarnoded tx wasm instantiate 1 "{}" --label test-2 --no-admin --from alice $EXTRA --gas auto
```

Due to a bug, instantiate does not return the actual address of the contract, but only the hash, so we query for the address
```
quasarnoded query tx TX_HASH_FROM_INSTANTIATE
```
set the address for easy use

```
ADDR="quasar1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrsmslfn4"
```

Now we start by registering an interchain account:
```
quasarnoded tx wasm execute $ADDR '{"register_interchain_account": {"connection_id": "connection-1"}}' $EXTRA --from alice --gas auto
```

If necessary, create a pool on osmosis
```
osmosisd tx gamm create-pool --pool-file ./demos/orion-manual-demo/sample_pool.json --node tcp://localhost:26679 --from alice --keyring-backend test --chain-id osmosis --gas auto
```

We also need to send tokens to the register interchain account, easiest way to do this is to send tokens from alice or bob on osmosis to the interchain address, to find the address that funds need to be transferred to:
```
quasarnoded query intergamm interchain-account-from-address connection-1 $ADDR --node tcp://localhost:26679 --chain-id osmosis
```
Funds using IBC transfer should also be sent to this address
## Regular IBC transfer
query the channel and port on the quasar-osmosis connection
```
hermes query connection channels quasar connection-1
```
look for the channel with the transfer port, in this case
```
PortChannelId {
        channel_id: ChannelId(
            "channel-2",
        ),
        port_id: PortId(
            "transfer",
        ),
    },
```
deposit some funds to send
```
quasarnoded tx wasm execute $ADDR '{"deposit": {}}' --node http://0.0.0.0:26659 --amount 100000uqsr --chain-id quasar --from alice --gas auto
```
We now send the ibc transfer from the smart contract, we'll use alice's address on osmosis as the to_address
```
quuasarnoded tx wasm execute $ADDR '{"send_token_ibc": {"channel_id":"channel-2","to_address":"osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq", "amount": {"denom": "uqsr", "amount": 1000}}}' --node http://0.0.0.0:26659 --chain-id quasar --from alice --gas auto
```
check the balance on osmosis
```
osmosisd query bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node tcp://localhost:26679 --chain-id osmosis
```
There should be a new ibc denom with our funds, eg:
```
- amount: "1000"
  denom: ibc/C18695C91D20F11FEE3919D7822B34651277CA84550EF33379E823AD9702B257
```