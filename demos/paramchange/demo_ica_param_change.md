# Example commands 
`
osmosisd tx gov submit-proposal param-change ica-host-osmo.json --node tcp://localhost:26679 --from alice   --chain-id osmosis --keyring-backend test --home ~/.osmosis --output json  | jq
`
`
osmosisd  q gov proposals --node tcp://localhost:26679  --chain-id osmosis  --output json | jq
`
`
osmosisd tx gov vote 3 yes --node tcp://localhost:26679 --chain-id osmosis --from alice --home ~/.osmosis --keyring-backend test --output json | jq
`
`
osmosisd  q gov proposals --node tcp://localhost:26679  --chain-id osmosis  --output json | jq
`
`
osmosisd q interchain-accounts host param --node tcp://localhost:26679
`
