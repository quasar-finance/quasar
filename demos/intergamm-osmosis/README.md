# Quasar Intergamm <> Osmosis Integration demo

This demo tutorial demonstrates how Quasar integrates with Osmosis by calling the Osmosis pool functions. Two chains need to run locally `quasar` and `osmosis`.

Repo links
1. [quasar](https://github.com/quasar-finance/quasar)
2. [osmosis](https://github.com/osmosis-labs/osmosis)

## Prerequisites

1. The ICA ready version of the `osmosis` repository need to be cloned in a `contrib/` directory at the same level as the `quasar` repository.

```bash
cd ../contrib
git clone git@github.com:schnetzlerjoe/osmosis.git osmosis-ica
```

3. `osmosis` require go version 1.18
4. `ignite` latest version should be installed (see https://docs.ignite.com/guide/install.html)
5. `gnome terminal` should also be installed if not already installed, to spawn terminal windows.

For ubuntu:

```bash
sudo apt-get install gnome-terminal
```

## Setup

1. Go to the `quasar` repo directory, and cd `intergamm-osmosis`. All steps below will be run from this directory.

```bash
cd demos/intergamm-osmosis
```

A `demo` script is there to run all the demo steps.

2. Start the 2 blockchains locally

```bash
./demo start_all
```

3. Configure and start the `transfer` channel on ignite relayer only and wait for it to finish creating the connections, it might take a couple of minutes.

```bash
./demo init_relayer
```

Now the 2 blockchains are able to communicate.

## Register interchain account

We want to register an interchain account for `quasar` at `osmosis`.

First we verify the IBC **connectionID** of the IBC connection that was established earlier during the setup between quasar and osmosis.

```bash
./demo quasarq ibc connection connections
```

In the response, a single connection should be present, with the ID `connection-0`.

If the connection_id field is an empty string it probably that the 2 chains are not connected yet, check the status of the relayer.

Now we can issue the ICA register account transaction.

```bash
./demo quasartx intergamm register-account connection-0 -y
```

If successful, the following tx events should be returned:

```yaml
logs:
- events:
  - attributes:
    - key: port_id
      value: icacontroller-quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec
    - key: channel_id
      value: channel-1
    - key: counterparty_port_id
      value: icahost
    - key: counterparty_channel_id
      value: ""
    - key: connection_id
      value: connection-0
    type: channel_open_init
  - attributes:
    - key: action
      value: register_account
    - key: module
      value: ibc_channel
    type: message
```

Now we can query the state of the registered account.

```bash
./demo quasarq intergamm interchain-account-from-address connection-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec
```

It should return the account address created on osmosis side:

```yaml
interchain_account_address: osmo1hphwfu3yjf82z8xpcl6e05gzkjwjmu8ts2m97mdk62feuqm77f2skm6qcy
```

Note that it might take a few seconds for the transaction on osmosis to be committed.

## Fund interchain account

Let's fund the newly registered ICA with the necessary balances to be able to create a pool later on.

```bash
./demo fund_host_ica
```

Check the balance:

```bash
curl http://localhost:1312/bank/balances/osmo1hphwfu3yjf82z8xpcl6e05gzkjwjmu8ts2m97mdk62feuqm77f2skm6qcy
```

## Create a pool on osmosis

First make sure there is indeed no pool yet on Osmosis:

```bash
./demo osmosisq gamm pools
```

```bash
./demo quasartx intergamm create-pool --data-file create-pool.json -y
```

Then check the newly created pool:

```bash
./demo osmosisq gamm pools
```

## IBC transfer

IBC transfers are also available, for instance, 10 uqsr can be sent across to Osmosis:

```bash
./demo quasartx ibc-transfer transfer transfer channel-0 osmo1hphwfu3yjf82z8xpcl6e05gzkjwjmu8ts2m97mdk62feuqm77f2skm6qcy 10uqsr
```

Check balance:

```bash
curl http://localhost:1312/bank/balances/osmo1hphwfu3yjf82z8xpcl6e05gzkjwjmu8ts2m97mdk62feuqm77f2skm6qcy
```
