This demo describes how to run local quasar, osmosis, and gaia chains,
and how to connect them to each other and to the band testnet using hermes IBC relayer.

1. Clone quasar-finance fork of gaia and checkout `bugfix/replace_default_transfer_with_router_module` branch, then cd into it.
```
git clone git@github.com:quasar-finance/gaia.git -b bugfix/replace_default_transfer_with_router_module
cd gaia
```

2. Update the dependencies with `go mod download` and rebuild gaia with `make install`

3. Clone osmosis and band, and build them with `make install`. Also rebuild quasar if not updated.
```
cd ..
clone git@github.com:bandprotocol/chain.git band
cd band
make install
cd ..
clone git@github.com:osmosis-labs/osmosis.git
cd osmosis
make install
```

4. Go into quasar dir and then into `demos/orion-manual-demo`

5. Run `band_testnet_init.sh` to initialize local config for band testnet.

6. Run `quasar_localnet.sh`, `osmo_localnet.sh`, and `cosmos_localnet.sh` in 3 separate terminals.
 Wait until all of them are initialized and recording blocks.

7. Run `run_hermes.sh` in a separate terminal. Wait until you see "INFO ThreadId(01) Hermes has started" message.

