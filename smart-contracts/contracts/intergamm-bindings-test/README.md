# Intergamm test contract
This contract demonstrates the usage of the intergamm 

## setting up
build the contract:
```
docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.6
```

Run the chains using the run_all script from the orion manual demo in a separate window:
```
./run_all.sh
```

once you see `starting relaying`, in a second terminal run the create_and_execute script

```
./create_and_execute.sh
```
Now a new contract should be deployed with a registered interchain account. The address of this contract can be found in the output of the script