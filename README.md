# Quasar

This is the official Quasar Labs repository. Quasar is building decentralized vaults for creating custom and sovereign investment vehicles in the Cosmos ecosystem and beyond.

Quasar is focused in utilizing the latest and contributing to building IBC features including:
IBC features that we are developing on:
1. Interchain Accounts (ICA).
2. Multihop ibc token transfer (packet forwarder).
3. Interchain Queries (ICQ).

Quasar is working hard to simplfy and add ease to collaborative investment with digital assets. We are creating a decentralized platform for creating custom, soverign vaults that can be molded into any imaginable investment financial instrument from ETFs, mutual fund, SPAC, or whatever. The galaxy is the limit (or maybe the gas fee is 😅)  Our flagship product is a vault that implements optimal LPing into pools on Osmosis DEX.

## DISCLAIMER

The current codebase is experimental and undergoing testing and auditing - no code is guarunteed to be ready for production environments at this time.

## Quasar Node

**quasarnode** is a blockchain built using Cosmos SDK and Tendermint and created with [Ignite](https://ignite.com) (formerly startport).

### Run Quasar

```bash
make run
```

This will use `ignite` to generate any code from protos, compile the quasar sources and an run the chain locally.

To install the latest version of `ignite`, execute the following command on your machine, or consult the [official documentation](https://github.com/ignite-hq/installer):

```bash
curl https://get.ignite.com/cli! | bash
```

#### Configure

Your blockchain in development can be configured with `config.yml`.
To learn more, see the [Ignite docs](https://docs.ignite.com/).

#### Important node commands

```bash
quasarnoded keys list --home ~/.quasarnode/

quasarnoded q qbank user-denom-deposit cosmos146c4e9w55su0czahwxrz8v660p0c2s93cmam6w uqsar

quasarnoded tx qbank request-deposit SENIOR VAULT-01 30000uqsar Days_7 --from alice

quasarnoded q qbank user-denom-deposit cosmos146c4e9w55su0czahwxrz8v660p0c2s93cmam6w uqsar

quasarnoded tx qbank request-withdraw "SENIOR" "VAULT-01" 30000 "uqsar" --from alice

quasarnoded q qbank user-denom-deposit cosmos146c4e9w55su0czahwxrz8v660p0c2s93cmam6w uqsar
```

### Release

To release a new version of your blockchain, create and push a new tag with `v` prefix. A new draft release with the configured targets will be created.

```
git tag v0.1
git push origin v0.1
```

After a draft release is created, make your final changes from the release page and publish it.

#### Release artifacts

To generate the release artifacts, run:

```bash
make build_artifacts
```

This will generate the quasar artifacts in the `build/` folder.

## Documentation

All the gRPC endpoints documentation can be generated as a swagger file and served with `swagger-ui`.

First generate the swagger file:

```bash
make docs-gen
```

Then serve it locally using docker:

```bash
make docs-serve
```

## Quasar Local Testnet (localnet)

To run a small network of Quasar nodes locally, first generate their respective configuration:

### Setup

```bash
make build_dev # make sure we have a quasar executable available first
scripts/localnet init_default_cluster
```

This will create a set of configuration files per node, under `run/localnet`.

By default, there is a total of 4 nodes: `node1`, `node2`, `node3`, `node4`

The network is now ready to be started:

### Start

```bash
scripts/localnet start_all
```

To see if the nodes are running:

```bash
scripts/localnet status_all
```

Once the network is running, you can see the logging per node, this way:

```bash
scripts/localnet node 0 log
```

Here, `0` is the node identifier, you can use, `1`, `2`, or `3` to see the logging of the other nodes.

### Commands

You can also issue query or tx commands against each node:

```bash
scripts/localnet node 0 cmd tx bank send main quasar1khfcjt5w0dfjgkcudlrnnun2rtq359ulrgv7gw 1000uqsar
```

This will send a bank transfer from `node0`'s `main` address, to another quasar address `quasar1khfcjt5w0dfjgkcudlrnnun2rtq359ulrgv7gw`.

You can control the balance after the transfer:

```bash
curl http://localhost:1300/bank/balances/quasar1khfcjt5w0dfjgkcudlrnnun2rtq359ulrgv7gw
```

Note that the API for `node0` is available at `localhost:1300`, while the API for `node1` is at `localhost:1301`, and so forth.

### Upgrade

TODO

### Stop

Last but not least, to stop the network:

```bash
scripts/localnet stop_all
```

## Learn more

- [Ignite](https://ignite.com)
- [Tutorials](https://docs.ignite.com/guide)
- [Ignite docs](https://docs.ignite.com)
- [Cosmos SDK docs](https://docs.cosmos.network)
- [Developer Chat](https://discord.gg/H6wGTY8sxw)
