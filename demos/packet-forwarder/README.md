# IBC Packet forwarder demo

1. Start 3 blockchains, quasar 1, quasar 2 and osmosis locally

```bash
./start all
```

1. Configure the `transfer` channel on ignite relayer only if you want to play with ibc transferred tokens

quasar <-> osmosis
```bash
ignite relayer configure \
    --source-rpc "http://localhost:26659" \
    --source-faucet "http://localhost:4500" \
    --source-account default \
    --source-gaslimit 300000 \
    --source-gasprice 0.00025stake \
    --source-prefix quasar \
    \
    --target-rpc "http://localhost:26559" \
    --target-faucet "http://localhost:4501" \
    --target-account default \
    --target-gaslimit 300000 \
    --target-gasprice 0.00025stake \
    --target-prefix osmo
```

quasar <-> cosmos (prefix is quasar because we run the quasar chain as cosmos)
```bash
ignite relayer configure \
    --source-rpc "http://localhost:26659" \
    --source-faucet "http://localhost:4500" \
    --source-account default \
    --source-gaslimit 300000 \
    --source-gasprice 0.00025stake \
    --source-prefix quasar \
    \
    --target-rpc "http://localhost:26669" \
    --target-faucet "http://localhost:4502" \
    --target-account default \
    --target-gaslimit 300000 \
    --target-gasprice 0.00025stake \
    --target-prefix quasar
```

osmosis <-> cosmos (prefix is quasar because we run the quasar chain as cosmos)
```bash
ignite relayer configure \
    --source-rpc "http://localhost:26559" \
    --source-faucet "http://localhost:4501" \
    --source-account default \
    --source-gaslimit 300000 \
    --source-gasprice 0.00025stake \
    --source-prefix osmo \
    \
    --target-rpc "http://localhost:26669" \
    --target-faucet "http://localhost:4502" \
    --target-account default \
    --target-gaslimit 300000 \
    --target-gasprice 0.00025stake \
    --target-prefix quasar
```

1. Start the ignite relayer and wait for it to finish creating the connection(s)

```
ignite relayer connect
```

1. transfer uatom from cosmos to osmosis

```
quasarnoded tx ibc-transfer transfer transfer channel-1 osmo1yuwm0nan9ls3jehjzzqq8xmk2pf090d2hsc5f2 2000uatom --home run/cosmos/home --node=http://localhost:26669 --from bob --chain-id cosmos
```

1. transfer stake token from quasar to osmosis via cosmos

Cosmos receiver address should be either alice or bob

```
quasarnoded tx ibc-transfer transfer transfer channel-1 "quasar1ve377dulznw5t46679e7uzl0s7q4uw6ze8sc25|transfer/channel-1:osmo1yuwm0nan9ls3jehjzzqq8xmk2pf090d2hsc5f2" 1000stake --home run/quasar/home --node=http://localhost:26659 --from bob --chain-id quasar
```

1. transfer uatom ibc token from quasar to osmosis via cosmos

```
quasarnoded tx ibc-transfer transfer transfer channel-1 "cosmos1vzxkv3lxccnttr9rs0002s93sgw72h7ghukuhs|transfer/channel-1:osmo1prsf8zac2t6uc6h67qcn7eqskf9w85r3p077wzqwkj8m03zq3e3qg0wvle" 1000ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9 --home run/quasar/home --node=http://localhost:26659 --from bob --chain-id quasar
```

## Random addresses

```
quasar1y9xka4klqwwmmkhcjz0k73wv0xxxdypmlr27l4
quasar19vax5d2jhvjvezly598za2wraaw0k3k9ynwdea
quasar14wmt9vejhsjaax5auesn4rucg8ssc8wf9kzmvr
quasar1k9xecg0hnyyswkm4s08ez36h2z59z3mnqlscha
quasar1tnt0vt62a0yeewr8cyvnge0fg3hnhqdwlgxl6d
quasar1jl00hu6lzjunrmkn6ansgfs300732249h7wxn9

osmo1yuwm0nan9ls3jehjzzqq8xmk2pf090d2hsc5f2
osmo1q0zdvw9824va4xezpusjq228rrjrez6cf4k0px
osmo184ernax4dk28vkdgvn8u646weu9m8s5vv2qfkc
osmo1uasyqspm9nljky8f0zfk06a3g7zw24acsvkf92
```
