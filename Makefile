#!/usr/bin/make -f

VERSION := $(shell echo $(shell git describe --tags) | sed 's/^v//')
COMMIT := $(shell git log -1 --format='%H')
LEDGER_ENABLED ?= true
SDK_PACK := $(shell go list -m github.com/cosmos/cosmos-sdk | sed  's/ /\@/g')
GO_VERSION := $(shell cat go.mod | grep -E 'go [0-9].[0-9]+' | cut -d ' ' -f 2)
DOCKER := $(shell which docker)
GOMOD := $(shell go list -m)
BUILDDIR ?= $(CURDIR)/build
MOCKSDIR = $(CURDIR)/testutil/mock

export GO111MODULE = on

## Helper function to show help with `make` or  `make help`

.DEFAULT_GOAL := help

HELP_FUN = \
	%help; while(<>){push@{$$help{$$2//'targets'}},[$$1,$$3] \
	if/^([\w-_]+)\s*:.*\#\#(?:@(\w+))?\s(.*)$$/}; \
	print"$$_:\n", map"  $$_->[0]".(" "x(40-length($$_->[0])))."$$_->[1]\n",\
	@{$$help{$$_}},"\n" for keys %help; \

help: ##@misc Show this help
	@echo "Usage: make [target] ...\n"
	@perl -e '$(HELP_FUN)' $(MAKEFILE_LIST)



# process build tags

build_tags = netgo
ifeq ($(LEDGER_ENABLED),true)
  ifeq ($(OS),Windows_NT)
    GCCEXE = $(shell where gcc.exe 2> NUL)
    ifeq ($(GCCEXE),)
      $(error gcc.exe not installed for ledger support, please install or set LEDGER_ENABLED=false)
    else
      build_tags += ledger
    endif
  else
    UNAME_S = $(shell uname -s)
    ifeq ($(UNAME_S),OpenBSD)
      $(warning OpenBSD detected, disabling ledger support (https://github.com/cosmos/cosmos-sdk/issues/1988))
    else
      GCC = $(shell command -v gcc 2> /dev/null)
      ifeq ($(GCC),)
        $(error gcc not installed for ledger support, please install or set LEDGER_ENABLED=false)
      else
        build_tags += ledger
      endif
    endif
  endif
endif

ifeq (cleveldb,$(findstring cleveldb,$(QUASAR_BUILD_OPTIONS)))
  build_tags += gcc
else ifeq (rocksdb,$(findstring rocksdb,$(QUASAR_BUILD_OPTIONS)))
  build_tags += gcc
endif
build_tags += $(BUILD_TAGS)
build_tags_debug += $(BUILD_TAGS)
build_tags := $(strip $(build_tags))

whitespace :=
whitespace += $(whitespace)
comma := ,
build_tags_comma_sep := $(subst $(whitespace),$(comma),$(build_tags))

# process linker flags

ldflags = -X github.com/cosmos/cosmos-sdk/version.Name=quasar \
		  -X github.com/cosmos/cosmos-sdk/version.AppName=quasarnoded \
		  -X github.com/cosmos/cosmos-sdk/version.Version=$(VERSION) \
		  -X github.com/cosmos/cosmos-sdk/version.Commit=$(COMMIT) \
		  -X "github.com/cosmos/cosmos-sdk/version.BuildTags=$(build_tags_comma_sep)"

ifeq (cleveldb,$(findstring cleveldb,$(QUASAR_BUILD_OPTIONS)))
  ldflags += -X github.com/cosmos/cosmos-sdk/types.DBBackend=cleveldb
else ifeq (rocksdb,$(findstring rocksdb,$(QUASAR_BUILD_OPTIONS)))
  ldflags += -X github.com/cosmos/cosmos-sdk/types.DBBackend=rocksdb
endif

ldflags-debug := $(ldflags)

ifeq (,$(findstring nostrip,$(QUASAR_BUILD_OPTIONS)))
  ldflags += -w -s
endif

ifeq ($(LINK_STATICALLY),true)
	ldflags += -linkmode=external -extldflags "-Wl,-z,muldefs -static"
	ldflags-debug += -linkmode=external -extldflags "-Wl,-z,muldefs -static"
endif

ldflags += $(LDFLAGS)
ldflags-debug += $(LDFLAGS)

ldflags := $(strip $(ldflags))

BUILD_FLAGS := -tags "$(build_tags)" -ldflags '$(ldflags)'
BUILD_FLAGS_DEBUG := -tags "$(build_tags_debug)" -ldflags '$(ldflags-debug)'
# check for nostrip option
ifeq (,$(findstring nostrip,$(QUASAR_BUILD_OPTIONS)))
  BUILD_FLAGS += -trimpath
endif

###############################################################################
###                                  Build                                  ###
###############################################################################

all: install lint test

BUILD_TARGETS := build install
#BUILD_TARGETS_DEBUG := build install
build: BUILD_ARGS=-o $(BUILDDIR)/

$(BUILD_TARGETS): go.sum $(BUILDDIR)/
	go $@ -mod=readonly $(BUILD_FLAGS) $(BUILD_ARGS) ./cmd/quasarnoded

$(BUILD_TARGETS_DEBUG): go.sum $(BUILDDIR)/
	go $@ -mod=readonly $(BUILD_FLAGS_DEBUG) -gcflags='all=-N -l' $(BUILD_ARGS) ./cmd/quasarnoded

$(BUILDDIR)/:
	mkdir -p $(BUILDDIR)/

# Cross-building for arm64 from amd64 (or viceversa) takes
# a lot of time due to QEMU virtualization but it's the only way (afaik)
# to get a statically linked binary with CosmWasm

build-reproducible: build-reproducible-amd64 build-reproducible-arm64

build-reproducible-amd64: $(BUILDDIR)/
	$(DOCKER) buildx create --name quasarbuilder || true
	$(DOCKER) buildx use quasarbuilder
	$(DOCKER) buildx build \
		--build-arg GO_VERSION=$(GO_VERSION) \
		--build-arg RUNNER_IMAGE=$(RUNNER_BASE_IMAGE_DISTROLESS) \
		--build-arg GIT_VERSION=$(VERSION) \
		--build-arg GIT_COMMIT=$(COMMIT) \
		--platform linux/amd64 \
		-t quasar-amd64 \
		--load \
		-f Dockerfile .
	$(DOCKER) rm -f quasarbinary || true
	$(DOCKER) create -ti --name quasarbinary quasar-amd64
	$(DOCKER) cp quasarbinary:/bin/quasarnoded $(BUILDDIR)/quasarnoded-linux-amd64
	$(DOCKER) rm -f quasarbinary

build-reproducible-arm64: $(BUILDDIR)/
	$(DOCKER) buildx create --name quasarbuilder || true
	$(DOCKER) buildx use quasarbuilder
	$(DOCKER) buildx build \
		--build-arg GO_VERSION=$(GO_VERSION) \
		--build-arg RUNNER_IMAGE=$(RUNNER_BASE_IMAGE_DISTROLESS) \
		--build-arg GIT_VERSION=$(VERSION) \
		--build-arg GIT_COMMIT=$(COMMIT) \
		--platform linux/arm64 \
		-t quasar-arm64 \
		--load \
		-f Dockerfile .
	$(DOCKER) rm -f quasarbinary || true
	$(DOCKER) create -ti --name quasarbinary quasar-arm64
	$(DOCKER) cp quasarbinary:/bin/quasarnoded $(BUILDDIR)/quasarnoded-linux-arm64
	$(DOCKER) rm -f quasarbinary

build-linux: go.sum
	LEDGER_ENABLED=false GOOS=linux GOARCH=amd64 $(MAKE) build

go.sum: go.mod
	@echo "--> Ensure dependencies have not been modified"
	@go mod verify

###############################################################################
###                         Proto & Mock Generation                         ###
###############################################################################

proto-all: proto-format proto-gen

proto-gen:
	@echo "Generating Protobuf files"
	@sh ./scripts/protocgen.sh

proto-doc:
	@echo "Generating Protoc docs"
	@sh ./scripts/generate-docs.sh

.PHONY: proto-gen proto-doc

mocks: $(MOCKSDIR)/ 
	mockgen -package=mock -destination=$(MOCKSDIR)/ibc_channel_mocks.go $(GOMOD)/x/qoracle/types ChannelKeeper
#	mockgen -package=mock -destination=$(MOCKSDIR)/ica_mocks.go $(GOMOD)/x/intergamm/types ICAControllerKeeper
#	mockgen -package=mock -destination=$(MOCKSDIR)/ibc_mocks.go $(GOMOD)/x/intergamm/types IBCTransferKeeper
	mockgen -package=mock -destination=$(MOCKSDIR)/ics4_wrapper_mocks.go $(GOMOD)/x/qoracle/types ICS4Wrapper
	mockgen -package=mock -destination=$(MOCKSDIR)/ibc_port_mocks.go $(GOMOD)/x/qoracle/types PortKeeper
#	mockgen -package=mock -destination=$(MOCKSDIR)/ibc_connection_mocks.go $(GOMOD)/x/intergamm/types ConnectionKeeper
#	mockgen -package=mock -destination=$(MOCKSDIR)/ibc_client_mocks.go $(GOMOD)/x/intergamm/types ClientKeeper

$(MOCKSDIR)/:
	mkdir -p $(MOCKSDIR)/

###############################################################################
###                           Tests & Simulation                            ###
###############################################################################

PACKAGES_UNIT=$(shell go list ./x/epochs/... ./x/qoracle/... | grep -E -v "simapp|e2e" | grep -E -v "x/qoracle/client/cli")
PACKAGES_E2E=$(shell go list ./... | grep '/e2e')
PACKAGES_SIM=$(shell go list ./... | grep '/tests/simulator')
TEST_PACKAGES=./...

test: test-unit test-build

test-all: check test-race test-cover

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
	@VERSION=$(VERSION) go test -mod=readonly -run ^TestAppStateDeterminism -v $(PACKAGES_SIM)

test-sim-bench:
	@VERSION=$(VERSION) go test -benchmem -run ^BenchmarkFullAppSimulation -bench ^BenchmarkFullAppSimulation -cpuprofile cpu.out $(PACKAGES_SIM)

benchmark:
	@go test -mod=readonly -bench=. $(PACKAGES_UNIT)

###############################################################################
###                         Smart Contracts                                 ###
###############################################################################

compile-wasm-artifacts:
	cd smart-contracts && \
	./compile_contracts.sh


###############################################################################
###                                Docker                                  ###
###############################################################################

RUNNER_BASE_IMAGE_DISTROLESS := gcr.io/distroless/static
RUNNER_BASE_IMAGE_ALPINE := alpine:3.16
RUNNER_BASE_IMAGE_NONROOT := gcr.io/distroless/static:nonroot

docker-build:
	@DOCKER_BUILDKIT=1 docker build \
		-t quasar:local \
		-t quasar:local-distroless \
		--build-arg GO_VERSION=$(GO_VERSION) \
		--build-arg RUNNER_IMAGE=$(RUNNER_BASE_IMAGE_DISTROLESS) \
		--build-arg GIT_VERSION=$(VERSION) \
		--build-arg GIT_COMMIT=$(COMMIT) \
		-f Dockerfile .

docker-build-distroless: docker-build

docker-build-alpine:
	@DOCKER_BUILDKIT=1 docker build \
		-t quasar:local-alpine \
		--build-arg GO_VERSION=$(GO_VERSION) \
		--build-arg RUNNER_IMAGE=$(RUNNER_BASE_IMAGE_ALPINE) \
		--build-arg GIT_VERSION=$(VERSION) \
		--build-arg GIT_COMMIT=$(COMMIT) \
		-f Dockerfile .

docker-build-nonroot:
	@DOCKER_BUILDKIT=1 docker build \
		-t quasar:local-nonroot \
		--build-arg GO_VERSION=$(GO_VERSION) \
		--build-arg RUNNER_IMAGE=$(RUNNER_BASE_IMAGE_NONROOT) \
		--build-arg GIT_VERSION=$(VERSION) \
		--build-arg GIT_COMMIT=$(COMMIT) \
		-f Dockerfile .


docker-compose-up: ##@docker Run (and build if needed) env in docker compose. Attach if running in background.
	@echo "Launching local env with docker-compose"
	DOCKER_BUILDKIT=1 COMPOSE_DOCKER_CLI_BUILD=1 docker compose -f docker-compose.yml up

docker-compose-up-detached: ##@docker Run (and build if needed) env in docker compose and detach console
	@echo "Launching local env with docker-compose"
	DOCKER_BUILDKIT=1 COMPOSE_DOCKER_CLI_BUILD=1 docker compose -f docker-compose.yml up -d

docker-compose-up-recreate: ##@docker DESTROY env containers and respawn them
	@echo "Recreate local env (will destroy application state)"
	DOCKER_BUILDKIT=1 COMPOSE_DOCKER_CLI_BUILD=1 docker compose -f docker-compose.yml up --force-recreate

docker-compose-up-recreate-rebuild: ##@docker DESTROY env containers and respawn them rebuilding image
	@echo "Rebuild image and recreate local env (will destroy application state)"
	DOCKER_BUILDKIT=1 COMPOSE_DOCKER_CLI_BUILD=1 docker compose -f docker-compose.yml up --force-recreate --build

docker-compose-stop: ##@docker Stop containers running in the background
	@echo "Stop docker containers and remove them (will destroy application state)"
	DOCKER_BUILDKIT=1 COMPOSE_DOCKER_CLI_BUILD=1 docker compose -f docker-compose.yml stop

docker-attach-quasar: ##@docker Connect to a terminal prompt in quasar node container
	@echo "Connecting to quasar docker container"
	docker exec -it quasar-quasar-1 /bin/bash

docker-attach: ##@docker Connect to a terminal prompt in desired node (ARGS="quasar" make docker-attach)
	@echo "Connecting to quasar docker container"
	docker exec -it $(ARGS) /bin/sh




###############################################################################
###                                Linting                                  ###
###############################################################################

lint:
	@echo "--> Running linter"
	@go run github.com/golangci/golangci-lint/cmd/golangci-lint run --timeout=10m

.PHONY: all build-linux install format lint build \
	test test-all test-build test-cover test-unit test-race benchmark
