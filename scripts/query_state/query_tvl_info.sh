#!/bin/bash

VAULT_ADDR="quasar1xzqhz0q969plap7awdjpls6vvrq57efk5vlkwr7kj5rzw9sq8j6s6wnxaj"

## try with the new contract
# output=$(quasarnoded q wasm contract-state smart $VAULT_ADDR '{"get_tvl_info":{}}' --node https://rpc-tst5.qsr.network:443)
# echo $output