## Queries 

qbank module supports the following queries.

## Following CLI queries are being supported by the qbank module. 

## Query the current total deposit of a user
quasarnoded query qbank user-deposit [user-acc] [flags] 

## Query the current total denom-wise deposit 
quasarnoded query qbank user-denom-deposit [user-acc] [flags]

## Query the users epoch wise lockup period despoit
This query will return tokens deposited on a specific epoch day with the specified lockup period and denom. 
quasarnoded query qbank user-denom-epoch-lockup-deposit [user-acc] [denom] [epoch-day] [lockup-type] [flags]
TODO - should change the sequence of arguments.

## Query the current total withdrawable amount
quasarnoded query qbank user-withdraw [user-acc] [flags]

## Query the current total denom-wise withdrawable amount
quasarnoded query qbank user-denom-withdraw [user-acc] [denom] [flags]

## Query the current reward amount available for claim
quasarnoded query qbank user-claim-rewards [user-acc] [flags]

### Below are the common flags that can be used in any query command 

Flags:
      --height int      Use a specific height to query state at (this can error if the node is pruning state)
  -h, --help            help for user-deposit
      --node string     <host>:<port> to Tendermint RPC interface for this chain (default "tcp://localhost:26657")
  -o, --output string   Output format (text|json) (default "text")

Global Flags:
      --chain-id string     The network chain ID (default "quasarnode")
      --home string         directory for config and data (default "/home/ak/.quasarnode")
      --log_format string   The logging format (json|plain) (default "plain")
      --log_level string    The logging level (trace|debug|info|warn|error|fatal|panic) (default "info")
      --trace               print out full stack trace on errors
