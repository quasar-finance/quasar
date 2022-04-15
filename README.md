# Quasar

This repository is for building yield aggregator and capital management cosmos chain project.

Quasar is highly focused in utlising the latest ibc features of cosmos.
IBC features that we are developing on:
1. Intechain accounts.
2. Multihop ibc token transfer
3. IBC cross chain Lping via ICA.

Quasar initial MVP includes Lping on osmosis dex via IBC messages/ICA. Quasar will provide the one stop for end users to come with their IBC enabled tokens for investment in to the cosmos ecosystem. And that too via just one place.
Quasar is implemeting strategies to aggregate users deposited fund to optimally Lping on osmosis dex.

## Notes

1. Quasar is under rapid development phase, and current code base have a lot of testing, experimental redundant code, and info level logs; Which will be removed from the code base in near future based on our testing, and experiments.
2. Current state of the repo ca not be used directly for a live production as there are many parallel moving developments in different branches.
3. Task management is happening via clickup project mangement internally. So we will not create any github task, till it is in good situation.

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
make docs_gen
```

Then serve it locally using docker:

```bash
make docs_serve
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
