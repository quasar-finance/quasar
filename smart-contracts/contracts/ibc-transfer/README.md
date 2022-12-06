# ibc-transfer demo contract
This contract is modified from Neutrons ibc-transfer demo contract.

The main difference is they use a Sudo message to re-enter the contract, while we comply with the ibc standards for it

## IBC transfer contract
Interacting with counterpart chain via ibc transfer is two phases process.
1. Send ibc transfer message
2. Accept and process ibc acknowlegement(ibc_packet_ack call)

to run the contract you need to init two chain network connected with go relayer


## See demos/ibc-transfer-test/README.md for running instructions