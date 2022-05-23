```
ignite chain serve --config demos/ica-packet-forward/quasar.yml --reset-once -v
```

Clone gaia from this repo (https://github.com/quasar-finance/gaia/tree/bugfix/replace_default_transfer_with_router_module) and run it by copying the the cosmos.yml file to gaia project root and running the following command:

```
cd ../gaia
ignite chain serve --config demos/ica-packet-forward/cosmos.yml --reset-once -v
```

(For now we use another instance of quasar as osmosis since icahost is not implemented yet in osmosis)

```
ignite chain serve --config demos/ica-packet-forward/osmosis.yml --reset-once -v
```

```
cp ./demos/ica-packet-forward/hermes_config.toml ~/.hermes/config.toml
```

```
hermes keys restore --mnemonic "jungle law popular reunion festival horn divorce quarter image gather october weird slide trend resource render abuse food tomorrow multiply price fun ask quarter" quasar

hermes keys restore --mnemonic "blade trap agent boy note critic jazz nuclear eight lion pipe fresh tourist make broken inquiry close agree usual human stock move remain swim" cosmos

hermes keys restore --mnemonic "act scale exhibit enough swamp vivid bleak eagle giggle brass desert debris network scrub hazard fame salon normal over between inform advance sick dinner" osmosis
```

```
hermes create connection quasar osmosis
```

```
hermes create channel osmosis cosmos --port-a transfer --port-b transfer

hermes create channel cosmos quasar --port-a transfer --port-b transfer
```

```
hermes start
```

```
quasarnoded tx intergamm register-account connection-0 --chain-id=quasar --node=tcp://localhost:26659 --home ~/.qsr --from alice
```

```
quasarnoded query intergamm interchain-account-from-address connection-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --node=tcp://localhost:26659
```

Send some tokens to the interchain account address in osmosis

Issue the interchain tx to ibc transfer some tokens to quasar from interchain account on osmosis through cosmos
```
quasarnoded tx intergamm forward-ibc-transfer connection-0 transfer channel-0 10uosmo transfer channel-1 cosmos1e3geute48fzym40um7gw2kjt87q7nkeel7mept quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --chain-id=quasar --node=tcp://localhost:26659 --home ~/.qsr --from alice
```