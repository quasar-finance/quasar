This demo demonstrates how to set intergamm params through gov procedures
and how to test intergamm token transferred.

There are 6 cases that need to be tested:
1. transfer quasar-native token (e.g. uqsr) from quasar to osmosis
2. transfer quasar-native token (e.g. uqsr) from osmosis to quasar
3. transfer osmosis-native token (e.g. uosmo) from quasar to osmosis
4. transfer osmosis-native token (e.g. uosmo) from osmosis to quasar
5. transfer 3rd party token (e.g. uatom) from quasar to osmosis
6. transfer 3rd party token (e.g. uatom) from osmosis to quasar

# Preparing the test setup

1. Run the steps described in the `run_integrated_testnet.md` to initialize the chains and the channels between them.

2. Run the steps described in the `verify-channel-ids-guide.md` and check if your channel IDs match what we assume in these guides.

3. Run the steps described in the `verify-ibc-denoms-guide.md` and check if the IBC denoms in your test setup match what we assume in these guides.

4. Check the initial balances of alice on quasar.
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
```

5. Set the intergamm module parameters by running `change_quasar_param.sh`.
   This scripts submits needed gov proposals and votes on them.
   You need to wait 90 seconds until these changes takes effect.

6. Register an ICA for alice on osmosis (necessary for testing cases #2, #4, and #6).
```
quasarnoded tx intergamm register-ica-on-zone osmosis --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y -b block
```
Wait for about 30 seconds until the ICA and its channels are fully initialized and opened.
If Successful the following command should output the ICA address of alice on osmosis:
```
quasarnoded q intergamm ica-address-on-zone $(quasarnoded keys show -a alice --keyring-backend test) osmosis
```

6. Register an ICA for alice on the native zone of uatom (necessary for testing cases #5 and #6).
```
quasarnoded tx intergamm register-ica-on-denom-native-zone ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y -b block
```
Wait for about 30 seconds until the ICA and its channels are fully initialized and opened.
If Successful the following command should output the ICA address of alice on osmosis:
```
quasarnoded q intergamm ica-address-on-denom-native-zone $(quasarnoded keys show -a alice --keyring-backend test) ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2
```

7. Check quasar balance of alice and also her ICA balance on osmosis
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
osmosisd q bank balances osmo1hphwfu3yjf82z8xpcl6e05gzkjwjmu8ts2m97mdk62feuqm77f2skm6qcy --node tcp://localhost:26679
```

# Testing case #1 and #2 (uqsr)

1. Test case #1 (uqsr from quasar to osmosis):
```
quasarnoded tx intergamm send-token-to-ica osmosis 5000uqsr --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y -b block
```
This command sends tokens from alice account on quasar to her ICA on osmosis.
You can also use `quasarnoded tx intergamm send-token` to send tokens to an arbitrary account on osmosis,
but for testing case #2 you need to send to alice's ICA.

2. Check balances (after a few seconds):
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
osmosisd q bank balances osmo1hphwfu3yjf82z8xpcl6e05gzkjwjmu8ts2m97mdk62feuqm77f2skm6qcy --node tcp://localhost:26679
```

3. Test case #2 (uqsr from osmosis to quasar):
```
quasarnoded tx intergamm transmit-ica-transfer quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 4000ibc/C18695C91D20F11FEE3919D7822B34651277CA84550EF33379E823AD9702B257 --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y -b block
```

# Testing case #3 and #4 (uosmo)

1. Test case #3 (uosmo from quasar to osmosis):
```
quasarnoded tx intergamm send-token-to-ica osmosis 5000ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y -b block
```
This command sends tokens from alice account on quasar to her ICA on osmosis.
You can also use `quasarnoded tx intergamm send-token` to send tokens to an arbitrary account on osmosis,
but for testing case #4 you need to send to alice's ICA.

2. Check balances (after a few seconds):
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
osmosisd q bank balances osmo1hphwfu3yjf82z8xpcl6e05gzkjwjmu8ts2m97mdk62feuqm77f2skm6qcy --node tcp://localhost:26679
```

3. Test case #4 (uosmo from osmosis to quasar):
```
quasarnoded tx intergamm transmit-ica-transfer quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 4000uosmo --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y -b block
```

# Testing case #5 and #6 (uatom)

1. Test case #5 (uatom from quasar to osmosis):
```
quasarnoded tx intergamm send-token-to-ica osmosis 5000ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y -b block
```
This command sends tokens from alice account on quasar to her ICA on osmosis.
You can also use `quasarnoded tx intergamm send-token` to send tokens to an arbitrary account on osmosis,
but for testing case #6 you need to send to alice's ICA.

2. Check balances (after a few seconds):
```
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
osmosisd q bank balances osmo1hphwfu3yjf82z8xpcl6e05gzkjwjmu8ts2m97mdk62feuqm77f2skm6qcy --node tcp://localhost:26679
```

3. Test case #6 (uatom from osmosis to quasar):
```
quasarnoded tx intergamm transmit-ica-transfer quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 4000ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2 --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test -y -b block
```
