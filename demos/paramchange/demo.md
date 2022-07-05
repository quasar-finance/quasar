
# This demo demonstrate the param change gov procedure

1. Kill any previous running chain binary. 
2. Clean previous chain state
3. And clone new chain source code. and Run the chain
`
ignite chain serve -c demos/paramchange/quasar.yml --home run/quasar/home --reset-once -v
`
4. Submit mind module param change proposal
`
quasarnoded tx gov submit-proposal param-change demos/paramchange/mint_param_change.json --node tcp://localhost:26659 --from quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --home run/quasar/home --chain-id quasar --output json | jq

`
5. Query the proposal state.
`
quasarnoded q gov proposals --node tcp://localhost:26659  --chain-id quasar  --output json | jq
`

6. Query the mint param and notice value
`
quasarnoded q mint params --node tcp://localhost:26659  --chain-id quasar  --output json | jq

`
7. Vote 
`
quasarnoded tx gov vote 1 yes --node tcp://localhost:26659   --chain-id quasar --from alice --output json | jq

`

8. Query the mint param again after 5 minutes of configured voting period.
`
quasarnoded q mint params --node tcp://localhost:26659  --chain-id quasar  --output json | jq
`
