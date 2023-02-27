## Prerequisites
1. `go` v1.19
2. `hermes` v1.0.0
3.  osmosis repo with async-icq v15 
4.  quasar repo sdk45 ( main-sdk45)

## Setup
Install the main binary of osmosis and quasar  with the following command:
```bash
make install
```
Clone osmosis v15 release candidate branch. and install the osmosisd binary with the following commands:
clone the tag/v15.0.0-rc0 , https://github.com/osmosis-labs/osmosis/releases/tag/v15.0.0-rc0
```bash
git clone git@github.com:osmosis-labs/osmosis.git
cd ./osmosis
git checkout tags/v15.0.0-rc0
make install
```

## Clone quasar node and install 

```bash
git clone git@github.com:quasar-finance/quasar.git
cd ./quasar
git switch main-sdk45
make install
```

# MANUAL START 
## Starting Quasar node
Run the following commands to start a single node of quasar in local machine with the preset of parameters needed for this demo:
```bash
cd ./demos/qtransfer-demos
./quasar_localnet.sh
```
You can do tail -f quasar.log to see logs in the terminals.

## Starting Osmosis node
Run the following commands to start a single node of osmosis in local machine with the preset of parameters needed for this demo:

```bash
./osmo_localnet.sh
```
You can do tail -f osmosis.log to see logs in the terminals.

## Run Relayer
```
./run_hermes.sh
```
# AUTO START ALL PROCESSES 
To start quasar, osmosis and hermes in one go, use `run_all.sh`

```bash
./run_all.sh
```

## TOKEN TRANSFER 

To do ibc token transfer use `ibc_token_transfer.sh` to quickly do the steps.

If you are debugging ; attach the quasarnode process id with the IDE (goland/vscode) and set the breakpoints in wasm_hooks or ibc middle ware receiver methods. 

## TESTING ICS-20 ACKNOWLEDGEMENTS OVERRIDES.
