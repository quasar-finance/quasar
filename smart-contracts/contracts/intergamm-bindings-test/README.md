# Intergamm test contract
This contract demonstrates the usage of the intergamm

## Prerequisites
A local quasarnoded, osmosisd and gaiad binary.
The quasarnoded binary can be installed by running 
```
go install go install ../../../cmd/quasarnoded/
```

For the osmosisd and gaiad, you need to build those from their respective source, which can be found here:
[Osmosis](https://github.com/osmosis-labs/osmosis)
[Gaia](https://github.com/cosmos/gaia)

## setting up
Run the chains using the [run_all](../../../demos/orion-manual-demo/run_all.sh) script from the orion manual demo in a separate window:
```
./run_all.sh
```

once you see `starting relaying`, in a second terminal run the [create_and_execute](../../../demos/orion-manual-demo/create_and_execute_contract.sh) script

```
./create_and_execute.sh
```

Now a new contract should be deployed with a registered interchain account. The address of this contract can be found in the output of [create_and_execute](../../../demos/orion-manual-demo/create_and_execute_contract.sh)

The created contract contains multiple execute messages to use the different intergamm messages from the intergamm-bindings package. The easiest way to get the correct funds on the interchain account address on cosmos is to send funds from alice/bob to the newly created interchain account.
A sample pool can be made using 
```
osmosisd tx gamm create-pool --pool-file ./sample_pool.json --node tcp://localhost:26679 --from bob  --chain-id osmosis --gas auto
```