# Steps to connect quasarnode to osmosis via ICQ protocol

1. Run the quasar node

```
ignite chain serve --reset-once -v
```

2. Get the osmosis source code from https://github.com/quasar-finance/osmosis and switch to branch "feature/icq_integration"

3. Run the osmosis node with osmosis.yml config file in demo directory

```
ignite chain serve -c osmosis.yml --reset-once -v
```

4. Clean the ignite relayer recent configs

```
rm -rf ~/.ignite/relayer
```

5. Configure a channel for bandchain testnet
```
ignite relayer configure -a \
--target-rpc "https://rpc.laozi-testnet5.bandchain.org" \
--target-faucet "https://laozi-testnet5.bandchain.org/faucet" \
--target-port "oracle" \
--target-gasprice "0uband" \
--target-gaslimit 5000000 \
--target-prefix "band" \
--target-version "bandchain-1" \
--source-rpc "http://localhost:26657" \
--source-faucet "http://localhost:4500" \
--source-port "qoracle" \
--source-gasprice "0.0stake" \
--source-gaslimit 300000 \
--source-prefix "quasar"  \
--source-version "bandchain-1"
```

6. Start the relayer and wait for the channel and relayer to come online
```
ignite relayer connect 
```

7. Wait for a packet transmission so that the oracle prices in quasar node be updated. You can check the prices from http://localhost:1317/quasarlabs/quasarnode/qoracle/oracle_prices

8. Stop the relayer and clean the ignite relayer config (**We have to reset the relayer because apparently bandchain testnet has a spam detection mechanism that removes the IBC channel if we request more than a certain packets**)

```
rm -rf ~/.ignite/relayer
```

9. Configure the relayer for icq connection between two chains

```
ignite relayer configure -a \
--source-rpc "http://localhost:26657" \
--source-faucet "http://localhost:4500" \
--source-port "qoracle" \
--source-gasprice "0.0stake" \
--source-gaslimit 5000000 \
--source-prefix "quasar" \
--source-version "icq-1" \
--target-rpc "http://localhost:26669" \
--target-faucet "http://localhost:4502" \
--target-port "icqhost" \
--target-gasprice "0.0stake" \
--target-gaslimit 300000 \
--target-prefix "osmo"  \
--target-version "icq-1"
```

7. Start the relayer and wait for it to establish the connection and channel
```
ignite relayer connect
```

8. Execute the following tx so quasar sends a request to update osmosis chain params to osmosis

```
quasarnoded tx qoracle update-osmosis-chain-params --node tcp://localhost:26657 --from alice --home ~/.quasar --chain-id quasarnode --output json
```

9. After the acknowledgement received by quasar you can query the osmosis chains params stored in the quasar from http://localhost:1317/quasarlabs/quasarnode/qoracle/osmosis/chain_params

10. Quasar will send a request to fetch incentivized pools from osmosis every minute. You can query the incentivized pools from http://localhost:1317/quasarlabs/quasarnode/qoracle/osmosis/incentivized_pools

11. In order to create a pool in osmosis run the following command:

```
osmosisd tx gamm create-pool --pool-file demo_pool.json --home ~/.osmo --chain-id osmosis --node=http://localhost:26669 --from alice --gas=300000
```

12. Check the pools endpoint in quasar from http://localhost:1317/quasarlabs/quasarnode/qoracle/osmosis/pools