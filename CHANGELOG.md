<!--
Guiding Principles:

Changelogs are for humans, not machines.
There should be an entry for every single version.
The same types of changes should be grouped.
Versions and sections should be linkable.
The latest version comes first.
The release date of each version is displayed.
Mention whether you follow Semantic Versioning.

Usage:

Change log entries are to be added to the Unreleased section under the
appropriate stanza (see below). Each entry should ideally include a tag and
the Github issue reference in the following format:

* (<tag>) \#<issue-number> message

The issue numbers will later be link-ified during the release process so you do
not have to worry about including a link manually, but you can if you wish.

Types of changes (Stanzas):

"Features" for new features.
"Improvements" for changes in existing functionality.
"Deprecated" for soon-to-be removed features.
"Bug Fixes" for any bug fixes.
"Client Breaking" for breaking CLI commands and REST routes used by end-users.
"API Breaking" for breaking exported APIs used by developers building on SDK.
"State Machine Breaking" for any changes that result in a different AppState
given same genesisState and txList.
Ref: https://keepachangelog.com/en/1.1.0/
-->
# Changelog


All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) after the `v1.0.1-rc-testnet` release.

## Unreleased


## v1.0.1-rc-testnet Update Wasm
* [#568](https://github.com/quasar-finance/quasar/pull/568) Adding the was capability support on chain for x/wasm osmosis,cosmwasm_1_1,cosmwasm_1_2,cosmwasm_1_4. and bump the wasm version


## v1.0.7-milktia-tia-patch
* [#560](https://github.com/quasar-finance/quasar/pull/560) Change the cl vault withdraw events to exist under their own event

## v1.0.6-milktia-tia-patch

* [#557](https://github.com/quasar-finance/quasar/pull/557) Paginate Cl vault distribute rewards

## v1.0.7-cl
* [#549](https://github.com/quasar-finance/quasar/pull/549) Validate and rebuildCL Tick caches of the CL vault
* [#555](https://github.com/quasar-finance/quasar/pull/555) Adds pool migration to migrate from to pools that only differ fee tier

## 1.0.6-cl
* [#508](https://github.com/quasar-finance/quasar/pull/508) Build the CL tick cache in CL vault instantiation
* [#512](https://github.com/quasar-finance/quasar/pull/512) Bump Osmosis-std to 0.19.2
* [#524](https://github.com/quasar-finance/quasar/pull/524) Bump interchain e2e tests and CL vault to Osmosis v20 
* [#493](https://github.com/quasar-finance/quasar/pull/493) Add an airdrop contract
* [#537](https://github.com/quasar-finance/quasar/pull/537) Change the CL vault initial mint to lower amounts


## 1.0.5-cl
* [#511](https://github.com/quasar-finance/quasar/pull/511) Add more balance queries to the CL vault
* [#513](https://github.com/quasar-finance/quasar/pull/513) Add swaps using less than 100% of the swap amount and twap to the CL vault

## 1.0.4-cl
* [#503](https://github.com/quasar-finance/quasar/pull/503) Set max slippage per rerange operation instead of in the vault config
* [#501](https://github.com/quasar-finance/quasar/pull/501) Use any free balance of the CL vault in reranging
* [#505](https://github.com/quasar-finance/quasar/pull/505) Add empty migrate entrypoint

## 1.0.3-cl [YANKED]

## 1.0.2-cl
* [#498](https://github.com/quasar-finance/quasar/pull/498) Add proptests, asserts and token sorting to the CL vault

## v1.0.1-cl
* [#488](https://github.com/quasar-finance/quasar/pull/448) Fix vault rewards claim query
* [#467](https://github.com/quasar-finance/quasar/pull/467) Add query flags to qVesting cmd
* [#447](https://github.com/quasar-finance/quasar/pull/447) Fix Apro withdraw slippage simulation
* [#469](https://github.com/quasar-finance/quasar/pull/469) Add queues to lp-strategy queries and purge failed_join_queue
* [#457](https://github.com/quasar-finance/quasar/pull/457) Bump the makefile versions
* [#471](https://github.com/quasar-finance/quasar/pull/471) Add PR and Issues template
* [#473](https://github.com/quasar-finance/quasar/pull/473) Expand CI
* [#456](https://github.com/quasar-finance/quasar/pull/456) Bump qMonitor Semver version
* [#466](https://github.com/quasar-finance/quasar/pull/466) Bump qMonitor protobufjs version
* [#477](https://github.com/quasar-finance/quasar/pull/477) Fix make lint
* [#429](https://github.com/quasar-finance/quasar/pull/429) Make execute_retry permissionless
* [#445](https://github.com/quasar-finance/quasar/pull/445) Add the initial CL vault


## v1.0.0
* [#308](https://github.com/quasar-finance/quasar/pull/308) Update the repository README
* [#304](https://github.com/quasar-finance/quasar/pull/304) Add multisign demo
* [#352](https://github.com/quasar-finance/quasar/pull/352) Add docker compose setup for local development environment
* [#362](https://github.com/quasar-finance/quasar/pull/362) Add interchain E2E test suite
* [#366](https://github.com/quasar-finance/quasar/pull/366) Limit CI jobs to one per ref
* [#398](https://github.com/quasar-finance/quasar/pull/398) add qTransfer E2E test cases and general suite improvement
* [#410](https://github.com/quasar-finance/quasar/pull/410) Parallize interchain E2E tests
* [#423](https://github.com/quasar-finance/quasar/pull/423) Fix account key name in E2E tests
* [#428](https://github.com/quasar-finance/quasar/pull/428) Add timeout flags to go test
* [#432](https://github.com/quasar-finance/quasar/pull/432) Bump osmosis to v16.1.0
* [#439](https://github.com/quasar-finance/quasar/pull/439) Add gauge testing demo
* [#440](https://github.com/quasar-finance/quasar/pull/440) Add osmosis gauge creation case to E2E
* [#436](https://github.com/quasar-finance/quasar/pull/436) Add token factory, qVesting and Authz
