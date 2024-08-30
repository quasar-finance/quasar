# ICA smart contract
This demo shows how to set and run the ica smart contract. Currently, this is not expected to work against the current Osmosis release. In order to run this, the osmosis binary has to be built from a version that replaces ibc-go with https://github.com/quasar-finance/ibc-go/tree/laurens-hacking, a prepared version of osmosis for this can be found at https://github.com/quasar-finance/osmosis/tree/laurens/hacking

## Setup
Running commands from the ica-smart-contract demo. First, run `./run_all` to setup a quasar and osmosis chain and start the relayer to relay messages between them. Once you see "starting relaying", in a separate terminal, run `./create_and_execute_contract`. This builts the smart contract, deploys it and sends a message over the channel to osmosis.

## Seeing the results
open the log `logs/quasar_osmosis.log`, there should be a transaction in there, check the RecvPacket by querying the hash on Osmosis, like: `osmosisd query tx 7D0AE9B63472766EEC0FEDDA66BABE94C96C1657674DC2C1ABF5F55AF1279A33 --node http://127.0.0.1:26679`, checking the ack on quasar: `quasard query tx CEA4814A9B8578ED82B0CFE7072A2E120C4E5C3E2EA95E7B9CAA0332A2B9F062`