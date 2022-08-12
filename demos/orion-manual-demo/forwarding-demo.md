1. Clone quasar-finance fork of gaia and checkout `bugfix/replace_default_transfer_with_router_module` branch, then cd into it.
```
git clone git@github.com:quasar-finance/gaia.git -b bugfix/replace_default_transfer_with_router_module
cd gaia
```

2. Update the dependencies with `go mod download` and rebuild gaia with `make install`

3. Go into quasar dir and then into `demos/orion-manual-demo`

4. Run `quasar_localnet.sh`, `osmo_localnet.sh`, and `cosmos_localnet.sh` in 4 separate terminals.
 Wait until all of them are initialized and recording blocks.

5. Run `run_hermes.sh` in a separate terminal. Wait until you see "INFO ThreadId(01) Hermes has started" message.

6. Check the initial balances
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
gaiad q bank balances cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu --node tcp://localhost:26669
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node tcp://localhost:26679
```

7. Transfer some atom to quasar and gaia
```
gaiad tx ibc-transfer transfer transfer channel-0 "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec" 100uatom --from alice --chain-id cosmos --home ~/.gaia/  --node tcp://localhost:26669 --keyring-backend test -y

gaiad tx ibc-transfer transfer transfer channel-1 "osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq" 100uatom --from alice --chain-id cosmos --home ~/.gaia/  --node tcp://localhost:26669 --keyring-backend test -y
```

8. Optionally you can transfer some atom back to gaia to see if the quasar->gaia route works.
```
quasarnoded tx ibc-transfer transfer transfer channel-0 "cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu" 5ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y
```

9. Finally transfer some atom from quasar to osmosis via gaia
```
quasarnoded tx ibc-transfer transfer transfer channel-0 "cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu|transfer/channel-1:osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq" 20ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y
```

