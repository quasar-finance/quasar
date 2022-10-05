The purpose of this demo is to demonstrate the ibc connection establishment between bandchain testnet and quasar localnet. And also checking the stable price query update in the quasar chain.


1. Remove previous relayer configuration by running the following command
```
rm -rf ~/.ignite/relayer
```

2. Start the chain with default config
```
ignite chain serve --reset-once -v
```

3. Configure a channel for bandchain testnet

Cross check the value of each field based on your current chain setup. Bandchain also regularly 
updating their testnet so its rpc and faucet address are also changing.

```
ignite relayer configure -a \
--target-rpc "https://rpc.laozi-testnet5.bandchain.org" \
--target-faucet "https://laozi-testnet5.bandchain.org/faucet" \
--target-port "oracle" \
--target-gasprice "0uband" \
--target-gaslimit 5000000 \
--target-prefix "band" \
--target-version "bandchain-1" \
--source-rpc "http://localhost:26659" \
--source-faucet "http://localhost:4500" \
--source-port "qoracle" \
--source-gasprice "0.0stake" \
--source-gaslimit 300000 \
--source-prefix "quasar"  \
--source-version "bandchain-1"
```

1. Start the relayer
```
ignite relayer connect 
```
Wait for the channel and relayer to come online

5. Watch for changes in state
Use the right api port based on config.yml file. 
In this repo it is 1311 
```
http://localhost:1317/quasarlabs/quasarnode/qoracle/state
or 
http://localhost:1311/quasarlabs/quasarnode/qoracle/state
```
