This demo shows us how to setup an environment and test packet forwarding.

1. Runn the steps described in the `run_integrated_testnet.md` to initialize the chains and the channels between them.

2. Check the initial balances
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
gaiad q bank balances cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu --node tcp://localhost:26669
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node tcp://localhost:26679
```

3. Transfer some atom to quasar and gaia
```
gaiad tx ibc-transfer transfer transfer channel-0 "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec" 100uatom --from alice --chain-id cosmos --home ~/.gaia/  --node tcp://localhost:26669 --keyring-backend test -y

gaiad tx ibc-transfer transfer transfer channel-1 "osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq" 100uatom --from alice --chain-id cosmos --home ~/.gaia/  --node tcp://localhost:26669 --keyring-backend test -y
```

4. Optionally you can transfer some atom back to gaia to see if the quasar->gaia route works.
```
quasarnoded tx ibc-transfer transfer transfer channel-0 "cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu" 5ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y
```

5. Finally transfer some atom from quasar to osmosis via gaia
```
quasarnoded tx ibc-transfer transfer transfer channel-0 "cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu|transfer/channel-1:osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq" 20ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y
```

