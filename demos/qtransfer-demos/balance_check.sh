#!/bin/bash

echo "osmosis sender balance"
echo "user address - $(osmosisd keys show alice -a --keyring-backend test --home ~/.osmosis) "
osmosisd q bank balances $(osmosisd keys show alice -a --keyring-backend test --home ~/.osmosis) --node tcp://localhost:26679 -o json

echo "quasar receiver balance "
echo "user address - $(quasarnoded keys show alice -a --keyring-backend test --home ~/.quasarnode)"
quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --node tcp://localhost:26659 -o json

