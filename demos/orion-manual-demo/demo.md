
# Set up - 
- Up the quasar chain
- Up the cosmos-hub chain
- Up the osmosis chain
- Relayer connect quasar and cosmos-hub
- Relayer connect quasar and osmosis
- Relayer connect osmosis and cosmos-hub

# Scenarios 
- IBC transfer 10000 uatom from cosmos-hub to quasar to alice account .
- IBC transfer 30000 uosmo from osmosis to quasar.
- Create pool uatom - usmo pool in osmosis.
- Deposit ibc uatom to orion vault using qbank for 7 days. And verify account balance. 
- Deposit ibc uosmo to orion vault using qbank for 7 days. And verify account balance.
- Verify the Join pool is happening or not. 
- Note down all the module accounts. 


Commands - 
## Prerequisites

1. `go` version 1.18
2. `ignite` latest version should be installed (see https://docs.ignite.com/guide/install.html)
3. The cosmos-hub `gaia` repo should be cloned from our fork https://github.com/quasar-finance/gaia and the branch `bugfix/replace_default_transfer_with_router_module` should be checked out.

## Set up
Create a demo directory in home directory. 
Clone a quasar and create a following directory  structure. 
```
mkdir quasar-demo
cd quasar-demo
```
- clone quasar, gaia and osmosis.
- For osmosis use,  git clone git@github.com:schnetzlerjoe/osmosis.git osmosis
- For gaia use, https://github.com/quasar-finance/gaia branch bugfix/replace_default_transfer_with_router_module

## Up the quasar-chain, in the cloned quasar directory. And use the already prepared config from demos/orion-manual-demo/quasar.yml
```
cd quasar-demo/quasar/
ignite chain serve -c demos/orion-manual-demo/quasar.yml  --reset-once --home demos/orion-manual-demo/run/home/quasarnode/  -v  > quasar.log 2>&1
```
You can `tail -f quasar.log` in a separate terminal to continusly check the logs. 

## Up the osmosis chain, in the cloned osmosis ( with ica ) directory. And use the already prepared config from demos/orion-manual-demo/osmosis.yml
```
cd quasar-demo/osmosis
ignite chain serve -c ~/quasar-demo/quasar/demos/orion-manual-demo/osmosis.yml  --reset-once --home  ~/quasar-demo/quasar/demos/orion-manual-demo/run/home/osmosis/ -v > osmosis.log 2>&1
```
You can `tail -f osmosis.log` in a separate terminal to continusly check the logs. 

## Up the cosmos-hub chain, in the gaid cloned directory. And use the already prepared config from demos/orion-manual-demos/cosmos.yml 
```
ignite chain serve -c  ~/quasar-demo/quasar/demos/orion-manual-demo/cosmos.yml  --reset-once --home  ~/quasar-demo/quasar/demos/orion-manual-demo/run/home/cosmos-hub/ -v > cosmos.log 2>&1
```
You can `tail -f cosmos.log` in a separate terminal to continusly check the logs. 

## Connecting the chains

### Connect quasar and cosmos 

### Connect quasar and osmosis

### Connect osmosis and cosmos hub


