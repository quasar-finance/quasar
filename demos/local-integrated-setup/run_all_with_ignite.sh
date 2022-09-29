#!/bin/sh

cd ~/quasar-demo/quasar
ignite chain serve -c demos/orion-manual-demo/quasar.yml  --reset-once --home demos/orion-manual-demo/run/home/quasarnode/  -v  > quasar.log 2>&1 &

cd ~/quasar-demo/gaia
go mod tidy -go=1.16 && go mod tidy -go=1.17
ignite chain serve -c  ~/quasar-demo/quasar/demos/orion-manual-demo/cosmos.yml  --reset-once --home  ~/quasar-demo/quasar/demos/orion-manual-demo/run/home/cosmos-hub/ -v > cosmos.log 2>&1 & 

cd ~/quasar-demo/osmosis
ignite chain serve -c ~/quasar-demo/quasar/demos/orion-manual-demo/osmosis.yml  --reset-once --home  ~/quasar-demo/quasar/demos/orion-manual-demo/run/home/osmosis/ -v > osmosis.log 2>&1 &


