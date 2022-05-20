```
ignite chain serve --config demos/ica-packet-forward/quasar1.yml --reset-once -v
```

```
cd ../gaia
ignite chain serve --config demos/ica-packet-forward/cosmos.yml --reset-once -v
```

```
ignite chain serve --config demos/ica-packet-forward/quasar3.yml --reset-once -v
```

```
cp ./demos/ica-packet-forward/hermes_config.toml ~/.hermes/config.toml
```

```
hermes keys restore --mnemonic "jungle law popular reunion festival horn divorce quarter image gather october weird slide trend resource render abuse food tomorrow multiply price fun ask quarter" quasar1
```

```
hermes keys restore --mnemonic "act scale exhibit enough swamp vivid bleak eagle giggle brass desert debris network scrub hazard fame salon normal over between inform advance sick dinner" quasar2
```

```
hermes keys restore --mnemonic "act scale exhibit enough swamp vivid bleak eagle giggle brass desert debris network scrub hazard fame salon normal over between inform advance sick dinner" quasar3
```

```
hermes create connection quasar1 quasar2
```

```
hermes create channel quasar2 cosmos --port-a transfer --port-b transfer
```

```
hermes create channel cosmos quasar1 --port-a transfer --port-b transfer
```

```
hermes start
```

```
quasarnoded tx intergamm register-account connection-0 --chain-id=quasar1 --node=tcp://localhost:26659 --home ~/.q1 --from alice
```

```
quasarnoded query intergamm interchain-account-from-address connection-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --node=tcp://localhost:26659
```

Send some tokens to the interchain account address in quasar2

Issue the interchain tx to ibc transfer some tokens to quasar1 from interchain account on quasar2 through cosmos
```
quasarnoded tx intergamm forward-ibc-transfer connection-0 transfer channel-0 10qsr transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --chain-id=quasar1 --node=tcp://localhost:26659 --home ~/.q1 --from alice
```