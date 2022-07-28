# IBC Test setup
This contract is setup to manually test different Quasar features that use IBC. 

## Building the contract
To build the contract,
in the smart contracts directory, run:
```
docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/workspace-optimizer:0.12.6
```


## Chains Setup
Most of the setup of the chains is reused from the ica packet forward demo
Start a quasar node

```
ignite chain serve --config demos/ica-packet-forward/quasar.yml --reset-once -v
```

Clone gaia from this repo (https://github.com/quasar-finance/gaia/tree/bugfix/replace_default_transfer_with_router_module) and run it by copying the the cosmos.yml file to gaia project root and running the following command:

```
cd ../gaia
ignite chain serve --config cosmos.yml --reset-once -v
```

(For now we use another instance of quasar as osmosis since icahost is not implemented yet in osmosis)

```
ignite chain serve --config demos/ica-packet-forward/osmosis.yml --reset-once -v
```

```
cp ./demos/ica-packet-forward/hermes_config.toml ~/.hermes/config.toml
```

Using hermes v0.15.0

```
hermes keys restore --mnemonic "jungle law popular reunion festival horn divorce quarter image gather october weird slide trend resource render abuse food tomorrow multiply price fun ask quarter" quasar

hermes keys restore --mnemonic "blade trap agent boy note critic jazz nuclear eight lion pipe fresh tourist make broken inquiry close agree usual human stock move remain swim" cosmos

hermes keys restore --mnemonic "act scale exhibit enough swamp vivid bleak eagle giggle brass desert debris network scrub hazard fame salon normal over between inform advance sick dinner" osmosis
```

Create ibc connection between quasar and osmosis for ICA usage

```
hermes create connection quasar osmosis
```

Create transfer channels between osmosis <-> cosmos and cosmos <-> quasar

```
hermes create channel osmosis --chain-b cosmos --port-a transfer --port-b transfer --new-client-connection

hermes create channel cosmos --chain-b quasar --port-a transfer --port-b transfer --new-client-connection
```

W