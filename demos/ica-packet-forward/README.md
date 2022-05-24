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

```
hermes start
```

Register interchain account

```
quasarnoded tx intergamm register-account connection-0 --chain-id=quasar --node=tcp://localhost:26659 --home ~/.qsr --from alice
```

Query the interchain account address on osmosis

```
quasarnoded query intergamm interchain-account-from-address connection-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --node=tcp://localhost:26659
```

Send some tokens to the interchain account address on osmosis

```
quasarnoded tx bank send alice quasar1hphwfu3yjf82z8xpcl6e05gzkjwjmu8ts2m97mdk62feuqm77f2sj52eta 10uosmo --chain-id=osmosis --node=tcp://localhost:26669 --home ~/.osmo --from alice
```

Issue the interchain tx to ibc transfer some tokens to quasar from interchain account on osmosis through cosmos

```
quasarnoded tx intergamm forward-ibc-transfer connection-0 transfer channel-0 10uosmo transfer channel-1 cosmos1e3geute48fzym40um7gw2kjt87q7nkeel7mept quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --chain-id=quasar --node=tcp://localhost:26659 --home ~/.qsr --from alice
```

Check balance of quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec from the link below, it should contain 10ibc/C9B459D627233FC6E5049A27B5465D76C7F69D7FCB7F843EAC2B6A38B668F9F1

http://localhost:1311/bank/balances/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec