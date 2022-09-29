# ICQ smart contract demo

## Demo Contents
This demo runs 2 local chains, Quasar and Osmosis, and sets up an ibc connection between them using the `run_all.sh` script. Using the demo, we then compile the icq demo smart contract, deploy the contract, create the necessary IBC channel and send a query over icq using our contract.

## Requirements
In order to run this demo, a local `quasarnoded` and a local `osmosisd` binary need to be present. The local `quasarnoded` only needs to have wasm intergrated, and thus can be built from main or any recent branch. The `osmosisd` binary needs to have icq with the correct packet format integrated. A working version can be built from our osmosis branch found [here](https://github.com/quasar-finance/osmosis/tree/feature/new_icq_packet_format) and should be built with `make install`.

## Instructions
Assuming you're in the icq-smart-contract directory,
first, run ```./run_all.sh``` and wait until you see `starting relaying`. Now in a seperate window run ```./create_and_execute_contract.sh```. The script will compile our smart contracts, store the `icq` contract on the quasar chain, instantiate the contract, open an IBC channel between our contract and the `icqhost` on the osmosis chain, and send a query over icq. The user will be propmted to send this query. 