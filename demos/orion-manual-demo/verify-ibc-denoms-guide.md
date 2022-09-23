This purpose of this guide is to demonstrate how to find
the IBC denoms of uqsr, uosmo, and uatom in other chains in your test setup and
find out if they match what we expect in the demos of this directory.

Our expectation:
* uqsr on osmosis: ibc/C18695C91D20F11FEE3919D7822B34651277CA84550EF33379E823AD9702B257
* uosmo on quasar: ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B
* uatom on quasar: inc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2
* uatom on osmosis: inc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2

Note: generally uatom should hae different denoms on quasar and osmosis.
The reason they're the same here is that it arrives via the same port and channel IDs. 

1. Check the initial balances (of account alice).
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
gaiad q bank balances cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu --node tcp://localhost:26669
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node tcp://localhost:26679
```

2. Transfer some uqsr to osmosis to find its IBC denom.
```
quasarnoded tx ibc-transfer transfer transfer channel-1 osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq 10000uqsr --from alice --chain-id quasar --home ~/.quasarnode/ --node tcp://localhost:26659 --keyring-backend test -y
```

3. Check the balances (of account alice) on osmosis and note the IBC denom of uqsr.
```
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node tcp://localhost:26679
```

4. Transfer some uosmo to quasar to find its IBC denom.
```
osmosisd tx ibc-transfer transfer transfer channel-1 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 10000uosmo --from alice --chain-id osmosis --home ~/.osmosis/ --node tcp://localhost:26679 --keyring-backend test -y
```

5. Check the balances (of account alice) on quasar and note the IBC denom of uosmo.
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
```

6. Transfer some uatom to quasar and osmosis to find its IBC denoms in those chains.
```
gaiad tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 10000uatom --from alice --chain-id cosmos --home ~/.gaia/  --node tcp://localhost:26669 --keyring-backend test -y
gaiad tx ibc-transfer transfer transfer channel-1 osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq 10000uatom --from alice --chain-id cosmos --home ~/.gaia/  --node tcp://localhost:26669 --keyring-backend test -y
```

7. Check the balances (of account alice) on quasar and osmosis and note the IBC denom of uatom in both chains.
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node tcp://localhost:26679
```

8. If the IBC denoms found in previous steps are different from the expectations listed in the beginning,
   edit the `quasar_denom_to_native_zone_id_map-proposal.json` and
   `osmosis_denom_to_quasar_denom_map-proposal` accordingly.
