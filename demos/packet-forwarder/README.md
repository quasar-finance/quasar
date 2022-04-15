# IBC Packet forwarder demo

1. Start 3 blockchains, quasar 1, quasar 2 and osmosis locally

```
./start all
```

2. Start `quasar` chain by running the follwing command at the root of repository

```
ignite chain serve --config quasar1.yml --reset-once
```

3. Start `quasar` cosmos chain by running the follwing command at the root of repository

```
ignite chain serve --config quasar2.yml --reset-once
```

3. Start osmosis (+`intergamm` module) chain by copying the `osmosis.yml` file to the root of osmosis repository and running the following command

```
ignite chain serve --config osmosis.yml --reset-once
```

4. Configure the `transfer` channel on ignite relayer only if you want to play with ibc transferred tokens

```
ignite relayer configure --source-rpc "http://localhost:26659" --source-faucet "http://localhost:4500" --source-account default --source-gaslimit 300000 --source-gasprice 0.00025stake --source-prefix quasar --target-rpc "http://localhost:26559" --target-faucet "http://localhost:4501" --target-account default --target-gaslimit 300000 --target-gasprice 0.00025stake --target-prefix osmo

ignite relayer configure --source-rpc "http://localhost:26659" --source-faucet "http://localhost:4500" --source-account default --source-gaslimit 300000 --source-gasprice 0.00025stake --source-prefix quasar --target-rpc "http://localhost:26669" --target-faucet "http://localhost:4502" --target-account default --target-gaslimit 300000 --target-gasprice 0.00025stake --target-prefix quasar

ignite relayer configure --source-rpc "http://localhost:26559" --source-faucet "http://localhost:4501" --source-account default --source-gaslimit 300000 --source-gasprice 0.00025stake --source-prefix osmo --target-rpc "http://localhost:26669" --target-faucet "http://localhost:4502" --target-account default --target-gaslimit 300000 --target-gasprice 0.00025stake --target-prefix quasar
```

5. Configure `intergamm` channel on ignite relayer

```
ignite relayer configure --advanced --source-rpc "http://localhost:26659" --source-faucet "http://localhost:4500" --source-account default --source-gaslimit 300000 --source-gasprice 0.00025stake --source-prefix quasar --source-port "intergamm" --source-version "intergamm-1" --target-rpc "http://localhost:26559" --target-faucet "http://localhost:4501" --target-account default --target-gaslimit 300000 --target-gasprice 0.00025stake --target-prefix osmo --target-port "intergamm" --target-version "intergamm-1"
```

6. Start the ignite relayer and wait for it to finish creating the connection(s)

```
ignite relayer connect
```

7. transfer uatom from quasar to osmosis

```
intergammd tx ibc-transfer transfer transfer channel-0 cosmos15xjfr557t9e7g2rxyn0t3sl647e4alwefew8nx 2000uatom --home run/quasar2/home --node=http://localhost:26669 --from bob --chain-id quasar2
```

8. transfer stake token from quasar to osmosis via cosmos

```
intergammd tx ibc-transfer transfer transfer channel-1 "cosmos1vzxkv3lxccnttr9rs0002s93sgw72h7ghukuhs|transfer/channel-1:osm o1prsf8zac2t6uc6h67qcn7eqskf9w85r3p077wzqwkj8m03zq3e3qg0wvle" 1000stake --home run/quasar1/home --node=http://localhost:26659 --from bob --chain-id quasar1
```

9. transfer uatom ibc token from quasar to osmosis via cosmos

```
intergammd tx ibc-transfer transfer transfer channel-1 "cosmos1vzxkv3lxccnttr9rs0002s93sgw72h7ghukuhs|transfer/channel-1:osmo1prsf8zac2t6uc6h67qcn7eqskf9w85r3p077wzqwkj8m03zq3e3qg0wvle" 1000ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9 --home run/quasar1/home --node=http://localhost:26659 --from bob --chain-id quasar1
```
