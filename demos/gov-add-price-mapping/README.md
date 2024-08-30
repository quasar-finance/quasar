
# This demo demonstrate the param change gov procedure

1. Kill any previous running chain binary. 
2. Clean previous chain state
3. And clone new chain source code. and Run the chain
`
ignite chain serve -c demos/gov-add-price-mapping/config.yml --home run/quasar/home --reset-once -v
`
4. Submit proposal
`
quasard tx gov submit-proposal param-change demos/gov-add-price-mapping/proposal.json --node tcp://localhost:26659 --from alice  --home run/quasar/home --chain-id quasar --output json | jq

`
5. Query the proposal state.
`
quasard q gov proposals --node tcp://localhost:26659  --chain-id quasar  --output json | jq
`

6. Query the qoracle params and notice value
`
quasard q qoracle params --node tcp://localhost:26659  --chain-id quasar  --output json | jq

`
7. Vote 
`
quasard tx gov vote 1 yes --node tcp://localhost:26659 --chain-id quasar --from alice --home run/quasar/home --chain-id quasar --output json | jq

`

8. Query the qoracle params again after 5 minutes of configured voting period.
`
quasard q qoracle params --node tcp://localhost:26659  --chain-id quasar  --output json | jq
`