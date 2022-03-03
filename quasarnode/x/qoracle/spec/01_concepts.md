# Concepts

qoracle module stores the osmosis data in its prefixed KV store from the oracle client transaction messages. Oracle client is quering the data from the osmosis full node and broadcasting the transaction message to the quasar full node address. Oracle

On receiving the oracle client transaction messages qoracle module does the processing of the messages in its keeper and storing the osmosis pool data in its KV store. 

qoracle stores pool meta data, token spot prices, pool ranking, and pool positions ( Total value locked, and current APY). 

