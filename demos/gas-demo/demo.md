# How to use other denom as gas price

I did a simple experiment to try with gas estimation and payment with other denom with bank tx.
- The minimum-gas-prices is set to `0.001stake` in the app.toml 
- In the real scenario; stake could be ibc hex hash of `uosmo` present in the quasar chain. 
- Signer can set the gas price he is willing to pay using --gas-price 

 `

## Estimate the gas using dry run
`
quasard tx bank send quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu  --node tcp://localhost:26659 --from quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 10uqsr  --home ~/.quasarnode --chain-id quasar --keyring-backend test  --gas-prices 0.01stake --gas-adjustment 1 --dry-run
`
gas estimate: 68607


## With Expected gas fee around 200, --gas-prices 0.001stake --gas-adjustment 1
`
quasard tx bank send quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu  --node tcp://localhost:26659 --from quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 10uqsr  --home ~/.quasarnode --chain-id quasar --keyring-backend test  --gas-prices 0.001stake --gas-adjustment 1
`
-
- Actual Gas was - 66983
- Fee deducted - 200stake


## Expected gas fee 2000, --gas-prices 0.01stake --gas-adjustment 1

`
quasard tx bank send quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu  --node tcp://localhost:26659 --from quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 10uqsr  --home ~/.quasarnode --chain-id quasar --keyring-backend test  --gas-prices 0.01stake --gas-adjustment 1 --dry-run
gas estimate: 68607
`

`
quasard tx bank send quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu  --node tcp://localhost:26659 --from quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 10uqsr  --home ~/.quasarnode --chain-id quasar --keyring-backend test  --gas-prices 0.01stake --gas-adjustment 1
`

- Actual Gas was - 66983
- Fee deducted 2000stake 

## By providing fees explicitly using --fees flag 

### Determine the expected fee without providing any fee details ; the output will tell you the amount of fees required 
`
quasard tx bank send quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu 10uqsr  --node tcp://localhost:26659 --from alice --home ~/.quasarnode --chain-id quasar --keyring-backend test 
`
-output 
`
{"body":{"messages":[{"@type":"/cosmos.bank.v1beta1.MsgSend","from_address":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec","to_address":"quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu","amount":[{"denom":"uqsr","amount":"10"}]}],"memo":"","timeout_height":"0","extension_options":[],"non_critical_extension_options":[]},"auth_info":{"signer_infos":[],"fee":{"amount":[],"gas_limit":"200000","payer":"","granter":""}},"signatures":[]}

confirm transaction before signing and broadcasting [y/N]: y
code: 13
codespace: sdk
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: 'insufficient fees; got:  required: 200stake: insufficient fee'
timestamp: ""
tx: null
txhash: 37759D295A02620E72ED356E0A8F5090140CBD73843F02C886A4731FE9807844
`

### Do the same tx using -fees flag ;this time the tx will pass through 
`
quasard tx bank send quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu 10uqsr  --node tcp://localhost:26659 --from alice --home ~/.quasarnode --chain-id quasar --keyring-backend test  --fees 200stake
`
