# End-to-end Tests

## Structure

### `e2e` Package

The `e2e` package defines an integration testing suite used for full
end-to-end testing functionality. This package is decoupled from
depending on the Quasar codebase and hence has it's own `go.mod`. The decoupling between chain initialization and start-up allows to
minimize the differences between our test suite and the production
environment.
It initializes the chains for testing via Docker. As a result, the test suite may provide the desired
Quasar version to Docker containers during the initialization. This
design allows for the opportunity of testing chain upgrades in the
future by providing an older Quasar version (Like the latest stable version) to the container,
performing the chain upgrade, and running the latest test suite.

## How It Works

Conceptually, we can split the e2e setup into 2 parts:

1. Chain Initialization

    The chain can either be initialized off of the current branch, or off the prior mainnet releases. This depends on
    On what branch you build quasar docker image. Currently we always use `quasar:local` image as it's hard-coded
    in the `config` package.

    Developers can initialize the setup using the `E2ETestSuiteBuilder` function as follows:

    ```go
    func TestIntergammTestSuite(t *testing.T) {
	    b := testsuite.NewE2ETestSuiteBuilder(t) // Create a new builder for every suite
	    b.UseCosmos() // Mark the builder to init a cosmos chain for test
	    b.UseOsmosis() // Mark the builder to init an osmosis chain
	    b.Link(b.Quasar(), b.Cosmos(), testconfig.  Quasar2CosmosPath) // Create IBC connection between quasar and cosmos chains
	    b.Link(b.Cosmos(), b.Osmosis(), testconfig. Cosmos2OsmosisPath) // Create IBC connection between cosmos and osmosis chains
	    b.Link(b.Quasar(), b.Osmosis(), testconfig. Quasar2OsmosisPath) // Create IBC connection between quasar and osmosis chains
	    b.AutomatedRelay() // Tell builder to spin up a relayer worker to automatically relay packets instead of doing it manually in tests
    
	    s := &IntergammTestSuite{E2ETestSuite: b.Build()} // Build initializes all the chains, ibc connections and relayer then returns an E2ETestSuite
	    suite.Run(t, s) 
    }
    ```

2. Running the tests

    Each `TestSuite` should embed the `E2ETestSuite` struct to be able to interact with chains and use helper functions. Note that as long as each test creates and uses it's own accounts they can be run in parallel using the flag function `t.Parallel()` at the beginning of the fuction.

## `suite` Package

The `suite` defines the testing suite and contains the
core bootstrapping logic that creates a testing environment via `ibctest` framework. This mechanism written as a **Builder** pattern so that, developers can customize testing suites for every scenario. For more information see [builder.go](suite/builder.go)
It also provides bunch of helper functions to interact with validators and chains like executing txs, queries and ...

## `config` Package

This package contains all the information necessary for running chain validators like docker image and tag for gaia, osmosis, bandchain and relayer as well as constants necessary for running the test setup like how many validators each test will have and ...
**Note that at the moment, testing network for each chain is created with only 1 validator.**

## How to run

Run the following commands at the root of repository to build the Quasar local image 
(You can either do this at this branch or any other brach that you want to run the e2e tests against)
and run the e2e tests:
```sh
    make docker-build
    make test-e2e
```

### Common Problems

Please note that if the tests are stopped mid-way, the e2e framework might fail to start again due to duplicated containers. Make sure that
containers are removed before running the tests again: `docker containers rm -f $(docker containers ls -a -q)`.

Additionally, Docker networks do not get auto-removed. Therefore, you can manually remove them by running `docker network prune`.

As the nature of e2e test is not deterministic (It depends on time and block heights). There might be some failures sometimes when running the tests. Some cases might be because of relayer failure to create connections between chains or send packets in the expected timestamp. We should investigate and find a way to make this issue less likely to happen in future.