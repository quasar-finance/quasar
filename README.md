# Quasar

This is the official Quasar Labs repository. Quasar is building decentralized vaults for creating custom and sovereign investment vehicles in the Cosmos ecosystem and beyond.

Quasar is focused in utilizing the latest and contributing to building IBC features including:
IBC features that we are developing on:
1. Interchain Accounts (ICA).
2. Multihop ibc token transfer
3. Interchain Queriees (ICQ).

Quasar is working hard to simplfy and add ease to collaborative investment with digital assets. We are creating a decentralized platform for creating custom, soverign vaults that can be molded into any imaginable investment financial instrument from ETFs, mutual fund, SPAC, or whatever. The galaxy is the limit (or maybe the gas fee is 😅)  Our flagship product is a vault that implements optimal LPing into pools on Osmosis DEX.

## DISCLAIMER
The current codebase is experimental and undergoing testing and auditing - no code is guarunteed to be ready for production environments at this time. 

## Quasar Node

**quasarnode** is a blockchain built using Cosmos SDK and Tendermint and created with [Ignite](https://ignite.com) (formerly startport).

### Get started

```
ignite chain serve
```

`serve` command installs dependencies, builds, initializes, and starts your blockchain in development.

#### Configure

Your blockchain in development can be configured with `config.yml`. To learn more, see the [Ignite docs](https://docs.ignite.com/).

#### Generate docs

All the gRPC endpoints documentation can be generated as a swagger file and served with `swagger-ui`.

First generate the swagger file:

```bash
make docs-gen
```

Then serve it locally using docker:

```bash
make docs-serve
```

#### Web Frontend

Ignite has scaffolded a Vue.js-based web app in the `vue` directory. Run the following commands to install dependencies and start the app:

```
cd vue
npm install
npm run serve
```

The frontend app is built using the `@starport/vue` and `@starport/vuex` packages. For details, see the [monorepo for Ignite front-end development](https://github.com/tendermint/vue).

### Release

To release a new version of your blockchain, create and push a new tag with `v` prefix. A new draft release with the configured targets will be created.

```
git tag v0.1
git push origin v0.1
```

After a draft release is created, make your final changes from the release page and publish it.

#### Install

To install the latest version of your blockchain node's binary, execute the following command on your machine:

```bash
curl https://get.ignite.com/cli! | bash
```

`abag/quasarnode` should match the `username` and `repo_name` of the Github repository to which the source code was pushed. Learn more about [the install process](https://github.com/ignite-hq/installer).

### Important node commands

```bash
quasarnoded keys list --home ~/.quasarnode/

quasarnoded q qbank user-denom-deposit cosmos146c4e9w55su0czahwxrz8v660p0c2s93cmam6w uqsar

quasarnoded tx qbank request-deposit SENIOR VAULT-01 30000uqsar Days_7 --from alice

quasarnoded q qbank user-denom-deposit cosmos146c4e9w55su0czahwxrz8v660p0c2s93cmam6w uqsar

quasarnoded tx qbank request-withdraw "SENIOR" "VAULT-01" 30000 "uqsar" --from alice

quasarnoded q qbank user-denom-deposit cosmos146c4e9w55su0czahwxrz8v660p0c2s93cmam6w uqsar
```

## Learn more

- [Ignite](https://ignite.com)
- [Tutorials](https://docs.ignite.com/guide)
- [Ignite docs](https://docs.ignite.com)
- [Cosmos SDK docs](https://docs.cosmos.network)
- [Developer Chat](https://discord.gg/H6wGTY8sxw)
