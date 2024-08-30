###############################################################################
###                                   Tests                                 ###
###############################################################################

PACKAGES_UNIT=$(shell go list ./... )
PACKAGES_E2E=$(shell go list ./... | grep '/tests/e2e')
PACKAGES_SIM=$(shell go list ./... )
TEST_PACKAGES=./...
SIMAPP = ./app

SIM_NUM_BLOCKS ?= 500
SIM_BLOCK_SIZE ?= 200
SIM_COMMIT ?= true

test-help:
	@echo "test subcommands"
	@echo ""
	@echo "Usage:"
	@echo "  make test-[command]"
	@echo ""
	@echo "Available Commands:"
	@echo "  all                Run all tests"
	@echo "  benchmark          Run benchmark tests"
	@echo "  cover              Run coverage tests"
	@echo "  race               Run race tests"
	@echo "  sim-app            Run sim app tests"
	@echo "  sim-bench          Run sim benchmark tests"
	@echo "  sim-determinism    Run sim determinism tests"
	@echo "  sim-suite          Run sim suite tests"
	@echo "  unit               Run unit tests"

test: test-help

test-all: test-unit test-race test-sim-app

test-unit:
	@VERSION=$(VERSION) go test -mod=readonly -tags='ledger test_ledger_mock norace' $(PACKAGES_UNIT)

test-race:
	@VERSION=$(VERSION) go test -mod=readonly -race -tags='ledger test_ledger_mock' $(PACKAGES_UNIT)

test-cover:
	@VERSION=$(VERSION) go test -mod=readonly -timeout 30m -coverprofile=coverage.txt -tags='norace' -covermode=atomic $(PACKAGES_UNIT)

test-sim-suite:
	@VERSION=$(VERSION) go test -mod=readonly $(PACKAGES_SIM)

test-sim-app:
	@VERSION=$(VERSION) go test -mod=readonly -run ^TestFullAppSimulation -v $(PACKAGES_SIM)

test-sim-determinism:
	@VERSION=$(VERSION) go test -mod=readonly $(SIMAPP) -run TestAppStateDeterminism -Enabled=true \
	-NumBlocks=100 -BlockSize=200 -Commit=true -Period=0 -v -timeout 24h

test-sim-benchmark:
	@echo "Running application benchmark for numBlocks=$(SIM_NUM_BLOCKS), blockSize=$(SIM_BLOCK_SIZE). This may take awhile!"
	@VERSION=$(VERSION) go test -mod=readonly -benchmem -run=^$$ $(SIMAPP) -bench ^BenchmarkFullAppSimulation$$ -Enabled=true -NumBlocks=$(SIM_NUM_BLOCKS) -BlockSize=$(SIM_BLOCK_SIZE) -Commit=$(SIM_COMMIT) -timeout 24h

test-sim-profile:
	@echo "Running application benchmark for numBlocks=$(SIM_NUM_BLOCKS), blockSize=$(SIM_BLOCK_SIZE). This may take awhile!"
	@@VERSION=$(VERSION) go test -mod=readonly -benchmem -run=^$$ $(SIMAPP) -bench ^BenchmarkFullAppSimulation$$ \
		-Enabled=true -NumBlocks=$(SIM_NUM_BLOCKS) -BlockSize=$(SIM_BLOCK_SIZE) -Commit=$(SIM_COMMIT) -timeout 24h -cpuprofile cpu.out -memprofile mem.out
