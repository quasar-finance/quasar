# Splitter

Splitter is a contract to split the underlying balance of the contract. The contract works by setting a set of receivers and then splits the underlying balance according to the 
preset receivers. The contract allows for an admin to be set and for that admin to update the receivers

## Claiming
The contract comes with an optional features of claiming rewards. This option allows for the contract to call any contract with an arbitrary message. Clearly this is not safe for the usecase of any cw-20 tokens. So this option should be used with caution

## Setting receivers
On instantiation of the contract, the receivers have to be set with a total `share` of 1.0. This is the same for updating receivers through the admin endpoint.