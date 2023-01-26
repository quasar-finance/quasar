# Demo steps to verify if osmosis can be used a hub for quasar vaults. 
## This demo will check if any denom ( say from other chains like cosmos-hub) can be transferred from osmosis 
## to quasar, and if this newly two hop denom can be used as a denom by quasar vault.  
## This will primarily checks that the flow can handle the denoms changes as it moves from one chain to another.
## And finally can be LPied on a pool


## STEP #1 Start the three chains cosmos-hub, quasar and osmosis using demos/local-integrated-setup/
```
cd demos/local-integrated-setup/
./run_all_chains_with_gentx.sh
```
## STEP #2 Do ibc connections and start relayer
```
cd demos/local-integrated-setup/hermes/
./run_hermes_v1_nobandchain.sh
```

## STEP #3 IBC Transfer uatom and uqsr from their native chains to osmosis. 
### Token movements are from alice account on cosmos-hub/quasar to osmosis 
```
gaiad tx ibc-transfer transfer transfer channel-1 $(osmosisd keys show alice -a  --keyring-backend test --home ~/.osmosis)  1000uatom --from alice --chain-id cosmos --home ~/.gaia --keyring-backend test --node tcp://localhost:26669
```
# denom on cosmos-hub -----> denom on osmosis 
# uatom ----> <ibc/transfer/channel-0/uatom>  OR  ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 
 
```
quasarnoded tx ibc-transfer transfer transfer channel-1 $(osmosisd keys show alice -a  --keyring-backend test --home ~/.osmosis)  1000uqsr --from alice --chain-id quasar --home ~/.quasarnode --keyring-backend test --node tcp://localhost:26659
```
# denom on quasar -----> denom on osmosis
# uqsr ----> <ibc/transfer/channel-1/uqsr>   OR  ibc/C18695C91D20F11FEE3919D7822B34651277CA84550EF33379E823AD9702B257

## NOTE - ibc denoms are predefined here due to pre established steps in setting up infra.

## STEP #4 Send Tokens from alice on osmosis to bob osmosis 

```
osmosisd tx bank send $(osmosisd keys show alice -a --home ~/.osmosis --keyring-backend test)  $(osmosisd keys show bob -a --home ~/.osmosis --keyring-backend test) 100ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2  --home ~/.osmosis --chain-id osmosis --node=http://localhost:26679 --from alice --output json --keyring-backend test
```

```
osmosisd tx bank send $(osmosisd keys show alice -a --home ~/.osmosis --keyring-backend test)  $(osmosisd keys show bob -a --home ~/.osmosis --keyring-backend test)  100ibc/C18695C91D20F11FEE3919D7822B34651277CA84550EF33379E823AD9702B257 --home ~/.osmosis --chain-id osmosis --node=http://localhost:26679 --from alice --output json --keyring-backend test
```


## STEP 5 Create Pools in osmosis dex using bobs account. 

## Make sure to check pool denoms in pool_1_ibc_uatom_uqsr_.json and pool_2_uosmo_ibcuqsr_.json

```
osmosisd tx gamm create-pool --pool-file pool_1_ibc_uatom_uqsr_.json --home ~/.osmosis --chain-id osmosis --node=http://localhost:26679 --from bob  --output json --keyring-backend test
```
 
```
osmosisd tx gamm create-pool --pool-file pool_2_uosmo_ibcuqsr_.json --home ~/.osmosis --chain-id osmosis --node=http://localhost:26679 --from bob  --output json --keyring-backend test
```

### Query if pools are created 
```
osmosisd q gamm pool 1  --node=http://localhost:26679
osmosisd q gamm pool 2  --node=http://localhost:26679
```

### Query bob balance on osmosis and check new shares token 
```
osmosisd query bank balances $(osmosisd keys show bob -a  --keyring-backend test --home ~/.osmosis) --node tcp://localhost:26679 
```
 
## STEP #6 
### Transfer 50 ibc uatom, and 50 uosmo from bob at osmosis to bob on quasar. 
### bob at quasar already have uqsr so don't need to txfer ibc uqsr 

```
osmosisd tx ibc-transfer transfer transfer channel-1 $(quasarnoded keys show bob -a  --keyring-backend test --home ~/.quasarnode)  1000uosmo --from bob --chain-id osmosis --home ~/.osmosis --keyring-backend test --node tcp://localhost:26679
```
# denom on osmosis -----> denom on quasar
# usomo ----> <ibc/transfer/channel-1/uosmo>   OR  ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B

### Transfer ibc uatom ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 from osmosis bob to quasar's bob 
```
osmosisd tx ibc-transfer transfer transfer channel-1 $(quasarnoded keys show bob -a  --keyring-backend test --home ~/.quasarnode)  50ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2  --from bob --chain-id osmosis --home ~/.osmosis --keyring-backend test --node tcp://localhost:26679
```

## At this point bob at quasar will have both 1000 uosmo -> ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B, and 
## 50 ibc/FA0006F056DB6719B8C16C551FC392B62F5729978FC0B125AC9A432DBB2AA1A5 [ two hop uatom ]
## uatom -> (on osmosis ) ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 -> (on quasar) ibc/FA0006F056DB6719B8C16C551FC392B62F5729978FC0B125AC9A432DBB2AA1A5

### Query bob's balance on quasar 
```
quasarnoded query bank balances $(quasarnoded keys show bob -a  --keyring-backend test --home ~/.quasarnode) --node tcp://localhost:26659
```

## STEP #6 Sending bobs uatom , uosmo and uqsr from quasar chain to osmosis chain's user1 account 

### send uqsr
```
quasarnoded tx ibc-transfer transfer transfer channel-1 $(osmosisd keys show user1 -a  --keyring-backend test --home ~/.osmosis)  1000uqsr --from bob --chain-id quasar --home ~/.quasarnode --keyring-backend test --node tcp://localhost:26659 
```
### send uosmo 
```
quasarnoded tx ibc-transfer transfer transfer channel-1 $(osmosisd keys show user1 -a  --keyring-backend test --home ~/.osmosis)  300ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B --from bob --chain-id quasar --home ~/.quasarnode --keyring-backend test --node tcp://localhost:26659
```
### send uatom 
```
quasarnoded tx ibc-transfer transfer transfer channel-1 $(osmosisd keys show user1 -a  --keyring-backend test --home ~/.osmosis) 50ibc/FA0006F056DB6719B8C16C551FC392B62F5729978FC0B125AC9A432DBB2AA1A5 --from bob --chain-id quasar --home ~/.quasarnode --keyring-backend test --node tcp://localhost:26659
```
### Query user1 account on osmosis 
```
osmosisd query bank balances $(osmosisd keys show user1 -a  --keyring-backend test --home ~/.osmosis) --node tcp://localhost:26679
```
```
balances:
- amount: "50"
  denom: ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2
- amount: "1000"
  denom: ibc/C18695C91D20F11FEE3919D7822B34651277CA84550EF33379E823AD9702B257
- amount: "10000000000"
  denom: stake
- amount: "10000000300"
  denom: uosmo
pagination:
  next_key: null
  total: "0"
```

### STEP #7 Join pool from user1 

```
osmosisd tx gamm join-pool --pool-id 1 --max-amounts-in 10ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2,10ibc/C18695C91D20F11FEE3919D7822B34651277CA84550EF33379E823AD9702B257 --share-amount-out 1000000000000000000 --from user1 --keyring-backend test --home ~/.osmosis --node tcp://localhost:26679 --chain-id osmosis
```
## Verify Join pool operations by chekcking pool states for increased shares and lp tokens 

```
osmosisd q gamm pools --node tcp://localhost:2667
```