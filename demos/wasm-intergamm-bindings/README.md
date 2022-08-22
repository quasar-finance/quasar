# Wasm <> Intergamm
In this demo, we setup 3 chains, quasar, cosmos-hub and osmosis and we demonstrate that all messages from intergamm can be succesfully sent from a smart contract on quasar to osmosis and that the smart contract is able to receive and respond to acknowledgments from intergamm

## Prerequisites
- The compiled intergamm-bindings-test contract
- quasarnoded compiled
- Local  Gaia repo (in the same root folder as the Quasar repo)
- Local Osmosis repo (in the same root folder as the Quasar repo)

## Setting up
### Running the chains
run all three chains with 
```
ignite chain serve --config CORRESPONDING-CONFIG --reset-once -v
```
### Setting up the IBC channels
Using Hermes:
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
### Storing and Instantiating the test contract
### Depositing funds in the test contract
To actually send funds to osmosis using Intergamm, we need the smart contract to have some funds available for sending.
## Testing
The first operation should be to send tokens from the smart contract to Osmosis. With tokens deposited in the smart contract, we continue
### Calling the smart contract

### Checking the results
We want to confirm both sides of the transaction. Thus we need to check the result of the transfer
#### Check the transfer on Osmosis
Check balance of quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec from the link below, it should contain 10ibc/C9B459D627233FC6E5049A27B5465D76C7F69D7FCB7F843EAC2B6A38B668F9F1

http://localhost:1311/bank/balances/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec
#### Checking the state of the smart contract
Query our smart contract and see whether