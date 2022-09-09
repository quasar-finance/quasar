This demo demonstrates how to set intergamm params through gov procedures
and how to test if their actually transfered to osmosis properly after deposit on quasar.

1. Runn the steps described in the `run_integrated_testnet.md` to initialize the chains and the channels between them.

2. Check the initial balances (of account alice).
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
gaiad q bank balances cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu --node tcp://localhost:26669
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node tcp://localhost:26679
```

3. Transfer some uatom to quasar and gaia to find its ibc denoms in those chains.
```
gaiad tx ibc-transfer transfer transfer channel-0 "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec" 10000uatom --from alice --chain-id cosmos --home ~/.gaia/  --node tcp://localhost:26669 --keyring-backend test -y

gaiad tx ibc-transfer transfer transfer channel-1 "osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq" 100uatom --from alice --chain-id cosmos --home ~/.gaia/  --node tcp://localhost:26669 --keyring-backend test -y
```

4. Check the balances (of account alice) on quasar and osmosis and note the ibc denom of uatom in both chains.
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node tcp://localhost:26679
```

5. If ibc denom of uatom on quasar is different from "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
modify the denom_to_native_zone_id_map-proposal.json file and change it.

6. Transfer some uosmo to quasar to find its ibc denom.
```
osmosisd tx ibc-transfer transfer transfer channel-1 "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec" 10000uosmo --from alice --chain-id osmosis --home ~/.osmosis/ --node tcp://localhost:26679 --keyring-backend test -y
```

7. Check the balances (of account alice) on quasar and note the ibc denom of uosmo.
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
```

8. If ibc denom of uosmo on quasar is different from "ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B",
modify the denom_to_native_zone_id_map-proposal.json file and change it.

9. Next we need to verify connection and channel IDs among chains.
All channels of a chain can be listed by:
```
hermes query channels osmosis
hermes query channels cosmos
hermes query channels quasar
```
Focus on channels whose port ID is "transfer".
For these channels view their details by:
```
hermes query channel ends osmosis transfer <channel-id>
```
For example:
```
hermes query channel ends osmosis transfer channel-0
```
Check their chain_id, counterparty_chain_id, channel_id, and counterparty_channel_id.
the channel IDs should be as follows:
quasar->osmosis channel-1
osmosis->quasar channel-1
quasar->cosmos  channel-0
cosmos->quasar  channel-0
cosmos->osmosis channel-1
osmosis->cosmos channel-0

10. If the channel IDs found in previous step are different,
edit the `complete_zone_info_map-proposal.json` accordingly.

11. Run the change_quasar_param.sh script to submit param change proposals and vote on them.
You need to wait 90 seconds until these changes takes effect.

12. Set stable prices of uqsr, uosmo, uatom.
```
quasarnoded tx qoracle stable-price uqsr 1.3 --from alice --chain-id quasar --home ~/.quasarnode/ --node tcp://localhost:26659 --keyring-backend test -y
quasarnoded tx qoracle stable-price ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B 1.3 --from alice --chain-id quasar --home ~/.quasarnode/ --node tcp://localhost:26659 --keyring-backend test -y
quasarnoded tx qoracle stable-price ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 1.3 --from alice --chain-id quasar --home ~/.quasarnode/ --node tcp://localhost:26659 --keyring-backend test -y
```

13. Submit request depost txs.
```
quasarnoded tx qbank request-deposit orion 1000uqsr Days_7 "" --from alice --chain-id quasar --home ~/.quasarnode/ --node tcp://localhost:26659 --keyring-backend test -y
quasarnoded tx qbank request-deposit orion 1000ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B Days_7 "" --from alice --chain-id quasar --home ~/.quasarnode/ --node tcp://localhost:26659 --keyring-backend test -y
quasarnoded tx qbank request-deposit orion 1000ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 Days_7 "" --from alice --chain-id quasar --home ~/.quasarnode/ --node tcp://localhost:26659 --keyring-backend test -y
```

14. Finally, check the balance of orion ICA on osmosis:
```
osmosisd q bank balances osmo1dzgzjwvtu77x7p36xh9ut0xpek34y706rt6p5djn038455qtxjmsg4akrw --node tcp://localhost:26679
```

