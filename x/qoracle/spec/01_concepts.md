# Concepts

qoracle module stores the osmosis data in its prefixed KV store from the oracle client transaction messages. Oracle client is quering the data from the osmosis full node and broadcasting the transaction message to the quasar full node address. Oracle

On receiving the oracle client transaction messages qoracle module does the processing of the messages in its keeper and storing the osmosis pool data in its KV store. 

qoracle stores pool meta data, token spot prices, pool ranking, and pool positions ( Total value locked, and current APY). 

# NOTE - 
The current implementation of oracle client and qoracle module is for the testnet version only. Less security majors are taken care for the testnet version. 

This version of the oracle needs its client to run one single node for submitting oracle data to the chain qoracle module. 

Team is working on the secure and scalable mainnet spec version. However the core business logic and formula can be very well re-used in the final version like TVL and APY calculations.

